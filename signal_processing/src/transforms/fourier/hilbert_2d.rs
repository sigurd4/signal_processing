use core::ops::MulAssign;

use num::{complex::ComplexFloat, Complex, Zero, One};

use crate::{quantities::{ContainerOrSingle, Matrix, OwnedLists, OwnedMatrix}, transforms::fourier::{Dft2d, Idft2d}, util::TruncateIm};

pub trait Hilbert2d<T>: Matrix<T>
where
    T: ComplexFloat
{
    fn hilbert_2d(self) -> Self::Owned;
}

impl<T, M> Hilbert2d<T> for M
where
    T: ComplexFloat<Real: Into<T> + Into<Complex<T::Real>>> + 'static,
    Complex<T::Real>: MulAssign<T::Real>,
    M: Matrix<T, Mapped<Complex<T::Real>>: OwnedMatrix<Complex<T::Real>, Mapped<Complex<T::Real>>: Matrix<Complex<T::Real>, Mapped<T>: Into<M::Owned>>> + Idft2d<Complex<T::Real>>> + Dft2d<T>,
{
    fn hilbert_2d(self) -> M::Owned
    {
        let one = T::Real::one();
        let zero = T::Real::zero();

        let (m, n) = self.matrix_dim();

        let mut y = self.dft_2d();

        let mhalf = m/2 + 1;
        let nhalf = n/2 + 1;

        for (i, y) in y.as_mut_slices()
            .into_iter()
            .enumerate()
        {
            for (j, y) in y.iter_mut()
                .enumerate()
            {
                if i == 0 || j == 0
                {
                    *y = zero.into()
                }
                else
                {
                    *y *= if (j > nhalf) ^ (i > mhalf) {one} else {-one}
                }
            }
        }

        y.idft_2d()
            .map_into_owned(|y| y.truncate_im::<T>())
            .into()
    }
}

#[cfg(test)]
mod test
{
    use super::Hilbert2d;

    #[test]
    fn test()
    {
        let x = [
            [1.0, 2.0, 3.0, 4.0],
            [5.0, 6.0, 0.0, 8.0],
            [9.0, 10.0, 11.0, 12.0],
            [13.0, 14.0, 15.0, 16.0]
        ];
        let y = Hilbert2d::<f64>::hilbert_2d(x);
        println!("{:?}", y)
    }
}