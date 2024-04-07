use core::{iter::Product, ops::{Add, Div, Mul, Neg, Sub}};

use num::{complex::ComplexFloat, traits::Inv};
use option_trait::Maybe;

use crate::{MaybeList, MaybeLists, Polynomial, ProductSequence, Tf};

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
pub struct Sos<T: ComplexFloat, B: Maybe<[T; 3]> + MaybeList<T>, A: Maybe<[T; 3]> + MaybeList<T>, S: MaybeList<Tf<T, B, A>> = ()>
{
    pub sos: ProductSequence<Tf<T, B, A>, S>
}

impl<T, S, B, A> Sos<T, B, A, S>
where
    T: ComplexFloat,
    B: Maybe<[T; 3]> + MaybeList<T>,
    A: Maybe<[T; 3]> + MaybeList<T>,
    S: MaybeList<Tf<T, B, A>>
{
    pub fn new(sos: S) -> Self
    {
        Self {
            sos: ProductSequence::new(sos)
        }
    }

    pub type View<'a> = Sos<T, B, A, S::View<'a>>
    where
        S: 'a,
        S::View<'a>: MaybeList<Tf<T, B, A>>;

    pub fn as_view<'a>(&'a self) -> Self::View<'a>
    where
        S: 'a,
        S::View<'a>: MaybeList<Tf<T, B, A>>
    {
        Sos {
            sos: self.sos.as_view()
        }
    }
    
    pub fn one() -> Self
    where
        Self: Default,
    {
        Sos::default()
    }
    pub fn zero() -> Self
    where
        Tf<T, B, A>: Default,
        B: Default,
        ProductSequence<Tf<T, B, A>, [Tf<T, B, A>; 1]>: Into<ProductSequence<Tf<T, B, A>, S>>
    {
        Sos {
            sos: ProductSequence::new([Tf::zero()]).into()
        }
    }
    pub fn is_one<'a>(&'a self) -> bool
    where
        B::View<'a>: MaybeLists<T>,
        A::View<'a>: MaybeList<T>,
        Tf<T, Vec<T>, Vec<T>>: Product<Tf<T, B::View<'a>, A::View<'a>>>
    {
        !self.sos.as_view_slice_option()
            .is_some_and(|sos| {
                !sos.iter()
                    .map(|sos| sos.as_view())
                    .product::<Tf<T, Vec<T>, Vec<T>>>()
                    .is_one()
            })
    }
    pub fn is_zero(&self) -> bool
    {
        self.sos.as_view_slice_option()
            .is_some_and(|sos| {
                sos.iter()
                    .any(|sos| sos.is_zero())
            })
    }
}

macro_rules! impl_op1_extra {
    ($t:ident :: $f:tt) => {
        impl<'a, T, B, A, S, O> $t for &'a Sos<T, B, A, S>
        where
            T: ComplexFloat,
            B: Maybe<[T; 3]> + MaybeList<T>,
            A: Maybe<[T; 3]> + MaybeList<T>,
            S: MaybeList<Tf<T, B, A>>,
            S::View<'a>: MaybeList<Tf<T, B, A>>,
            Sos<T, B, A, S::View<'a>>: $t<Output = O>
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
        impl<'a, T1, T2, B1, B2, A1, A2, S1, S2, O> $t<Sos<T2, B2, A2, S2>> for &'a Sos<T1, B1, A1, S1>
        where
            T1: ComplexFloat,
            T2: ComplexFloat,
            B1: Maybe<[T1; 3]> + MaybeList<T1>,
            B2: Maybe<[T2; 3]> + MaybeList<T2>,
            A1: Maybe<[T1; 3]> + MaybeList<T1>,
            A2: Maybe<[T2; 3]> + MaybeList<T2>,
            S1: MaybeList<Tf<T1, B1, A1>>,
            S2: MaybeList<Tf<T2, B2, A2>>,
            S1::View<'a>: MaybeList<Tf<T1, B1, A1>>,
            Sos<T1, B1, A1, S1::View<'a>>: $t<Sos<T2, B2, A2, S2>, Output = O>
        {
            type Output = O;

            fn $f(self, rhs: Sos<T2, B2, A2, S2>) -> Self::Output
            {
                self.as_view().$f(rhs)
            }
        }
        impl<'b, T1, T2, B1, B2, A1, A2, S1, S2, O> $t<&'b Sos<T2, B2, A2, S2>> for Sos<T1, B1, A1, S1>
        where
            T1: ComplexFloat,
            T2: ComplexFloat,
            B1: Maybe<[T1; 3]> + MaybeList<T1>,
            B2: Maybe<[T2; 3]> + MaybeList<T2>,
            A1: Maybe<[T1; 3]> + MaybeList<T1>,
            A2: Maybe<[T2; 3]> + MaybeList<T2>,
            S1: MaybeList<Tf<T1, B1, A1>>,
            S2: MaybeList<Tf<T2, B2, A2>>,
            S2::View<'b>: MaybeList<Tf<T2, B2, A2>>,
            Sos<T1, B1, A1, S1>: $t<Sos<T2, B2, A2, S2::View<'b>>, Output = O>
        {
            type Output = O;

            fn $f(self, rhs: &'b Sos<T2, B2, A2, S2>) -> Self::Output
            {
                self.$f(rhs.as_view())
            }
        }
        impl<'a, 'b, T1, T2, B1, B2, A1, A2, S1, S2, O> $t<&'b Sos<T2, B2, A2, S2>> for &'a Sos<T1, B1, A1, S1>
        where
            T1: ComplexFloat,
            T2: ComplexFloat,
            B1: Maybe<[T1; 3]> + MaybeList<T1>,
            B2: Maybe<[T2; 3]> + MaybeList<T2>,
            A1: Maybe<[T1; 3]> + MaybeList<T1>,
            A2: Maybe<[T2; 3]> + MaybeList<T2>,
            S1: MaybeList<Tf<T1, B1, A1>>,
            S2: MaybeList<Tf<T2, B2, A2>>,
            S1::View<'a>: MaybeList<Tf<T1, B1, A1>>,
            S2::View<'b>: MaybeList<Tf<T2, B2, A2>>,
            Sos<T1, B1, A1, S1::View<'a>>: $t<Sos<T2, B2, A2, S2::View<'b>>, Output = O>
        {
            type Output = O;

            fn $f(self, rhs: &'b Sos<T2, B2, A2, S2>) -> Self::Output
            {
                self.as_view().$f(rhs.as_view())
            }
        }

        impl<T, B, A, S, O> $t<T> for Sos<T, B, A, S>
        where
            T: ComplexFloat,
            B: Maybe<[T; 3]> + MaybeList<T>,
            A: Maybe<[T; 3]> + MaybeList<T>,
            (): Maybe<[T; 3]>,
            S: MaybeList<Tf<T, B, A>>,
            Self: $t<Sos<T, [T; 3], (), [Tf<T, [T; 3], ()>; 1]>, Output = O>
        {
            type Output = O;

            fn $f(self, rhs: T) -> Self::Output
            {
                self.$f(Sos {
                    sos: ProductSequence::new([
                        Tf {
                            b: Polynomial::new([T::zero(), T::zero(), rhs]),
                            a: Polynomial::new(())
                        }
                    ])
                })
            }
        }
    };
}
impl_op2_extra!(Add::add);
impl_op2_extra!(Sub::sub);
impl_op2_extra!(Mul::mul);
impl_op2_extra!(Div::div);