use core::ops::{AddAssign, DivAssign, Mul, MulAssign};

use num::{complex::ComplexFloat, Complex};
use array_math::SliceMath;

use crate::{quantities::{Lists, OwnedLists}, transforms::fourier::Dft};

pub trait Dct<T>: Lists<T>
where
    T: ComplexFloat
{
    fn dct_i(self) -> Self::Owned;
    fn dct_ii(self) -> Self::Owned;
    fn dct_iii(self) -> Self::Owned;
    fn dct_iv(self) -> Self::Owned;
}

impl<T, L> Dct<T> for L
where
    L: Lists<T>,
    L::Owned: OwnedLists<T>,
    T: ComplexFloat + Into<Complex<T::Real>> + MulAssign<T::Real> + DivAssign<T::Real> + 'static,
    Complex<T::Real>: AddAssign + MulAssign + DivAssign<T::Real> + Mul<T, Output = Complex<T::Real>> + Mul<T::Real, Output = Complex<T::Real>>,
    T::Real: Into<T> + Into<Complex<T::Real>>,
    Self: Dft<T>,
{
    fn dct_i(self) -> Self::Owned
    {
        let mut y = self.into_owned();
        for x in y.as_mut_slices()
            .into_iter()
        {
            SliceMath::dct_i(x)
        }
        y
    }
    fn dct_ii(self) -> Self::Owned
    {
        let mut y = self.into_owned();
        for x in y.as_mut_slices()
            .into_iter()
        {
            SliceMath::dct_ii(x)
        }
        y
    }
    fn dct_iii(self) -> Self::Owned
    {
        let mut y = self.into_owned();
        for x in y.as_mut_slices()
            .into_iter()
        {
            SliceMath::dct_iii(x)
        }
        y
    }
    fn dct_iv(self) -> Self::Owned
    {
        let mut y = self.into_owned();
        for x in y.as_mut_slices()
            .into_iter()
        {
            SliceMath::dct_iv(x)
        }
        y
    }
}

#[cfg(test)]
mod test
{
    use core::f64::consts::TAU;

    use array_math::ArrayOps;
    use linspace::LinspaceArray;

    use crate::{plot, transforms::fourier::Dct};

    #[test]
    fn test()
    {
        const N: usize = 1024;
        const T: f64 = 1.0;
        const F: f64 = 220.0;
        
        let x: [_; N] = ArrayOps::fill(|i| (TAU*F*i as f64/N as f64*T).sin());

        let xf = [
            x.dct_i(),
            x.dct_ii(),
            x.dct_iii(),
            x.dct_iv()
        ];

        let w = (0.0..TAU).linspace_array();

        plot::plot_curves("X(e^jw)", "plots/x_z_dct.png", [
                &w.zip(xf[0]),
                &w.zip(xf[1]),
                &w.zip(xf[2]),
                &w.zip(xf[3])
            ]).unwrap()
    }
}