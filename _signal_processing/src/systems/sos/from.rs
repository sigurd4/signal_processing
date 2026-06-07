use num::complex::ComplexFloat;
use option_trait::Maybe;

use crate::{quantities::{List, MaybeOwnedList, ProductSequence}, systems::{Sos, Tf}};

impl<'a, T1, T2, B1, B2, A1, A2, S1, S2> From<&'a Sos<T1, B1, A1, S1>> for Sos<T2, B2, A2, S2>
where
    T1: ComplexFloat,
    T2: ComplexFloat,
    B1: Maybe<[T1; 3]> + MaybeOwnedList<T1>,
    B2: Maybe<[T2; 3]> + MaybeOwnedList<T2>,
    A1: Maybe<[T1; 3]> + MaybeOwnedList<T1>,
    A2: Maybe<[T2; 3]> + MaybeOwnedList<T2>,
    S1: List<Tf<T1, B1, A1>>,
    S2: List<Tf<T2, B2, A2>>,
    S1::View<'a>: List<Tf<T1, B1, A1>>,
    ProductSequence<Tf<T1, B1, A1>, S1::View<'a>>: Into<ProductSequence<Tf<T2, B2, A2>, S2>>
{
    fn from(sos: &'a Sos<T1, B1, A1, S1>) -> Self
    {
        Sos {
            sos: sos.sos.as_view().into()
        }
    }
}