use core::ops::DivAssign;


use array_math::ArrayOps;
use num::{traits::FloatConst, Float, NumCast};
use thiserror::Error;


#[derive(Debug, Clone, Copy, PartialEq, Error)]
pub enum FirPmOrdError
{
    #[error("Bands must be monotonic starting at zero.")]
    EdgesNotNondecreasing,
    #[error("Band edges must be less than 1/2 the sampling frequency, or if no sampling frequency is specified, between 0 and 1.")]
    EdgesOutOfRange,
    #[error("Sampling frequency must be a positive number.")]
    InvalidSamplingFrequency,
    #[error("Deviations must be positive numbers.")]
    DeviationsOutOfRange
}

pub fn firpmord<T, const F: usize, const D: usize>(
    mut frequencies: [T; F],
    amplitudes: [T; (F + 2)/2],
    deviations: [T; D],
    sampling_frequency: Option<T>
) -> Result<(usize, [T; F + 2], [T; F + 2], [T; (F + 2)/2]), FirPmOrdError>
where
    T: Float + FloatConst + DivAssign,
    [(); 0 - F%2]:,
    [(); F/2]:,
    [(); (F + 2)/2 - D]:,
    [(); F + 2]:
{
    let one = T::one();
    let two = one + one;
    let half = two.recip();
    let zero = T::zero();

    if let Some(fs) = sampling_frequency
    {
        let nyq = fs*half;
        if !(fs > zero) || !fs.is_finite()
        {
            return Err(FirPmOrdError::InvalidSamplingFrequency)
        }
        for f in frequencies.iter_mut()
        {
            *f /= nyq
        }
    }
    if !frequencies.is_sorted()
    {
        return Err(FirPmOrdError::EdgesNotNondecreasing)
    }
    if frequencies.iter()
        .any(|f| *f < zero || *f > one)
    {
        return Err(FirPmOrdError::EdgesOutOfRange)
    }
    if deviations.iter()
        .any(|d| *d <= zero)
    {
        return Err(FirPmOrdError::DeviationsOutOfRange)
    }
    let mut d: [_; (F + 2)/2] = deviations.resize(|_| one);

    for (d, a) in d.each_mut()
        .zip(amplitudes)
    {
        *d /= a.abs() + T::from((a == zero) as u8).unwrap()
    }

    let [fl, fr] = frequencies.spread_exact();
    let mut l = 2;
    for k in 0..F/2
    {
        if amplitudes[k] > amplitudes[k + 1]
        {
            l = l.max(estimate_lp(fl[k], fr[k], d[k], d[k + 1]))
        }
        else if amplitudes[k] < amplitudes[k + 1]
        {
            l = l.max(estimate_lp(half - fr[k], half - fl[k], d[k + 1], d[k]))
        }
    }

    let mut f_out = [one; F + 2];
    f_out[1..F + 1].copy_from_slice(&frequencies);
    f_out[0] = zero;
    let w = d.mul_all_inv(d.first_max().unwrap_or(zero));
    let mut n = l - 1;
    n += (*amplitudes.last().unwrap() != zero && n % 2 != 0) as usize;
    let a: [T; F + 2] = ArrayOps::fill(|i| amplitudes[i/2]);

    Ok((n, f_out, a, w))
}

fn estimate_lp<T>(fp: T, fs: T, dp: T, ds: T) -> usize
where
    T: Float + FloatConst
{
    let zero = T::zero();
    let one = T::one();
    let two = one + one;
    let half = two.recip();
    let three = one + two;

    let df = fs - fp;
    assert!(df > zero);

    let v = |d: T| {
        T::from(2.325).unwrap()*(-d.log10()).powf(T::from(-0.445).unwrap())*df.powf(T::from(-1.39).unwrap())
    };
    let g = |fp: T, d: T| {
        T::FRAC_2_PI()*(v(d)*(fp.recip() - (half - df).recip())).atan()
    };
    let h = |fp: T, c: T| {
        T::FRAC_2_PI()*(c/df*(fp.recip() - (half - df).recip())).atan()
    };

    let nc = (T::from(1.101).unwrap()*(-(two*dp).log10()).powf(T::from(1.1).unwrap())/df + one).ceil();
    let n3 = (nc*(g(fp, dp) + g(half - fs, dp) + one)/three).ceil();
    let nm = T::from(0.52).unwrap()*(dp/ds).log10()/df*(-dp.log10()).powf(T::from(0.17).unwrap());
    let dn = (nm*(h(fp, T::from(1.1).unwrap()) - (h(half - fs, T::from(0.29).unwrap()) - one)*half)).ceil();
    
    NumCast::from(n3 + dn).unwrap()
}