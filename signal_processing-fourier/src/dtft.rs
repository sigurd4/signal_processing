use num_traits::{One, Zero};
use num_complex::{Complex, ComplexFloat};

pub trait Dtft: IntoIterator<Item: ComplexFloat>
{
    /// Discrete time fourier transform
    fn dtft(self, omega: <Self::Item as ComplexFloat>::Real) -> Complex<<Self::Item as ComplexFloat>::Real>;
}

impl<I, T> Dtft for I
where
    I: IntoIterator<Item = T>,
    T: ComplexFloat
{   
    fn dtft(self, omega: T::Real) -> Complex<T::Real>
    {
        let mut y = Complex::zero();
        let z1 = Complex::cis(-omega);
        let mut z = Complex::one();
        for x in self
        {
            y = y + Complex { re: x.re(), im: x.im() } *z;
            z = z*z1;
        }
        y
    }
}

#[cfg(test)]
mod test
{
    use core::f64::consts::FRAC_1_SQRT_2;

    use crate::Dtft;

    #[test]
    fn it_works()
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