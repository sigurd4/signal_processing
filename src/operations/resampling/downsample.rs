use array_math::max_len;
use option_trait::Maybe;

use crate::quantities::{List, ListOrSingle, Lists};

pub trait Downsample<T, N, Y>: Lists<T>
where
    N: Maybe<usize>,
    Y: List<T>
{
    fn downsample(self, n: N, phase: usize) -> Self::RowsMapped<Y>;
}

impl<T, L> Downsample<T, usize, Vec<T>> for L
where
    L: Lists<T, RowOwned: List<T>>,
    T: Clone
{
    fn downsample(self, n: usize, mut phase: usize) -> Self::RowsMapped<Vec<T>>
    {
        phase %= n;

        self.map_rows_into_owned(|x| {
            x.into_vec()
                .into_iter()
                .skip(phase)
                .step_by(n)
                .collect()
        })
    }
}

impl<T, L, const N: usize> Downsample<T, (), [T; N]> for L
where
    L: Lists<T, RowOwned: List<T>, Width = usize>,
    T: Clone,
    [(); 0 - L::WIDTH % max_len(N, 1)]:
{
    fn downsample(self, (): (), mut phase: usize) -> Self::RowsMapped<[T; N]>
    {
        let n = L::WIDTH/max_len(N, 1);
        phase %= n;

        self.map_rows_into_owned(|x| {
            x.into_vec()
                .into_iter()
                .skip(phase)
                .step_by(n)
                .collect::<Vec<_>>()
                .try_into()
                .ok()
                .unwrap()
        })
    }
}