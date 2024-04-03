use num::complex::ComplexFloat;
use option_trait::Maybe;

use crate::{Matrix, MaybeList, MaybeLists, Rtf, Sos, Ss, System, Tf, Zpk};

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
    A: Matrix<T>,
    B: Matrix<T>,
    C: Matrix<T>,
    D: Matrix<T>
{
    type Domain = T;
}

impl<T, B, A, S> RtfOrSystem for Sos<T, B, A, S>
where
    T: ComplexFloat,
    B: Maybe<[T; 3]> + MaybeList<T>,
    A: Maybe<[T; 3]> + MaybeList<T>,
    S: MaybeList<Tf<T, B, A>>
{
    type Domain = T;
}