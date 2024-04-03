use num::{complex::ComplexFloat, traits::FloatConst, Complex, Float};

pub trait ComplexOp<Rhs>: ComplexFloat + Into<<Self as ComplexOp<Rhs>>::Output>
where
    Rhs: ComplexFloat + Into<<Self as ComplexOp<Rhs>>::Output>
{
    type Output: ComplexFloat;
}

impl<T> ComplexOp<T> for T
where
    T: Float + FloatConst
{
    type Output = T;
}
impl<T> ComplexOp<Complex<T>> for T
where
    T: Float + FloatConst
{
    type Output = Complex<T>;
}
impl<T> ComplexOp<T> for Complex<T>
where
    T: Float + FloatConst
{
    type Output = Complex<T>;
}
default impl<T> ComplexOp<T> for T
where
    T: ComplexFloat
{
    type Output = Complex<T::Real>;
}