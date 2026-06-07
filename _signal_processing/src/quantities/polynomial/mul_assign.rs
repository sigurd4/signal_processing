use core::ops::{Mul, MulAssign};

use crate::quantities::{MaybeLists, Polynomial};

impl<T, C, Rhs> MulAssign<Rhs> for Polynomial<T, C>
where
    C: MaybeLists<T>,
    for<'a> C::View<'a>: MaybeLists<T>,
    for<'a> Polynomial<T, C::View<'a>>: Mul<Rhs, Output = Self>
{
    fn mul_assign(&mut self, rhs: Rhs)
    {
        *self = self.as_view()*rhs
    }
}