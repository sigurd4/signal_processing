use num::{complex::ComplexFloat, traits::FloatConst, Complex, Float};

pub trait ComplexOp<Rhs>: ComplexFloat + Into<<Self as ComplexOp<Rhs>>::Output>
where
    Rhs: ComplexFloat
{
    type Output: ComplexFloat + From<Rhs> + From<Self>;
}

impl<T> ComplexOp<T> for T
where
    T: ComplexFloat
{
    type Output = T;
}
impl<T> ComplexOp<T> for Complex<T>
where
    T: Float + FloatConst
{
    type Output = Complex<T>;
}
impl<T> ComplexOp<Complex<T>> for T
where
    T: Float + FloatConst
{
    type Output = Complex<T>;
}