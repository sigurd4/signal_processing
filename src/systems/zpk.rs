use core::ops::{Div, Mul, Neg};

use num::{complex::ComplexFloat, traits::Inv, Complex, Float, NumCast, One, Zero};
use option_trait::Maybe;
use thiserror::Error;

use crate::{quantities::{ListOrSingle, MaybeList, ProductSequence}, transforms::system::ToZpk};

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
        zero
    }
);

#[derive(Debug, Clone, Copy, PartialEq, Error)]
pub enum ComplexRealError
{
    #[error("Tolerance must be a number in the range [0.0, 1.0].")]
    TolaranceOutOfRange,
    #[error("Complex roots and/or poles did not come in conjugate pairs. Something is wrong with this system.")]
    OddNumberComplex
}

#[derive(Debug, Clone, Copy)]
pub struct Zpk<T: ComplexFloat, Z: MaybeList<T> = (), P: MaybeList<T> = (), K: ComplexFloat<Real = T::Real> = T>
{
    pub z: ProductSequence<T, Z>,
    pub p: ProductSequence<T, P>,
    pub k: K
}

impl<T, Z, P, K> Zpk<T, Z, P, K>
where
    T: ComplexFloat,
    Z: MaybeList<T>,
    P: MaybeList<T>,
    K: ComplexFloat<Real = T::Real>
{
    pub fn new(z: Z, p: P, k: K) -> Self
    {
        Self {
            z: ProductSequence::new(z),
            p: ProductSequence::new(p),
            k
        }
    }

    pub type View<'a> = Zpk<T, Z::View<'a>, P::View<'a>, K>
    where
        Z: 'a,
        P: 'a,
        Z::View<'a>: MaybeList<T>,
        P::View<'a>: MaybeList<T>;
    pub type Owned = Zpk<T, Z::Owned, P::Owned, K>
    where
        Z::Owned: MaybeList<T>,
        P::Owned: MaybeList<T>;

    pub fn as_view<'a>(&'a self) -> Zpk<T, Z::View<'a>, P::View<'a>, K>
    where
        Z::View<'a>: MaybeList<T>,
        P::View<'a>: MaybeList<T>
    {
        Zpk {
            z: self.z.as_view(),
            p: self.p.as_view(),
            k: self.k
        }
    }
    pub fn to_owned(&self) -> Zpk<T, Z::Owned, P::Owned, K>
    where
        Z::Owned: MaybeList<T>,
        P::Owned: MaybeList<T>
    {
        Zpk {
            z: self.z.to_owned(),
            p: self.p.to_owned(),
            k: self.k
        }
    }
    pub fn into_owned(self) -> Zpk<T, Z::Owned, P::Owned, K>
    where
        Z::Owned: MaybeList<T>,
        P::Owned: MaybeList<T>
    {
        Zpk {
            z: self.z.into_owned(),
            p: self.p.into_owned(),
            k: self.k
        }
    }
    pub fn s() -> Self
    where
        T: Zero,
        K: One,
        ProductSequence<T, [T; 1]>: Into<ProductSequence<T, Z>>,
        ProductSequence<T, ()>: Into<ProductSequence<T, P>>
    {
        Zpk {
            z: ProductSequence::new([T::zero()]).into(),
            p: ProductSequence::new(()).into(),
            k: One::one()
        }
    }
    pub fn z() -> Self
    where
        T: Zero,
        K: One,
        ProductSequence<T, [T; 1]>: Into<ProductSequence<T, Z>>,
        ProductSequence<T, ()>: Into<ProductSequence<T, P>>
    {
        Zpk {
            z: ProductSequence::new([T::zero()]).into(),
            p: ProductSequence::new(()).into(),
            k: One::one()
        }
    }
    pub fn one() -> Self
    where
        Self: Default,
    {
        Zpk::default()
    }
    pub fn zero() -> Self
    where
        Self: Default
    {
        Zpk {k: K::zero(), ..Default::default()}
    }
    pub fn is_one(&self) -> bool
    {
        self.z.length() == 0 && self.p.length() == 0 && self.k.is_one()
    }
    pub fn is_zero(&self) -> bool
    {
        self.k.is_zero()
    }

    pub fn poles(&self) -> &[T]
    {
        self.p.as_view_slice_option()
            .unwrap_or(&[])
    }
    pub fn zeros(&self) -> &[T]
    {
        self.z.as_view_slice_option()
            .unwrap_or(&[])
    }

    pub fn complex_real<Tol>(self, tolerance: Tol) -> Result<(Vec<[Complex<T::Real>; 2]>, Vec<[Complex<T::Real>; 2]>, Vec<T::Real>, Vec<T::Real>, K), ComplexRealError>
    where
        Tol: Maybe<T::Real>,
        T: Into<Complex<T::Real>>,
        Self: ToZpk<T, Vec<T>, Vec<T>, K, (), ()>
    {
        let tol = if let Some(tol) = tolerance.into_option()
        {
            if tol < Zero::zero() || tol > One::one()
            {
                return Err(ComplexRealError::TolaranceOutOfRange)
            }
            tol
        }
        else
        {
            <T::Real as NumCast>::from(100.0).unwrap()*<T::Real as Float>::epsilon()
        };

        let Zpk {z, p, k}: Zpk<T, Vec<T>, Vec<T>, K> = self.to_zpk((), ());
        let mut zc = vec![];
        let mut pc = vec![];
        let mut zr = vec![];
        let mut pr = vec![];

        for (mut m, c, r) in [(z, &mut zc, &mut zr), (p, &mut pc, &mut pr)]
        {
            while let Some(z) = m.pop()
            {
                if z.is_zero() || Float::abs(z.im()) <= tol*z.abs()
                {
                    r.push(z.re());
                }
                else
                {
                    let z_conj = z.conj();
                    if let Some(i) = m.iter()
                        .enumerate()
                        .filter(|(_, z)| !(z.is_zero() || Float::abs(z.im()) <= tol*z.abs()))
                        .reduce(|a, b| if (*a.1 - z_conj).abs() < (*b.1 - z_conj).abs()
                        {
                            a
                        }
                        else
                        {
                            b
                        }).map(|(i, _)| i)
                    {
                        let z = [z.into(), m.remove(i).into()];
                        c.push(z)
                    }
                    else
                    {
                        return Err(ComplexRealError::OddNumberComplex)
                    }
                }
            }
        }

        Ok((zc, pc, zr, pr, k))
    }
}

macro_rules! impl_op1_extra {
    ($t:ident :: $f:tt) => {
        impl<'a, T, Z, P, K, O> $t for &'a Zpk<T, Z, P, K>
        where
            T: ComplexFloat,
            Z: MaybeList<T>,
            P: MaybeList<T>,
            K: ComplexFloat<Real = T::Real>,
            Z::View<'a>: MaybeList<T>,
            P::View<'a>: MaybeList<T>,
            Zpk<T, Z::View<'a>, P::View<'a>, K>: $t<Output = O>
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
        impl<'a, T1, T2, Z1, Z2, P1, P2, K1, K2, O> $t<Zpk<T2, Z2, P2, K2>> for &'a Zpk<T1, Z1, P1, K1>
        where
            T1: ComplexFloat,
            T2: ComplexFloat,
            Z1: MaybeList<T1>,
            Z2: MaybeList<T2>,
            P1: MaybeList<T1>,
            P2: MaybeList<T2>,
            K1: ComplexFloat<Real = T1::Real>,
            K2: ComplexFloat<Real = T2::Real>,
            Z1::View<'a>: MaybeList<T1>,
            P1::View<'a>: MaybeList<T1>,
            Zpk<T1, Z1::View<'a>, P1::View<'a>, K1>: $t<Zpk<T2, Z2, P2, K2>, Output = O>
        {
            type Output = O;

            fn $f(self, rhs: Zpk<T2, Z2, P2, K2>) -> Self::Output
            {
                self.as_view().$f(rhs)
            }
        }
        impl<'b, T1, T2, Z1, Z2, P1, P2, K1, K2, O> $t<&'b Zpk<T2, Z2, P2, K2>> for Zpk<T1, Z1, P1, K1>
        where
            T1: ComplexFloat,
            T2: ComplexFloat,
            Z1: MaybeList<T1>,
            Z2: MaybeList<T2>,
            P1: MaybeList<T1>,
            P2: MaybeList<T2>,
            K1: ComplexFloat<Real = T1::Real>,
            K2: ComplexFloat<Real = T2::Real>,
            Z2::View<'b>: MaybeList<T2>,
            P2::View<'b>: MaybeList<T2>,
            Zpk<T1, Z1, P1, K1>: $t<Zpk<T2, Z2::View<'b>, P2::View<'b>, K2>, Output = O>
        {
            type Output = O;

            fn $f(self, rhs: &'b Zpk<T2, Z2, P2, K2>) -> Self::Output
            {
                self.$f(rhs.as_view())
            }
        }
        impl<'a, 'b, T1, T2, Z1, Z2, P1, P2, K1, K2, O> $t<&'b Zpk<T2, Z2, P2, K2>> for &'a Zpk<T1, Z1, P1, K1>
        where
            T1: ComplexFloat,
            T2: ComplexFloat,
            Z1: MaybeList<T1>,
            Z2: MaybeList<T2>,
            P1: MaybeList<T1>,
            P2: MaybeList<T2>,
            K1: ComplexFloat<Real = T1::Real>,
            K2: ComplexFloat<Real = T2::Real>,
            Z1::View<'a>: MaybeList<T1>,
            P1::View<'a>: MaybeList<T1>,
            Z2::View<'b>: MaybeList<T2>,
            P2::View<'b>: MaybeList<T2>,
            Zpk<T1, Z1::View<'a>, P1::View<'a>, K1>: $t<Zpk<T2, Z2::View<'b>, P2::View<'b>, K2>, Output = O>
        {
            type Output = O;

            fn $f(self, rhs: &'b Zpk<T2, Z2, P2, K2>) -> Self::Output
            {
                self.as_view().$f(rhs.as_view())
            }
        }

        impl<T, Z, P, K, O> $t<K> for Zpk<T, Z, P, K>
        where
            T: ComplexFloat,
            Z: MaybeList<T>,
            P: MaybeList<T>,
            K: ComplexFloat<Real = T::Real>,
            Self: $t<Zpk<T, (), (), K>, Output = O>
        {
            type Output = O;

            fn $f(self, rhs: K) -> Self::Output
            {
                self.$f(Zpk {
                    z: ProductSequence::new(()),
                    p: ProductSequence::new(()),
                    k: rhs
                })
            }
        }
        impl<'b, T, Z, P, K, O> $t<K> for &'b Zpk<T, Z, P, K>
        where
            T: ComplexFloat,
            Z: MaybeList<T>,
            P: MaybeList<T>,
            K: ComplexFloat<Real = T::Real>,
            Self: $t<Zpk<T, (), (), K>, Output = O>
        {
            type Output = O;

            fn $f(self, rhs: K) -> Self::Output
            {
                self.$f(Zpk {
                    z: ProductSequence::new(()),
                    p: ProductSequence::new(()),
                    k: rhs
                })
            }
        }
    };
}
impl_op2_extra!(Mul::mul);
impl_op2_extra!(Div::div);

#[allow(unused)]
macro s {
    (s) => {},
    (z) => {},
}

pub macro zpk {
    ($t:path[$s:ident]= {$e:expr}) => {
        {
            Zpk::<$t, [_; 1], (), $t>::$s();
            s!($s);
            $e
        }
    },
    ($t:path[$s:ident]=) => {
        {
            Zpk::<$t, [_; 1], (), $t>::$s();
            s!($s);
            Zpk::<$t, _, _, $t>::new((), ())
        }
    },
    ($t:path[$s:ident]= $c:literal) => {
        {
            Zpk::<$t, [_; 1], (), $t>::$s();
            s!($s);
            Zpk::<$t, _, _, $t>::new((), (), <$t as num::NumCast>::from($c).unwrap())
        }
    },
    ($t:path[$s:ident]= $c:literal + $ci:literal j) => {
        {
            Zpk::<$t, [_; 1], (), $t>::$s();
            s!($s);
            Zpk::<$t, _, _, num::Complex<<$t as num::complex::ComplexFloat>::Real>>::new((), (), num::Complex::new(
                <$t as num::NumCast>::from($c).unwrap(),
                <$t as num::NumCast>::from($ci).unwrap()
            ))
        }
    },
    ($t:path[$s:ident]= $c:literal - $ci:literal j) => {
        {
            Zpk::<$t, [_; 1], (), $t>::$s();
            s!($s);
            Zpk::<$t, _, _, num::Complex<<$t as num::complex::ComplexFloat>::Real>>::new((), (), num::Complex::new(
                <$t as num::NumCast>::from($c).unwrap(),
                -<$t as num::NumCast>::from($ci).unwrap()
            ))
        }
    },
    ($t:path[$s:ident]= $ci:literal j) => {
        {
            Zpk::<$t, [_; 1], (), $t>::$s();
            s!($s);
            Zpk::<$t, _, _, num::Complex<<$t as num::complex::ComplexFloat>::Real>>::new((), (), num::Complex::new(
                <$t as num::NumCast>::from(0).unwrap(),
                <$t as num::NumCast>::from($ci).unwrap()
            ))
        }
    },

    ($t:path[$s:ident]= $ss:ident + $c:literal) => {
        {
            #[allow(unused)]
            let $s = Zpk::<$t, [_; 1], (), $t>::$s();
            s!($s);
            s!($ss);
            let _ = $ss;
            Zpk::<$t, _, _, $t>::new([-<$t as num::NumCast>::from($c).unwrap()], (), <$t as num::NumCast>::from(1).unwrap())
        }
    },
    ($t:path[$s:ident]= $ss:ident + $c:literal + $ci:literal j) => {
        {
            #[allow(unused)]
            let $s = Zpk::<$t, [_; 1], (), $t>::$s();
            s!($s);
            s!($ss);
            let _ = $ss;
            Zpk::<num::Complex<<$t as num::complex::ComplexFloat>::Real>, _, _, $t>::new([num::Complex::new(
                -<$t as num::NumCast>::from($c).unwrap(),
                -<$t as num::NumCast>::from($ci).unwrap()
            )], (), <$t as num::NumCast>::from(1).unwrap())
        }
    },
    ($t:path[$s:ident]= $ss:ident + $c:literal - $ci:literal j) => {
        {
            #[allow(unused)]
            let $s = Zpk::<$t, [_; 1], (), $t>::$s();
            s!($s);
            s!($ss);
            let _ = $ss;
            Zpk::<num::Complex<<$t as num::complex::ComplexFloat>::Real>, _, _, $t>::new([num::Complex::new(
                -<$t as num::NumCast>::from($c).unwrap(),
                <$t as num::NumCast>::from($ci).unwrap()
            )], (), <$t as num::NumCast>::from(1).unwrap())
        }
    },
    ($t:path[$s:ident]= $ss:ident + $ci:literal j) => {
        {
            #[allow(unused)]
            let $s = Zpk::<$t, [_; 1], (), $t>::$s();
            s!($s);
            s!($ss);
            let _ = $ss;
            Zpk::<num::Complex<<$t as num::complex::ComplexFloat>::Real>, _, _, $t>::new([num::Complex::new(
                <$t as num::NumCast>::from(0).unwrap(),
                -<$t as num::NumCast>::from($ci).unwrap()
            )], (), <$t as num::NumCast>::from(1).unwrap())
        }
    },
    
    ($t:path[$s:ident]= $ss:ident - $c:literal) => {
        {
            #[allow(unused)]
            let $s = Zpk::<$t, [_; 1], (), $t>::$s();
            s!($s);
            s!($ss);
            let _ = $ss;
            Zpk::<$t, _, _, $t>::new([<$t as num::NumCast>::from($c).unwrap()], (), <$t as num::NumCast>::from(1).unwrap())
        }
    },
    ($t:path[$s:ident]= $ss:ident - $c:literal + $ci:literal j) => {
        {
            #[allow(unused)]
            let $s = Zpk::<$t, [_; 1], (), $t>::$s();
            s!($s);
            s!($ss);
            let _ = $ss;
            Zpk::<num::Complex<<$t as num::complex::ComplexFloat>::Real>, _, _, $t>::new([num::Complex::new(
                <$t as num::NumCast>::from($c).unwrap(),
                -<$t as num::NumCast>::from($ci).unwrap()
            )], (), <$t as num::NumCast>::from(1).unwrap())
        }
    },
    ($t:path[$s:ident]= $ss:ident - $c:literal - $ci:literal j) => {
        {
            #[allow(unused)]
            let $s = Zpk::<$t, [_; 1], (), $t>::$s();
            s!($s);
            s!($ss);
            let _ = $ss;
            Zpk::<num::Complex<<$t as num::complex::ComplexFloat>::Real>, _, _, $t>::new([num::Complex::new(
                <$t as num::NumCast>::from($c).unwrap(),
                <$t as num::NumCast>::from($ci).unwrap()
            )], (), <$t as num::NumCast>::from(1).unwrap())
        }
    },
    ($t:path[$s:ident]= $ss:ident - $ci:literal j) => {
        {
            #[allow(unused)]
            let $s = Zpk::<$t, [_; 1], (), $t>::$s();
            s!($s);
            s!($ss);
            let _ = $ss;
            Zpk::<num::Complex<<$t as num::complex::ComplexFloat>::Real>, _, _, $t>::new([num::Complex::new(
                <$t as num::NumCast>::from(0).unwrap(),
                <$t as num::NumCast>::from($ci).unwrap()
            )], (), <$t as num::NumCast>::from(1).unwrap())
        }
    },

    ($t:path[$s:ident]= $ss:ident) => {
        {
            #[allow(unused)]
            let $s = Zpk::<$t, [_; 1], (), $t>::$s();
            s!($s);
            s!($ss);
            Zpk::<$t, [_; 1], (), $t>::from($ss)
        }
    },
    ($t:path[$s:ident]= $c:literal^$pc:literal) => {
        {
            #[allow(unused)]
            let $s = Zpk::<$t, [_; 1], (), $t>::$s();
            s!($s);
            s!($ss);
            let _ = $ss;
            num::traits::Pow::pow(Zpk::<$t, _, _, $t>::new((), (), <$t as num::NumCast>::from($c).unwrap()), $pc)
        }
    },
    ($t:path[$s:ident]= $ss:ident^$ps:literal) => {
        {
            #[allow(unused)]
            let $s = Zpk::<$t, [_; 1], (), $t>::$s();
            s!($s);
            s!($ss);
            num::traits::Pow::pow(Zpk::<$t, [_; 1], (), $t>::from($ss), $ps)
        }
    },
    ($t:path[$s:ident]= ($($lhs:tt)*)^$lp:literal $($op:tt ($($rhs:tt)*))?) => {
        num::traits::Pow::pow(zpk!($t[$s]= $($lhs)*), $lp)$($op zpk!($t[$s]= $($rhs)*))*
    },
    ($t:path[$s:ident]= ($($lhs:tt)*) $($op:tt ($($rhs:tt)*))?) => {
        zpk!($t[$s]= $($lhs)*)$($op zpk!($t[$s]= $($rhs)*))*
    },
    ($t:path[$s:ident]= $lhs:tt$(^$lp:literal)? $($op:tt $rhs:tt$(^$rp:literal)?)*) => {
        zpk!($t[$s]= $lhs$(^$lp)?)$($op zpk!($t[$s]= $rhs$(^$rp)?))*
    },
}

#[cfg(test)]
mod test
{
    use crate::systems::zpk;

    #[test]
    fn test()
    {
        let h1 = zpk!(f64[s] = (s - 1 + 1 j)*(s - 1 - 1 j));
        let h2 = zpk!(f64[s] = s + 2);

        let h = (h1 + h2)/zpk!(f64[s] = 1 - s);

        println!("{:?}", h);
    }
}