use num::complex::ComplexFloat;
use option_trait::Maybe;

use crate::{Matrix, MaybeList, MaybeLists, RtfOrSystem, Sos, Ss, Tf, Zpk};

pub trait System: RtfOrSystem
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
    A: Matrix<T>,
    B: Matrix<T>,
    C: Matrix<T>,
    D: Matrix<T>
{
    
}

impl<T, B, A, S> System for Sos<T, B, A, S>
where
    T: ComplexFloat,
    B: Maybe<[T; 3]> + MaybeList<T>,
    A: Maybe<[T; 3]> + MaybeList<T>,
    S: MaybeList<Tf<T, B, A>>
{
    
}