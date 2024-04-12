use crate::{MaybeList, SumSequence};

impl<T, S> Default for SumSequence<T, S>
where
    S: MaybeList<T>,
    SumSequence<T, ()>: Into<Self>
{
    fn default() -> Self
    {
        SumSequence::new(()).into()
    }
}