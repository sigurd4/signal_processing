use core::iter::Product;
use std::ops::Mul;

use num::{complex::ComplexFloat, One};
use option_trait::Maybe;

use crate::{MaybeList, MaybeLists, Sos, Tf};

impl<T, B, A, S> One for Sos<T, B, A, S>
where
    T: ComplexFloat,
    B: Maybe<[T; 3]> + MaybeList<T>,
    A: Maybe<[T; 3]> + MaybeList<T>,
    S: MaybeList<Tf<T, B, A>>,
    Self: Mul<Output = Self> + Default,
    for<'a> B::View<'a>: MaybeLists<T>,
    for<'a> A::View<'a>: MaybeList<T>,
    Tf<T, Vec<T>, Vec<T>>: for<'a> Product<Tf<T, B::View<'a>, A::View<'a>>>
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