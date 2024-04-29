use core::ops::Neg;

use crate::quantities::{MaybeList, SumSequence};

impl<T, S> Neg for SumSequence<T, S>
where
    T: Neg + Clone,
    S: MaybeList<T>,
    S::MaybeMapped<<T as Neg>::Output>: MaybeList<<T as Neg>::Output>
{
    type Output = SumSequence<<T as Neg>::Output, S::MaybeMapped<<T as Neg>::Output>>;

    fn neg(self) -> Self::Output
    {
        SumSequence::new(
            self.into_inner()
                .maybe_map_into_owned(Neg::neg)
        )
    }
}