use array_trait::length::Length;
use bulks::CollectNearest;
use num_traits::{Float, FloatConst, NumCast, Zero};

use crate::{Shape, WindowFn};

#[derive(Clone, Copy)]
pub struct Ultraspherical<T>
where
    T: Float
{
    pub mu: T,
    pub xmu: T
}

impl<L, T> WindowFn<L> for Ultraspherical<T>
where
    L: Length<Elem = T> + ?Sized,
    T: Float + FloatConst
{
    type Functor = impl Fn(usize) -> T;

    fn window_fn(self, len: L::Value, range: Shape) -> Self::Functor
    {
        let nm1 = range.window_len(len);

        let zero = T::zero();
        let one = T::one();
        let two = one + one;
        let nfm1 = <T as NumCast>::from(nm1).unwrap();

        let xmu_min = <T as NumCast>::from(0.99).unwrap()*(one - T::epsilon());
        let beta_max = <T as NumCast>::from(12u8).unwrap()*(one + T::epsilon());

        let bad = self.xmu < xmu_min || (self.mu.is_zero() && self.xmu.is_one()) ||
            ((nfm1 + one) > beta_max*two && self.xmu * Float::cos(T::PI()*beta_max/(nfm1 + one)) > one);
        if bad
        {
            //panic!("Invalid parameters");
            //return None
        }

        let mut w = bulks::repeat_n(zero, len)
            .collect_nearest();
        let mut l = 0;
        let m = (nm1 + 2)/2;
        let idivs = m - 1;
        let c = one - (self.xmu*self.xmu).recip();
        let mut v = bulks::repeat_n(zero, len)
            .collect_nearest();
        if !m.is_zero()
        {
            for i in 0..m
            {
                let mut vp = v[0];
                w[idivs + i] = <T as NumCast>::from(i).unwrap().recip();
                let mut s = if i != 0 {(v[0] + v[1])*self.mu*(w[idivs + i])} else {one};
                v[0] = s;
                let mut met = false;
                let mut j = 1;
                let mut u = one;
                loop
                {
                    let mut f = |j: &mut usize, met: &mut bool| {
                        let mut t = v[*j];
                        v[*j] = v[*j] + vp;
                        vp = t;
                        t = s;
                        u = u*c*(nfm1 + one - <T as NumCast>::from(i).unwrap() - <T as NumCast>::from(*j).unwrap())*w[idivs + *j];
                        s = s + v[*j]*u;
                        *met = !s.is_zero() && s == t;
                        *j += 1;
                    };

                    let k = (((1 + l as i32 - j as i32) & !7) + j as i32) as usize;
                    while j < k && !met
                    {
                        f(&mut j, &mut met);
                        f(&mut j, &mut met);
                        f(&mut j, &mut met);
                        f(&mut j, &mut met);
                        f(&mut j, &mut met);
                        f(&mut j, &mut met);
                        f(&mut j, &mut met);
                        f(&mut j, &mut met);
                    }
                    while j <= l && !met
                    {
                        f(&mut j, &mut met);
                    }
                    if met || j > i
                    {
                        break
                    }

                    l += 1;
                    v[l] = vp*<T as NumCast>::from(i as i128 - l as i128).unwrap()/(self.mu + <T as NumCast>::from(l).unwrap() - one)
                }
                w[i] = s/(nfm1 - <T as NumCast>::from(i).unwrap());
                l = j - (j <= i) as usize;
            }
        }
        else
        {
            w[0] = one;
        }

        let mut i = m - 1;
        let mut u = w[i].recip();
        w[i] = one;
        while i > 0
        {
            i -= 1;

            u = u*(self.mu + nfm1 - one - <T as NumCast>::from(i).unwrap())/(nfm1 - one - <T as NumCast>::from(i).unwrap());
            w[i] = w[i]*u;
        }
        for i in 0..m
        {
            w[nm1 - i] = w[i];
        }

        move |i| {
            if m.is_zero()
            {
                return T::one()
            }

            w[i]
        }
    }
}

#[cfg(test)]
mod test
{
    use crate::{tests, windows::Ultraspherical};

    #[test]
    fn test()
    {
        tests::plot_window(Ultraspherical {
            mu: -0.5,
            xmu: 1.2
        })
    }
}