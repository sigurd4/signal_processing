use core::ops::{AddAssign, Div, DivAssign, Mul, MulAssign};

use array_math::SliceMath;
use num::{complex::ComplexFloat, Complex};

use crate::{Matrix, OwnedMatrix, OwnedLists};

pub trait DCT2D<T>: Matrix<T>
where
    T: ComplexFloat,
    Self::Owned: Matrix<T>
{
    fn dct_i_2d(&self) -> <<Self::Owned as Matrix<T>>::Transpose as Matrix<T>>::Transpose;
    fn dct_ii_2d(&self) -> <<Self::Owned as Matrix<T>>::Transpose as Matrix<T>>::Transpose;
    fn dct_iii_2d(&self) -> <<Self::Owned as Matrix<T>>::Transpose as Matrix<T>>::Transpose;
    fn dct_iv_2d(&self) -> <<Self::Owned as Matrix<T>>::Transpose as Matrix<T>>::Transpose;
}

impl<T, M> DCT2D<T> for M
where
    M: Matrix<T>,
    M::Owned: OwnedMatrix<T>,
    T: ComplexFloat<Real: Into<T>> + Into<Complex<T::Real>> + DivAssign<T::Real> + 'static,
    Complex<T::Real>: AddAssign + MulAssign + Mul<T, Output = Complex<T::Real>> + Mul<T::Real, Output = Complex<T::Real>> + DivAssign<T::Real>
{
    fn dct_i_2d(&self) -> <<Self::Owned as Matrix<T>>::Transpose as Matrix<T>>::Transpose
    {
        let mut h = self.to_owned();
        for h in h.as_mut_slice2()
        {
            h.dct_i();
        }
        let mut ht = h.matrix_transpose();
        for ht in ht.as_mut_slice2()
        {
            ht.dct_i();
        }
        ht.matrix_transpose()
    }
    fn dct_ii_2d(&self) -> <<Self::Owned as Matrix<T>>::Transpose as Matrix<T>>::Transpose
    {
        let mut h = self.to_owned();
        for h in h.as_mut_slice2()
        {
            h.dct_ii();
        }
        let mut ht = h.matrix_transpose();
        for ht in ht.as_mut_slice2()
        {
            ht.dct_ii();
        }
        ht.matrix_transpose()
    }
    fn dct_iii_2d(&self) -> <<Self::Owned as Matrix<T>>::Transpose as Matrix<T>>::Transpose
    {
        let mut h = self.to_owned();
        for h in h.as_mut_slice2()
        {
            h.dct_iii();
        }
        let mut ht = h.matrix_transpose();
        for ht in ht.as_mut_slice2()
        {
            ht.dct_iii();
        }
        ht.matrix_transpose()
    }
    fn dct_iv_2d(&self) -> <<Self::Owned as Matrix<T>>::Transpose as Matrix<T>>::Transpose
    {
        let mut h = self.to_owned();
        for h in h.as_mut_slice2()
        {
            h.dct_iv();
        }
        let mut ht = h.matrix_transpose();
        for ht in ht.as_mut_slice2()
        {
            ht.dct_iv();
        }
        ht.matrix_transpose()
    }
}