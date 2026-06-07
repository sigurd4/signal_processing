use core::borrow::BorrowMut;

use crate::quantities::{MaybeList, SumSequence};

impl<T, S> BorrowMut<S> for SumSequence<T, S>
where
    S: MaybeList<T>
{
    fn borrow_mut(&mut self) -> &mut S
    {
        &mut self.s
    }
}