use core::ops::Div;
use std::ops::Mul;

use num::complex::ComplexFloat;

use crate::{List, ProductSequence, Zpk};

impl<T1, T2, T3, Z1, Z2, Z3, P1, P2, P3, K1, K2, K3> Div<Zpk<T2, Z2, P2, K2>> for Zpk<T1, Z1, P1, K1>
where
    T1: ComplexFloat,
    T2: ComplexFloat,
    T3: ComplexFloat,
    K1: ComplexFloat<Real = T1::Real> + Div<K2, Output = K3>,
    K2: ComplexFloat<Real = T2::Real>,
    K3: ComplexFloat<Real = T3::Real>,
    Z1: List<T1>,
    Z2: List<T2>,
    Z3: List<T3>,
    P1: List<T1>,
    P2: List<T2>,
    P3: List<T3>,
    ProductSequence<T1, Z1>: Mul<ProductSequence<T2, P2>, Output = ProductSequence<T3, Z3>>,
    ProductSequence<T1, P1>: Mul<ProductSequence<T2, Z2>, Output = ProductSequence<T3, P3>>
{
    type Output = Zpk<T3, Z3, P3, K3>;

    fn div(self, rhs: Zpk<T2, Z2, P2, K2>) -> Self::Output
    {
        Zpk {
            z: self.z*rhs.p,
            p: self.p*rhs.z,
            k: self.k/rhs.k
        }
    }
}