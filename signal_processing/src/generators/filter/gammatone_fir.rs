use num::{complex::ComplexFloat, traits::FloatConst, Float, NumCast};
use option_trait::{Maybe, StaticMaybe};
use thiserror::Error;

use crate::{quantities::OwnedList, systems::Tf, util, System};

#[derive(Debug, Clone, Copy, PartialEq, Error)]
pub enum GammatoneError
{
    #[error("Frequency must be positive, and if the filter is digital; less than 1/2 the sampling frequency, or if no sampling frequency is specified, between 0 and 1.")]
    FrequencyOutOfRange,
    #[error("Sampling frequency must be a positive number.")]
    InvalidSamplingFrequency,
    #[error("Order must be an integer in the range [1, 24].")]
    OrderOutOfRange
}

pub trait GammatoneFir<N>: System + Sized
where
    N: Maybe<usize>
{
    fn gammatone_fir<O, FS>(
        numtaps: N,
        order: O,
        frequency: <Self::Set as ComplexFloat>::Real,
        sampling_frequency: FS
    ) -> Result<Self, GammatoneError>
    where
        O: Maybe<usize>,
        FS: Maybe<<Self::Set as ComplexFloat>::Real>;
}

impl<T, B, N> GammatoneFir<N> for Tf<T, B>
where
    T: Float + FloatConst,
    B: OwnedList<T>,
    <B::Length as StaticMaybe<usize>>::Opposite: StaticMaybe<usize, Maybe<N> = N> + Sized,
    N: StaticMaybe<N> + StaticMaybe<usize>,
    [(); B::LENGTH - 1]:
{
    fn gammatone_fir<O, FS>(
        numtaps: N,
        order: O,
        frequency: T,
        sampling_frequency: FS
    ) -> Result<Self, GammatoneError>
    where
        O: Maybe<usize>,
        FS: Maybe<T>
    {
        let zero = T::zero();
        let one = T::one();
        let two = one + one;

        let fs = if let Some(fs) = sampling_frequency.into_option()
        {
            if !(fs > zero)
            {
                return Err(GammatoneError::InvalidSamplingFrequency)
            }
            fs
        }
        else
        {
            two
        };

        if !(zero < frequency && frequency < fs/two)
        {
            return Err(GammatoneError::FrequencyOutOfRange)
        }

        let order = if let Some(order) = order.into_option()
        {
            if !(0 < order && order <= 24)
            {
                return Err(GammatoneError::OrderOutOfRange)
            }
            order
        }
        else
        {
            4
        };
        
        let numtaps = numtaps.into_option()
            .or_else(|| <B::Length as StaticMaybe<usize>>::Opposite::maybe_from_fn(|| {
                    <usize as NumCast>::from(fs * T::from(0.015).unwrap()).unwrap().max(15)
                }).into_option()
            ).unwrap_or(B::LENGTH);

        let hz_to_erb = |hz: T| {
            const EAR_Q: f64 = 9.26449;
            const MIN_BW: f64 = 24.7;
            hz/T::from(EAR_Q).unwrap() + T::from(MIN_BW).unwrap()
        };

        let bw = T::from(1.019).unwrap()*hz_to_erb(frequency);
        
        let scale_factor = two*(T::TAU()*bw).powi(order as i32)/util::factorial(order - 1)/fs;

        Ok(Tf::new(B::from_len_fn(StaticMaybe::maybe_from_fn(|| numtaps), |i| {
            let i = T::from(i).unwrap();
            let t = i/fs;

            t.powi(order as i32 - 1)*(-T::TAU()*bw*t).exp()*(T::TAU()*frequency*t).cos()*scale_factor
        }), ()))
    }
}

#[cfg(test)]
mod test
{
    use array_math::ArrayOps;

    use crate::{analysis::RealFreqZ, gen::filter::GammatoneFir, plot, systems::{Tf, Zpk}, transforms::system::ToZpk, Plane};

    #[test]
    fn test()
    {
        let h: Tf<_, Vec<_>> = Tf::gammatone_fir(16, (), 440.0, 16000.0)
            .unwrap();

        println!("{:?}", h);

        const N: usize = 1024;
        let (h_f, w): ([_; N], _) = h.real_freqz(());

        plot::plot_curves("H(e^jw)", "plots/h_z_gammatone_fir.png", [&w.zip(h_f.map(|h| h.norm()))])
            .unwrap();

        let h: Zpk<_, Vec<_>, Vec<_>, _> = h.to_zpk((), ());

        plot::plot_pz("H(z)", "plots/pz_z_gammatone_fir.png", &h.p, &h.z, Plane::Z)
            .unwrap();
    }
}