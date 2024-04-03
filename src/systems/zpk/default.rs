use num::complex::ComplexFloat;

use crate::{MaybeList, ProductSequence, Zpk};

impl<T, Z, P, K> Default for Zpk<T, Z, P, K>
where
    T: ComplexFloat,
    Z: MaybeList<T>,
    P: MaybeList<T>,
    K: ComplexFloat<Real = T::Real>,
    ProductSequence<T, Z>: Default,
    ProductSequence<T, P>: Default
{
    fn default() -> Self
    {
        Self {
            z: Default::default(),
            p: Default::default(),
            k: K::one()
        }
    }
}