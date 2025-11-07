use core::ops::Add;

use num::{complex::ComplexFloat, Float, NumCast, Zero};

use crate::quantities::{List, ListOrSingle, Lists};

pub trait Rms<'a, T>: Lists<T>
where
    T: ComplexFloat
{
    fn rms(&'a self) -> Self::RowsMapped<T::Real>;
}

impl<'a, T, L> Rms<'a, T> for L
where
    L: Lists<T, RowView<'a>: List<T>> + 'a,
    T: ComplexFloat + 'a
{
    fn rms(&'a self) -> Self::RowsMapped<T::Real>
    {
        self.map_rows_to_owned(|x| {
            let x = x.to_vec();
            let l = <T::Real as NumCast>::from(x.len()).unwrap();
            x.into_iter()
                .map(|x| (x.conj()*x).re())
                .reduce(Add::add)
                .map(|x| Float::sqrt(x/l))
                .unwrap_or(T::Real::zero())
        })
    }
}