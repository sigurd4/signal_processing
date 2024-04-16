use core::ops::Neg;

use num::complex::ComplexFloat;
use option_trait::Maybe;

use crate::{MaybeList, MaybeOwnedList, Sos, Tf, ToSos, ToTf};

impl<T1, T2, B1, B2, A1, A2, S> Neg for Sos<T1, B1, A1, S>
where
    T1: ComplexFloat,
    T2: ComplexFloat,
    B1: Maybe<[T1; 3]> + MaybeOwnedList<T1>,
    B2: Maybe<[T2; 3]> + MaybeOwnedList<T2>,
    A1: Maybe<[T1; 3]> + MaybeOwnedList<T1>,
    A2: Maybe<[T2; 3]> + MaybeOwnedList<T2>,
    S: MaybeList<Tf<T1, B1, A1>>,
    Tf<T1, B1, A1>: Neg<Output = Tf<T2, B2, A2>> + ToTf<T2, B2, A2, (), ()> + Default,
    Self: ToSos<T1, B1, A1, Vec<Tf<T1, B1, A1>>, (), ()>
{
    type Output = Sos<T2, B2, A2, Vec<Tf<T2, B2, A2>>>;

    fn neg(self) -> Self::Output
    {
        let Sos::<_, _, _, Vec<_>> {sos} = self.to_sos((), ());
        let mut first = true;

        let mut sos: Vec<_> = sos.into_inner()
            .into_iter()
            .map(|sos| {
                if first
                {
                    first = false;
                    -sos
                }
                else
                {
                    sos.to_tf((), ())
                }
            }).collect();
        if first
        {
            sos.push(-Tf::one())
        }

        Sos::new(sos)
    }
}