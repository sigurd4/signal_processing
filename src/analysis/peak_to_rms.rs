use num::{traits::FloatConst, Float};

use crate::quantities::{List, ListOrSingle, Lists};

pub trait PeakToRms<'a, T>: Lists<T>
where
    T: Float
{
    fn peak_to_rms(&'a self) -> Self::RowsMapped<T>;
}

impl<'a, T, L> PeakToRms<'a, T> for L
where
    L: Lists<T, RowView<'a>: List<T>> + 'a,
    T: Float + FloatConst + 'a
{
    fn peak_to_rms(&'a self) -> Self::RowsMapped<T>
    {
        let one = T::one();
        let two = one + one;
        let p2p2rms = T::FRAC_1_SQRT_2()/two;

        self.map_rows_to_owned(|x| {
            let x = x.to_vec();
            let p2p = x.iter()
                .map(|&x| x)
                .reduce(T::max)
                .unwrap_or(T::zero())
            - x.into_iter()
                .reduce(T::min)
                .unwrap_or(T::zero());
            p2p*p2p2rms
        })
    }
}