use core::ops::{AddAssign, MulAssign};

use array_math::SliceMath;
use num::{complex::ComplexFloat, Complex};

use crate::{Lists, OwnedLists};

pub trait Idft<T>: Lists<T>
where
    T: ComplexFloat
{
    fn idft(self) -> Self::Mapped<Complex<T::Real>>;
}

impl<T, L> Idft<T> for L
where
    T: ComplexFloat + Into<Complex<T::Real>>,
    L: Lists<T>,
    L::Mapped<Complex<T::Real>>: OwnedLists<Complex<T::Real>>,
    Complex<T::Real>: ComplexFloat<Real = T::Real> + MulAssign + AddAssign + MulAssign<T::Real>
{
    fn idft(self) -> Self::Mapped<Complex<T::Real>>
    {
        let mut h = self.map_into_owned(|h| h.into());
        for h in h.as_mut_slices()
        {
            h.ifft()
        }
        h
    }
}

#[cfg(test)]
mod test
{
    use core::f64::consts::TAU;

    use array_math::ArrayOps;
    use linspace::LinspaceArray;

    use crate::{plot, Dft, Idft};

    #[test]
    fn test()
    {
        const N: usize = 1024;
        const T: f64 = 0.1;
        const F: f64 = 220.0;
        
        let t: [_; N] = (0.0..T).linspace_array();
        let x = t.map(|t| (TAU*F*t).sin());

        let xf = x.dft();
        let y = xf.idft();

        plot::plot_curves("x(t)", "plots/x_t_idft.png", [&t.zip(y.map(|y| y.re))])
            .unwrap()
    }
}