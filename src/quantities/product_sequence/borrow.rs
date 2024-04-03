use core::borrow::Borrow;

use crate::{MaybeList, ProductSequence};

impl<T, S> Borrow<S> for ProductSequence<T, S>
where
    S: MaybeList<T>
{
    fn borrow(&self) -> &S
    {
        &self.s
    }
}