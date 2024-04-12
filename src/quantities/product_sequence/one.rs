use core::{iter::Product, ops::Mul};

use num::One;

use crate::{MaybeList, ProductSequence};

impl<T, S> One for ProductSequence<T, S>
where
    T: Clone + Product + One + PartialEq,
    S: MaybeList<T>,
    Self: Mul<Output = Self>,
    ProductSequence<T, ()>: Into<Self>
{
    fn one() -> Self
    {
        ProductSequence::new(()).into()
    }
    fn is_one(&self) -> bool
    where
        Self: PartialEq
    {
        self.is_one()
    }
}