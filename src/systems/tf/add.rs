use core::ops::Mul;
use std::ops::Add;

use num::complex::ComplexFloat;

use crate::{MaybeList, MaybeLists, Normalize, Polynomial, Tf};

impl<T1, B1, A1, T2, B2, A2, T3, B3, A3> Add<Tf<T2, B2, A2>> for Tf<T1, B1, A1>
where
    T1: ComplexFloat,
    T2: ComplexFloat,
    T3: ComplexFloat,
    B1: MaybeLists<T1>,
    A1: MaybeList<T1> + Clone,
    B2: MaybeLists<T2>,
    A2: MaybeList<T2> + Clone,
    B3: MaybeLists<T3>,
    A3: MaybeList<T3>,
    Polynomial<T1, A1>: Mul<Polynomial<T2, A2>, Output = Polynomial<T3, A3>>,
    Polynomial<T1, B1>: Mul<Polynomial<T2, A2>, Output: Add<<Polynomial<T2, B2> as Mul<Polynomial<T1, A1>>>::Output, Output = Polynomial<T3, B3>>>,
    Polynomial<T2, B2>: Mul<Polynomial<T1, A1>>,
    Tf<T3, B3, A3>: Normalize
{
    type Output = <Tf<T3, B3, A3> as Normalize>::Output;

    fn add(self, rhs: Tf<T2, B2, A2>) -> Self::Output
    {
        Tf {
            b: self.b*rhs.a.clone() + rhs.b*self.a.clone(),
            a: self.a*rhs.a
        }.normalize()
    }
}