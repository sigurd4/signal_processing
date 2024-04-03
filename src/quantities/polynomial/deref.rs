use core::ops::Deref;

use crate::{MaybeLists, Polynomial};

impl<T, C> Deref for Polynomial<T, C>
where
    C: MaybeLists<T>
{
    type Target = C;

    fn deref(&self) -> &Self::Target
    {
        &self.c
    }
}