use core::ops::Mul;

use ndarray::{prelude::Axis, Array2, Slice};
use ndarray_linalg::{error::LinalgError, Lapack, SVDInto, Scalar};
use num::{complex::ComplexFloat, traits::FloatConst, Complex, Float, Integer, NumCast, ToPrimitive, Unsigned};

moddef::moddef!(
    flat(pub) mod {
        complex_op,
        not_range,
        result_or_ok,
        truncate_im,
        two_sided_range,
    },
    mod {
        expm, // This should be pushed to the ndarray_linalg crate and does not belong here.
    }
);

pub(crate) fn transpose_vec_vec<T, I>(y_t: Vec<I>) -> Vec<Vec<T>>
where
    I: IntoIterator<Item = T>
{
    let mut y_t: Vec<_> = y_t.into_iter()
        .map(|y| y.into_iter())
        .collect();
    let mut y = vec![];
    'lp:
    loop
    {
        let mut first = true;

        for y_t in y_t.iter_mut()
        {
            if let Some(y_t) = y_t.next()
            {
                if first
                {
                    y.push(vec![]);
                    first = false;
                }
                let y = y.last_mut().unwrap();
                y.push(y_t)
            }
            else
            {
                break 'lp
            }
        }
    }
    y
}

pub(crate) fn expm<T>(m: Array2<T>) -> Result<Array2<T>, LinalgError>
where
    T: Scalar + Lapack + ComplexFloat<Real: Into<T>>
{
    Ok(expm::expm(&m.map(|&a| {
        Complex::new(a.re().to_f64().unwrap(), a.im().to_f64().unwrap())
    }))?.map(|&a| {
        Complex::new(<<T as ComplexFloat>::Real as NumCast>::from(a.re()).unwrap(), <<T as ComplexFloat>::Real as NumCast>::from(a.im()).unwrap())
            .truncate_im::<T>()
    }))
}

pub(crate) fn pinv<T>(m: Array2<T>) -> Array2<T>
where
    T: Lapack<Real: Into<T>> + Mul<T::Real, Output = T>
{
    let mdim = m.dim();
    let (u, s, v_h) = m.svd_into(true, true).unwrap();
    let u = u.unwrap();
    let v_h = v_h.unwrap();

    let threshold = T::Real::epsilon()*NumCast::from(mdim.0.max(mdim.1)).unwrap();

    // Determine how many singular values to keep and compute the
    // values of `V Σ⁺` (up to `num_keep` columns).
    let (num_keep, v_s_inv) = {
        let mut v_h_t = v_h.reversed_axes();
        let mut num_keep = 0;
        for (&sing_val, mut v_h_t_col) in s.iter().zip(v_h_t.columns_mut()) {
            if sing_val > threshold {
                let sing_val_recip = sing_val.recip();
                v_h_t_col.map_inplace(|v_h_t| *v_h_t = T::from_real(sing_val_recip) * v_h_t.conj());
                num_keep += 1;
            } else {
                break;
            }
        }
        v_h_t.slice_axis_inplace(Axis(1), Slice::from(..num_keep));
        (num_keep, v_h_t)
    };

    // Compute `U^H` (up to `num_keep` rows).
    let u_h = {
        let mut u_t = u.reversed_axes();
        u_t.slice_axis_inplace(Axis(0), Slice::from(..num_keep));
        u_t.map_inplace(|x| *x = x.conj());
        u_t
    };

    v_s_inv.dot(&u_h)
}

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

pub(crate) fn gamma<T>(x: T) -> T
where
    T: Float
{
    NumCast::from(f64::gamma(NumCast::from(x).unwrap())).unwrap()
}

pub(crate) fn erf_inv<T>(x: T) -> T
where
    T: Float
{
    NumCast::from(statrs::function::erf::erf_inv(NumCast::from(x).unwrap())).unwrap()
}

pub(crate) fn factorial<T, U>(x: U) -> T
where
    T: Float,
    U: Unsigned + Integer + ToPrimitive + Copy
{
    let n: u128 = NumCast::from(x).unwrap();
    if let Some(y) = (1..n.max(1)).try_fold(n.max(1), u128::checked_mul)
    {
        T::from(y).unwrap()
    }
    else
    {
        gamma(T::from(x).unwrap())
    }.max(T::one())
}

pub(crate) fn bincoeff<T, U>(n: U, k: U) -> T
where
    T: Float,
    U: Unsigned + Integer + ToPrimitive + Copy
{
    let nn: u128 = NumCast::from(n).unwrap();
    let kk: u128 = NumCast::from(k).unwrap();

    if kk == 0
    {
        return T::one()
    }

    let b = if let Some(b) = nn.checked_sub(kk)
        .and_then(|nmkp1| {
            ((nmkp1 + 1).max(1)..nn.max(1)).try_fold(nn.max(1), u128::checked_mul)
        })
    {
        T::from(b).unwrap()
    }
    else
    {
        factorial::<T, u128>(nn)/gamma(-T::from(n - k).unwrap())
    };
    b/factorial(kk)
}


/*pub(crate) fn gegenbauer_polynomial<T>(n: usize, alpha: T) -> Polynomial<T, Vec<T>>
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