use num::complex::ComplexFloat;
use option_trait::Maybe;

use crate::{Ar, ListOrSingle, MaybeList, MaybeLists, MaybeOwnedList, MaybeRtfOrSystem, Rpk, Sos, Ss, SsAMatrix, SsBMatrix, SsCMatrix, SsDMatrix, Tf, Zpk};

pub trait MaybeSystem<D>: MaybeRtfOrSystem<D>
where
    D: ComplexFloat
{
    
}

impl<D> MaybeSystem<D> for ()
where   
    D: ComplexFloat
{

}

impl<T, B, A> MaybeSystem<T> for Tf<T, B, A>
where
    T: ComplexFloat,
    B: MaybeLists<T>,
    A: MaybeList<T>
{

}

impl<T, Z, P, K, R> MaybeSystem<K> for Zpk<T, Z, P, K>
where
    T: ComplexFloat<Real = R>,
    K: ComplexFloat<Real = R>,
    Z: MaybeList<T>,
    P: MaybeList<T>
{
    
}

impl<T, A, B, C, D> MaybeSystem<T> for Ss<T, A, B, C, D>
where
    T: ComplexFloat,
    A: SsAMatrix<T, B, C, D>,
    B: SsBMatrix<T, A, C, D>,
    C: SsCMatrix<T, A, B, D>,
    D: SsDMatrix<T, A, B, C>
{
    
}

impl<T, B, A, S> MaybeSystem<T> for Sos<T, B, A, S>
where
    T: ComplexFloat,
    B: Maybe<[T; 3]> + MaybeOwnedList<T>,
    A: Maybe<[T; 3]> + MaybeOwnedList<T>,
    S: MaybeList<Tf<T, B, A>>
{
    
}

impl<T, R, P, RP, K> MaybeSystem<T> for Rpk<T, R, P, RP, K>
where
    T: ComplexFloat,
    R: ComplexFloat<Real = T::Real>,
    P: ComplexFloat<Real = T::Real>,
    RP: MaybeList<(R, P)>,
    K: MaybeList<T>
{

}

impl<T, A, AV> MaybeSystem<T> for Ar<T, A, AV>
where
    T: ComplexFloat,
    A: MaybeList<T>,
    AV: ListOrSingle<(A, T::Real)>
{

}