use core::ops::{Add, Neg, Sub};

use num::complex::ComplexFloat;

use crate::{MaybeList, Polynomial, SumSequence};

moddef::moddef!(
    mod {
        add,
        neg,
        sub
    }
);

#[derive(Debug, Clone, Copy)]
pub struct Rpk<T, R, P, RP, K>
where
    T: ComplexFloat,
    R: ComplexFloat<Real = T::Real>,
    P: ComplexFloat<Real = T::Real>,
    RP: MaybeList<(R, P)>,
    K: MaybeList<T>
{
    pub rp: SumSequence<(R, P), RP>,
    pub k: Polynomial<T, K>
}

impl<T, R, P, RP, K> Rpk<T, R, P, RP, K>
where
    T: ComplexFloat,
    R: ComplexFloat<Real = T::Real>,
    P: ComplexFloat<Real = T::Real>,
    RP: MaybeList<(R, P)>,
    K: MaybeList<T>
{
    pub fn new(rp: RP, k: K) -> Self
    {
        Self {
            rp: SumSequence::new(rp),
            k: Polynomial::new(k)
        }
    }

    pub type Owned = Rpk<T, R, P, RP::Owned, K::Owned>
    where
        RP::Owned: MaybeList<(R, P)>,
        K::Owned: MaybeList<T>;
    pub type View<'a> = Rpk<T, R, P, RP::View<'a>, K::View<'a>>
    where
        Self: 'a,
        RP::View<'a>: MaybeList<(R, P)>,
        K::View<'a>: MaybeList<T>;

    pub fn as_view<'a>(&'a self) -> Rpk<T, R, P, RP::View<'a>, K::View<'a>>
    where
        Self: 'a,
        RP::View<'a>: MaybeList<(R, P)>,
        K::View<'a>: MaybeList<T>
    {
        Rpk {
            rp: self.rp.as_view(),
            k: self.k.as_view()
        }
    }
    pub fn to_owned(&self) -> Rpk<T, R, P, RP::Owned, K::Owned>
    where
        RP::Owned: MaybeList<(R, P)>,
        K::Owned: MaybeList<T>
    {
        Rpk {
            rp: self.rp.to_owned(),
            k: self.k.to_owned()
        }
    }
    pub fn into_owned(self) -> Rpk<T, R, P, RP::Owned, K::Owned>
    where
        RP::Owned: MaybeList<(R, P)>,
        K::Owned: MaybeList<T>
    {
        Rpk {
            rp: self.rp.into_owned(),
            k: self.k.into_owned()
        }
    }
}

macro_rules! impl_op1_extra {
    ($t:ident :: $f:tt) => {
        impl<'a, T, R, P, RP, K, O> $t for &'a Rpk<T, R, P, RP, K>
        where
            T: ComplexFloat,
            R: ComplexFloat<Real = T::Real>,
            P: ComplexFloat<Real = T::Real>,
            RP: MaybeList<(R, P)>,
            K: MaybeList<T>,
            RP::View<'a>: MaybeList<(R, P)>,
            K::View<'a>: MaybeList<T>,
            Rpk<T, R, P, RP::View<'a>, K::View<'a>>: $t<Output = O>
        {
            type Output = O;

            fn $f(self) -> Self::Output
            {
                self.as_view().$f()
            }
        }
    };
}
impl_op1_extra!(Neg::neg);

macro_rules! impl_op2_extra {
    ($t:ident :: $f:tt) => {
        impl<'a, T1, R1, P1, RP1, K1, T2, R2, P2, RP2, K2, O> $t<Rpk<T2, R2, P2, RP2, K2>> for &'a Rpk<T1, R1, P1, RP1, K1>
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
            RP1::View<'a>: MaybeList<(R1, P1)>,
            K1::View<'a>: MaybeList<T1>,
            Rpk<T1, R1, P1, RP1::View<'a>, K1::View<'a>>: $t<Rpk<T2, R2, P2, RP2, K2>, Output = O>
        {
            type Output = O;

            fn $f(self, rhs: Rpk<T2, R2, P2, RP2, K2>) -> Self::Output
            {
                self.as_view().$f(rhs)
            }
        }
        impl<'b, T1, R1, P1, RP1, K1, T2, R2, P2, RP2, K2, O> $t<&'b Rpk<T2, R2, P2, RP2, K2>> for Rpk<T1, R1, P1, RP1, K1>
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
            RP2::View<'b>: MaybeList<(R2, P2)>,
            K2::View<'b>: MaybeList<T2>,
            Rpk<T1, R1, P1, RP1, K1>: $t<Rpk<T2, R2, P2, RP2::View<'b>, K2::View<'b>>, Output = O>
        {
            type Output = O;

            fn $f(self, rhs: &'b Rpk<T2, R2, P2, RP2, K2>) -> Self::Output
            {
                self.$f(rhs.as_view())
            }
        }
        impl<'a, 'b, T1, R1, P1, RP1, K1, T2, R2, P2, RP2, K2, O> $t<&'b Rpk<T2, R2, P2, RP2, K2>> for &'a Rpk<T1, R1, P1, RP1, K1>
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
            RP1::View<'a>: MaybeList<(R1, P1)>,
            K1::View<'a>: MaybeList<T1>,
            RP2::View<'b>: MaybeList<(R2, P2)>,
            K2::View<'b>: MaybeList<T2>,
            Rpk<T1, R1, P1, RP1::View<'a>, K1::View<'a>>: $t<Rpk<T2, R2, P2, RP2::View<'b>, K2::View<'b>>, Output = O>
        {
            type Output = O;

            fn $f(self, rhs: &'b Rpk<T2, R2, P2, RP2, K2>) -> Self::Output
            {
                self.as_view().$f(rhs.as_view())
            }
        }

        impl<T, R, P, RP, K, O> $t<T> for Rpk<T, R, P, RP, K>
        where
            T: ComplexFloat,
            R: ComplexFloat<Real = T::Real>,
            P: ComplexFloat<Real = T::Real>,
            RP: MaybeList<(R, P)>,
            K: MaybeList<T>,
            Rpk<T, R, P, RP, K>: $t<Rpk<T, R, P, (), [T; 1]>, Output = O>
        {
            type Output = O;

            fn $f(self, rhs: T) -> Self::Output
            {
                self.$f(Rpk::new((), [rhs]))
            }
        }
        impl<'a, T, R, P, RP, K, O> $t<T> for &'a Rpk<T, R, P, RP, K>
        where
            T: ComplexFloat,
            R: ComplexFloat<Real = T::Real>,
            P: ComplexFloat<Real = T::Real>,
            RP: MaybeList<(R, P)>,
            K: MaybeList<T>,
            RP::View<'a>: MaybeList<(R, P)>,
            K::View<'a>: MaybeList<T>,
            Rpk<T, R, P, RP::View<'a>, K::View<'a>>: $t<Rpk<T, R, P, (), [T; 1]>, Output = O>
        {
            type Output = O;

            fn $f(self, rhs: T) -> Self::Output
            {
                self.as_view().$f(Rpk::new((), [rhs]))
            }
        }
        impl<'b, T, R, P, RP, K, O> $t<&'b T> for Rpk<T, R, P, RP, K>
        where
            T: ComplexFloat,
            R: ComplexFloat<Real = T::Real>,
            P: ComplexFloat<Real = T::Real>,
            RP: MaybeList<(R, P)>,
            K: MaybeList<T>,
            Rpk<T, R, P, RP, K>: $t<Rpk<T, R, P, (), &'b [T; 1]>, Output = O>
        {
            type Output = O;

            fn $f(self, rhs: &'b T) -> Self::Output
            {
                self.$f(Rpk::new((), core::array::from_ref(rhs)))
            }
        }
        impl<'a, 'b, T, R, P, RP, K, O> $t<&'b T> for &'a Rpk<T, R, P, RP, K>
        where
            T: ComplexFloat,
            R: ComplexFloat<Real = T::Real>,
            P: ComplexFloat<Real = T::Real>,
            RP: MaybeList<(R, P)>,
            K: MaybeList<T>,
            RP::View<'a>: MaybeList<(R, P)>,
            K::View<'a>: MaybeList<T>,
            Rpk<T, R, P, RP::View<'a>, K::View<'a>>: $t<Rpk<T, R, P, (), &'b [T; 1]>, Output = O>
        {
            type Output = O;

            fn $f(self, rhs: &'b T) -> Self::Output
            {
                self.as_view().$f(Rpk::new((), core::array::from_ref(rhs)))
            }
        }
    };
}
impl_op2_extra!(Add::add);
impl_op2_extra!(Sub::sub);