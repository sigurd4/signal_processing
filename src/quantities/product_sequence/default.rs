use crate::{MaybeList, ProductSequence};

impl<T, S> Default for ProductSequence<T, S>
where
    S: MaybeList<T>,
    ProductSequence<T, ()>: Into<Self>
{
    fn default() -> Self
    {
        ProductSequence::new(()).into()
    }
}