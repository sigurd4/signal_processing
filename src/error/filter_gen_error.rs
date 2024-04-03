use thiserror::Error;

#[derive(Debug, Clone, Copy, PartialEq, Error)]
pub enum FilterGenError
{
    #[error("Filter order must be at least 1.")]
    ZeroOrder,
    #[error("Frequencies must be monotonic starting at zero.")]
    FrequenciesNotNondecreasing,
    #[error("Frequencies must be positive, and if the filter is digital; less than 1/2 the sampling frequency, or if no sampling frequency is specified, between 0 and 1.")]
    FrequenciesOutOfRange,
    #[error("List of frequencies and list of magnitudes must have equal length.")]
    FrequenciesAndMagnitudesDifferentLength,
}