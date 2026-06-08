#![feature(const_trait_impl)]
#![feature(const_ops)]

use core::ops::{Add, BitAnd, BitOr, BitXor, Div, Mul, Neg, Not, Rem, Shl, Shr, Sub};

use bulks::{Bulk, Map};
use currying::{Curried, RCurry};
use num_complex::{Complex, ComplexFloat};
use num_traits::{Float, FloatConst};

pub const trait MathBulk: ~const Bulk
{
    fn add_all<Rhs>(self, rhs: Rhs) -> Map<Self, impl ~const Fn(Self::Item) -> <Self::Item as Add<Rhs>>::Output>
    where
        Self::Item: ~const Add<Rhs>,
        Rhs: Copy,
        Self: Sized
    {
        self.map(Add::add.rcurry_once(rhs))
    }
    fn sub_all<Rhs>(self, rhs: Rhs) -> Map<Self, impl ~const Fn(Self::Item) -> <Self::Item as Sub<Rhs>>::Output>
    where
        Self::Item: ~const Sub<Rhs>,
        Rhs: Copy,
        Self: Sized
    {
        self.map(Sub::sub.rcurry_once(rhs))
    }
    fn mul_all<Rhs>(self, rhs: Rhs) -> Map<Self, impl ~const Fn(Self::Item) -> <Self::Item as Mul<Rhs>>::Output>
    where
        Self::Item: ~const Mul<Rhs>,
        Rhs: Copy,
        Self: Sized
    {
        self.map(Mul::mul.rcurry_once(rhs))
    }
    fn div_all<Rhs>(self, rhs: Rhs) -> Map<Self, impl ~const Fn(Self::Item) -> <Self::Item as Div<Rhs>>::Output>
    where
        Self::Item: ~const Div<Rhs>,
        Rhs: Copy,
        Self: Sized
    {
        self.map(Div::div.rcurry_once(rhs))
    }
    fn bitand_all<Rhs>(self, rhs: Rhs) -> Map<Self, impl ~const Fn(Self::Item) -> <Self::Item as BitAnd<Rhs>>::Output>
    where
        Self::Item: ~const BitAnd<Rhs>,
        Rhs: Copy,
        Self: Sized
    {
        self.map(BitAnd::bitand.rcurry_once(rhs))
    }
    fn bitor_all<Rhs>(self, rhs: Rhs) -> Map<Self, impl ~const Fn(Self::Item) -> <Self::Item as BitOr<Rhs>>::Output>
    where
        Self::Item: ~const BitOr<Rhs>,
        Rhs: Copy,
        Self: Sized
    {
        self.map(BitOr::bitor.rcurry_once(rhs))
    }
    fn bitxor_all<Rhs>(self, rhs: Rhs) -> Map<Self, impl ~const Fn(Self::Item) -> <Self::Item as BitXor<Rhs>>::Output>
    where
        Self::Item: ~const BitXor<Rhs>,
        Rhs: Copy,
        Self: Sized
    {
        self.map(BitXor::bitxor.rcurry_once(rhs))
    }
    fn shl_all<Rhs>(self, rhs: Rhs) -> Map<Self, impl ~const Fn(Self::Item) -> <Self::Item as Shl<Rhs>>::Output>
    where
        Self::Item: ~const Shl<Rhs>,
        Rhs: Copy,
        Self: Sized
    {
        self.map(Shl::shl.rcurry_once(rhs))
    }
    fn shr_all<Rhs>(self, rhs: Rhs) -> Map<Self, impl ~const Fn(Self::Item) -> <Self::Item as Shr<Rhs>>::Output>
    where
        Self::Item: ~const Shr<Rhs>,
        Rhs: Copy,
        Self: Sized
    {
        self.map(Shr::shr.rcurry_once(rhs))
    }
    fn neg_all(self) -> Map<Self, impl ~const Fn(Self::Item) -> <Self::Item as Neg>::Output>
    where
        Self::Item: ~const Neg,
        Self: Sized
    {
        self.map(Neg::neg)
    }
    fn not_all(self) -> Map<Self, impl ~const Fn(Self::Item) -> <Self::Item as Not>::Output>
    where
        Self::Item: ~const Not,
        Self: Sized
    {
        self.map(Not::not)
    }
}

pub trait ComplexBulk: Bulk<Item: ComplexFloat>
{
    type ItemReal: Float + FloatConst;

    fn conj_all(self) -> Map<Self, impl Fn(Self::Item) -> Self::Item>
    where
        Self: Sized
    {
        self.map(ComplexFloat::conj)
    }
}
impl<I> ComplexBulk for I
where
    I: Bulk<Item: ComplexFloat>
{
    type ItemReal = <I::Item as ComplexFloat>::Real;
}