use num::{traits::FloatConst, Float};

moddef::moddef!(
    flat(pub) mod {
        chain,
        complex_op,
        not_range,
        truncate_im
    }
);

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

/*pub(crate) fn gamma<T>(x: T) -> T
where
    T: Float
{
    NumCast::from(f64::gamma(NumCast::from(x).unwrap())).unwrap()
}

pub(crate) fn factorial<T, U>(x: U) -> T
where
    T: Float,
    U: Unsigned + Integer + ToPrimitive + Copy
{
    let n = NumCast::from(x).unwrap();
    if let Some(y) = (1..=n).try_fold(n, u128::checked_mul)
    {
        T::from(y).unwrap()
    }
    else
    {
        gamma(T::from(x).unwrap())
    }.max(T::one())
}

pub(crate) fn gegenbauer_polynomial<T>(n: usize, alpha: T) -> Polynomial<T, Vec<T>>
where
    T: Float + AddAssign + MulAssign + DivAssign
{
    let zero = T::zero();
    let one = T::one();
    let two = one + one;

    let mut c_prev = Polynomial::new(vec![one]);
    if n == 0
    {
        return c_prev
    }

    let mut c = Polynomial::new(vec![two*alpha, zero]);
    for m in 1..n
    {
        let m = T::from(m).unwrap();
        let c_next = Polynomial::new(vec![two*(m + alpha)/(m + one), zero])*c.as_view() - Polynomial::new(vec![(m + two*alpha - one)/(m + one)])*c_prev;
        c_prev = c;
        c = c_next;
    }

    c
}*/