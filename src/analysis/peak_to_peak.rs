use num::Float;

use crate::{List, ListOrSingle, Lists};

pub trait PeakToPeak<'a, T>: Lists<T>
where
    T: Float
{
    fn peak_to_peak(&'a self) -> Self::RowsMapped<T>;
}

impl<'a, T, L> PeakToPeak<'a, T> for L
where
    L: Lists<T, RowView<'a>: List<T>> + 'a,
    T: Float + 'a
{
    fn peak_to_peak(&'a self) -> Self::RowsMapped<T>
    {
        self.map_rows_to_owned(|x| {
            let x = x.to_vec();
            x.iter()
                .map(|&x| x)
                .reduce(T::max)
                .unwrap_or(T::zero())
            - x.into_iter()
                .reduce(T::min)
                .unwrap_or(T::zero())
        })
    }
}