use core::ops::{Range, RangeInclusive};

use array_math::ArrayOps;
use num::{traits::FloatConst, Float};
use option_trait::Maybe;

use crate::{List, NotRange};

pub trait Meyeraux<T, L, N>
where
    T: Float,
    L: List<T>,
    N: Maybe<usize>
{
    fn meyeraux(self, numtaps: N) -> (L::Mapped<T>, L);
}

impl<T, L> Meyeraux<T, L, ()> for L
where
    T: Float + FloatConst,
    L: List<T> + NotRange
{
    fn meyeraux(self, (): ()) -> (L::Mapped<T>, L)
    {
        let y = self.map_to_owned(|&x| {
            let x4 = x*x*x*x;
            x4*(T::from(35u8).unwrap() + x*(-T::from(84u8).unwrap() + x*(T::from(70u8).unwrap() - x*T::from(20u8).unwrap())))
        });

        (y, self)
    }
}

impl<T, const N: usize> Meyeraux<T, [T; N], ()> for Range<T>
where
    T: Float + FloatConst,
    [T; N]: NotRange
{
    fn meyeraux(self, (): ()) -> ([T; N], [T; N])
    {
        let x: [_; N] = ArrayOps::fill(|i| {
            let p = T::from(i).unwrap()/T::from(N).unwrap();
            self.start + (self.end - self.start)*p
        });
        
        (x.meyeraux(()).0, x)
    }
}
impl<T> Meyeraux<T, Vec<T>, usize> for Range<T>
where
    T: Float + FloatConst,
    Vec<T>: NotRange
{
    fn meyeraux(self, n: usize) -> (Vec<T>, Vec<T>)
    {
        let x: Vec<_> = (0..n).map(|i| {
                let p = T::from(i).unwrap()/T::from(n).unwrap();
                self.start + (self.end - self.start)*p
            }).collect();

        x.meyeraux(())
    }
}

impl<T, const N: usize> Meyeraux<T, [T; N], ()> for RangeInclusive<T>
where
    T: Float + FloatConst,
    [T; N]: NotRange
{
    fn meyeraux(self, (): ()) -> ([T; N], [T; N])
    {
        let x: [_; N] = ArrayOps::fill(|i| {
            let p = T::from(i).unwrap()/T::from(N - 1).unwrap();
            *self.start() + (*self.end() - *self.start())*p
        });
        
        x.meyeraux(())
    }
}
impl<T> Meyeraux<T, Vec<T>, usize> for RangeInclusive<T>
where
    T: Float + FloatConst,
    Vec<T>: NotRange
{
    fn meyeraux(self, n: usize) -> (Vec<T>, Vec<T>)
    {
        let x: Vec<_> = (0..n).map(|i| {
                let p = T::from(i).unwrap()/T::from(n - 1).unwrap();
                *self.start() + (*self.end() - *self.start())*p
            }).collect();

        x.meyeraux(())
    }
}

#[cfg(test)]
mod test
{
    use array_math::ArrayOps;

    use crate::{plot, Meyeraux};

    #[test]
    fn test()
    {
        const N: usize = 1024;
        let (y, t): ([_; N], _) = (0.0..=1.0).meyeraux(());

        plot::plot_curves("ψ(t)", "plots/y_t_meyeraux.png", [&t.zip(y)])
            .unwrap()
    }
}