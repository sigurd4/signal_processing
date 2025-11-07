use core::{iter::Sum, ops::Add};

use num::Zero;

use crate::quantities::{MaybeList, SumSequence};

impl<T, S> Zero for SumSequence<T, S>
where
    T: Zero + Clone + Sum,
    S: MaybeList<T>,
    Self: Add<Output = Self>,
    SumSequence<T, ()>: Into<Self>
{
    fn zero() -> Self
    {
        SumSequence::new(()).into()
    }
    fn is_zero(&self) -> bool
    {
        self.is_zero()
    }
}