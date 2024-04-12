use core::ops::Add;

use crate::{Chain, MaybeList, SumSequence};

impl<T, S1, S2> Add<SumSequence<T, S2>> for SumSequence<T, S1>
where
    S1: MaybeList<T> + Chain<S2, Output: MaybeList<T>>,
    S2: MaybeList<T>
{
    type Output = SumSequence<T, <S1 as Chain<S2>>::Output>;

    fn add(self, rhs: SumSequence<T, S2>) -> Self::Output
    {
        SumSequence::new(
            self.s.chain(rhs.s)
        )
    }
}