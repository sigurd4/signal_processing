use num::{complex::ComplexFloat, traits::Inv};
use option_trait::Maybe;

use crate::{MaybeList, Sos, Tf};

impl<T1, T2, B1, B2, A1, A2, S1, S2> Inv for Sos<T1, B1, A1, S1>
where
    T1: ComplexFloat,
    T2: ComplexFloat,
    B1: Maybe<[T1; 3]> + MaybeList<T1> + Clone,
    B2: Maybe<[T2; 3]> + MaybeList<T2>,
    A1: Maybe<[T1; 3]> + MaybeList<T1> + Clone,
    A2: Maybe<[T2; 3]> + MaybeList<T2>,
    S1: MaybeList<Tf<T1, B1, A1>, MaybeMapped<Tf<T2, B2, A2>> = S2>,
    S2: MaybeList<Tf<T2, B2, A2>>,
    Tf<T1, B1, A1>: Inv<Output = Tf<T2, B2, A2>>
{
    type Output = Sos<T2, B2, A2, S2>;

    fn inv(self) -> Self::Output
    {
        Sos::new(self.sos.into_inner()
            .maybe_map_into_owned(|sos| sos.inv())
        )
    }
}