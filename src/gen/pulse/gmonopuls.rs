use core::ops::{Range, RangeInclusive};

use array_math::ArrayOps;
use num::{traits::FloatConst, Float};
use option_trait::Maybe;

use crate::{List, NotRange};

pub trait GMonoPuls<T, L, N>
where
    T: Float,
    L: List<T>,
    N: Maybe<usize>
{
    fn gmonopuls(self, numtaps: N, fc: T) -> (L::Mapped<T>, L);
}

impl<T, L> GMonoPuls<T, L, ()> for L
where
    T: Float + FloatConst,
    L: List<T> + NotRange
{
    fn gmonopuls(self, (): (), fc: T) -> (L::Mapped<T>, L)
    {
        let scale = T::E().sqrt()*T::TAU()*fc;
        let y = self.map_to_owned(|&t| {
            if t.is_finite()
            {
                scale*t*(-T::TAU()*T::PI()*t*t*fc*fc).exp()
            }
            else
            {
                T::zero()
            }
        });
        (y, self)
    }
}

impl<T, const N: usize> GMonoPuls<T, [T; N], ()> for Range<T>
where
    T: Float + FloatConst,
    [T; N]: NotRange
{
    fn gmonopuls(self, (): (), fc: T) -> ([T; N], [T; N])
    {
        let x: [_; N] = ArrayOps::fill(|i| {
            let p = T::from(i).unwrap()/T::from(N - 1).unwrap();
            self.start + (self.end - self.start)*p
        });
        
        x.gmonopuls((), fc)
    }
}
impl<T> GMonoPuls<T, Vec<T>, usize> for Range<T>
where
    T: Float + FloatConst,
    Vec<T>: NotRange
{
    fn gmonopuls(self, n: usize, fc: T) -> (Vec<T>, Vec<T>)
    {
        let x: Vec<_> = (0..n).map(|i| {
                let p = T::from(i).unwrap()/T::from(n - 1).unwrap();
                self.start + (self.end - self.start)*p
            }).collect();

        x.gmonopuls((), fc)
    }
}

impl<T, const N: usize> GMonoPuls<T, [T; N], ()> for RangeInclusive<T>
where
    T: Float + FloatConst,
    [T; N]: NotRange
{
    fn gmonopuls(self, (): (), fc: T) -> ([T; N], [T; N])
    {
        let x: [_; N] = ArrayOps::fill(|i| {
            let p = T::from(i).unwrap()/T::from(N - 1).unwrap();
            *self.start() + (*self.end() - *self.start())*p
        });
        
        x.gmonopuls((), fc)
    }
}
impl<T> GMonoPuls<T, Vec<T>, usize> for RangeInclusive<T>
where
    T: Float + FloatConst,
    Vec<T>: NotRange
{
    fn gmonopuls(self, n: usize, fc: T) -> (Vec<T>, Vec<T>)
    {
        let x: Vec<_> = (0..n).map(|i| {
                let p = T::from(i).unwrap()/T::from(n - 1).unwrap();
                *self.start() + (*self.end() - *self.start())*p
            }).collect();

        x.gmonopuls((), fc)
    }
}

#[cfg(test)]
mod test
{
    use array_math::ArrayOps;

    use crate::{plot, GMonoPuls};

    #[test]
    fn test()
    {
        const N: usize = 1024;
        let (y, t): ([_; N], _) = (-0.001..=0.001).gmonopuls((), 1e3);

        plot::plot_curves("y(t)", "plots/y_t_gmonopuls.png", [&t.zip(y)])
            .unwrap()
    }
}