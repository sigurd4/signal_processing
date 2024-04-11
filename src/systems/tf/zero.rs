use core::ops::Add;

use num::{complex::ComplexFloat, Zero};

use crate::{MaybeList, MaybeLists, Polynomial, Tf};

impl<T, B, A> Zero for Tf<T, B, A>
where
    T: ComplexFloat,
    B: MaybeLists<T>,
    A: MaybeList<T>,
    Polynomial<T, [T; 0]>: Into<Polynomial<T, B>>,
    Polynomial<T, ()>: Into<Polynomial<T, A>>,
    Self: Add<Output = Self>
{
    fn zero() -> Self
    {
        Tf::zero()
    }
    fn is_zero(&self) -> bool
    {
        self.is_zero()
    }
}