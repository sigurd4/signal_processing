use num::complex::ComplexFloat;

use crate::{MaybeList, Polynomial, SumSequence};

moddef::moddef!(
    mod {
        add,
        neg,
        sub
    }
);

#[derive(Debug, Clone, Copy)]
pub struct Rpk<T, R, P, RP, K>
where
    T: ComplexFloat,
    R: ComplexFloat<Real = T::Real>,
    P: ComplexFloat<Real = T::Real>,
    RP: MaybeList<(R, P)>,
    K: MaybeList<T>
{
    pub rp: SumSequence<(R, P), RP>,
    pub k: Polynomial<T, K>
}

impl<T, R, P, RP, K> Rpk<T, R, P, RP, K>
where
    T: ComplexFloat,
    R: ComplexFloat<Real = T::Real>,
    P: ComplexFloat<Real = T::Real>,
    RP: MaybeList<(R, P)>,
    K: MaybeList<T>
{
    pub fn new(rp: RP, k: K) -> Self
    {
        Self {
            rp: SumSequence::new(rp),
            k: Polynomial::new(k)
        }
    }
}