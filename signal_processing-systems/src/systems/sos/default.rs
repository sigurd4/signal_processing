use num::complex::ComplexFloat;
use option_trait::Maybe;

use crate::{quantities::{List, MaybeOwnedList, ProductSequence}, systems::{Sos, Tf}};

impl<T, B, A, S> Default for Sos<T, B, A, S>
where
    T: ComplexFloat,
    B: Maybe<[T; 3]> + MaybeOwnedList<T>,
    A: Maybe<[T; 3]> + MaybeOwnedList<T>,
    S: List<Tf<T, B, A>>,
    ProductSequence<Tf<T, B, A>, S>: Default
{
    fn default() -> Self
    {
        Self {
            sos: Default::default()
        }
    }
}