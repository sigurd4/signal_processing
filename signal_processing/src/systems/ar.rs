use core::marker::PhantomData;

use num::complex::ComplexFloat;

use crate::quantities::{ListOrSingle, MaybeList};

moddef::moddef!(
    mod {
        debug
    }
);

#[derive(Clone, Copy)]
pub struct Ar<T, A, AV>
where
    T: ComplexFloat,
    A: MaybeList<T>,
    AV: ListOrSingle<(A, T::Real)>
{
    pub av: AV,
    phantom: PhantomData<(A, T)>
}

impl<T, A, AV> Ar<T, A, AV>
where
    T: ComplexFloat,
    A: MaybeList<T>,
    AV: ListOrSingle<(A, T::Real)>
{
    pub fn new(av: AV) -> Self
    {
        Self {
            av,
            phantom: PhantomData
        }
    }
}