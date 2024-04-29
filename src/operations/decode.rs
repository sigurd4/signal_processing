use core::ops::RangeInclusive;

use num::{Bounded, Float, Integer, NumCast};

use crate::quantities::Lists;

pub trait Decode<T, I>: Lists<I>
where
    I: Integer + Bounded,
    T: Float
{
    fn decode(self, dynamic_range: RangeInclusive<T>) -> Self::Mapped<T>;
}

impl<T, L, I> Decode<T, I> for L
where
    I: Integer + Bounded + NumCast + Clone,
    T: Float,
    L: Lists<I>
{
    fn decode(self, dynamic_range: RangeInclusive<T>) -> Self::Mapped<T>
    {
        let one = T::one();

        let imax = T::from(I::max_value())
            .map(|m| m + one)
            .unwrap_or_else(T::max_value);
        let imin = T::from(I::min_value())
            .unwrap_or_else(T::min_value);

        self.map_into_owned(|y| {
            let y = T::from(y).unwrap();
            (y - imin)/(imax - imin)*(*dynamic_range.end() - *dynamic_range.start()) + *dynamic_range.start()
        })
    }
}