use core::{fmt::Debug, ops::{AddAssign, DivAssign, MulAssign}};


use ndarray_linalg::Lapack;
use num::{traits::FloatConst, Complex, Float, NumCast};



use super::{WindowGen, WindowRange};

pub struct Ultraspherical<T>
where
    T: Float
{
    pub mu: T,
    pub xmu: T
}

impl<T, const N: usize> WindowGen<T, [T; N], ()> for Ultraspherical<T>
where
    T: Float + FloatConst + AddAssign + MulAssign + DivAssign + Debug + Lapack<Complex = Complex<T>>,
    Complex<T>: AddAssign + MulAssign
{
    type Output = Option<[T; N]>;

    fn window_gen(&self, (): (), r: WindowRange) -> Self::Output
    {
        if N <= 1
        {
            return Some([T::one(); N])
        }

        let nm1 = match r
        {
            WindowRange::Symmetric => N - 1,
            WindowRange::Periodic => N,
        };

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
            return None
        }

        let mut w = [zero; N];
        let mut l = 0;
        let m = (nm1 + 2)/2;
        let idivs = m - 1;
        let c = one - (self.xmu*self.xmu).recip();
        let mut v = [zero; N];
        if N > 1
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
                        v[*j] += vp;
                        vp = t;
                        t = s;
                        u *= c*(nfm1 + one - <T as NumCast>::from(i).unwrap() - <T as NumCast>::from(*j).unwrap())*w[idivs + *j];
                        s += v[*j]*u;
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

            u *= (self.mu + nfm1 - one - <T as NumCast>::from(i).unwrap())/(nfm1 - one - <T as NumCast>::from(i).unwrap());
            w[i] *= u;
        }
        for i in 0..m
        {
            w[nm1 - i] = w[i];
        }

        Some(w)
    }
}
impl<T> WindowGen<T, Vec<T>, usize> for Ultraspherical<T>
where
    T: Float + FloatConst + AddAssign + MulAssign + DivAssign
{
    type Output = Option<Vec<T>>;

    fn window_gen(&self, n: usize, r: WindowRange) -> Self::Output
    {
        if n <= 1
        {
            return Some(vec![T::one(); n])
        }

        let nm1 = match r
        {
            WindowRange::Symmetric => n - 1,
            WindowRange::Periodic => n,
        };

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
            return None
        }

        let mut w = vec![zero; n];
        let mut l = 0;
        let m = (nm1 + 2)/2;
        let idivs = m - 1;
        let c = one - (self.xmu*self.xmu).recip();
        let mut v = vec![zero; n];
        if n > 1
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
                        v[*j] += vp;
                        vp = t;
                        t = s;
                        u *= c*(nfm1 + one - <T as NumCast>::from(i).unwrap() - <T as NumCast>::from(*j).unwrap())*w[idivs + *j];
                        s += v[*j]*u;
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

            u *= (self.mu + nfm1 - one - <T as NumCast>::from(i).unwrap())/(nfm1 - one - <T as NumCast>::from(i).unwrap());
            w[i] *= u;
        }
        for i in 0..m
        {
            w[nm1 - i] = w[i];
        }

        Some(w)
    }
}

#[cfg(test)]
mod test
{
    use core::f64::consts::{PI, TAU};

    use array_math::ArrayOps;
    use linspace::LinspaceArray;

    use crate::{plot, window::{WindowGen, WindowRange}, FreqZ, Tf};

    use super::Ultraspherical;

    #[test]
    fn test()
    {
        const N: usize = 1024;
        let w: [_; N/2] = Ultraspherical {mu: -0.5, xmu: 1.2}
            .window_gen((), WindowRange::Symmetric)
            .unwrap();
        let n = (0.0..1.0).linspace_array();

        plot::plot_curves("g(n/N)", "plots/windows/g_n_ultraspherical.png", [&n.zip(w)]).unwrap();

        let (mut w_f, mut omega): ([_; N], _) = Tf::new(w, ()).freqz(());
        omega.map_assign(|omega| (omega + PI) % TAU - PI);
        w_f.rotate_right(N/2);
        omega.rotate_right(N/2);
        
        plot::plot_curves("G(e^jw)", "plots/windows/g_f_ultraspherical.png", [&omega.zip(w_f.map(|w| 20.0*w.norm().log10()))]).unwrap();
    }
}