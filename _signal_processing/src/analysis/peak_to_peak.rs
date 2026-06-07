use core::ops::Sub;

use num::Zero;

use crate::quantities::{List, ListOrSingle, Lists};

pub trait PeakToPeak<'a, T>: Lists<T>
where
    T: PartialOrd + Sub
{
    fn peak_to_peak(&'a self) -> Self::RowsMapped<<T as Sub>::Output>;
}

impl<'a, T, L> PeakToPeak<'a, T> for L
where
    L: Lists<T, RowView<'a>: List<T>> + 'a,
    T: PartialOrd + Sub + Zero + Clone + 'a
{
    fn peak_to_peak(&'a self) -> Self::RowsMapped<<T as Sub>::Output>
    {
        self.map_rows_to_owned(|x| {
            let x = x.to_vec();
            x.iter()
                .map(|x| x.clone())
                .reduce(|a, b| if a == a && a >= b {a} else {b})
                .unwrap_or(T::zero())
            - x.into_iter()
                .reduce(|a, b| if a == a && a <= b {a} else {b})
                .unwrap_or(T::zero())
        })
    }
}