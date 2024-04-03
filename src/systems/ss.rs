use core::marker::PhantomData;

use num::complex::ComplexFloat;

use crate::Matrix;

#[derive(Debug, Clone, Copy)]
pub struct Ss<T: ComplexFloat, A: Matrix<T>, B: Matrix<T>, C: Matrix<T>, D: Matrix<T>>
{
    pub a: A,
    pub b: B,
    pub c: C,
    pub d: D,
    phantom: PhantomData<T>
}

impl<T: ComplexFloat, A: Matrix<T>, B: Matrix<T>, C: Matrix<T>, D: Matrix<T>> Ss<T, A, B, C, D>
{
    pub fn new(a: A, b: B, c: C, d: D) -> Self
    {
        Self {
            a,
            b,
            c,
            d,
            phantom: PhantomData
        }
    }
}