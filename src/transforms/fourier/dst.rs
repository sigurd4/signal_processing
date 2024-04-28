use core::ops::{AddAssign, DivAssign, Mul, MulAssign};

use num::{complex::ComplexFloat, Complex};
use array_math::SliceMath;

use crate::{Lists, OwnedLists};

pub trait Dst<T>: Lists<T>
where
    T: ComplexFloat
{
    fn dst_i(self) -> Self::Owned;
    fn dst_ii(self) -> Self::Owned;
    fn dst_iii(self) -> Self::Owned;
    fn dst_iv(self) -> Self::Owned;
}

impl<T, L> Dst<T> for L
where
    L: Lists<T, Owned: OwnedLists<T>>,
    T: ComplexFloat + Into<Complex<T::Real>> + MulAssign<T::Real> + DivAssign<T::Real> + 'static,
    Complex<T::Real>: AddAssign + MulAssign + MulAssign<T::Real> + DivAssign<T::Real> + Mul<T, Output = Complex<T::Real>> + Mul<T::Real, Output = Complex<T::Real>>,
    T::Real: Into<T> + Into<Complex<T::Real>>
{
    fn dst_i(self) -> Self::Owned
    {
        let mut y = self.into_owned();
        for x in y.as_mut_slices()
            .into_iter()
        {
            SliceMath::dst_i(x)
        }
        y
    }
    fn dst_ii(self) -> Self::Owned
    {
        let mut y = self.into_owned();
        for x in y.as_mut_slices()
            .into_iter()
        {
            SliceMath::dst_ii(x)
        }
        y
    }
    fn dst_iii(self) -> Self::Owned
    {
        let mut y = self.into_owned();
        for x in y.as_mut_slices()
            .into_iter()
        {
            SliceMath::dst_iii(x)
        }
        y
    }
    fn dst_iv(self) -> Self::Owned
    {
        let mut y = self.into_owned();
        for x in y.as_mut_slices()
            .into_iter()
        {
            SliceMath::dst_iv(x)
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

    use crate::{plot, Dst};

    #[test]
    fn test()
    {
        const N: usize = 1024;
        const T: f64 = 1.0;
        const F: f64 = 220.0;
        
        let x: [_; N] = ArrayOps::fill(|i| (TAU*F*i as f64/N as f64*T).sin());

        let xf = [
            x.dst_i(),
            x.dst_ii(),
            x.dst_iii(),
            x.dst_iv()
        ];

        let w = (0.0..TAU).linspace_array();

        plot::plot_curves("X(e^jw)", "plots/x_z_dst.png", [
                &w.zip(xf[0]),
                &w.zip(xf[1]),
                &w.zip(xf[2]),
                &w.zip(xf[3])
            ]).unwrap()
    }
}