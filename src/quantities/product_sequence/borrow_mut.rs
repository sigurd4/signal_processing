use core::borrow::BorrowMut;

use crate::{MaybeList, ProductSequence};

impl<T, S> BorrowMut<S> for ProductSequence<T, S>
where
    S: MaybeList<T>
{
    fn borrow_mut(&mut self) -> &mut S
    {
        &mut self.s
    }
}