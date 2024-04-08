use core::ops::{Add, AddAssign};

use crate::{MaybeLists, Polynomial};

impl<T, C, Rhs> AddAssign<Rhs> for Polynomial<T, C>
where
    C: MaybeLists<T>,
    for<'a> C::View<'a>: MaybeLists<T>,
    for<'a> Polynomial<T, C::View<'a>>: Add<Rhs, Output = Self>
{
    fn add_assign(&mut self, rhs: Rhs)
    {
        *self = self.as_view() + rhs
    }
}