use core::ops::{Range, RangeInclusive};

use array_math::ArrayOps;
use num::{traits::FloatConst, Float};
use option_trait::Maybe;

use crate::{List, NotRange};


pub trait GausPuls<'a, T, L, N>
where
    T: Float,
    L: List<T>,
    N: Maybe<usize>
{
    fn gauspuls(&'a self, numtaps: N, fc: T, bw: T) -> (L::Mapped<T>, L::Mapped<T>, L::Mapped<T>, L);
}

impl<'a, T, L> GausPuls<'a, T, L::View<'a>, ()> for L
where
    T: Float + FloatConst,
    L: List<T> + NotRange,
    L::View<'a>: List<T, Mapped<T> = L::Mapped<T>>
{
    fn gauspuls(&'a self, (): (), fc: T, bw: T) -> (L::Mapped<T>, L::Mapped<T>, L::Mapped<T>, L::View<'a>)
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
        (yi, yq, ye, self.as_view())
    }
}

impl<'a, T, const N: usize> GausPuls<'a, T, [T; N], ()> for Range<T>
where
    T: Float + FloatConst,
    [T; N]: NotRange
{
    fn gauspuls(&self, (): (), fc: T, bw: T) -> ([T; N], [T; N], [T; N], [T; N])
    {
        let x: [_; N] = ArrayOps::fill(|i| {
            let p = T::from(i).unwrap()/T::from(N - 1).unwrap();
            self.start + (self.end - self.start)*p
        });
        
        let (yi, yq, ye, _) = x.gauspuls((), fc, bw);
        (yi, yq, ye, x)
    }
}
impl<'a, T> GausPuls<'a, T, Vec<T>, usize> for Range<T>
where
    T: Float + FloatConst,
    Vec<T>: NotRange
{
    fn gauspuls(&self, n: usize, fc: T, bw: T) -> (Vec<T>, Vec<T>, Vec<T>, Vec<T>)
    {
        let x: Vec<_> = (0..n).map(|i| {
                let p = T::from(i).unwrap()/T::from(n - 1).unwrap();
                self.start + (self.end - self.start)*p
            }).collect();

        let (yi, yq, ye, _) = x.gauspuls((), fc, bw);
        (yi, yq, ye, x)
    }
}

impl<'a, T, const N: usize> GausPuls<'a, T, [T; N], ()> for RangeInclusive<T>
where
    T: Float + FloatConst,
    [T; N]: NotRange
{
    fn gauspuls(&self, (): (), fc: T, bw: T) -> ([T; N], [T; N], [T; N], [T; N])
    {
        let x: [_; N] = ArrayOps::fill(|i| {
            let p = T::from(i).unwrap()/T::from(N - 1).unwrap();
            *self.start() + (*self.end() - *self.start())*p
        });
        
        let (yi, yq, ye, _) = x.gauspuls((), fc, bw);
        (yi, yq, ye, x)
    }
}
impl<'a, T> GausPuls<'a, T, Vec<T>, usize> for RangeInclusive<T>
where
    T: Float + FloatConst,
    Vec<T>: NotRange
{
    fn gauspuls(&self, n: usize, fc: T, bw: T) -> (Vec<T>, Vec<T>, Vec<T>, Vec<T>)
    {
        let x: Vec<_> = (0..n).map(|i| {
                let p = T::from(i).unwrap()/T::from(n - 1).unwrap();
                *self.start() + (*self.end() - *self.start())*p
            }).collect();

        let (yi, yq, ye, _) = x.gauspuls((), fc, bw);
        (yi, yq, ye, x)
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