

use array_math::{ArrayOps};
use num::{traits::FloatConst, Float};

use crate::util;

use super::{WindowGen, WindowRange};

pub struct Kaiser<T>
where
    T: Float
{
    pub beta: T
}

impl<T, const N: usize> WindowGen<T, [T; N], ()> for Kaiser<T>
where
    T: Float + FloatConst
{
    type Output = [T; N];

    fn window_gen(&self, (): (), r: WindowRange) -> Self::Output
    {
        if N <= 1
        {
            return [T::one(); N]
        }
        
        let m = match r
        {
            WindowRange::Symmetric => N - 1,
            WindowRange::Periodic => N,
        };

        //let i0 = |x| T::from(scilib::math::bessel::i_nu(0.0, NumCast::from(x).unwrap()).re).unwrap();
        let i0 = |x| util::i0(x);

        let l = T::from(m).unwrap();
        let one = T::one();
        let two = one + one;
        let d = i0(self.beta);
        ArrayOps::fill(|i| {
            let z = two*T::from(i).unwrap()/l - one;
            i0(self.beta*(one - z*z).sqrt())/d
        })
    }
}
impl<T> WindowGen<T, Vec<T>, usize> for Kaiser<T>
where
    T: Float + FloatConst
{
    type Output = Vec<T>;

    fn window_gen(&self, n: usize, r: WindowRange) -> Self::Output
    {
        if n <= 1
        {
            return vec![T::one(); n]
        }
        
        let m = match r
        {
            WindowRange::Symmetric => n - 1,
            WindowRange::Periodic => n,
        };

        //let i0 = |x| T::from(scilib::math::bessel::i_nu(0.0, NumCast::from(x).unwrap()).re).unwrap();
        let i0 = |x| util::i0(x);

        let l = T::from(m).unwrap();
        let one = T::one();
        let two = one + one;
        let d = i0(self.beta);
        (0..n).map(|i| {
            let z = two*T::from(i).unwrap()/l - one;
            i0(self.beta*(one - z*z).sqrt())/d
        }).collect()
    }
}

#[cfg(test)]
mod test
{
    use core::f64::consts::{PI, TAU};

    use array_math::ArrayOps;
    use linspace::LinspaceArray;

    use crate::{plot, window::{WindowGen, WindowRange}, FreqZ, Tf};

    use super::Kaiser;

    #[test]
    fn test()
    {
        const N: usize = 1024;
        let w: [_; N/2] = Kaiser {beta: PI*3.0}.window_gen((), WindowRange::Symmetric);
        let n = (0.0..1.0).linspace_array();

        plot::plot_curves("g(n/N)", "plots/windows/g_n_kaiser.png", [&n.zip(w)]).unwrap();

        let (mut w_f, mut omega): ([_; N], _) = Tf::new(w, ()).freqz(());
        omega.map_assign(|omega| (omega + PI) % TAU - PI);
        w_f.rotate_right(N/2);
        omega.rotate_right(N/2);
        
        plot::plot_curves("G(e^jw)", "plots/windows/g_f_kaiser.png", [&omega.zip(w_f.map(|w| 20.0*w.norm().log10()))]).unwrap();
    }
}