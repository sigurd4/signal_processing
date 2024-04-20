use core::ops::{AddAssign, MulAssign};

use array_math::SliceMath;
use num::{complex::ComplexFloat, Complex};

use crate::{Matrix, MaybeMatrix, OwnedLists, OwnedMatrix};

pub trait Idft2d<T>: Matrix<T>
where
    T: ComplexFloat,
    Self::Mapped<Complex<T::Real>>: Matrix<Complex<T::Real>>
{
    fn idft_2d(self) -> Self::Mapped<Complex<T::Real>>;
}

impl<T, M> Idft2d<T> for M
where
    T: ComplexFloat + Into<Complex<T::Real>>,
    M: Matrix<T>,
    M::Mapped<Complex<T::Real>>: OwnedMatrix<Complex<T::Real>> + 'static,
    Complex<T::Real>: ComplexFloat<Real = T::Real> + MulAssign + AddAssign + MulAssign<T::Real>,
    <Self::Mapped<Complex<T::Real>> as MaybeMatrix<Complex<T::Real>>>::Transpose: OwnedMatrix<Complex<T::Real>, Transpose = Self::Mapped<Complex<T::Real>>>
{
    fn idft_2d(self) -> Self::Mapped<Complex<T::Real>>
    {
        let mut h = self.map_into_owned(|h| h.into());
        for h in h.as_mut_slices()
        {
            h.ifft();
        }
        let mut ht = h.matrix_transpose();
        for ht in ht.as_mut_slices()
        {
            ht.ifft();
        }
        ht.matrix_transpose()
    }
}