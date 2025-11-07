use core::ops::{AddAssign, DivAssign, Mul, MulAssign};

use array_math::SliceMath;
use num::{complex::ComplexFloat, Complex};

use crate::quantities::{Matrix, MaybeMatrix, OwnedLists, OwnedMatrix};

pub trait Dst2d<T>: Matrix<T>
where
    T: ComplexFloat,
    Self::Owned: Matrix<T>
{
    fn dst_i_2d(self) -> Self::Owned;
    fn dst_ii_2d(self) -> Self::Owned;
    fn dst_iii_2d(self) -> Self::Owned;
    fn dst_iv_2d(self) -> Self::Owned;
}

impl<T, M> Dst2d<T> for M
where
    M: Matrix<T>,
    M::Owned: OwnedMatrix<T>,
    T: ComplexFloat<Real: Into<T>> + Into<Complex<T::Real>> + 'static,
    Complex<T::Real>: AddAssign + MulAssign + MulAssign<T::Real> + DivAssign<T::Real> + Mul<T::Real, Output = Complex<T::Real>> + Mul<T, Output = Complex<T::Real>>,
    <Self::Owned as MaybeMatrix<T>>::Transpose: OwnedMatrix<T, Transpose: Into<M::Owned>>
{
    fn dst_i_2d(self) -> Self::Owned
    {
        let mut h = self.into_owned();
        for h in h.as_mut_slices()
        {
            h.dst_i();
        }
        let mut ht = h.matrix_transpose();
        for ht in ht.as_mut_slices()
        {
            ht.dst_i();
        }
        ht.matrix_transpose()
            .into()
    }
    fn dst_ii_2d(self) -> Self::Owned
    {
        let mut h = self.into_owned();
        for h in h.as_mut_slices()
        {
            h.dst_ii();
        }
        let mut ht = h.matrix_transpose();
        for ht in ht.as_mut_slices()
        {
            ht.dst_ii();
        }
        ht.matrix_transpose()
            .into()
    }
    fn dst_iii_2d(self) -> Self::Owned
    {
        let mut h = self.into_owned();
        for h in h.as_mut_slices()
        {
            h.dst_iii();
        }
        let mut ht = h.matrix_transpose();
        for ht in ht.as_mut_slices()
        {
            ht.dst_iii();
        }
        ht.matrix_transpose()
            .into()
    }
    fn dst_iv_2d(self) -> Self::Owned
    {
        let mut h = self.into_owned();
        for h in h.as_mut_slices()
        {
            h.dst_iv();
        }
        let mut ht = h.matrix_transpose();
        for ht in ht.as_mut_slices()
        {
            ht.dst_iv();
        }
        ht.matrix_transpose()
            .into()
    }
}