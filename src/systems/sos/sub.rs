use core::ops::{Sub, Mul};

use num::complex::ComplexFloat;
use option_trait::Maybe;

use crate::{MaybeList, Sos, SplitNumerDenom, Tf, ToSos, ToTf};

impl<T1, T2, T3, B1, B2, A1, A2, A3, S1, S2, S3, S4> Sub<Sos<T2, B2, A2, S2>> for Sos<T1, B1, A1, S1>
where
    T1: ComplexFloat,
    T2: ComplexFloat,
    T3: ComplexFloat,
    B1: Maybe<[T1; 3]> + MaybeList<T1> + Clone,
    B2: Maybe<[T2; 3]> + MaybeList<T2> + Clone,
    A1: Maybe<[T1; 3]> + MaybeList<T1> + Clone,
    A2: Maybe<[T2; 3]> + MaybeList<T2> + Clone,
    A3: Maybe<[T3; 3]> + MaybeList<T3>,
    S1: MaybeList<Tf<T1, B1, A1>>,
    S2: MaybeList<Tf<T2, B2, A2>>,
    S3: MaybeList<Tf<T3, (), A3>>,
    S4: MaybeList<Tf<T3, [T3; 3], A3>>,
    
    Self: SplitNumerDenom,
    Sos<T2, B2, A2, S2>: SplitNumerDenom,

    <Self as SplitNumerDenom>::OutputNum: ToTf<T1, Vec<T1>, (), (), ()>,
    <Sos<T2, B2, A2, S2> as SplitNumerDenom>::OutputNum: ToTf<T2, Vec<T2>, (), (), ()>,

    Tf<T1, Vec<T1>>: Sub<Tf<T2, Vec<T2>>, Output = Tf<T3, Vec<T3>>>,
    Tf<T3, Vec<T3>>: ToSos<T3, [T3; 3], (), Vec<Tf<T3, [T3; 3]>>, (), ()>,

    <Self as SplitNumerDenom>::OutputDen: Mul<<Sos<T2, B2, A2, S2> as SplitNumerDenom>::OutputDen, Output = Sos<T3, (), A3, S3>>,
    Sos<T3, [T3; 3], (), Vec<Tf<T3, [T3; 3]>>>: Mul<Sos<T3, (), A3, S3>, Output = Sos<T3, [T3; 3], A3, S4>>
{
    type Output = Sos<T3, [T3; 3], A3, S4>;

    fn sub(self, rhs: Sos<T2, B2, A2, S2>) -> Self::Output
    {
        let (n1, d1) = self.split_numer_denom();
        let (n2, d2) = rhs.split_numer_denom();

        let n1 = n1.to_tf((), ());
        let n2 = n2.to_tf((), ());

        let n = n1 - n2;
        let n = n.to_sos((), ());

        let d = d1*d2;
        n*d
    }
}