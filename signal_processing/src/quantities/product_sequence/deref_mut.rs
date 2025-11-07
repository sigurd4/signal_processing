use core::ops::DerefMut;

use crate::quantities::{MaybeList, ProductSequence};

impl<T, S> DerefMut for ProductSequence<T, S>
where
    S: MaybeList<T>
{
    fn deref_mut(&mut self) -> &mut Self::Target
    {
        &mut self.s
    }
}