use core::ops::Neg;

use num::complex::ComplexFloat;

use crate::{quantities::{MaybeList, Polynomial, SumSequence}, systems::Rpk};

impl<T, T2, R, P, RP, K, K2> Neg for Rpk<T, R, P, RP, K>
where
    T: ComplexFloat,
    T2: ComplexFloat<Real = T::Real>,
    R: ComplexFloat<Real = T::Real>,
    P: ComplexFloat<Real = T::Real>,
    RP: MaybeList<(R, P)>,
    RP::MaybeMapped<(R, P)>: MaybeList<(R, P)>,
    K: MaybeList<T>,
    K2: MaybeList<T2>,
    Polynomial<T, K>: Neg<Output = Polynomial<T2, K2>>
{
    type Output = Rpk<T2, R, P, RP::MaybeMapped<(R, P)>, K2>;

    fn neg(self) -> Self::Output
    {
        Rpk {
            rp: SumSequence::new(self.rp.into_inner().maybe_map_into_owned(|(r, p)| (-r, p))),
            k: -self.k
        }
    }
}