use core::ops::Add;

use num::{complex::ComplexFloat, Float, Zero};

use crate::quantities::{List, ListOrSingle, Lists};

pub trait Rssq<'a, T>: Lists<T>
where
    T: ComplexFloat
{
    fn rssq(&'a self) -> Self::RowsMapped<T::Real>;
}

impl<'a, T, L> Rssq<'a, T> for L
where
    L: Lists<T, RowView<'a>: List<T>> + 'a,
    T: ComplexFloat + 'a
{
    fn rssq(&'a self) -> Self::RowsMapped<T::Real>
    {
        self.map_rows_to_owned(|x| {
            let x = x.to_vec();
            x.into_iter()
                .map(|x| (x.conj()*x).re())
                .reduce(Add::add)
                .map(|x| Float::sqrt(x))
                .unwrap_or(T::Real::zero())
        })
    }
}