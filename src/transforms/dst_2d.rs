use core::ops::{AddAssign, Div, DivAssign, Mul, MulAssign};

use array_math::SliceMath;
use num::{complex::ComplexFloat, Complex};

use crate::{Matrix, OwnedMatrix, OwnedLists};

pub trait DST2D<T>: Matrix<T>
where
    T: ComplexFloat,
    Self::Owned: Matrix<T>
{
    fn dst_i_2d(&self) -> <<Self::Owned as Matrix<T>>::Transpose as Matrix<T>>::Transpose;
    fn dst_ii_2d(&self) -> <<Self::Owned as Matrix<T>>::Transpose as Matrix<T>>::Transpose;
    fn dst_iii_2d(&self) -> <<Self::Owned as Matrix<T>>::Transpose as Matrix<T>>::Transpose;
    fn dst_iv_2d(&self) -> <<Self::Owned as Matrix<T>>::Transpose as Matrix<T>>::Transpose;
}

impl<T, M> DST2D<T> for M
where
    M: Matrix<T>,
    M::Owned: OwnedMatrix<T>,
    T: ComplexFloat<Real: Into<T>> + Into<Complex<T::Real>> + 'static,
    Complex<T::Real>: AddAssign + MulAssign + DivAssign<T::Real> + Mul<T::Real, Output = Complex<T::Real>> + Mul<T, Output = Complex<T::Real>>
{
    fn dst_i_2d(&self) -> <<Self::Owned as Matrix<T>>::Transpose as Matrix<T>>::Transpose
    {
        let mut h = self.to_owned();
        for h in h.as_mut_slice2()
        {
            h.dst_i();
        }
        let mut ht = h.matrix_transpose();
        for ht in ht.as_mut_slice2()
        {
            ht.dst_i();
        }
        ht.matrix_transpose()
    }
    fn dst_ii_2d(&self) -> <<Self::Owned as Matrix<T>>::Transpose as Matrix<T>>::Transpose
    {
        let mut h = self.to_owned();
        for h in h.as_mut_slice2()
        {
            h.dst_ii();
        }
        let mut ht = h.matrix_transpose();
        for ht in ht.as_mut_slice2()
        {
            ht.dst_ii();
        }
        ht.matrix_transpose()
    }
    fn dst_iii_2d(&self) -> <<Self::Owned as Matrix<T>>::Transpose as Matrix<T>>::Transpose
    {
        let mut h = self.to_owned();
        for h in h.as_mut_slice2()
        {
            h.dst_iii();
        }
        let mut ht = h.matrix_transpose();
        for ht in ht.as_mut_slice2()
        {
            ht.dst_iii();
        }
        ht.matrix_transpose()
    }
    fn dst_iv_2d(&self) -> <<Self::Owned as Matrix<T>>::Transpose as Matrix<T>>::Transpose
    {
        let mut h = self.to_owned();
        for h in h.as_mut_slice2()
        {
            h.dst_iv();
        }
        let mut ht = h.matrix_transpose();
        for ht in ht.as_mut_slice2()
        {
            ht.dst_iv();
        }
        ht.matrix_transpose()
    }
}