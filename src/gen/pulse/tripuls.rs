use num::{traits::FloatConst, Float};
use option_trait::Maybe;

use crate::{IntoList, ListOrSingle};

pub trait TriPuls<T, L, N>: IntoList<T, L, N>
where
    T: Float,
    L: ListOrSingle<T>,
    N: Maybe<usize>
{
    fn tripuls<SK>(self, numtaps: N, bandwidth: T, skew: SK) -> (L::Mapped<T>, L)
    where
        SK: Maybe<T>;
}

impl<T, L, R, N> TriPuls<T, L, N> for R
where
    T: Float + FloatConst,
    L: ListOrSingle<T>,
    R: IntoList<T, L, N>,
    N: Maybe<usize>
{
    fn tripuls<SK>(self, n: N, bandwidth: T, skew: SK) -> (L::Mapped<T>, L)
    where
        SK: Maybe<T>
    {
        let t = self.into_list(n);

        let zero = T::zero();
        let one = T::one();
        let two = one + one;
        let bw_half = bandwidth/two;
        let skew = skew.into_option()
            .unwrap_or(zero);
        let peak = skew*bw_half;

        let y = t.map_to_owned(|&t| {
            if t > -bw_half && t <= peak
            {
                (t + bw_half)/(peak + bw_half)
            }
            else if t > peak && t < bw_half
            {
                (t - bw_half)/(peak - bw_half)
            }
            else
            {
                zero
            }
        });
        (y, t)
    }
}