use core::ops::{Sub, SubAssign};

use crate::{MaybeLists, Polynomial};

impl<T, C, Rhs> SubAssign<Rhs> for Polynomial<T, C>
where
    C: MaybeLists<T>,
    for<'a> C::View<'a>: MaybeLists<T>,
    for<'a> Polynomial<T, C::View<'a>>: Sub<Rhs, Output = Self>
{
    fn sub_assign(&mut self, rhs: Rhs)
    {
        *self = self.as_view() - rhs
    }
}