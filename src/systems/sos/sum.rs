use core::{iter::Sum, ops::Add};

use num::{complex::ComplexFloat, Zero};
use option_trait::Maybe;

use crate::{MaybeList, MaybeOwnedList, Sos, Tf, ToSos};

impl<T1, B1, A1, S1, T2, B2, A2, S2> Sum<Sos<T1, B1, A1, S1>> for Sos<T2, B2, A2, S2>
where
    T1: ComplexFloat,
    T2: ComplexFloat,
    B1: Maybe<[T1; 3]> + MaybeOwnedList<T1>,
    B2: Maybe<[T2; 3]> + MaybeOwnedList<T2>,
    A1: Maybe<[T1; 3]> + MaybeOwnedList<T1>,
    A2: Maybe<[T2; 3]> + MaybeOwnedList<T2>,
    S1: MaybeList<Tf<T1, B1, A1>>,
    S2: MaybeList<Tf<T2, B2, A2>>,
    Sos<T1, B1, A1, S1>: ToSos<T2, B2, A2, S2, (), ()>,
    Sos<T2, B2, A2, S2>: Add<Output = Sos<T2, B2, A2, S2>> + Zero
{
    fn sum<I: Iterator<Item = Sos<T1, B1, A1, S1>>>(iter: I) -> Self
    {
        iter.map(|sos| sos.to_sos((), ()))
            .reduce(|a, b| a + b)
            .unwrap_or_else(Zero::zero)
    }
}