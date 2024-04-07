use core::ops::Div;
use std::ops::Mul;

use num::complex::ComplexFloat;

use crate::{Lists, MaybeList, Polynomial, Tf};

impl<T1, T2, T3, B1, B2, B3, A1, A2, A3> Div<Tf<T2, B2, A2>> for Tf<T1, B1, A1>
where
    T1: ComplexFloat,
    T2: ComplexFloat,
    T3: ComplexFloat,
    B1: Lists<T1>,
    A1: MaybeList<T1>,
    B2: Lists<T2>,
    A2: MaybeList<T2>,
    B3: Lists<T3>,
    A3: MaybeList<T3>,
    Polynomial<T1, B1>: Mul<Polynomial<T2, A2>, Output = Polynomial<T3, B3>>,
    Polynomial<T1, A1>: Mul<Polynomial<T2, B2>, Output = Polynomial<T3, A3>>,
{
    type Output = Tf<T3, B3, A3>;

    fn div(self, rhs: Tf<T2, B2, A2>) -> Self::Output
    {
        Tf {
            b: self.b*rhs.a,
            a: self.a*rhs.b
        }
    }
}