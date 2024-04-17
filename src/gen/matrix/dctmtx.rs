use ndarray::Array2;
use num::{traits::FloatConst, Float};
use option_trait::Maybe;

use crate::Matrix;

pub trait DctMatrix<T, N>: Matrix<T>
where
    T: Float + FloatConst,
    N: Maybe<usize>
{
    fn dct_i_matrix(n: N) -> Self;
    fn dct_ii_matrix(n: N) -> Self;
    fn dct_iii_matrix(n: N) -> Self;
    fn dct_iv_matrix(n: N) -> Self;
}

impl<T, const N: usize> DctMatrix<T, ()> for [[T; N]; N]
where
    T: Float + FloatConst
{
    fn dct_i_matrix((): ()) -> Self
    {
        let nf = T::from(N).unwrap();
        let one = T::one();
        let nm1 = nf - one;
        let nm1_sqrt = nm1.sqrt();
        let sqrt2 = T::SQRT_2();

        let scale0 = T::FRAC_1_SQRT_2();
        let scale1 = nm1_sqrt.recip();
        let scale2 = sqrt2/nm1_sqrt;
        
        core::array::from_fn(|i| core::array::from_fn(|j| {
            let scale = if i == 0 || i == N - 1
            {
                scale1
            }
            else
            {
                scale2
            };

            (if j == 0 || j == N - 1
            {
                if j == N - 1 && i % 2 == 1 {-scale0} else {scale0}
            }
            else
            {
                let i = T::from(i).unwrap();
                let j = T::from(j).unwrap();
                (T::PI()/nm1*j*i).cos()
            })*scale
        }))
    }

    fn dct_ii_matrix((): ()) -> Self
    {
        let nf = T::from(N).unwrap();
        let n_sqrt = nf.sqrt();
        let one = T::one();
        let two = one + one;
        let half = two.recip();
        let scale1 = n_sqrt.recip();
        let scale2 = T::SQRT_2()/n_sqrt;

        core::array::from_fn(|i| core::array::from_fn(|j| {
            let scale = if i == 0
            {
                scale1
            }
            else
            {
                scale2
            };
            let i = T::from(i).unwrap();
            let j = T::from(j).unwrap();
            scale*(T::PI()/nf*(j + half)*i).cos()
        }))
    }
    
    fn dct_iii_matrix((): ()) -> Self
    {
        let nf = T::from(N).unwrap();
        let n_sqrt = nf.sqrt();
        let one = T::one();
        let two = one + one;
        let half = two.recip();
        let scale1 = n_sqrt.recip();
        let scale2 = T::SQRT_2()/n_sqrt;

        core::array::from_fn(|i| core::array::from_fn(|j| {
            let scale = if j == 0
            {
                scale1
            }
            else
            {
                scale2
            };
            let i = T::from(i).unwrap();
            let j = T::from(j).unwrap();
            scale*(T::PI()/nf*(i + half)*j).cos()
        }))
    }
    
    fn dct_iv_matrix((): ()) -> Self
    {
        let nf = T::from(N).unwrap();
        let n_sqrt = nf.sqrt();
        let one = T::one();
        let two = one + one;
        let half = two.recip();
        let scale = T::SQRT_2()/n_sqrt;

        core::array::from_fn(|i| core::array::from_fn(|j| {
            let i = T::from(i).unwrap();
            let j = T::from(j).unwrap();
            scale*(T::PI()/nf*(i + half)*(j + half)).cos()
        }))
    }
}

impl<T> DctMatrix<T, usize> for Array2<T>
where
    T: Float + FloatConst
{
    fn dct_i_matrix(n: usize) -> Self
    {
        let nf = T::from(n).unwrap();
        let one = T::one();
        let nm1 = nf - one;
        let nm1_sqrt = nm1.sqrt();
        let sqrt2 = T::SQRT_2();
        let scale0 = T::FRAC_1_SQRT_2();
        let scale1 = nm1_sqrt.recip();
        let scale2 = sqrt2/nm1_sqrt;
        
        Array2::from_shape_fn((n, n), |(i, j)| {
            let scale = if i == 0 || i + 1 == n
            {
                scale1
            }
            else
            {
                scale2
            };

            (if j == 0 || j + 1 == n
            {
                if j + 1 == n && i % 2 == 1 {-scale0} else {scale0}
            }
            else
            {
                let i = T::from(i).unwrap();
                let j = T::from(j).unwrap();
                (T::PI()/nm1*j*i).cos()
            })*scale
        })
    }

    fn dct_ii_matrix(n: usize) -> Self
    {
        let nf = T::from(n).unwrap();
        let n_sqrt = nf.sqrt();
        let one = T::one();
        let two = one + one;
        let half = two.recip();
        let scale1 = n_sqrt.recip();
        let scale2 = T::SQRT_2()/n_sqrt;

        Array2::from_shape_fn((n, n), |(i, j)| {
            let scale = if i == 0
            {
                scale1
            }
            else
            {
                scale2
            };
            let i = T::from(i).unwrap();
            let j = T::from(j).unwrap();
            scale*(T::PI()/nf*(j + half)*i).cos()
        })
    }
    
    fn dct_iii_matrix(n: usize) -> Self
    {
        let nf = T::from(n).unwrap();
        let n_sqrt = nf.sqrt();
        let one = T::one();
        let two = one + one;
        let half = two.recip();
        let scale1 = n_sqrt.recip();
        let scale2 = T::SQRT_2()/n_sqrt;

        Array2::from_shape_fn((n, n), |(i, j)| {
            let scale = if j == 0
            {
                scale1
            }
            else
            {
                scale2
            };
            let i = T::from(i).unwrap();
            let j = T::from(j).unwrap();
            scale*(T::PI()/nf*(i + half)*j).cos()
        })
    }
    
    fn dct_iv_matrix(n: usize) -> Self
    {
        let nf = T::from(n).unwrap();
        let n_sqrt = nf.sqrt();
        let one = T::one();
        let two = one + one;
        let half = two.recip();
        let scale = T::SQRT_2()/n_sqrt;

        Array2::from_shape_fn((n, n), |(i, j)| {
            let i = T::from(i).unwrap();
            let j = T::from(j).unwrap();
            scale*(T::PI()/nf*(i + half)*(j + half)).cos()
        })
    }
}

#[cfg(test)]
mod test
{
    use array_math::{ArrayOps, CollumnArrayOps, MatrixMath};

    use crate::{Dct, DctMatrix};

    #[test]
    fn test()
    {
        let x = [1.0, 2.0, 3.0, 4.0, 5.0, 6.0];

        let y1 = x.dct_iv();
        println!("{:?}", y1);

        let t: [[f64; _]; _] = DctMatrix::dct_iv_matrix(());

        let y2 = t.mul_matrix(x.as_collumn()).into_uncollumn();
        println!("{:?}", y2);
    }
}