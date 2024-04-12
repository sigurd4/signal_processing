use core::ops::{Add, Neg, Sub};

use crate::{MaybeList, SumSequence};

impl<T1, T2, T3, S1, S2, S3> Sub<SumSequence<T2, S2>> for SumSequence<T1, S1>
where
    S1: MaybeList<T1>,
    S2: MaybeList<T2>,
    S3: MaybeList<T3>,
    SumSequence<T2, S2>: Neg<Output = SumSequence<T3, S3>>,
    Self: Add<SumSequence<T3, S3>>
{
    type Output = <Self as Add<SumSequence<T3, S3>>>::Output;

    fn sub(self, rhs: SumSequence<T2, S2>) -> Self::Output
    {
        self + (-rhs)
    }
}