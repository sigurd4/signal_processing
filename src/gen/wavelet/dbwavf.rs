use core::ops::{DivAssign, MulAssign};

use ndarray_linalg::Lapack;
use num::{traits::{float::TotalOrder, FloatConst}, Complex, Float};
use option_trait::Maybe;

pub fn dbwavf<T>(order: usize) -> Vec<T>
where
    T: Float + FloatConst + MulAssign + DivAssign + TotalOrder + Lapack<Complex = Complex<T>>,
    (): Maybe<T>
{
    crate::gen::wavelet::dbaux(order, ())
}

#[cfg(test)]
mod test
{
    use linspace::Linspace;

    use crate::plot;

    #[test]
    fn test()
    {
        let psi = crate::gen::wavelet::dbwavf(38);

        plot::plot_curves("ψ[n]", "plots/psi_n_dbwavf.png", [
                &(0.0..psi.len() as f64).linspace(psi.len())
                    .into_iter()
                    .zip(psi)
                    .collect::<Vec<_>>()
            ]).unwrap()
    }
}