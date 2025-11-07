use core::f64::consts::TAU;

use array_trait::length::{self, LengthValue};
use bulks::Bulk;
use num::{Float, complex::ComplexFloat};

use crate::generators::window::{WindowGen, WindowRange};

pub struct Barthann;

impl<T, N> WindowGen<T, N> for Barthann
where
    T: ComplexFloat,
    N: LengthValue
{
    type Output = impl Bulk<Item = T>;

    fn window_gen(&self, numtaps: N, range: WindowRange) -> Self::Output
    {
        let mut i = 0;
        let n = length::value::len(numtaps);
        let m = match range
        {
            WindowRange::Symmetric => n - 1,
            WindowRange::Periodic => n,
        };
        bulks::repeat_n_with(|| {
            let p = i as f64/m as f64 - 0.5;
            let g = 0.62 - 0.48*p.abs() + 0.38*(TAU*p).cos();
            i += 1;
            T::from(g).unwrap()
        }, numtaps)
    }
}

#[cfg(test)]
mod test
{
    use core::f64::consts::{PI, TAU};

    use linspace::Linspace;

    use crate::{plot, generators::window::{WindowGen, WindowRange}, analysis::FreqZ, systems::Tf};

    use super::Barthann;

    #[test]
    fn test()
    {
        const N: usize = 1024;
        let w: [_; N/2] = Barthann.window_gen((), WindowRange::Symmetric);
        let n = (0.0..1.0).linspace_array();

        plot::plot_curves("g(n/N)", "plots/windows/g_n_barthann.png", [&n.zip(w)]).unwrap();

        let (mut w_f, mut omega): ([_; N], _) = Tf::new(w, ()).freqz((), false);
        omega.map_assign(|omega| (omega + PI) % TAU - PI);
        w_f.rotate_right(N/2);
        omega.rotate_right(N/2);
        
        plot::plot_curves("G(e^jw)", "plots/windows/g_f_barthann.png", [&omega.zip(w_f.map(|w| 20.0*w.norm().log10()))]).unwrap();
    }
}