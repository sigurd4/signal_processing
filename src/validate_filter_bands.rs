use num::Float;

use crate::FilterBandError;

pub(crate) fn validate_filter_bands<T, const F: usize>(
    passband_frequencies: &[T; F],
    stopband_frequencies: &[T; F],
    sampling_frequency: Option<T>
) -> Result<(), FilterBandError>
where
    T: Float
{
    if let Some(sampling_frequency) = sampling_frequency
    {
        if !(sampling_frequency > T::zero()) || !sampling_frequency.is_finite()
        {
            return Err(FilterBandError::InvalidSamplingFrequency)
        }
    }
    if !passband_frequencies.is_sorted()
    {
        return Err(FilterBandError::EdgesNotNondecreasing)
    }
    if !stopband_frequencies.is_sorted()
    {
        return Err(FilterBandError::EdgesNotNondecreasing)
    }
    if passband_frequencies.iter()
        .any(|f| *f < T::zero() || *f > T::one())
    {
        return Err(FilterBandError::EdgesOutOfRange)
    }
    if stopband_frequencies.iter()
        .any(|f| *f < T::zero() || *f > T::one())
    {
        return Err(FilterBandError::EdgesOutOfRange)
    }

    Ok(())
}