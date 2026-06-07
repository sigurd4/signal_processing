use core::ops::DerefMut;

use crate::quantities::{MaybeLists, Polynomial};

impl<T, C> DerefMut for Polynomial<T, C>
where
    C: MaybeLists<T>
{
    fn deref_mut(&mut self) -> &mut Self::Target
    {
        &mut self.c
    }
}