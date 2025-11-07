use core::ops::MulAssign;

use num::{complex::ComplexFloat, traits::FloatConst, Float};
use option_trait::Maybe;
use thiserror::Error;

use crate::{systems::Tf, System};

#[derive(Debug, Clone, Copy, PartialEq, Error)]
pub enum SingleFreqFilterError
{
    #[error("Frequency must be positive, and if the filter is digital; less than 1/2 the sampling frequency, or if no sampling frequency is specified, between 0 and 1.")]
    FrequencyOutOfRange,
    #[error("Sampling frequency must be a positive number.")]
    InvalidSamplingFrequency
}

pub trait IirNotch: System + Sized
{
    fn iir_notch<FS>(
        frequency: <Self::Set as ComplexFloat>::Real,
        quality_factor: <Self::Set as ComplexFloat>::Real,
        sampling_frequency: FS
    ) -> Result<Self, SingleFreqFilterError>
    where
        FS: Maybe<<Self::Set as ComplexFloat>::Real>;
}

impl<T> IirNotch for Tf<T, [T; 3], [T; 3]>
where
    T: Float + FloatConst + MulAssign
{
    fn iir_notch<FS>(
        frequency: T,
        quality_factor: T,
        sampling_frequency: FS
    ) -> Result<Self, SingleFreqFilterError>
    where
        FS: Maybe<T>
    {
        let zero = T::zero();
        let one = T::one();
        let two = one + one;

        let fs = if let Some(fs) = sampling_frequency.into_option()
        {
            if !(fs > zero)
            {
                return Err(SingleFreqFilterError::InvalidSamplingFrequency)
            }
            fs
        }
        else
        {
            two
        };

        if !(zero < frequency && frequency < fs/two)
        {
            return Err(SingleFreqFilterError::FrequencyOutOfRange)
        }

        let mut w0 = frequency/fs*two;
        let bw = quality_factor/w0*T::PI();
        w0 *= T::PI();

        let gb = T::FRAC_1_SQRT_2();
        let beta = ((one - gb*gb).sqrt()/gb)*(bw/two).tan();
        let gain = one/(one + beta);

        Ok(Tf::new(
            [
                gain,
                -two*gain*w0.cos(),
                gain
            ],
            [
                one,
                -two*gain*w0.cos(),
                two*gain - one
            ]
        ))
    }
}

#[cfg(test)]
mod test
{
    use array_math::ArrayOps;

    use crate::{analysis::RealFreqZ, plot, systems::{Tf, Zpk}, transforms::system::ToZpk, Plane};

    use super::IirNotch;

    #[test]
    fn test()
    {
        let h = Tf::iir_notch(440.0, 30.0, 16000.0)
            .unwrap();

        const N: usize = 1024;
        let (h_f, w): ([_; N], _) = h.real_freqz(());

        plot::plot_curves("H(e^jw)", "plots/h_z_iir_notch.png", [&w.zip(h_f.map(|h| h.norm()))])
            .unwrap();

        let h: Zpk<_, Vec<_>, Vec<_>, _> = h.to_zpk((), ());

        plot::plot_pz("H(z)", "plots/pz_z_iir_notch.png", &h.p, &h.z, Plane::Z)
            .unwrap();
    }
}