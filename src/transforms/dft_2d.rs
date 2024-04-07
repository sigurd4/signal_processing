use core::{ops::{AddAssign, MulAssign}};

use array_math::SliceMath;
use num::{complex::ComplexFloat, Complex};

use crate::{Matrix, OwnedMatrix, OwnedLists};

pub trait DFT2D<T>: Matrix<T>
where
    T: ComplexFloat,
    Self::Mapped<Complex<T::Real>>: Matrix<Complex<T::Real>>
{
    fn dft_2d(&self) -> <<Self::Mapped<Complex<T::Real>> as Matrix<Complex<T::Real>>>::Transpose as Matrix<Complex<T::Real>>>::Transpose;
}

impl<T, M> DFT2D<T> for M
where
    T: ComplexFloat + Into<Complex<T::Real>>,
    M: Matrix<T>,
    M::Mapped<Complex<T::Real>>: OwnedMatrix<Complex<T::Real>> + 'static,
    Complex<T::Real>: ComplexFloat<Real = T::Real> + MulAssign + AddAssign
{
    fn dft_2d(&self) -> <<Self::Mapped<Complex<T::Real>> as Matrix<Complex<T::Real>>>::Transpose as Matrix<Complex<T::Real>>>::Transpose
    {
        let mut h = self.map_to_owned(|&h| h.into());
        for h in h.as_mut_slice2()
        {
            h.fft();
        }
        let mut ht = h.matrix_transpose();
        for ht in ht.as_mut_slice2()
        {
            ht.fft();
        }
        ht.matrix_transpose()
    }
}