

use array_math::ArrayOps;
use num::{traits::FloatConst, Float};

use super::{WindowGen, WindowRange};

#[doc(alias = "Bartlett")]
pub struct Triangular;

impl<T, const N: usize> WindowGen<T, [T; N], ()> for Triangular
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

        let one = T::one();
        let two = one + one;
        ArrayOps::fill(|i| {
            one - ((T::from(i).unwrap()*two - T::from(N - 1).unwrap())/T::from(m).unwrap()).abs()
        })
    }
}
impl<T> WindowGen<T, Vec<T>, usize> for Triangular
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

        let one = T::one();
        let two = one + one;
        (0..n).map(|i| {
            one - ((T::from(i).unwrap()*two - T::from(n - 1).unwrap())/T::from(m).unwrap()).abs()
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

    use super::Triangular;

    #[test]
    fn test()
    {
        const N: usize = 1024;
        let w: [_; N/2] = Triangular.window_gen((), WindowRange::Symmetric);
        let n = (0.0..1.0).linspace_array();

        plot::plot_curves("g(n/N)", "plots/windows/g_n_triangular.png", [&n.zip(w)]).unwrap();

        let (mut w_f, mut omega): ([_; N], _) = Tf::new(w, ()).freqz((), false);
        omega.map_assign(|omega| (omega + PI) % TAU - PI);
        w_f.rotate_right(N/2);
        omega.rotate_right(N/2);
        
        plot::plot_curves("G(e^jw)", "plots/windows/g_f_triangular.png", [&omega.zip(w_f.map(|w| 20.0*w.norm().log10()))]).unwrap();
    }
}