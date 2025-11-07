use core::ops::Neg;

use num::complex::ComplexFloat;

use crate::{quantities::{MaybeList, MaybeLists, Polynomial}, systems::Tf};

impl<T, B1, B2, A> Neg for Tf<T, B1, A>
where
    T: ComplexFloat,
    B1: MaybeLists<T>,
    B2: MaybeLists<T>,
    A: MaybeList<T>,
    Polynomial<T, B1>: Neg<Output = Polynomial<T, B2>>
{
    type Output = Tf<T, B2, A>;

    fn neg(self) -> Self::Output
    {
        Tf {
            b: -self.b,
            a: self.a
        }
    }
}