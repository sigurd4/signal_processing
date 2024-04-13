use core::fmt::Debug;

use crate::{MaybeList, SumSequence};

impl<T, S> Debug for SumSequence<T, S>
where
    S: MaybeList<T> + Debug
{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result
    {
        self.s.fmt(f)
    }
}