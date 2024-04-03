use core::ops::Mul;

use crate::{Chain, MaybeList, ProductSequence};

impl<T, S1, S2> Mul<ProductSequence<T, S2>> for ProductSequence<T, S1>
where
    S1: MaybeList<T> + Chain<S2, Output: MaybeList<T>>,
    S2: MaybeList<T>
{
    type Output = ProductSequence<T, <S1 as Chain<S2>>::Output>;

    fn mul(self, rhs: ProductSequence<T, S2>) -> Self::Output
    {
        ProductSequence::new(
            self.s.chain(rhs.s)
        )
    }
}