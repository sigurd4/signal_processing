


use num::{traits::FloatConst, Float};

use crate::generators::window::{WindowGen, WindowRange};

pub struct Boxcar;

impl<T, const N: usize> WindowGen<T, [T; N], ()> for Boxcar
where
    T: Float + FloatConst
{
    type Output = [T; N];

    fn window_gen(&self, (): (), _: WindowRange) -> Self::Output
    {
        [T::one(); N]
    }
}
impl<T> WindowGen<T, Vec<T>, usize> for Boxcar
where
    T: Float + FloatConst
{
    type Output = Vec<T>;

    fn window_gen(&self, n: usize, _: WindowRange) -> Self::Output
    {
        vec![T::one(); n]
    }
}

#[cfg(test)]
mod test
{
    use core::f64::consts::{PI, TAU};

    use array_math::ArrayOps;
    use linspace::LinspaceArray;

    use crate::{plot, gen::window::{WindowGen, WindowRange}, analysis::FreqZ, systems::Tf};

    use super::Boxcar;

    #[test]
    fn test()
    {
        const N: usize = 1024;
        let w: [_; N/2] = Boxcar.window_gen((), WindowRange::Symmetric);
        let n = (0.0..1.0).linspace_array();

        plot::plot_curves("g(n/N)", "plots/windows/g_n_boxcar.png", [&n.zip(w)]).unwrap();

        let (mut w_f, mut omega): ([_; N], _) = Tf::new(w, ()).freqz((), false);
        omega.map_assign(|omega| (omega + PI) % TAU - PI);
        w_f.rotate_right(N/2);
        omega.rotate_right(N/2);
        
        plot::plot_curves("G(e^jw)", "plots/windows/g_f_boxcar.png", [&omega.zip(w_f.map(|w| 20.0*w.norm().log10()))]).unwrap();
    }
}