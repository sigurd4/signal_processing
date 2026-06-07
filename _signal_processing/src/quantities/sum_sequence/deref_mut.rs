use core::ops::DerefMut;

use crate::quantities::{MaybeList, SumSequence};

impl<T, S> DerefMut for SumSequence<T, S>
where
    S: MaybeList<T>
{
    fn deref_mut(&mut self) -> &mut Self::Target
    {
        &mut self.s
    }
}