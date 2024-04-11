use std::ops::Mul;

use num::complex::ComplexFloat;

use crate::{MaybeList, MaybeLists, Normalize, Polynomial, Tf};

impl<T1, T2, T3, B1, B2, B3, A1, A2, A3> Mul<Tf<T2, B2, A2>> for Tf<T1, B1, A1>
where
    T1: ComplexFloat,
    T2: ComplexFloat,
    T3: ComplexFloat,
    B1: MaybeLists<T1>,
    A1: MaybeList<T1>,
    B2: MaybeLists<T2>,
    A2: MaybeList<T2>,
    B3: MaybeLists<T3>,
    A3: MaybeList<T3>,
    Polynomial<T1, B1>: Mul<Polynomial<T2, B2>, Output = Polynomial<T3, B3>>,
    Polynomial<T1, A1>: Mul<Polynomial<T2, A2>, Output = Polynomial<T3, A3>>,
    Tf<T3, B3, A3>: Normalize
{
    type Output = <Tf<T3, B3, A3> as Normalize>::Output;

    fn mul(self, rhs: Tf<T2, B2, A2>) -> Self::Output
    {
        Tf {
            b: self.b*rhs.b,
            a: self.a*rhs.a
        }.normalize()
    }
}