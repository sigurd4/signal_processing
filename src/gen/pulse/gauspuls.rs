use core::ops::{Range, RangeInclusive};

use array_math::ArrayOps;
use num::{traits::FloatConst, Float};
use option_trait::Maybe;

use crate::{List, NotRange};


pub trait GausPuls<T, L, N>
where
    T: Float,
    L: List<T>,
    N: Maybe<usize>
{
    fn gauspuls(self, numtaps: N, fc: T, bw: T) -> (L::Mapped<T>, L::Mapped<T>, L::Mapped<T>, L);
}

impl<T, L> GausPuls<T, L, ()> for L
where
    T: Float + FloatConst,
    L: List<T> + NotRange
{
    fn gauspuls(self, (): (), fc: T, bw: T) -> (L::Mapped<T>, L::Mapped<T>, L::Mapped<T>, L)
    {
        let fv = -bw*bw*fc*fc/(T::from(8u8).unwrap()*(T::from(10u8).unwrap().powf(-T::from(3u8).unwrap()/T::from(10u8).unwrap())).ln());
        let tv = (T::TAU()*T::TAU()*fv).recip();
        let yi = self.map_to_owned(|&t| {
            (-t*t/(T::from(2u8).unwrap()*tv)).exp()*(T::TAU()*fc*t).cos()
        });
        let yq = self.map_to_owned(|&t| {
            (-t*t/(T::from(2u8).unwrap()*tv)).exp()*(T::TAU()*fc*t).sin()
        });
        let ye = self.map_to_owned(|&t| {
            (-t*t/(T::from(2u8).unwrap()*tv)).exp()
        });
        (yi, yq, ye, self)
    }
}

impl<T, const N: usize> GausPuls<T, [T; N], ()> for Range<T>
where
    T: Float + FloatConst,
    [T; N]: NotRange
{
    fn gauspuls(self, (): (), fc: T, bw: T) -> ([T; N], [T; N], [T; N], [T; N])
    {
        let x: [_; N] = ArrayOps::fill(|i| {
            let p = T::from(i).unwrap()/T::from(N - 1).unwrap();
            self.start + (self.end - self.start)*p
        });
        
        x.gauspuls((), fc, bw)
    }
}
impl<T> GausPuls<T, Vec<T>, usize> for Range<T>
where
    T: Float + FloatConst,
    Vec<T>: NotRange
{
    fn gauspuls(self, n: usize, fc: T, bw: T) -> (Vec<T>, Vec<T>, Vec<T>, Vec<T>)
    {
        let x: Vec<_> = (0..n).map(|i| {
                let p = T::from(i).unwrap()/T::from(n - 1).unwrap();
                self.start + (self.end - self.start)*p
            }).collect();

        x.gauspuls((), fc, bw)
    }
}

impl<T, const N: usize> GausPuls<T, [T; N], ()> for RangeInclusive<T>
where
    T: Float + FloatConst,
    [T; N]: NotRange
{
    fn gauspuls(self, (): (), fc: T, bw: T) -> ([T; N], [T; N], [T; N], [T; N])
    {
        let x: [_; N] = ArrayOps::fill(|i| {
            let p = T::from(i).unwrap()/T::from(N - 1).unwrap();
            *self.start() + (*self.end() - *self.start())*p
        });
        
        x.gauspuls((), fc, bw)
    }
}
impl<T> GausPuls<T, Vec<T>, usize> for RangeInclusive<T>
where
    T: Float + FloatConst,
    Vec<T>: NotRange
{
    fn gauspuls(self, n: usize, fc: T, bw: T) -> (Vec<T>, Vec<T>, Vec<T>, Vec<T>)
    {
        let x: Vec<_> = (0..n).map(|i| {
                let p = T::from(i).unwrap()/T::from(n - 1).unwrap();
                *self.start() + (*self.end() - *self.start())*p
            }).collect();

        x.gauspuls((), fc, bw)
    }
}

#[cfg(test)]
mod test
{
    use array_math::ArrayOps;

    use crate::{plot, GausPuls};

    #[test]
    fn test()
    {
        const N: usize = 1024;
        let (yi, yq, ye, t): ([_; N], _, _, _) = (-0.003..=0.003).gauspuls((), 1e3, 0.5);

        plot::plot_curves("y(t)", "plots/y_t_gauspuls.png", [&t.zip(yi), &t.zip(yq), &t.zip(ye)])
            .unwrap()
    }
}