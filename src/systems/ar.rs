use core::marker::PhantomData;

use num::complex::ComplexFloat;

use crate::{ListOrSingle, MaybeList};

pub struct Ar<T, A, AV>
where
    T: ComplexFloat,
    A: MaybeList<T>,
    AV: ListOrSingle<(A, T)>
{
    pub av: AV,
    phantom: PhantomData<(A, T)>
}

impl<T, A, AV> Ar<T, A, AV>
where
    T: ComplexFloat,
    A: MaybeList<T>,
    AV: ListOrSingle<(A, T)>
{
    pub fn new(av: AV) -> Self
    {
        Self {
            av,
            phantom: PhantomData
        }
    }
}