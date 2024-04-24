use core::{iter::Sum, ops::Mul};

use ndarray::Array2;
use num::{complex::ComplexFloat, traits::FloatConst, Complex, NumCast};
use option_trait::Maybe;

use crate::{List, Matrix};


pub trait ToKw<T, K, N>: List<T>
where
    T: ComplexFloat,
    K: Matrix<Complex<T::Real>>,
    N: Maybe<usize>
{
    fn to_kw(self, n: N) -> K;
}

impl<T, W, const N: usize> ToKw<T, [[Complex<T::Real>; N]; N], ()> for W
where
    T: ComplexFloat,
    W: List<T>,
    Complex<T::Real>: Mul<T, Output = Complex<T::Real>> + Sum
{
    fn to_kw(self, (): ()) -> [[Complex<T::Real>; N]; N]
    {
        let nf = <T::Real as NumCast>::from(N).unwrap();
        core::array::from_fn(|k| {
            let kf = <T::Real as NumCast>::from(k).unwrap();
            core::array::from_fn(|k_| {
                let kf_ = <T::Real as NumCast>::from(k_).unwrap();
                self.as_view_slice()
                    .iter()
                    .enumerate()
                    .map(|(i, w)| {
                        let i = <T::Real as NumCast>::from(i).unwrap();
                        Complex::cis(T::Real::TAU()*i/nf*(kf_ - kf))**w
                    }).sum()
            })
        })
    }
}

impl<T, W> ToKw<T, Array2<Complex<T::Real>>, usize> for W
where
    T: ComplexFloat,
    W: List<T>,
    Complex<T::Real>: Mul<T, Output = Complex<T::Real>> + Sum
{
    fn to_kw(self, n: usize) -> Array2<Complex<T::Real>>
    {
        let nf = <T::Real as NumCast>::from(n).unwrap();
        Array2::from_shape_fn((n, n), |(k, k_)| {
            let kf = <T::Real as NumCast>::from(k).unwrap();
            let kf_ = <T::Real as NumCast>::from(k_).unwrap();
            self.as_view_slice()
                .iter()
                .enumerate()
                .map(|(i, w)| {
                    let i = <T::Real as NumCast>::from(i).unwrap();
                    Complex::cis(T::Real::TAU()*i/nf*(kf_ - kf))**w
                }).sum()
        })
    }
}