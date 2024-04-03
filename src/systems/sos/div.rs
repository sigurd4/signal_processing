use core::ops::{Div, Mul};

use num::{complex::ComplexFloat, traits::Inv};
use option_trait::Maybe;

use crate::{MaybeList, Sos, Tf};

impl<T1, T2, B1, B2, A1, A2, S1, S2, O> Div<Sos<T2, B2, A2, S2>> for Sos<T1, B1, A1, S1>
where
    T1: ComplexFloat,
    T2: ComplexFloat,
    B1: Maybe<[T1; 3]> + MaybeList<T1>,
    B2: Maybe<[T2; 3]> + MaybeList<T2>,
    A1: Maybe<[T1; 3]> + MaybeList<T1>,
    A2: Maybe<[T2; 3]> + MaybeList<T2>,
    S1: MaybeList<Tf<T1, B1, A1>>,
    S2: MaybeList<Tf<T2, B2, A2>>,
    Sos<T2, B2, A2, S2>: Inv,
    Self: Mul<<Sos<T2, B2, A2, S2> as Inv>::Output, Output = O>
{
    type Output = O;

    fn div(self, rhs: Sos<T2, B2, A2, S2>) -> Self::Output
    {
        self*rhs.inv()
    }
}