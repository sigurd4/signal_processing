use core::ops::{Add, Div, Mul, Neg, Sub};

use num::{complex::ComplexFloat, traits::Inv};
use option_trait::Maybe;

use crate::{MaybeList, Polynomial, ProductSequence, Tf, ToTf};

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
    pub fn is_one(&self) -> bool
    where
        Self: ToTf<T, Vec<T>, Vec<T>, (), ()> + Clone
    {
        self.clone()
            .to_tf((), ())
            .is_one()
    }
    pub fn is_zero(&self) -> bool
    {
        self.sos.as_view_slice_option()
            .is_some_and(|sos| {
                sos.iter()
                    .any(|sos| sos.is_zero())
            })
    }

    pub fn s() -> Self
    where
        Polynomial<T, [T; 3]>: Into<Polynomial<T, B>>,
        Polynomial<T, ()>: Into<Polynomial<T, A>>,
        ProductSequence<Tf<T, B, A>, [Tf<T, B, A>; 1]>: Into<ProductSequence<Tf<T, B, A>, S>>
    {
        Sos {
            sos: ProductSequence::new([
                Tf {
                    b: Polynomial::new([T::zero(), T::one(), T::zero()]).into(),
                    a: Polynomial::new(()).into()
                }
            ]).into()
        }
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

#[allow(unused)]
macro s {
    (s) => {},
    (z) => {},
}
pub macro sos {
    ($t:path[$s:ident]= {$e:expr}) => {
        {
            Sos::<$t, [$t; 3], (), [_; 1]>::$s();
            s!($s);
            $e
        }
    },
    ($t:path[$s:ident]=) => {
        {
            Sos::<$t, [$t; 3], (), [_; 1]>::$s();
            s!($s);
            Tf::<$t, _, _>::new([], ())
        }
    },
    ($t:path[$s:ident]= $c:literal) => {
        {
            Sos::<$t, [$t; 3], (), [_; 1]>::$s();
            s!($s);
            Sos::<$t, _, _, _>::new([Tf::<$t, _, _>::new([<$t as num::NumCast>::from(0).unwrap(), <$t as num::NumCast>::from(0).unwrap(), <$t as num::NumCast>::from($c).unwrap()], ())])
        }
    },
    ($t:path[$s:ident]= $c:literal + $ci:literal j) => {
        {
            Sos::<$t, [$t; 3], (), [_; 1]>::$s();
            s!($s);
            Sos::<num::Complex<<$t as num::complex::ComplexFloat>::Real>, _, _, _>::new([Tf::<num::Complex<<$t as num::complex::ComplexFloat>::Real>, _, _>::new([num::Complex::new(<$t as num::NumCast>::from(0).unwrap(), <$t as num::NumCast>::from(0).unwrap()), num::Complex::new(<$t as num::NumCast>::from(0).unwrap(), <$t as num::NumCast>::from(0).unwrap()), num::Complex::new(<$t as num::NumCast>::from($c).unwrap(), <$t as num::NumCast>::from($ci).unwrap())], ())])
        }
    },
    ($t:path[$s:ident]= $c:literal - $ci:literal j) => {
        {
            Sos::<$t, [$t; 3], (), [_; 1]>::$s();
            s!($s);
            Sos::<num::Complex<<$t as num::complex::ComplexFloat>::Real>, _, _, _>::new([Tf::<num::Complex<<$t as num::complex::ComplexFloat>::Real>, _, _>::new([num::Complex::new(<$t as num::NumCast>::from(0).unwrap(), <$t as num::NumCast>::from(0).unwrap()), num::Complex::new(<$t as num::NumCast>::from(0).unwrap(), <$t as num::NumCast>::from(0).unwrap()), num::Complex::new(<$t as num::NumCast>::from($c).unwrap(), -<$t as num::NumCast>::from($ci).unwrap())], ())])
        }
    },
    ($t:path[$s:ident]= $ci:literal j) => {
        {
            Sos::<$t, [$t; 3], (), [_; 1]>::$s();
            s!($s);
            Sos::<num::Complex<<$t as num::complex::ComplexFloat>::Real>, _, _, _>::new([Tf::<num::Complex<<$t as num::complex::ComplexFloat>::Real>, _, _>::new([num::Complex::new(<$t as num::NumCast>::from(0).unwrap(), <$t as num::NumCast>::from(0).unwrap()), num::Complex::new(<$t as num::NumCast>::from(0).unwrap(), <$t as num::NumCast>::from(0).unwrap()), num::Complex::new(<$t as num::NumCast>::from(0).unwrap(), <$t as num::NumCast>::from($ci).unwrap())], ())])
        }
    },
    ($t:path[$s:ident]= $ss:ident) => {
        {
            #[allow(unused)]
            let $s = Sos::<$t, [$t; 3], (), [_; 1]>::$s();
            s!($s);
            s!($ss);
            Sos::<$t, [$t; 3], (), [_; 1]>::from($ss)
        }
    },
    ($t:path[$s:ident]= $c:literal^$pc:literal) => {
        {
            Sos::<$t, [$t; 3], (), [_; 1]>::$s();
            s!($s);
            num::traits::Pow::pow(Sos::<$t, _, _, _>::new([Tf::<$t, _, _>::new([<$t as num::NumCast>::from(0).unwrap(), <$t as num::NumCast>::from(0).unwrap(), <$t as num::NumCast>::from($c).unwrap()], ())]), $pc)
        }
    },
    ($t:path[$s:ident]= $ss:ident^$ps:literal) => {
        {
            #[allow(unused)]
            let $s = Sos::<$t, [$t; 3], (), [_; 1]>::$s();
            s!($s);
            s!($ss);
            num::traits::Pow::pow(Sos::<$t, [$t; 3], (), [_; 1]>::from($ss), $ps)
        }
    },
    ($t:path[$s:ident]= ($($lhs:tt)*)^$lp:literal $($op:tt ($($rhs:tt)*))?) => {
        num::traits::Pow::pow(sos!($t[$s]= $($lhs)*), $lp)$($op sos!($t[$s]= $($rhs)*))*
    },
    ($t:path[$s:ident]= ($($lhs:tt)*) $($op:tt ($($rhs:tt)*))?) => {
        sos!($t[$s]= $($lhs)*)$($op sos!($t[$s]= $($rhs)*))*
    },
    ($t:path[$s:ident]= $lhs:tt$(^$lp:literal)? $($op:tt $rhs:tt$(^$rp:literal)?)*) => {
        sos!($t[$s]= $lhs$(^$lp)?)$($op sos!($t[$s]= $rhs$(^$rp)?))*
    },
}

#[cfg(test)]
mod test
{
    use super::sos;

    #[test]
    fn test()
    {
        let h = sos!(f64[s] = (((2*(s^-3))^-2) + 1 + 2*(s^2))/(s + 1)*2);
        let h2 = sos!(f64[s] = {h}*2);

        println!("{:?}", h2);
    }
}