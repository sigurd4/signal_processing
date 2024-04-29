use core::ops::Deref;

use crate::quantities::{MaybeList, ProductSequence};

impl<T, S> Deref for ProductSequence<T, S>
where
    S: MaybeList<T>
{
    type Target = S;

    fn deref(&self) -> &Self::Target
    {
        &self.s
    }
}