use core::ops::{DivAssign, MulAssign};

use array_math::ArrayMath;
use num::{traits::FloatConst, Float, NumCast};

use crate::{validate_filter_bands, FilterBandError, FilterGenPlane, FilterGenType};

pub fn ellipord<T, const F: usize>(
    mut passband_frequencies: [T; F],
    mut stopband_frequencies: [T; F],
    passband_ripple: T,
    stopband_attenuation: T,
    plane: FilterGenPlane<T>
) -> Result<(usize, [T; F], [T; F], T, T, FilterGenType), FilterBandError>
where
    T: Float + FloatConst + MulAssign + DivAssign,
    [(); F - 1]:,
    [(); 2 - F]:
{
    let two = T::from(2.0).unwrap();
    let t = if let FilterGenPlane::Z { sampling_frequency } = plane
    {
        let t = sampling_frequency.unwrap_or(two);
        for wc in passband_frequencies.iter_mut()
        {
            if *wc > t/two
            {
                return Err(FilterBandError::EdgesOutOfRange)
            }
            *wc = two/t*(T::PI()**wc/t).tan()
        }
        for wc in stopband_frequencies.iter_mut()
        {
            if *wc > t/two
            {
                return Err(FilterBandError::EdgesOutOfRange)
            }
            *wc = two/t*(T::PI()**wc/t).tan()
        }
        Some(t)
    }
    else
    {
        None
    };

    validate_filter_bands(&passband_frequencies, &stopband_frequencies, t)?;

    let filter_type;
    let mut wp;
    let mut ws;

    let pp = passband_frequencies.product();
    let ps = stopband_frequencies.product();

    let one = T::one();

    if F == 2
    {
        if passband_frequencies[0] > stopband_frequencies[0]
        {
            filter_type = FilterGenType::BandPass;

            if pp < ps
            {
                stopband_frequencies[1] = pp/stopband_frequencies[0]
            }
            else
            {
                stopband_frequencies[0] = pp/stopband_frequencies[1]
            }

            wp = passband_frequencies[1] - passband_frequencies[0];
            ws = stopband_frequencies[1] - stopband_frequencies[0];
        }
        else
        {
            filter_type = FilterGenType::BandStop;

            if pp > ps
            {
                passband_frequencies[1] = ps/passband_frequencies[0]
            }
            else
            {
                passband_frequencies[0] = ps/passband_frequencies[1]
            }

            wp = pp/(passband_frequencies[1] - passband_frequencies[0]);
            ws = pp/(stopband_frequencies[1] - stopband_frequencies[0]);
        }

        ws /= wp;
        wp = one;
    }
    else if passband_frequencies[0] > stopband_frequencies[0]
    {
        filter_type = FilterGenType::HighPass;

        wp = stopband_frequencies[0];
        ws = passband_frequencies[0];
    }
    else
    {
        filter_type = FilterGenType::LowPass;

        wp = passband_frequencies[0];
        ws = stopband_frequencies[0];
    }

    let half = two.recip();
    let ten = T::from(10u8).unwrap();
    let fifteen = ten + ten*half;
    let sixteen = fifteen + one;
    let one_hundred_and_fifty = fifteen*ten;

    let k = wp/ws;
    let k1 = (one - k*k).sqrt();
    let k1_sqrt = k1.sqrt();
    let q0 = half*((-k1_sqrt + one) / (k1_sqrt + one));
    let q0_p4 = q0*q0*q0*q0;
    let q = q0*(one + q0_p4*(two + q0_p4*(fifteen + q0_p4*one_hundred_and_fifty)));
    let d = (ten.powf(stopband_attenuation/ten) - one)/(ten.powf(passband_ripple/ten) - one);
    
    let nf = ((sixteen*d).log10()/q.recip().log10()).ceil()
        .max(T::zero())
        .min(T::from(usize::MAX).unwrap());
    let n = NumCast::from(nf).unwrap();

    if let Some(t) = t
    {
        Ok((n, passband_frequencies.map(|w| (w*t/two).atan()*two/T::PI()), stopband_frequencies.map(|w| (w*t/two).atan()*two/T::PI()), passband_ripple, stopband_attenuation, filter_type))
    }
    else
    {
        Ok((n, passband_frequencies, stopband_frequencies, passband_ripple, stopband_attenuation, filter_type))
    }
}