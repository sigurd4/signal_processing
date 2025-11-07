use std::ops::Mul;

use num::{complex::ComplexFloat, One};
use option_trait::Maybe;

use crate::{quantities::{MaybeList, MaybeOwnedList}, systems::{Sos, Tf}, transforms::system::ToTf};

impl<T, B, A, S> One for Sos<T, B, A, S>
where
    T: ComplexFloat,
    B: Maybe<[T; 3]> + MaybeOwnedList<T>,
    A: Maybe<[T; 3]> + MaybeOwnedList<T>,
    S: MaybeList<Tf<T, B, A>>,
    Self: Mul<Output = Self> + Default + ToTf<T, Vec<T>, Vec<T>, (), ()> + Clone,
{
    fn one() -> Self
    {
        Self::one()
    }
    fn is_one(&self) -> bool
    where
        Self: PartialEq,
    {
        self.is_one()
    }
}