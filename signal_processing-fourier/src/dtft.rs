use num_traits::{Float, NumCast, One, Zero};
use num_complex::{Complex, ComplexFloat};

use crate::SpectrumScaling;

/// Discrete-time Fourier transform
pub trait Dtft: IntoIterator<Item: ComplexFloat>
{
    /// Discrete-time Fourier transform
    fn dtft(self, omega: <Self::Item as ComplexFloat>::Real) -> Complex<<Self::Item as ComplexFloat>::Real>;

    fn dtft_scaled(self, omega: <Self::Item as ComplexFloat>::Real, scaling: SpectrumScaling) -> Complex<<Self::Item as ComplexFloat>::Real>;
}

impl<I, T> Dtft for I
where
    I: IntoIterator<Item = T>,
    T: ComplexFloat
{
    fn dtft(self, omega: <Self::Item as ComplexFloat>::Real) -> Complex<<Self::Item as ComplexFloat>::Real>
    {
        self.dtft_scaled(omega, SpectrumScaling::Balanced)
    }

    fn dtft_scaled(self, omega: T::Real, scaling: SpectrumScaling) -> Complex<T::Real>
    {
        let mut y = Complex::zero();
        let z1 = Complex::cis(-omega);
        let mut z = Complex::one();
        let mut l = 0;
        for x in self
        {
            y = y + Complex { re: x.re(), im: x.im() } *z;
            z = z*z1;
            l += 1;
        }
        if let Some(scale) = match scaling
        {
            SpectrumScaling::Summed => None,
            SpectrumScaling::Balanced => Some(Float::sqrt(<T::Real as NumCast>::from(l).unwrap())),
            SpectrumScaling::Averaged => Some(<T::Real as NumCast>::from(l).unwrap()),
        }
        {
            y = y/scale
        }
        y
    }
}

#[cfg(test)]
mod test
{
    use core::f64::consts::{FRAC_1_SQRT_2, TAU};

    use num_complex::Complex;

    use crate::{Dft, Dtft, tests};

    #[test]
    fn comparison_with_dft()
    {
        let x = [1, 2, 3, 4, 5, 6, 7, 8].map(|x| x as f64);

        let mut x_dft = x.map(|x| Complex::from(x));
        x_dft.dft();

        for (n, xf_dft) in x_dft.into_iter()
            .enumerate()
        {
            let omega = TAU*n as f64/x.len() as f64;

            let xf_dtft = x.dtft(omega);

            println!("{xf_dtft}");
            assert!(tests::approx_eq(&[xf_dft], &[xf_dtft], 1e-5))
        }
    }

    #[test]
    fn filter_edge()
    {
        let omega = 0.01;

        // First order filter
        let b = [omega, omega];
        let a = [2.0 + omega, -2.0 + omega];

        let bf = b.dtft(omega);
        let af = a.dtft(omega);

        let gf = (bf/af).norm();

        println!("{}", gf);
        assert!((gf - FRAC_1_SQRT_2).abs() < 1e-2)
    }
}