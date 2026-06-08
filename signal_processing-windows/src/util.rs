use num_traits::{Float, FloatConst};

pub(crate) fn i0<T>(x: T) -> T
where
    T: Float + FloatConst
{
    let one = T::one();
    let two = one + one;
    let four = two + two;
    let half = two.recip();

    let lambda = half;

    let p0 = one;
    let q1 = (one - lambda*lambda)/four/(one - T::SQRT_2()*(lambda/T::PI()).sqrt());
    let p1 = two*(lambda/T::PI()).sqrt()*q1*T::FRAC_1_SQRT_2();

    (one + lambda*lambda*x*x).sqrt().sqrt().recip()*x.cosh()*(p0 + p1*x*x)/(one + q1*x*x)
}