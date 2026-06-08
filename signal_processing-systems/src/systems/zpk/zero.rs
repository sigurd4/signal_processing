use core::ops::Add;

use num::{complex::ComplexFloat, Zero};

use crate::{quantities::MaybeList, systems::Zpk};

impl<T, Z, P, K> Zero for Zpk<T, Z, P, K>
where
    T: ComplexFloat,
    Z: MaybeList<T>,
    P: MaybeList<T>,
    K: ComplexFloat<Real = T::Real>,
    Self: Default + Add<Output = Self>
{
    fn zero() -> Self
    {
        Self::zero()
    }
    fn is_zero(&self) -> bool
    {
        self.is_zero()
    }
}