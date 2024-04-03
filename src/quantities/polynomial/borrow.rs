use core::borrow::Borrow;

use crate::{MaybeLists, Polynomial};

impl<T, C> Borrow<C> for Polynomial<T, C>
where
    C: MaybeLists<T>
{
    fn borrow(&self) -> &C
    {
        &self.c
    }
}