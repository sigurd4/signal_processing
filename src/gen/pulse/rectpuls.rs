use core::ops::{Range, RangeInclusive};

use array_math::ArrayOps;
use num::{traits::FloatConst, Float};
use option_trait::Maybe;

use crate::{List, NotRange};

pub trait RectPuls<T, L, N>
where
    T: Float,
    L: List<T>,
    N: Maybe<usize>
{
    fn rectpuls(self, numtaps: N, bandwidth: T) -> (L::Mapped<T>, L);
}

impl<T, L> RectPuls<T, L, ()> for L
where
    T: Float + FloatConst,
    L: List<T> + NotRange
{
    fn rectpuls(self, (): (), bandwidth: T) -> (L::Mapped<T>, L)
    {
        let one = T::one();
        let two = one + one;
        let bw_half = bandwidth/two;

        let y = self.map_to_owned(|&t| {
            T::from((t >= -bw_half && t < bw_half) as u8).unwrap()
        });
        (y, self)
    }
}

impl<T, const N: usize> RectPuls<T, [T; N], ()> for Range<T>
where
    T: Float + FloatConst,
    [T; N]: NotRange
{
    fn rectpuls(self, (): (), bandwidth: T) -> ([T; N], [T; N])
    {
        let x: [_; N] = ArrayOps::fill(|i| {
            let p = T::from(i).unwrap()/T::from(N - 1).unwrap();
            self.start + (self.end - self.start)*p
        });
        
        x.rectpuls((), bandwidth)
    }
}
impl<T> RectPuls<T, Vec<T>, usize> for Range<T>
where
    T: Float + FloatConst,
    Vec<T>: NotRange
{
    fn rectpuls(self, n: usize, bandwidth: T) -> (Vec<T>, Vec<T>)
    {
        let x: Vec<_> = (0..n).map(|i| {
                let p = T::from(i).unwrap()/T::from(n - 1).unwrap();
                self.start + (self.end - self.start)*p
            }).collect();

        x.rectpuls((), bandwidth)
    }
}

impl<T, const N: usize> RectPuls<T, [T; N], ()> for RangeInclusive<T>
where
    T: Float + FloatConst,
    [T; N]: NotRange
{
    fn rectpuls(self, (): (), bandwidth: T) -> ([T; N], [T; N])
    {
        let x: [_; N] = ArrayOps::fill(|i| {
            let p = T::from(i).unwrap()/T::from(N - 1).unwrap();
            *self.start() + (*self.end() - *self.start())*p
        });
        
        x.rectpuls((), bandwidth)
    }
}
impl<T> RectPuls<T, Vec<T>, usize> for RangeInclusive<T>
where
    T: Float + FloatConst,
    Vec<T>: NotRange
{
    fn rectpuls(self, n: usize, bandwidth: T) -> (Vec<T>, Vec<T>)
    {
        let x: Vec<_> = (0..n).map(|i| {
                let p = T::from(i).unwrap()/T::from(n - 1).unwrap();
                *self.start() + (*self.end() - *self.start())*p
            }).collect();

        x.rectpuls((), bandwidth)
    }
}