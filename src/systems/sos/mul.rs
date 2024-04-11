use core::ops::Mul;

use num::complex::ComplexFloat;
use option_trait::{Maybe, MaybeOr, StaticMaybe};

use crate::{ComplexOp, MaybeList, ProductSequence, Sos, Tf, ToTf};

impl<T1, T2, T3, B1, B2, B3, A1, A2, A3, S1, S2, S3> Mul<Sos<T2, B2, A2, S2>> for Sos<T1, B1, A1, S1>
where
    T1: ComplexFloat + ComplexOp<T2, Output = T3>,
    T2: ComplexFloat + Into<T3>,
    T3: ComplexFloat,
    B1: Maybe<[T1; 3]> + MaybeList<T1> + Clone,
    B2: Maybe<[T2; 3]> + MaybeList<T2> + Clone,
    B3: Maybe<[T3; 3]> + MaybeList<T3>,
    A1: Maybe<[T1; 3]> + MaybeList<T1> + Clone,
    A2: Maybe<[T2; 3]> + MaybeList<T2> + Clone,
    A3: Maybe<[T3; 3]> + MaybeList<T3>,
    S1: MaybeList<Tf<T1, B1, A1>>,
    S2: MaybeList<Tf<T2, B2, A2>>,
    S3: MaybeList<Tf<T3, B3, A3>>,
    S1::MaybeMapped<Tf<T3, B3, A3>>: MaybeList<Tf<T3, B3, A3>>,
    S2::MaybeMapped<Tf<T3, B3, A3>>: MaybeList<Tf<T3, B3, A3>>,
    ProductSequence<Tf<T3, B3, A3>, S1::MaybeMapped<Tf<T3, B3, A3>>>: Mul<ProductSequence<Tf<T3, B3, A3>, S2::MaybeMapped<Tf<T3, B3, A3>>>, Output = ProductSequence<Tf<T3, B3, A3>, S3>>,
    Tf<T1, B1, A1>: ToTf<T3, B3, A3, (), ()>,
    Tf<T2, B2, A2>: ToTf<T3, B3, A3, (), ()>,
    B1::MaybeMapped<T3>: MaybeOr<[T3; 3], B2::MaybeMapped<T3>, Output = B3>,
    A1::MaybeMapped<T3>: MaybeOr<[T3; 3], A2::MaybeMapped<T3>, Output = A3>,
    B2::MaybeMapped<T3>: StaticMaybe<[T3; 3]>,
    A2::MaybeMapped<T3>: StaticMaybe<[T3; 3]>
{
    type Output = Sos<T3, B3, A3, S3>;

    fn mul(self, rhs: Sos<T2, B2, A2, S2>) -> Self::Output
    {
        Sos {
            sos: ProductSequence::new(self.sos.into_inner().maybe_map_into_owned(|sos| sos.to_tf((), ())))
                *ProductSequence::new(rhs.sos.into_inner().maybe_map_into_owned(|sos| sos.to_tf((), ())))
        }
    }
}