use thiserror::Error;

#[derive(Debug, Clone, Copy, PartialEq, Error)]
pub enum FilterBandError
{
    #[error("Bands must be monotonic starting at zero.")]
    EdgesNotNondecreasing,
    #[error("Band edges must be less than 1/2 the sampling frequency, or if no sampling frequency is specified, between 0 and 1.")]
    EdgesOutOfRange,
    #[error("Sampling frequency must be a positive number.")]
    InvalidSamplingFrequency,
    #[error("One band must surround the other.")]
    BandNotSurrounding,
}