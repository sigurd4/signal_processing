use core::array::TryFromSliceError;

use crate::quantities::ProductSequence;

impl<T1, T2, const M: usize> TryFrom<ProductSequence<T1, Vec<T1>>> for ProductSequence<T2, [T2; M]>
where
    T1: Into<T2>
{
    type Error = Vec<T1>;

    fn try_from(s: ProductSequence<T1, Vec<T1>>) -> Result<Self, Self::Error>
    {
        Ok(ProductSequence::new(
            <[T1; M]>::try_from(s.s)?
                .map(Into::into)
        ))
    }
}
impl<T1, T2, const M: usize> TryFrom<ProductSequence<T1, &[T1]>> for ProductSequence<T2, [T2; M]>
where
    T1: Clone + Into<T2>
{
    type Error = TryFromSliceError;

    fn try_from(s: ProductSequence<T1, &[T1]>) -> Result<Self, Self::Error>
    {
        Ok(ProductSequence::new(
            <&[T1; M]>::try_from(s.s)?
                .clone()
                .map(Into::into)
        ))
    }
}
impl<'a, T, const M: usize> TryFrom<ProductSequence<T, &'a [T]>> for ProductSequence<T, &'a [T; M]>
{
    type Error = TryFromSliceError;

    fn try_from(s: ProductSequence<T, &'a [T]>) -> Result<Self, Self::Error>
    {
        Ok(ProductSequence::new(
            <&'a [T; M]>::try_from(s.s)?
        ))
    }
}