


use crate::ProductSequence;

impl<T> From<ProductSequence<T, ()>> for ProductSequence<T, [T; 0]>
{
    fn from(_: ProductSequence<T, ()>) -> Self
    {
        Self::new(
            []
        )
    }
}
impl<T> From<ProductSequence<T, ()>> for ProductSequence<T, &[T; 0]>
{
    fn from(_: ProductSequence<T, ()>) -> Self
    {
        Self::new(
            &[]
        )
    }
}
impl<T> From<ProductSequence<T, ()>> for ProductSequence<T, Vec<T>>
{
    fn from(_: ProductSequence<T, ()>) -> Self
    {
        Self::new(
            vec![]
        )
    }
}
impl<T> From<ProductSequence<T, ()>> for ProductSequence<T, &[T]>
{
    fn from(_: ProductSequence<T, ()>) -> Self
    {
        Self::new(
            &[]
        )
    }
}

impl<T, const M: usize> From<ProductSequence<T, [T; M]>> for ProductSequence<T, Vec<T>>
{
    fn from(s: ProductSequence<T, [T; M]>) -> Self
    {
        Self::new(
            s.s.into_iter()
                .collect()
        )
    }
}

impl<T, const M: usize> From<ProductSequence<T, &[T; M]>> for ProductSequence<T, [T; M]>
where
    T: Clone
{
    fn from(s: ProductSequence<T, &[T; M]>) -> Self
    {
        Self::new(
            s.s.clone()
        )
    }
}
impl<T, const M: usize> From<ProductSequence<T, &[T; M]>> for ProductSequence<T, Vec<T>>
where
    T: Clone
{
    fn from(s: ProductSequence<T, &[T; M]>) -> Self
    {
        Self::new(
            s.s.to_vec()
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

impl<T> From<ProductSequence<T, &[T]>> for ProductSequence<T, Vec<T>>
where
    T: Clone
{
    fn from(s: ProductSequence<T, &[T]>) -> Self
    {
        Self::new(
            s.s.to_vec()
        )
    }
}