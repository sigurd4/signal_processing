use core::ops::Mul;

use num::{One};

use crate::{MaybeList, ProductSequence};

impl<T, S> One for ProductSequence<T, S>
where
    S: MaybeList<T>,
    Self: Mul<Output = Self>,
    ProductSequence<T, ()>: Into<Self>
{
    fn one() -> Self
    {
        ProductSequence::new(()).into()
    }
}