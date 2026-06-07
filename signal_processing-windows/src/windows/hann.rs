

use array_math::ArrayOps;
use num::{traits::FloatConst, Float};

use crate::generators::window::{WindowGen, WindowRange};

#[doc(alias = "Hanning")]
pub struct Hann;

impl<T, const N: usize> WindowGen<T, [T; N], ()> for Hann
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

        ArrayOps::fill(|i| {
            let s = (T::PI()*T::from(i).unwrap()/T::from(m).unwrap()).sin();
            s*s
        })
    }
}
impl<T> WindowGen<T, Vec<T>, usize> for Hann
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

        (0..n).map(|i| {
            let s = (T::PI()*T::from(i).unwrap()/T::from(m).unwrap()).sin();
            s*s
        }).collect()
    }
}

#[cfg(test)]
mod test
{
    use core::f64::consts::{PI, TAU};

    use array_math::ArrayOps;
    use linspace::LinspaceArray;

    use crate::{plot, gen::window::{WindowGen, WindowRange}, analysis::FreqZ, systems::Tf};

    use super::Hann;

    #[test]
    fn test()
    {
        const N: usize = 1024;
        let w: [_; N/2] = Hann.window_gen((), WindowRange::Symmetric);
        let n = (0.0..1.0).linspace_array();

        plot::plot_curves("g(n/N)", "plots/windows/g_n_hann.png", [&n.zip(w)]).unwrap();

        let (mut w_f, mut omega): ([_; N], _) = Tf::new(w, ()).freqz((), false);
        omega.map_assign(|omega| (omega + PI) % TAU - PI);
        w_f.rotate_right(N/2);
        omega.rotate_right(N/2);
        
        plot::plot_curves("G(e^jw)", "plots/windows/g_f_hann.png", [&omega.zip(w_f.map(|w| 20.0*w.norm().log10()))]).unwrap();
    }
}