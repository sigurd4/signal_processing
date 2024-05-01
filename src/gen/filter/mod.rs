use num::Float;
use thiserror::Error;

moddef::moddef!(
    flat(pub) mod {
        besselap,
        besself,
        buttap,
        butter,
        buttord,
        cheb1ap,
        cheb1ord,
        cheb2ap,
        cheb2ord,
        cheby1,
        cheby2,
        ellip,
        ellipap,
        ellipord,
        fir1,
        fir2,
        firgr,
        firls,
        firpm,
        firpmord,
        gammatone_fir,
        gammatone_iir,
        iir_design,
        iir_notch,
        iir_peak,
        kaiserord,
        pei_tseng_notch,
        qp_kaiser,
        sgolay
    }
);

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum FilterGenType
{
    LowPass,
    HighPass,
    BandPass,
    BandStop
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum FilterClassType
{
    Symmetric,
    Antisymmetric,
    Differentiator
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum FilterGenPlane<T>
where
    T: Float
{
    S,
    Z {
        sampling_frequency: Option<T>
    }
}

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