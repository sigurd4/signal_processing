use core::ops::Mul;

use num::complex::ComplexFloat;
use option_trait::Maybe;

use crate::{MaybeList, ProductSequence, Sos, Tf};

impl<T1, T2, T3, B1, B2, B3, A1, A2, A3, S1, S2, S3> Mul<Sos<T2, B2, A2, S2>> for Sos<T1, B1, A1, S1>
where
    T1: ComplexFloat,
    T2: ComplexFloat,
    T3: ComplexFloat,
    B1: Maybe<[T1; 3]> + MaybeList<T1>,
    B2: Maybe<[T2; 3]> + MaybeList<T2>,
    B3: Maybe<[T3; 3]> + MaybeList<T3>,
    A1: Maybe<[T1; 3]> + MaybeList<T1>,
    A2: Maybe<[T2; 3]> + MaybeList<T2>,
    A3: Maybe<[T3; 3]> + MaybeList<T3>,
    S1: MaybeList<Tf<T1, B1, A1>>,
    S2: MaybeList<Tf<T2, B2, A2>>,
    S3: MaybeList<Tf<T3, B3, A3>>,
    ProductSequence<Tf<T1, B1, A1>, S1>: Mul<ProductSequence<Tf<T2, B2, A2>, S2>, Output = ProductSequence<Tf<T3, B3, A3>, S3>>
{
    type Output = Sos<T3, B3, A3, S3>;

    fn mul(self, rhs: Sos<T2, B2, A2, S2>) -> Self::Output
    {
        Sos {
            sos: self.sos*rhs.sos
        }
    }
}