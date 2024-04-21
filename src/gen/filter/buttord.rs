use core::ops::{DivAssign, MulAssign};


use array_math::{ArrayMath, ArrayOps};
use num::{traits::FloatConst, Float, NumCast};

use crate::{validate_filter_bands, FilterGenPlane, FilterGenType, FilterBandError};

pub fn buttord<T, const F: usize>(
    mut passband_frequencies: [T; F],
    mut stopband_frequencies: [T; F],
    passband_ripple: T,
    stopband_attenuation: T,
    plane: FilterGenPlane<T>
) -> Result<(usize, [T; F], [T; F], FilterGenType), FilterBandError>
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

    let ten = T::from(10.0).unwrap();
    let half = T::from(0.5).unwrap();
    let qs = (ten.powf(stopband_attenuation/ten) - one).ln();
    let qp = (ten.powf(passband_ripple/ten) - one).ln();
    let nm = half*(qs - qp);
    let nf = (nm/(ws/wp).ln()).ceil()
        .max(T::zero())
        .min(T::from(usize::MAX).unwrap());
    let n = <usize as NumCast>::from(nf).unwrap();

    let wcw_p;
    let wcw_s;

    match filter_type
    {
        FilterGenType::LowPass => {
            wcw_p = passband_frequencies.map(|wpw| (wpw.ln() - qp/two/nf).exp());
            wcw_s = stopband_frequencies.map(|wsw| (wsw.ln() - qs/two/nf).exp());
        },
        FilterGenType::HighPass => {
            wcw_p = passband_frequencies.map(|wpw| (wpw.ln() + qp/two/nf).exp());
            wcw_s = stopband_frequencies.map(|wsw| (wsw.ln() + qs/two/nf).exp());
        },
        FilterGenType::BandPass | FilterGenType::BandStop => {
            let w_prime_p;
            let w_prime_s;

            if filter_type == FilterGenType::BandPass
            {
                w_prime_p = passband_frequencies.map(|wpw| (wpw.ln() - qp/two/nf).exp());
                w_prime_s = stopband_frequencies.map(|wsw| (wsw.ln() - qs/two/nf).exp());
            }
            else
            {
                w_prime_p = passband_frequencies.map(|wpw| (wpw.ln() + qp/two/nf).exp());
                w_prime_s = stopband_frequencies.map(|wsw| (wsw.ln() + qs/two/nf).exp());
            }

            let four = two + two;

            let w0 = pp.sqrt();
            let q = w0/(passband_frequencies[1] - passband_frequencies[0]);
            let wc = &passband_frequencies;
            let w_prime = w_prime_p[0]/wc[0];
            let s = (w_prime*w_prime + four*q*q).sqrt();
            let d = two*q/w0;
            let wa = (w_prime + s).abs()/d;
            let wb = (w_prime - s).abs()/d;
            wcw_p = [wb, wa].try_reformulate_length().map_err(|_| ()).unwrap();
            
            let w0 = ps.sqrt();
            let q = w0/(stopband_frequencies[1] - stopband_frequencies[0]);
            let wc = &stopband_frequencies;
            let w_prime = w_prime_s[0]/wc[0];
            let s = (w_prime*w_prime + four*q*q).sqrt();
            let d = two*q/w0;
            let wa = (w_prime + s).abs()/d;
            let wb = (w_prime - s).abs()/d;
            wcw_s = [wb, wa].try_reformulate_length().map_err(|_| ()).unwrap();
        },
    }

    if let Some(t) = t
    {
        Ok((n, wcw_p.map(|w| (w*t/two).atan()*two/T::PI()), wcw_s.map(|w| (w*t/two).atan()*two/T::PI()), filter_type))
    }
    else
    {
        Ok((n, wcw_p, wcw_s, filter_type))
    }
}