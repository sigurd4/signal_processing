use core::ops::{Add, Neg, Sub};

use num::complex::ComplexFloat;

use crate::{MaybeList, Rpk};

impl<T1, T2, R1, R2, P1, P2, RP1, RP2, K1, K2> Sub<Rpk<T2, R2, P2, RP2, K2>> for Rpk<T1, R1, P1, RP1, K1>
where
    T1: ComplexFloat,
    T2: ComplexFloat,
    R1: ComplexFloat<Real = T1::Real>,
    R2: ComplexFloat<Real = T2::Real>,
    P1: ComplexFloat<Real = T1::Real>,
    P2: ComplexFloat<Real = T2::Real>,
    RP1: MaybeList<(R1, P1)>,
    RP2: MaybeList<(R2, P2)>,
    K1: MaybeList<T1>,
    K2: MaybeList<T2>,
    Rpk<T2, R2, P2, RP2, K2>: Neg,
    Self: Add<<Rpk<T2, R2, P2, RP2, K2> as Neg>::Output>,
{
    type Output = <Self as Add<<Rpk<T2, R2, P2, RP2, K2> as Neg>::Output>>::Output;

    fn sub(self, rhs: Rpk<T2, R2, P2, RP2, K2>) -> Self::Output
    {
        self + (-rhs)
    }
}