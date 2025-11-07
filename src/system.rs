use num::complex::ComplexFloat;

use crate::{
    quantities::{ListOrSingle, MaybeList, MaybeLists, MaybeOwnedList},
    systems::{Ar, Rpk, Sos, Ss, SsAMatrix, SsBMatrix, SsCMatrix, SsDMatrix, Tf, Zpk},
    RtfOrSystem,
    MaybeSystem
};

pub trait System: RtfOrSystem + MaybeSystem<Self::Set>
{
    
}

impl<T, B, A> System for Tf<T, B, A>
where
    T: ComplexFloat,
    B: MaybeLists<T>,
    A: MaybeList<T>
{

}

impl<T, Z, P, K, R> System for Zpk<T, Z, P, K>
where
    T: ComplexFloat<Real = R>,
    K: ComplexFloat<Real = R>,
    Z: MaybeList<T>,
    P: MaybeList<T>
{
    
}

impl<T, A, B, C, D> System for Ss<T, A, B, C, D>
where
    T: ComplexFloat,
    A: SsAMatrix<T, B, C, D>,
    B: SsBMatrix<T, A, C, D>,
    C: SsCMatrix<T, A, B, D>,
    D: SsDMatrix<T, A, B, C>
{
    
}

impl<T, B, A, S> System for Sos<T, B, A, S>
where
    T: ComplexFloat,
    B: Maybe<[T; 3]> + MaybeOwnedList<T>,
    A: Maybe<[T; 3]> + MaybeOwnedList<T>,
    S: MaybeList<Tf<T, B, A>>
{
    
}

impl<T, R, P, RP, K> System for Rpk<T, R, P, RP, K>
where
    T: ComplexFloat,
    R: ComplexFloat<Real = T::Real>,
    P: ComplexFloat<Real = T::Real>,
    RP: MaybeList<(R, P)>,
    K: MaybeList<T>
{

}

impl<T, A, AV> System for Ar<T, A, AV>
where
    T: ComplexFloat,
    A: MaybeList<T>,
    AV: ListOrSingle<(A, T::Real)>
{

}