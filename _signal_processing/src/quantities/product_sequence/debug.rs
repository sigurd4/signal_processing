use core::fmt::Debug;

use crate::quantities::{MaybeList, ProductSequence};

impl<T, S> Debug for ProductSequence<T, S>
where
    S: MaybeList<T> + Debug
{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result
    {
        self.s.fmt(f)
    }
}