use core::ops::DivAssign;

use num::{traits::FloatConst, Float, NumCast};
use option_trait::Maybe;

use crate::{FilterBandError, Fir1Type, ListOrSingle};

pub fn kaiserord<T, FS, const F: usize, const D: usize>(
    frequencies: [T; F],
    magnitudes: [bool; (F + 2)/2],
    deviations: [T; D],
    sampling_frequency: FS
) -> Result<(usize, Vec<T>, T, Fir1Type), FilterBandError>
where
    T: Float + FloatConst + DivAssign,
    FS: Maybe<T>,
    [(); F - 2]:,
    [(); 0 - F % 2]:,
    [(); (D == 1) as usize + (D == (F + 2)/2) as usize - 1]:
{
    let zero = T::zero();
    let one = T::one();
    let two = one + one;
    let half = two.recip();

    let mut f: Vec<T> = frequencies.into_vec();
    if let Some(fs) = sampling_frequency.into_option()
    {
        let nyq = fs*half;
        if !(fs > zero) || !fs.is_finite()
        {
            return Err(FilterBandError::InvalidSamplingFrequency)
        }
        for f in f.iter_mut()
        {
            *f /= nyq
        }
    }

    let mut m: Vec<bool> = magnitudes.into_vec();
    let mut dev: Vec<_> = deviations.into_vec()
        .into_iter()
        .map(|dev: T| dev.abs())
        .collect();
    let mut i = 0;
    while i + 1 < m.len()
    {
        if m[i] == m[i + 1]
        {
            m.remove(i);
            f.remove(i*2);
            f.remove(i*2 + 1);
            dev[i] = (dev[i] + dev.remove(i + 1))/two
        }
        else
        {
            i += 1;
        }
    }

    let w: Vec<_> = f.iter()
        .zip(f[1..].iter())
        .map(|(&f1, &f2)| (f1 + f2)/two)
        .collect();

    let ftype = if w.len() == 1
    {
        if m[0] {Fir1Type::LowPass} else {Fir1Type::HighPass}
    }
    else if w.len() == 2
    {
        if m[0] {Fir1Type::BandStop} else {Fir1Type::BandPass}
    }
    else
    {
        if m[0] {Fir1Type::DC1} else {Fir1Type::DC0}
    };

    let dev = dev.into_iter()
        .reduce(Float::min)
        .unwrap();

    let a = -T::from(20u8).unwrap()*dev.log10();
    let beta = if a > T::from(50u8).unwrap()
    {
        T::from(0.1102).unwrap()*(a - T::from(8.7).unwrap())
    }
    else if a >= T::from(21u8).unwrap()
    {
        let am21 = a - T::from(21).unwrap();
        T::from(0.5842).unwrap()*am21.powf(T::from(0.4).unwrap()) + T::from(0.07886).unwrap()*am21
    }
    else
    {
        zero
    };

    let dw = T::PI()*f.iter()
        .zip(f[1..].iter())
        .map(|(&f1, &f2)| f2 - f1)
        .reduce(Float::min)
        .unwrap_or_else(T::zero);
    let mut n = <usize as NumCast>::from(
            (((a - T::from(8u8).unwrap())/(T::from(2.285).unwrap()*dw)).max(one))
                .ceil()
        ).unwrap();

    if m[0] == (w.len() % 2 == 0) && n % 2 == 1
    {
        n += 1
    }

    Ok((n, w, beta, ftype))
}

#[cfg(test)]
mod test
{
    use array_math::ArrayOps;

    use crate::{plot, window::{Kaiser, WindowGen, WindowRange}, Fir1, RealFreqZ, Tf};

    #[test]
    fn test()
    {
        let (n, f, beta, t) = crate::kaiserord([1000.0, 1500.0], [true, false], [0.05, 0.01], 8000.0)
            .unwrap();

        let w: Vec<_> = Kaiser {beta}
            .window_gen(n + 1, WindowRange::Symmetric);

        let h = Tf::fir1((), f, t, w, true, ())
            .unwrap();

        const N: usize = 1024;
        let (h_f, w): ([_; N], _) = h.real_freqz(());

        plot::plot_curves("H(e^jw)", "plots/h_z_kaiserord.png", [&w.zip(h_f.map(|h_f| h_f.norm()))])
            .unwrap()
    }
}