use core::ops::{AddAssign, MulAssign};

use array_math::SliceMath;
use num::{complex::ComplexFloat, Complex};

use crate::{Matrix, MaybeMatrix, OwnedLists, OwnedMatrix};

pub trait Dft2d<T>: Matrix<T>
where
    T: ComplexFloat,
    Self::Mapped<Complex<T::Real>>: Matrix<Complex<T::Real>>
{
    fn dft_2d(self) -> Self::Mapped<Complex<T::Real>>;
}

impl<T, M> Dft2d<T> for M
where
    T: ComplexFloat + Into<Complex<T::Real>>,
    M: Matrix<T>,
    M::Mapped<Complex<T::Real>>: OwnedMatrix<Complex<T::Real>> + 'static,
    Complex<T::Real>: ComplexFloat<Real = T::Real> + MulAssign + AddAssign,
    <Self::Mapped<Complex<T::Real>> as MaybeMatrix<Complex<T::Real>>>::Transpose: OwnedMatrix<Complex<T::Real>, Transpose: Into<M::Mapped<Complex<T::Real>>>>
{
    fn dft_2d(self) -> Self::Mapped<Complex<T::Real>>
    {
        let mut h = self.map_into_owned(|h| h.into());
        for h in h.as_mut_slices()
        {
            h.fft();
        }
        let mut ht = h.matrix_transpose();
        for ht in ht.as_mut_slices()
        {
            ht.fft();
        }
        ht.matrix_transpose()
            .into()
    }
}