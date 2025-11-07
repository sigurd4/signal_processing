use num::{traits::FloatConst, Float};
use option_trait::Maybe;

use crate::quantities::{IntoList, ListOrSingle};

pub trait RectPuls<T, L, N>: IntoList<T, L, N>
where
    T: Float,
    L: ListOrSingle<T>,
    N: Maybe<usize>
{
    fn rectpuls(self, numtaps: N, bandwidth: T) -> (L::Mapped<T>, L);
}

impl<T, L, R, N> RectPuls<T, L, N> for R
where
    T: Float + FloatConst,
    L: ListOrSingle<T>,
    R: IntoList<T, L, N>,
    N: Maybe<usize>
{
    fn rectpuls(self, n: N, bandwidth: T) -> (L::Mapped<T>, L)
    {
        let t = self.into_list(n);

        let one = T::one();
        let two = one + one;
        let bw_half = bandwidth/two;

        let y = t.map_to_owned(|&t| {
            T::from((t >= -bw_half && t < bw_half) as u8).unwrap()
        });
        (y, t)
    }
}