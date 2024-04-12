use core::ops::Deref;

use crate::{MaybeList, SumSequence};

impl<T, S> Deref for SumSequence<T, S>
where
    S: MaybeList<T>
{
    type Target = S;

    fn deref(&self) -> &Self::Target
    {
        &self.s
    }
}