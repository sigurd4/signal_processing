use ndarray::Array2;
use num::{traits::FloatConst, Complex, Float};
use option_trait::Maybe;

use crate::Matrix;

pub trait DftMatrix<T, N>: Matrix<Complex<T>>
where
    T: Float + FloatConst,
    N: Maybe<usize>
{
    fn dft_matrix(n: N) -> Self;
}

impl<T, const N: usize> DftMatrix<T, ()> for [[Complex<T>; N]; N]
where
    T: Float + FloatConst
{
    fn dft_matrix((): ()) -> Self
    {
        let nf = T::from(N).unwrap();
        let w = T::TAU()/nf;

        core::array::from_fn(|i| core::array::from_fn(|j| {
            let i = T::from(i).unwrap();
            let j = T::from(j).unwrap();
            Complex::cis(-w*i*j)
        }))
    }
}

impl<T> DftMatrix<T, usize> for Array2<Complex<T>>
where
    T: Float + FloatConst
{
    fn dft_matrix(n: usize) -> Self
    {
        let nf = T::from(n).unwrap();
        let w = T::TAU()/nf;
        
        Array2::from_shape_fn((n, n), |(i, j)| {
            let i = T::from(i).unwrap();
            let j = T::from(j).unwrap();
            Complex::cis(-w*i*j)
        })
    }
}

#[cfg(test)]
mod test
{
    use array_math::{ArrayOps, CollumnArrayOps, MatrixMath};
    use num::Complex;

    use crate::{DftMatrix, Dft};

    #[test]
    fn test()
    {
        let x = [1.0, 2.0, 3.0, 4.0, 5.0, 6.0];

        let y1 = x.dft();
        println!("{:?}", y1);

        let t: [[Complex<f64>; _]; _] = DftMatrix::dft_matrix(());

        let y2 = t.mul_matrix(x.as_collumn()).into_uncollumn();
        println!("{:?}", y2);
    }
}