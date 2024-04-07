use core::ops::Mul;


use num::{One};

use crate::Polynomial;

impl<T> One for Polynomial<T, Vec<T>>
where
    Self: Mul<Output = Self>,
    Polynomial<T, ()>: Into<Self>
{
    fn one() -> Self
    {
        Polynomial::new(()).into()
    }
}