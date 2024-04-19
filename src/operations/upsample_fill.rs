use crate::{List, ListOrSingle, Lists};

pub trait UpsampleFill<T, V, Y>: Lists<T>
where
    V: List<T>,
    Y: List<T>
{
    fn upsample_fill(self, interleave: V, phase: usize) -> Self::RowsMapped<Y>;
}

impl<T, V, L> UpsampleFill<T, V, Vec<T>> for L
where
    V: List<T>,
    L: Lists<T, RowOwned: List<T>>,
    T: Clone
{
    fn upsample_fill(self, interleave: V, mut phase: usize) -> Self::RowsMapped<Vec<T>>
    {
        let interleave = interleave.into_vec();

        phase %= interleave.len() + 1;

        self.map_rows_into_owned(|x| {
            x.into_vec()
                .into_iter()
                .flat_map(|x| {
                    let mut y = interleave.clone();
                    y.insert(phase, x);
                    y
                }).collect()
        })
    }
}

impl<T, L, V> UpsampleFill<T, V, [T; L::WIDTH*(V::WIDTH + 1)]> for L
where
    V: List<T, Width = usize>,
    L: Lists<T, RowOwned: List<T>, Width = usize>,
    T: Clone
{
    fn upsample_fill(self, interleave: V, mut phase: usize) -> Self::RowsMapped<[T; L::WIDTH*(V::WIDTH + 1)]>
    {
        let interleave = interleave.into_vec();

        phase %= interleave.len() + 1;

        self.map_rows_into_owned(|x| {
            x.into_vec()
                .into_iter()
                .flat_map(|x| {
                    let mut y = interleave.clone();
                    y.insert(phase, x);
                    y
                }).collect::<Vec<_>>()
                .try_into()
                .ok()
                .unwrap()
        })
    }
}