use core::borrow::BorrowMut;

use crate::quantities::{MaybeLists, Polynomial};

impl<T, C> BorrowMut<C> for Polynomial<T, C>
where
    C: MaybeLists<T>
{
    fn borrow_mut(&mut self) -> &mut C
    {
        &mut self.c
    }
}