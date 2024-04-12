use core::iter::Sum;

use num::Zero;

use crate::{MaybeList, SumSequence};

impl<T, S1, S2> Sum<SumSequence<T, S1>> for SumSequence<T, S2>
where
    S1: MaybeList<T>,
    S2: MaybeList<T>,
    SumSequence<T, S1>: Into<SumSequence<T, S2>>,
    SumSequence<T, S2>: Zero
{
    fn sum<I: Iterator<Item = SumSequence<T, S1>>>(iter: I) -> Self
    {
        iter.map(Into::into)
            .reduce(|a, b| a + b)
            .unwrap_or_else(Zero::zero)
    }
}