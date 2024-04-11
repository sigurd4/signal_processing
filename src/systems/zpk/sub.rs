use core::ops::{Sub, Mul};

use num::{complex::ComplexFloat, Complex};
use option_trait::{MaybeOr, StaticMaybe};

use crate::{MaybeList, SplitNumerDenom, Tf, ToTf, ToZpk, Zpk};

impl<T1, T2, T3, T4, Z1, Z2, Z3, Z4, P1, P2, P3, P4, K1, K2, K3, K4, K5, B1, B2, B3> Sub<Zpk<T2, Z2, P2, K2>> for Zpk<T1, Z1, P1, K1>
where
    T1: ComplexFloat,
    T2: ComplexFloat,
    T3: ComplexFloat,
    T4: ComplexFloat<Real = T3::Real>,
    Z1: MaybeList<T1, MaybeSome: StaticMaybe<Z1::Some, Maybe<Vec<Complex<T1::Real>>>: MaybeList<Complex<T1::Real>, Some = B1>>>,
    Z2: MaybeList<T2, MaybeSome: StaticMaybe<Z2::Some, Maybe<Vec<Complex<T2::Real>>>: MaybeList<Complex<T2::Real>, Some = B2>>>,
    Z3: MaybeList<Complex<T3::Real>>,
    Z4: MaybeList<T4>,
    P1: MaybeList<T1>,
    P2: MaybeList<T2>,
    P3: MaybeList<T3>,
    P4: MaybeList<T4>,
    K1: ComplexFloat<Real = T1::Real>,
    K2: ComplexFloat<Real = T2::Real>,
    K3: ComplexFloat<Real = T3::Real>,
    K4: ComplexFloat<Real = T3::Real>,
    K5: ComplexFloat<Real = T3::Real>,
    B1: MaybeList<Complex<T1::Real>, MaybeSome: StaticMaybe<B1::Some, Maybe<Vec<Complex<T3::Real>>>: MaybeOr<Vec<Complex<T3::Real>>, <B2::MaybeSome as StaticMaybe<B2::Some>>::Maybe<Vec<Complex<T3::Real>>>, Output = Z3>>>,
    B2: MaybeList<Complex<T2::Real>>,
    B3: MaybeList<K3>,

    Self: SplitNumerDenom,
    Zpk<T2, Z2, P2, K2>: SplitNumerDenom,

    <Self as SplitNumerDenom>::OutputNum: ToTf<Complex<K1::Real>, B1, (), (), ()>,
    <Zpk<T2, Z2, P2, K2> as SplitNumerDenom>::OutputNum: ToTf<Complex<K2::Real>, B2, (), (), ()>,

    Tf<Complex<K1::Real>, B1, ()>: Sub<Tf<Complex<K2::Real>, B2, ()>, Output = Tf<K3, B3, ()>>,
    Tf<K3, B3, ()>: ToZpk<Complex<T3::Real>, Z3, (), K3, (), ()>,

    <Self as SplitNumerDenom>::OutputDen: Mul<<Zpk<T2, Z2, P2, K2> as SplitNumerDenom>::OutputDen, Output = Zpk<T3, (), P3, K4>>,
    Zpk<Complex<T3::Real>, Z3, (), K3>: Mul<Zpk<T3, (), P3, K4>, Output = Zpk<T4, Z4, P4, K5>>
{
    type Output = Zpk<T4, Z4, P4, K5>;

    fn sub(self, rhs: Zpk<T2, Z2, P2, K2>) -> Self::Output
    {
        let (n1, d1) = self.split_numer_denom();
        let (n2, d2) = rhs.split_numer_denom();

        let n1 = n1.to_tf((), ());
        let n2 = n2.to_tf((), ());

        let n = n1 - n2;
        let n = n.to_zpk((), ());

        let d = d1*d2;
        n*d
    }
}