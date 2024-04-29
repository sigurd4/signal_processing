use ndarray::Array2;
use num::{traits::FloatConst, Float};
use option_trait::Maybe;

use crate::quantities::Matrix;

pub trait DstMatrix<T, N>: Matrix<T>
where
    T: Float + FloatConst,
    N: Maybe<usize>
{
    fn dst_i_matrix(n: N) -> Self;
    fn dst_ii_matrix(n: N) -> Self;
    fn dst_iii_matrix(n: N) -> Self;
    fn dst_iv_matrix(n: N) -> Self;
}

impl<T, const N: usize> DstMatrix<T, ()> for [[T; N]; N]
where
    T: Float + FloatConst
{
    fn dst_i_matrix((): ()) -> Self
    {
        let nf = T::from(N).unwrap();
        let one = T::one();
        let np1 = nf + one;
        let scale = T::SQRT_2()/np1.sqrt();

        core::array::from_fn(|i| core::array::from_fn(|j| {
            let i = T::from(i + 1).unwrap();
            let j = T::from(j + 1).unwrap();
            (T::PI()/np1*j*i).sin()*scale
        }))
    }

    fn dst_ii_matrix((): ()) -> Self
    {
        let nf = T::from(N).unwrap();
        let n_sqrt = nf.sqrt();
        let one = T::one();
        let two = one + one;
        let half = two.recip();
        let scale1 = T::SQRT_2()/n_sqrt;
        let scale2 = n_sqrt.recip();
        
        core::array::from_fn(|i| core::array::from_fn(|j| {
            let scale = if i == N - 1
            {
                scale2
            }
            else
            {
                scale1
            };
            let i = T::from(i + 1).unwrap();
            let j = T::from(j).unwrap();
            scale*(T::PI()/nf*(j + half)*i).sin()
        }))
    }
    
    fn dst_iii_matrix((): ()) -> Self
    {
        let nf = T::from(N).unwrap();
        let n_sqrt = nf.sqrt();
        let one = T::one();
        let two = one + one;
        let half = two.recip();
        let scale = T::SQRT_2()/n_sqrt;

        core::array::from_fn(|i| core::array::from_fn(|j| {
            scale*(if j == N - 1
            {
                if i % 2 == 1 {-T::FRAC_1_SQRT_2()} else {T::FRAC_1_SQRT_2()}
            }
            else
            {
                let i = T::from(i).unwrap();
                let j = T::from(j + 1).unwrap();
                (T::PI()/nf*(i + half)*j).sin()
            })
        }))
    }
    
    fn dst_iv_matrix((): ()) -> Self
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
            scale*(T::PI()/nf*(i + half)*(j + half)).sin()
        }))
    }
}

impl<T> DstMatrix<T, usize> for Array2<T>
where
    T: Float + FloatConst
{
    fn dst_i_matrix(n: usize) -> Self
    {
        let nf = T::from(n).unwrap();
        let one = T::one();
        let np1 = nf + one;
        let scale = T::SQRT_2()/np1.sqrt();

        Array2::from_shape_fn((n, n), |(i, j)| {
            let i = T::from(i + 1).unwrap();
            let j = T::from(j + 1).unwrap();
            (T::PI()/np1*j*i).sin()*scale
        })
    }

    fn dst_ii_matrix(n: usize) -> Self
    {
        let nf = T::from(n).unwrap();
        let n_sqrt = nf.sqrt();
        let one = T::one();
        let two = one + one;
        let half = two.recip();
        let scale1 = T::SQRT_2()/n_sqrt;
        let scale2 = n_sqrt.recip();
        
        Array2::from_shape_fn((n, n), |(i, j)| {
            let scale = if i + 1 == n
            {
                scale2
            }
            else
            {
                scale1
            };
            let i = T::from(i + 1).unwrap();
            let j = T::from(j).unwrap();
            scale*(T::PI()/nf*(j + half)*i).sin()
        })
    }
    
    fn dst_iii_matrix(n: usize) -> Self
    {
        let nf = T::from(n).unwrap();
        let n_sqrt = nf.sqrt();
        let one = T::one();
        let two = one + one;
        let half = two.recip();
        let scale = T::SQRT_2()/n_sqrt;

        Array2::from_shape_fn((n, n), |(i, j)| {
            scale*(if j + 1 == n
            {
                if i % 2 == 1 {-T::FRAC_1_SQRT_2()} else {T::FRAC_1_SQRT_2()}
            }
            else
            {
                let i = T::from(i).unwrap();
                let j = T::from(j + 1).unwrap();
                (T::PI()/nf*(i + half)*j).sin()
            })
        })
    }
    
    fn dst_iv_matrix(n: usize) -> Self
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
            scale*(T::PI()/nf*(i + half)*(j + half)).sin()
        })
    }
}

#[cfg(test)]
mod test
{
    use array_math::{ArrayOps, CollumnArrayOps, MatrixMath};

    use crate::{transforms::fourier::Dst, gen::matrix::DstMatrix};

    #[test]
    fn test()
    {
        let x = [1.0, 2.0, 3.0, 4.0, 5.0, 6.0];

        let y1 = x.dst_iv();
        println!("{:?}", y1);

        let t: [[f64; _]; _] = DstMatrix::dst_iv_matrix(());

        let y2 = t.mul_matrix(x.as_collumn()).into_uncollumn();
        println!("{:?}", y2);
    }
}