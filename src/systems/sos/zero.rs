use core::ops::Add;

use num::{complex::ComplexFloat, Zero};
use option_trait::Maybe;

use crate::{MaybeList, ProductSequence, Sos, Tf};

impl<T, B, A, S> Zero for Sos<T, B, A, S>
where
    T: ComplexFloat,
    B: Maybe<[T; 3]> + MaybeList<T> + Default,
    A: Maybe<[T; 3]> + MaybeList<T>,
    S: MaybeList<Tf<T, B, A>>,
    Tf<T, B, A>: Default,
    ProductSequence<Tf<T, B, A>, [Tf<T, B, A>; 1]>: Into<ProductSequence<Tf<T, B, A>, S>>,
    Self: Add<Output = Self>
{
    fn zero() -> Self
    {
        Self::zero()
    }
    fn is_zero(&self) -> bool
    {
        self.is_zero()
    }
}