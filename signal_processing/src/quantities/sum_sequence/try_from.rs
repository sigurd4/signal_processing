use core::array::TryFromSliceError;

use crate::quantities::SumSequence;

impl<T1, T2, const M: usize> TryFrom<SumSequence<T1, Vec<T1>>> for SumSequence<T2, [T2; M]>
where
    T1: Into<T2>
{
    type Error = Vec<T1>;

    fn try_from(s: SumSequence<T1, Vec<T1>>) -> Result<Self, Self::Error>
    {
        Ok(SumSequence::new(
            <[T1; M]>::try_from(s.s)?
                .map(Into::into)
        ))
    }
}
impl<T1, T2, const M: usize> TryFrom<SumSequence<T1, &[T1]>> for SumSequence<T2, [T2; M]>
where
    T1: Clone + Into<T2>
{
    type Error = TryFromSliceError;

    fn try_from(s: SumSequence<T1, &[T1]>) -> Result<Self, Self::Error>
    {
        Ok(SumSequence::new(
            <&[T1; M]>::try_from(s.s)?
                .clone()
                .map(Into::into)
        ))
    }
}
impl<'a, T, const M: usize> TryFrom<SumSequence<T, &'a [T]>> for SumSequence<T, &'a [T; M]>
{
    type Error = TryFromSliceError;

    fn try_from(s: SumSequence<T, &'a [T]>) -> Result<Self, Self::Error>
    {
        Ok(SumSequence::new(
            <&'a [T; M]>::try_from(s.s)?
        ))
    }
}