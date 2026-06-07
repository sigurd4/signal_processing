use core::ops::{AddAssign, MulAssign};

use bulks::{IntoBulk, Map};
use num::{complex::ComplexFloat, Complex};

use crate::quantities::{Lists, OwnedLists};

pub trait Dft<T>: IntoBulk<Item: ComplexFloat>
where
    T: ComplexFloat
{
    fn dft(self) -> Map<Self::IntoBulk, fn()>;
}

impl<T, L> Dft<T> for L
where
    T: ComplexFloat + Into<Complex<T::Real>>,
    L: Lists<T>,
    L::Mapped<Complex<T::Real>>: OwnedLists<Complex<T::Real>>,
    Complex<T::Real>: ComplexFloat<Real = T::Real> + MulAssign + AddAssign
{
    fn dft(self) -> Self::Mapped<Complex<T::Real>>
    {
        let mut h = self.map_into_owned(|h| h.into());
        for h in h.as_mut_slices()
        {
            h.fft()
        }
        h
    }
}

#[cfg(test)]
mod test
{
    use core::f64::consts::TAU;

    
    use linspace::Linspace;

    use crate::{plot, transforms::fourier::Dft};

    #[test]
    fn test()
    {
        const N: usize = 1024;
        const T: f64 = 1.0;
        const F: f64 = 220.0;
        
        let x: [_; N] = core::array::from_fn(|i| (TAU*F*i as f64/N as f64*T).sin());

        let xf = x.dft();
        let w = (0.0..TAU).linspace_array();

        plot::plot_curves("X(e^jw)", "plots/x_z_dft.png", [&w.zip(xf.map(|xf| xf.norm()))])
            .unwrap()
    }
}