use array_math::max_len;
use num::Zero;
use option_trait::Maybe;

use crate::quantities::{List, ListOrSingle, Lists};

pub trait Upsample<T, N, Y>: Lists<T>
where
    N: Maybe<usize>,
    Y: List<T>
{
    fn upsample(self, n: N, phase: usize) -> Self::RowsMapped<Y>;
}

impl<T, L> Upsample<T, usize, Vec<T>> for L
where
    L: Lists<T, RowOwned: List<T>>,
    T: Copy + Zero
{
    fn upsample(self, n: usize, mut phase: usize) -> Self::RowsMapped<Vec<T>>
    {
        phase %= n;

        let zero = T::zero();

        self.map_rows_into_owned(|x| {
            x.into_vec()
                .into_iter()
                .flat_map(|x| {
                    let mut y = vec![zero; n];
                    if let Some(y) = y.get_mut(phase)
                    {
                        *y = x
                    }
                    y
                }).collect()
        })
    }
}

impl<T, L, const N: usize> Upsample<T, (), [T; N]> for L
where
    L: Lists<T, RowOwned: List<T>, Width = usize>,
    T: Copy + Zero,
    [(); 0 - N % max_len(L::WIDTH, 1)]:
{
    fn upsample(self, (): (), mut phase: usize) -> Self::RowsMapped<[T; N]>
    {
        let n = N / L::WIDTH.max(1);
        phase %= n;
        
        let zero = T::zero();

        self.map_rows_into_owned(|x| {
            x.into_vec()
                .into_iter()
                .flat_map(|x| {
                    let mut y = vec![zero; n];
                    if let Some(y) = y.get_mut(phase)
                    {
                        *y = x
                    }
                    y
                }).collect::<Vec<_>>()
                .try_into()
                .ok()
                .unwrap()
        })
    }
}