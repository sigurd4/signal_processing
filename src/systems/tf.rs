use core::ops::{Add, Div, Mul, Neg, Sub};

use num::{complex::ComplexFloat, traits::Inv, NumCast};

use crate::{MaybeList, MaybeLists, Polynomial};

moddef::moddef!(
    mod {
        add,
        default,
        div,
        from,
        inv,
        mul,
        neg,
        one,
        pow,
        product,
        sub,
        sum,
        zero
    }
);

#[derive(Debug, Clone, Copy)]
pub struct Tf<T: ComplexFloat, B: MaybeLists<T> = (), A: MaybeList<T> = ()>
{
    pub b: Polynomial<T, B>,
    pub a: Polynomial<T, A>
}

impl<T: ComplexFloat, B: MaybeLists<T>, A: MaybeList<T>> Tf<T, B, A>
{
    pub type View<'a> = Tf<T, B::View<'a>, A::View<'a>>
    where
        B::View<'a>: MaybeLists<T>,
        A::View<'a>: MaybeList<T>,
        B: 'a,
        A: 'a;
    pub type Owned = Tf<T, B::Owned, A::Owned>
    where
        B::Owned: MaybeLists<T>,
        A::Owned: MaybeList<T>;

    pub fn as_view<'a>(&'a self) -> Self::View<'a>
    where
        B::View<'a>: MaybeLists<T>,
        A::View<'a>: MaybeList<T>
    {
        Tf {
            b: self.b.as_view(),
            a: self.a.as_view()
        }
    }
    pub fn to_owned(&self) -> Self::Owned
    where
        B::Owned: MaybeLists<T>,
        A::Owned: MaybeList<T>
    {
        Tf {
            b: self.b.to_owned(),
            a: self.a.to_owned()
        }
    }
    pub fn new(b: B, a: A) -> Self
    {
        Self {
            b: Polynomial::new(b),
            a: Polynomial::new(a)
        }
    }
    pub fn one() -> Self
    where
        Self: Default,
    {
        Tf::default()
    }
    pub fn zero() -> Self
    where
        Self: Default,
        B: Default
    {
        Tf {b: Polynomial::new(B::default()), ..Default::default()}
    }
    pub fn is_one(&self) -> bool
    where
        B: MaybeList<T>
    {
        !self.b.is_zero() && !self.a.is_zero() && self.a == self.b
    }
    pub fn is_zero(&self) -> bool
    where
        B: MaybeList<T>
    {
        self.b.is_zero() && !self.a.is_zero()
    }
    pub fn s() -> Self
    where
        for<'a> &'a Tf<T, [T; 2], ()>: Into<Self>
    {
        (&Tf {
            b: Polynomial::new([T::one(), T::zero()]),
            a: Polynomial::new(())
        }).into()
    }
    pub fn z() -> Self
    where
        for<'a> &'a Tf<T, [T; 2], ()>: Into<Self>
    {
        (&Tf {
            b: Polynomial::new([T::one(), T::zero()]),
            a: Polynomial::new(())
        }).into()
    }
    pub fn truncate<'a, const N: usize, const M: usize>(&'a self) -> Tf<T, [T; N], [T; M]>
    where
        B::View<'a>: MaybeLists<T>,
        A::View<'a>: MaybeLists<T>,
        Polynomial<T, B::View<'a>>: Into<Polynomial<T, Vec<T>>>,
        Polynomial<T, A::View<'a>>: Into<Polynomial<T, Vec<T>>>
    {
        Tf {
            b: self.b.as_view().truncate(),
            a: self.a.as_view().truncate()
        }
    }
}

macro_rules! impl_op1_extra {
    ($t:ident :: $f:tt) => {
        impl<'a, T, B, A, O> $t for &'a Tf<T, B, A>
        where
            T: ComplexFloat,
            B: MaybeLists<T>,
            A: MaybeList<T>,
            B::View<'a>: MaybeLists<T>,
            A::View<'a>: MaybeList<T>,
            Tf<T, B::View<'a>, A::View<'a>>: $t<Output = O>
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
impl_op1_extra!(Inv::inv);

macro_rules! impl_op2_extra {
    ($t:ident :: $f:tt) => {
        impl<'a, T1, B1, A1, T2, B2, A2, O> $t<Tf<T2, B2, A2>> for &'a Tf<T1, B1, A1>
        where
            T1: ComplexFloat,
            T2: ComplexFloat,
            B1: MaybeLists<T1>,
            A1: MaybeList<T1>,
            B2: MaybeLists<T2>,
            A2: MaybeList<T2>,
            B1::View<'a>: MaybeLists<T1>,
            A1::View<'a>: MaybeList<T1>,
            Tf<T1, B1::View<'a>, A1::View<'a>>: $t<Tf<T2, B2, A2>, Output = O>
        {
            type Output = O;

            fn $f(self, rhs: Tf<T2, B2, A2>) -> Self::Output
            {
                self.as_view().$f(rhs)
            }
        }
        impl<'b, T1, B1, A1, T2, B2, A2, O> $t<&'b Tf<T2, B2, A2>> for Tf<T1, B1, A1>
        where
            T1: ComplexFloat,
            T2: ComplexFloat,
            B1: MaybeLists<T1>,
            A1: MaybeList<T1>,
            B2: MaybeLists<T2>,
            A2: MaybeList<T2>,
            B2::View<'b>: MaybeLists<T2>,
            A2::View<'b>: MaybeList<T2>,
            Tf<T1, B1, A1>: $t<Tf<T2, B2::View<'b>, A2::View<'b>>, Output = O>
        {
            type Output = O;

            fn $f(self, rhs: &'b Tf<T2, B2, A2>) -> Self::Output
            {
                self.$f(rhs.as_view())
            }
        }
        impl<'a, 'b, T1, B1, A1, T2, B2, A2, O> $t<&'b Tf<T2, B2, A2>> for &'a Tf<T1, B1, A1>
        where
            T1: ComplexFloat,
            T2: ComplexFloat,
            B1: MaybeLists<T1>,
            A1: MaybeList<T1>,
            B2: MaybeLists<T2>,
            A2: MaybeList<T2>,
            B1::View<'a>: MaybeLists<T1>,
            A1::View<'a>: MaybeList<T1>,
            B2::View<'b>: MaybeLists<T2>,
            A2::View<'b>: MaybeList<T2>,
            Tf<T1, B1::View<'a>, A1::View<'a>>: $t<Tf<T2, B2::View<'b>, A2::View<'b>>, Output = O>
        {
            type Output = O;

            fn $f(self, rhs: &'b Tf<T2, B2, A2>) -> Self::Output
            {
                self.as_view().$f(rhs.as_view())
            }
        }

        impl<T, B, A, O> $t<T> for Tf<T, B, A>
        where
            T: ComplexFloat,
            B: MaybeLists<T>,
            A: MaybeList<T>,
            Self: $t<Tf<T, [T; 1], ()>, Output = O>
        {
            type Output = O;

            fn $f(self, rhs: T) -> Self::Output
            {
                self.$f(Tf {
                    b: Polynomial::new([rhs]),
                    a: Polynomial::new(())
                })
            }
        }
        impl<'a, T, B, A, O> $t<T> for &'a Tf<T, B, A>
        where
            T: ComplexFloat,
            B: MaybeLists<T>,
            A: MaybeList<T>,
            Self: $t<Tf<T, [T; 1], ()>, Output = O>
        {
            type Output = O;

            fn $f(self, rhs: T) -> Self::Output
            {
                self.$f(Tf {
                    b: Polynomial::new([rhs]),
                    a: Polynomial::new(())
                })
            }
        }
        impl<'b, T, B, A, O> $t<&'b T> for Tf<T, B, A>
        where
            T: ComplexFloat,
            B: MaybeLists<T>,
            A: MaybeList<T>,
            Self: $t<Tf<T, &'b [T; 1], ()>, Output = O>
        {
            type Output = O;

            fn $f(self, rhs: &'b T) -> Self::Output
            {
                self.$f(Tf {
                    b: Polynomial::new(core::array::from_ref(rhs)),
                    a: Polynomial::new(())
                })
            }
        }
        impl<'a, 'b, T, B, A, O> $t<&'b T> for &'a Tf<T, B, A>
        where
            T: ComplexFloat,
            B: MaybeLists<T>,
            A: MaybeList<T>,
            Self: $t<Tf<T, &'b [T; 1], ()>, Output = O>
        {
            type Output = O;

            fn $f(self, rhs: &'b T) -> Self::Output
            {
                self.$f(Tf {
                    b: Polynomial::new(core::array::from_ref(rhs)),
                    a: Polynomial::new(())
                })
            }
        }
    };
}
impl_op2_extra!(Add::add);
impl_op2_extra!(Sub::sub);
impl_op2_extra!(Mul::mul);
impl_op2_extra!(Div::div);