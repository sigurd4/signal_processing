use core::ops::{DivAssign, MulAssign};

use ndarray::{Array1, Array2};
use ndarray_linalg::{Lapack, Solve};
use num::{complex::ComplexFloat, traits::FloatConst, Float};
use option_trait::Maybe;
use thiserror::Error;

use crate::{quantities::List, System, systems::Tf};

#[derive(Debug, Clone, Copy, PartialEq, Error)]
pub enum PeiTsengNotchError
{
    #[error("Frequencies must be less than 1/2 the sampling frequency, or if no sampling frequency is specified, between 0 and 1.")]
    FrequenciesOutOfRange,
    #[error("Bandwiths must be less than 1/2 the sampling frequency, or if no sampling frequency is specified, between 0 and 1.")]
    BandwidthsOutOfRange,
    #[error("Sampling frequency must be a positive number.")]
    InvalidSamplingFrequency,
}

pub trait PeiTsengNotch<B>: System + Sized
where
    B: List<(<Self::Domain as ComplexFloat>::Real, <Self::Domain as ComplexFloat>::Real)>
{
    fn pei_tseng_notch<FS>(bands: B, sampling_frequency: FS) -> Result<Self, PeiTsengNotchError>
    where
        FS: Maybe<<Self::Domain as ComplexFloat>::Real>;
}

impl<T, BB> PeiTsengNotch<BB> for Tf<T, Vec<T>, Vec<T>>
where
    T: Float + FloatConst + DivAssign + MulAssign + Lapack,
    BB: List<(T, T)>
{
    fn pei_tseng_notch<FS>(bands: BB, sampling_frequency: FS) -> Result<Self, PeiTsengNotchError>
    where
        FS: Maybe<<Self::Domain as ComplexFloat>::Real>
    {
        let zero = T::zero();
        let one = T::one();
        let two = one + one;

        let mut bands = bands.into_vec();

        if let Some(fs) = sampling_frequency.into_option()
        {
            let nyq = fs/two;
            for (f, bw) in bands.iter_mut()
            {
                *f /= nyq;
                *bw /= nyq;
            }
        }
        for (f, bw) in bands.iter_mut()
        {
            if !(*f >= zero && *f <= one)
            {
                return Err(PeiTsengNotchError::FrequenciesOutOfRange)
            }
            if !(*bw >= zero && *bw <= one)
            {
                return Err(PeiTsengNotchError::BandwidthsOutOfRange)
            }
            *f *= T::PI();
            *bw *= T::PI();
        }

        let m2 = bands.len()*2;
        let m2f = T::from(m2).unwrap();

        let omega: Vec<_> = bands.iter()
            .map(|&(f, bw)| f - bw/two)
            .chain(bands.iter()
                .map(|(f, _)| *f)
            ).collect();
        let mfpi: Vec<_> = (1..=m2).step_by(2)
            .map(|i| -T::PI()*T::from(i).unwrap())
            .collect();
        let phi: Vec<_> = mfpi.iter()
            .map(|&mfpi| mfpi + T::FRAC_PI_2())
            .collect::<Vec<_>>()
            .into_iter()
            .chain(mfpi.into_iter())
            .collect();
        let t_beta: Vec<_> = omega.iter()
            .zip(phi)
            .map(|(&omega, phi)| Float::tan((phi + m2f*omega)/two))
            .collect();

        let q = Array2::from_shape_fn((m2, m2), |(i, k)| {
            let (s, c) = (T::from(k + 1).unwrap()*omega[i]).sin_cos();
            s - t_beta[i]*c
        });

        let h_a = q.solve(&Array1::from_vec(t_beta))
            .unwrap()
            .to_vec();

        let denom: Vec<_> = core::iter::once(one)
            .chain(h_a)
            .collect();
        let mut numer = denom.clone();
        numer.reverse();

        let a = denom.clone();
        let b = numer.into_iter()
            .zip(denom)
            .map(|(n, d)| (n + d)/two)
            .collect();

        Ok(Tf::new(b, a))
    }
}

#[cfg(test)]
mod test
{
    use array_math::ArrayOps;

    use crate::{plot, gen::filter::PeiTsengNotch, Plane, analysis::RealFreqZ, transforms::system::ToZpk, systems::{Tf, Zpk}};

    #[test]
    fn test()
    {
        let h = Tf::pei_tseng_notch([(0.5, 0.01), (0.7, 0.1)], ())
            .unwrap();

        const N: usize = 1024;
        let (h_f, w): ([_; N], _) = h.real_freqz(());

        plot::plot_curves("H(e^jw)", "plots/h_z_pei_tseng_notch.png", [&w.zip(h_f.map(|h| h.norm()))])
            .unwrap();

        let h: Zpk<_, Vec<_>, Vec<_>, _> = h.to_zpk((), ());

        plot::plot_pz("H(z)", "plots/pz_z_pei_tseng_notch.png", &h.p, &h.z, Plane::Z)
            .unwrap();
    }
}