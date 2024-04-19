use core::ops::{RangeInclusive, RemAssign, SubAssign};

use num::{Bounded, Float, Integer, NumCast};
use option_trait::Maybe;

use crate::Lists;

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum EncodeOverflow
{
    Saturate,
    Wrap
}

pub trait Encode<T, I>: Lists<T>
where
    I: Integer + Bounded,
    T: Float
{
    fn encode<V>(self, dynamic_range: V, overflow: EncodeOverflow) -> Self::Mapped<I>
    where
        V: Maybe<RangeInclusive<T>>;
}

impl<T, L, I> Encode<T, I> for L
where
    I: Integer + Bounded + NumCast,
    T: Float + RemAssign + SubAssign,
    L: Lists<T>
{
    fn encode<V>(self, dynamic_range: V, overflow: EncodeOverflow) -> Self::Mapped<I>
    where
        V: Maybe<RangeInclusive<T>>
    {
        let one = T::one();
        let two = one + one;

        let imax = T::from(I::max_value())
            .map(|m| m + one)
            .unwrap_or_else(T::max_value);
        let imin = T::from(I::min_value())
            .unwrap_or_else(T::min_value);

        let dynamic_range = dynamic_range.into_option()
            .unwrap_or(-one..=one);
        let mid = (*dynamic_range.start() + *dynamic_range.end())/two;

        self.map_into_owned(|x| {
            let mut y = ((x - *dynamic_range.start())/(*dynamic_range.end() - *dynamic_range.start())*(imax - imin) + imin).round();
            if overflow == EncodeOverflow::Wrap
            {
                y %= imax.max(-imin*two);
                while y >= imax
                {
                    y -= imax
                }
                while y < imin
                {
                    y -= imin
                }
            }
            I::from(y)
                .unwrap_or_else(|| if x > mid
                {
                    I::max_value()
                }
                else if x < mid
                {
                    I::min_value()
                }
                else
                {
                    (I::max_value() + I::min_value())/(I::one() + I::one())
                })
        })
    }
}