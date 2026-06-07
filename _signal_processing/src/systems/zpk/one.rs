use core::ops::Mul;

use num::{complex::ComplexFloat, One};

use crate::{quantities::List, systems::Zpk};

impl<T, Z, P, K> One for Zpk<T, Z, P, K>
where
    T: ComplexFloat,
    K: ComplexFloat<Real = T::Real>,
    Z: List<T>,
    P: List<T>,
    Self: Default + Mul<Output = Self>
{
    fn one() -> Self
    {
        Zpk::default()
    }

    fn is_one(&self) -> bool
    {
        self.k.is_one() && self.p.as_view_slice().is_empty() && self.z.as_view_slice().is_empty()
    }
}