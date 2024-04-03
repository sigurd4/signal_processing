use core::ops::Mul;
use std::iter::Product;

use num::{complex::ComplexFloat, One};

use crate::{List, Zpk};

impl<'a, T1, T2, Z1, Z2, P1, P2, K1, K2> Product<&'a Zpk<T1, Z1, P1, K1>> for Zpk<T2, Z2, P2, K2>
where
    T1: ComplexFloat,
    T2: ComplexFloat,
    K1: ComplexFloat<Real = T1::Real>,
    K2: ComplexFloat<Real = T2::Real>,
    Z1: List<T1>,
    P1: List<T1>,
    Z2: List<T2>,
    P2: List<T2>,
    &'a Zpk<T1, Z1, P1, K1>: Into<Self>,
    Self: One + 'a
{
    fn product<I: Iterator<Item = &'a Zpk<T1, Z1, P1, K1>>>(iter: I) -> Self
    {
        iter.map(|zpk| zpk.into())
            .reduce(Mul::mul)
            .unwrap_or_else(One::one)
    }
}