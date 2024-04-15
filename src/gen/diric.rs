use core::ops::{Range, RangeInclusive};

use array_math::ArrayOps;
use num::{traits::FloatConst, Float};
use option_trait::Maybe;

use crate::{List, NotRange};

pub trait Diric<T, L, N>
where
    T: Float,
    L: List<T>,
    N: Maybe<usize>
{
    fn diric(self, numtaps: N, order: usize) -> (L::Mapped<T>, L);
}

impl<T, L> Diric<T, L, ()> for L
where
    T: Float + FloatConst,
    L: List<T> + NotRange
{
    fn diric(self, (): (), order: usize) -> (L::Mapped<T>, L)
    {
        let n = T::from(order).unwrap();
        let one = T::one();
        let two = one + one;

        let y = self.map_to_owned(|&x| {
            if !(x % T::TAU()).is_zero()
            {
                (n*x/two).sin()/(n*(x/two).sin())
            }
            else if (((n - one)*x) % T::TAU()).abs() < T::FRAC_PI_2()
            {
                one
            }
            else
            {
                -one
            }
        });

        (y, self)
    }
}

impl<T, const N: usize> Diric<T, [T; N], ()> for Range<T>
where
    T: Float + FloatConst,
    [T; N]: NotRange
{
    fn diric(self, (): (), order: usize) -> ([T; N], [T; N])
    {
        let x: [_; N] = ArrayOps::fill(|i| {
            let p = T::from(i).unwrap()/T::from(N).unwrap();
            self.start + (self.end - self.start)*p
        });
        
        x.diric((), order)
    }
}

impl<T> Diric<T, Vec<T>, usize> for Range<T>
where
    T: Float + FloatConst,
    Vec<T>: NotRange
{
    fn diric(self, n: usize, order: usize) -> (Vec<T>, Vec<T>)
    {
        let x: Vec<_> = (0..n).map(|i| {
                let p = T::from(i).unwrap()/T::from(n).unwrap();
                self.start + (self.end - self.start)*p
            }).collect();

        x.diric((), order)
    }
}

impl<T, const N: usize> Diric<T, [T; N], ()> for RangeInclusive<T>
where
    T: Float + FloatConst,
    [T; N]: NotRange
{
    fn diric(self, (): (), order: usize) -> ([T; N], [T; N])
    {
        let x: [_; N] = ArrayOps::fill(|i| {
            let p = T::from(i).unwrap()/T::from(N - 1).unwrap();
            *self.start() + (*self.end() - *self.start())*p
        });
        
        x.diric((), order)
    }
}

impl<T> Diric<T, Vec<T>, usize> for RangeInclusive<T>
where
    T: Float + FloatConst,
    Vec<T>: NotRange
{
    fn diric(self, n: usize, order: usize) -> (Vec<T>, Vec<T>)
    {
        let x: Vec<_> = (0..n).map(|i| {
                let p = T::from(i).unwrap()/T::from(n - 1).unwrap();
                *self.start() + (*self.end() - *self.start())*p
            }).collect();

        x.diric((), order)
    }
}

#[cfg(test)]
mod test
{
    use array_math::ArrayOps;

    use crate::{plot, Diric};

    #[test]
    fn test()
    {
        const N: usize = 1024;
        let (y, t): ([_; N], _) = (-8.0..=8.0).diric((), 6);

        plot::plot_curves("y(t)", "plots/y_t_diric.png", [&t.zip(y)])
            .unwrap()
    }
}