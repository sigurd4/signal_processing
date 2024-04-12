use core::borrow::Borrow;

use crate::{MaybeList, SumSequence};

impl<T, S> Borrow<S> for SumSequence<T, S>
where
    S: MaybeList<T>
{
    fn borrow(&self) -> &S
    {
        &self.s
    }
}