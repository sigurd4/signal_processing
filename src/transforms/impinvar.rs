use core::{any::Any, iter::Product, ops::{Add, Div, Mul, MulAssign}};

use num::{complex::ComplexFloat, Complex, NumCast, One};
use option_trait::Maybe;
use thiserror::Error;

use crate::{MaybeList, ProductSequence, System, Tf, ToTf, ToZpk, Zpk};

#[derive(Debug, Clone, Copy, PartialEq, Error)]
pub enum ImpInvarError
{
    #[error("Non causal transfer function, i.e. it contains one or more poles at infinity.")]
    NonCausal,
    #[error("The system must contain at least one pole.")]
    ZeroPoles
}

pub trait ImpInvar: System
{
    type Output: Sized;

    fn impinvar<TOL>(
        self,
        sampling_frequency: <Self::Domain as ComplexFloat>::Real,
        tolerance: TOL
    ) -> Result<Self::Output, ImpInvarError>
    where
        TOL: Maybe<<Self::Domain as ComplexFloat>::Real>;
}

impl<T, B, A> ImpInvar for Tf<T, B, A>
where
    T: ComplexFloat,
    B: MaybeList<T>,
    A: MaybeList<T>,
{
    type Output = Tf<T, Vec<T>, Vec<T>>;

    fn impinvar<TOL>(self, sampling_frequency: T::Real, tolerance: TOL) -> Result<Self::Output, ImpInvarError>
    where
        TOL: Maybe<T::Real>
    {
        let ts = sampling_frequency.recip();

        let []
    }
}