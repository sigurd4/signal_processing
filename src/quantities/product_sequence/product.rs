use core::iter::Product;

use num::One;

use crate::{MaybeList, ProductSequence};

impl<T, S1, S2> Product<ProductSequence<T, S1>> for ProductSequence<T, S2>
where
    S1: MaybeList<T>,
    S2: MaybeList<T>,
    ProductSequence<T, S1>: Into<ProductSequence<T, S2>>,
    ProductSequence<T, S2>: One
{
    fn product<I: Iterator<Item = ProductSequence<T, S1>>>(iter: I) -> Self
    {
        iter.map(Into::into)
            .reduce(|a, b| a*b)
            .unwrap_or_else(One::one)
    }
}