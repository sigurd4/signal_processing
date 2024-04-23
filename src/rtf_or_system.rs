use num::complex::ComplexFloat;
use option_trait::Maybe;

use crate::{Ar, ListOrSingle, MaybeList, MaybeLists, MaybeOwnedList, Rpk, Rtf, Sos, Ss, SsAMatrix, SsBMatrix, SsCMatrix, SsDMatrix, System, Tf, Zpk};

pub trait RtfOrSystem
{
    type Domain: ComplexFloat;
}

impl<'a, W, S> RtfOrSystem for Rtf<'a, W, S>
where
    W: ComplexFloat<Real = <S::Domain as ComplexFloat>::Real>,
    S::Domain: Into<W>,
    S: System
{
    type Domain = W;
}

impl<T, B, A> RtfOrSystem for Tf<T, B, A>
where
    T: ComplexFloat,
    B: MaybeLists<T>,
    A: MaybeList<T>
{
    type Domain = T;
}

impl<T, Z, P, K, R> RtfOrSystem for Zpk<T, Z, P, K>
where
    T: ComplexFloat<Real = R>,
    K: ComplexFloat<Real = R>,
    Z: MaybeList<T>,
    P: MaybeList<T>
{
    type Domain = K;
}

impl<T, A, B, C, D> RtfOrSystem for Ss<T, A, B, C, D>
where
    T: ComplexFloat,
    A: SsAMatrix<T, B, C, D>,
    B: SsBMatrix<T, A, C, D>,
    C: SsCMatrix<T, A, B, D>,
    D: SsDMatrix<T, A, B, C>
{
    type Domain = T;
}

impl<T, B, A, S> RtfOrSystem for Sos<T, B, A, S>
where
    T: ComplexFloat,
    B: Maybe<[T; 3]> + MaybeOwnedList<T>,
    A: Maybe<[T; 3]> + MaybeOwnedList<T>,
    S: MaybeList<Tf<T, B, A>>
{
    type Domain = T;
}

impl<T, R, P, RP, K> RtfOrSystem for Rpk<T, R, P, RP, K>
where
    T: ComplexFloat,
    R: ComplexFloat<Real = T::Real>,
    P: ComplexFloat<Real = T::Real>,
    RP: MaybeList<(R, P)>,
    K: MaybeList<T>
{
    type Domain = T;
}

impl<T, A, AV> RtfOrSystem for Ar<T, A, AV>
where
    T: ComplexFloat,
    A: MaybeList<T>,
    AV: ListOrSingle<(A, T::Real)>
{
    type Domain = T;
}