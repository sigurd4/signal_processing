use crate::quantities::ProductSequence;

impl<T1, T2> From<ProductSequence<T1, ()>> for ProductSequence<T2, [T2; 0]>
{
    fn from(_: ProductSequence<T1, ()>) -> Self
    {
        Self::new(
            []
        )
    }
}
impl<T1, T2> From<ProductSequence<T1, ()>> for ProductSequence<T2, &[T2; 0]>
{
    fn from(_: ProductSequence<T1, ()>) -> Self
    {
        Self::new(
            &[]
        )
    }
}
impl<T1, T2> From<ProductSequence<T1, ()>> for ProductSequence<T2, Vec<T2>>
{
    fn from(_: ProductSequence<T1, ()>) -> Self
    {
        Self::new(
            vec![]
        )
    }
}
impl<T1, T2> From<ProductSequence<T1, ()>> for ProductSequence<T2, &[T2]>
{
    fn from(_: ProductSequence<T1, ()>) -> Self
    {
        Self::new(
            &[]
        )
    }
}

impl<T1, T2, const M: usize> From<ProductSequence<T1, [T1; M]>> for ProductSequence<T2, Vec<T2>>
where
    T1: Into<T2>
{
    fn from(s: ProductSequence<T1, [T1; M]>) -> Self
    {
        Self::new(
            s.s.into_iter()
                .map(Into::into)
                .collect()
        )
    }
}

impl<T1, T2, const M: usize> From<ProductSequence<T1, &[T1; M]>> for ProductSequence<T2, [T2; M]>
where
    T1: Clone + Into<T2>
{
    fn from(s: ProductSequence<T1, &[T1; M]>) -> Self
    {
        Self::new(
            s.s.clone()
                .map(Into::into)
        )
    }
}
impl<T1, T2, const M: usize> From<ProductSequence<T1, &[T1; M]>> for ProductSequence<T2, Vec<T2>>
where
    T1: Clone + Into<T2>
{
    fn from(s: ProductSequence<T1, &[T1; M]>) -> Self
    {
        Self::new(
            s.s.iter()
                .map(|s| s.clone().into())
                .collect()
        )
    }
}
impl<'a, T, const M: usize> From<ProductSequence<T, &'a [T; M]>> for ProductSequence<T, &'a [T]>
{
    fn from(s: ProductSequence<T, &'a [T; M]>) -> Self
    {
        Self::new(
            s.s.as_slice()
        )
    }
}

impl<T1, T2> From<ProductSequence<T1, &[T1]>> for ProductSequence<T2, Vec<T2>>
where
    T1: Clone + Into<T2>
{
    fn from(s: ProductSequence<T1, &[T1]>) -> Self
    {
        Self::new(
            s.s.iter()
                .map(|s| s.clone().into())
                .collect()
        )
    }
}