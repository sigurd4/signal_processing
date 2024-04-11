use std::ops::Mul;

use num::{complex::ComplexFloat, One};

use crate::{MaybeList, Tf};

impl<T, B, A> One for Tf<T, B, A>
where
    T: ComplexFloat,
    Self: Mul<Output = Self> + Default,
    B: MaybeList<T>,
    A: MaybeList<T>
{
    fn one() -> Self
    {
        Self::one()
    }
    fn is_one(&self) -> bool
    {
        self.is_one()
    }
}