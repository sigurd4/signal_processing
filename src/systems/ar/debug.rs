use core::fmt::Debug;
use num::complex::ComplexFloat;

use crate::{Ar, ListOrSingle, MaybeList};

impl<T, A, AV> Debug for Ar<T, A, AV>
where
    T: ComplexFloat,
    A: MaybeList<T>,
    AV: ListOrSingle<(A, T::Real)> + Debug
{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result
    {
        f.debug_struct("Ar")
            .field("av", &self.av)
            .finish()
    }
}