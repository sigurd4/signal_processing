use core::fmt::Debug;

use crate::{MaybeLists, Polynomial};

impl<T, C> Debug for Polynomial<T, C>
where
    C: MaybeLists<T> + Debug
{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result
    {
        self.c.fmt(f)
    }
}