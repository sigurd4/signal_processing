use core::iter::Product;

use num::One;

use crate::quantities::{MaybeLists, Polynomial};

impl<T, C1, C2> Product<Polynomial<T, C1>> for Polynomial<T, C2>
where
    C1: MaybeLists<T>,
    C2: MaybeLists<T>,
    Polynomial<T, C1>: Into<Self>,
    Self: One
{
    fn product<I: Iterator<Item = Polynomial<T, C1>>>(iter: I) -> Self
    {
        iter.map(|p| p.into())
            .reduce(|a, b| a*b)
            .unwrap_or_else(One::one)
    }
}