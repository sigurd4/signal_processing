use core::ops::Neg;

use num::complex::ComplexFloat;

use crate::{MaybeList, Zpk};

impl<T, Z, P, K> Neg for Zpk<T, Z, P, K>
where
    T: ComplexFloat,
    K: ComplexFloat<Real = T::Real>,
    Z: MaybeList<T>,
    P: MaybeList<T>
{
    type Output = Zpk<T, Z, P, K>;

    fn neg(self) -> Self::Output
    {
        Zpk {
            z: self.z,
            p: self.p,
            k: -self.k
        }
    }
}