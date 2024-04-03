use thiserror::Error;

#[derive(Debug, Clone, Copy, PartialEq, Error)]
pub enum ComplexRealError
{
    #[error("Tolerance must be a number in the range [0.0, 1.0].")]
    TolaranceOutOfRange,
    #[error("Complex roots and/or poles did not come in conjugate pairs. Something is wrong with this system.")]
    OddNumberComplex
}