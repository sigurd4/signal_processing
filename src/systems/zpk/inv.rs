use num::{complex::ComplexFloat, traits::Inv};

use crate::{MaybeList, Zpk};

impl<T, Z, P, K> Inv for Zpk<T, Z, P, K>
where
    T: ComplexFloat,
    K: ComplexFloat<Real = T::Real> + Inv<Output: ComplexFloat<Real = T::Real>>,
    Z: MaybeList<T>,
    P: MaybeList<T>
{
    type Output = Zpk<T, P, Z, <K as Inv>::Output>;

    fn inv(self) -> Self::Output
    {
        Zpk {
            z: self.p,
            p: self.z,
            k: self.k.inv()
        }
    }
}