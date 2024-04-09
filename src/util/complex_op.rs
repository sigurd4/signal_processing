use num::{complex::ComplexFloat, traits::FloatConst, Complex, Float};

pub trait ComplexOp<Rhs>: ComplexFloat + Into<<Self as ComplexOp<Rhs>>::Output>
where
    Rhs: ComplexFloat + Into<<Self as ComplexOp<Rhs>>::Output>
{
    type Output: ComplexFloat;
}

macro_rules! impl_complex_op {
    ($t:ty;) => {
        impl_complex_op!($t, $t => $t);
        impl_complex_op!(Complex<$t>, $t => Complex<$t>);
        impl_complex_op!($t, Complex<$t> => Complex<$t>);
        impl_complex_op!(Complex<$t>, Complex<$t> => Complex<$t>);
    };
    ($t1:ty, $t2:ty => $t3:ty) => {
        impl ComplexOp<$t2> for $t1
        {
            type Output = $t3;
        }
    };
}

impl_complex_op!(f32;);
impl_complex_op!(f64;);