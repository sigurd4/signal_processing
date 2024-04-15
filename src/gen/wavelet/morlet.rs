use core::ops::{Range, RangeInclusive};

use array_math::ArrayOps;
use num::{traits::FloatConst, Float};
use option_trait::Maybe;

use crate::{List, NotRange};

pub trait Morlet<T, L, N>
where
    T: Float,
    L: List<T>,
    N: Maybe<usize>
{
    fn morlet(self, numtaps: N) -> (L::Mapped<T>, L);
}

impl<T, L> Morlet<T, L, ()> for L
where
    T: Float + FloatConst,
    L: List<T> + NotRange
{
    fn morlet(self, (): ()) -> (L::Mapped<T>, L)
    {
        let one = T::one();
        let two = one + one;
        let five = T::from(5u8).unwrap();

        let psi = self.map_to_owned(|&x| {
            (five*x).cos()*(-x*x/two).exp()
        });

        (psi, self)
    }
}

impl<T, const N: usize> Morlet<T, [T; N], ()> for Range<T>
where
    T: Float + FloatConst,
    [T; N]: NotRange
{
    fn morlet(self, (): ()) -> ([T; N], [T; N])
    {
        let x: [_; N] = ArrayOps::fill(|i| {
            let p = T::from(i).unwrap()/T::from(N).unwrap();
            self.start + (self.end - self.start)*p
        });
        
        x.morlet(())
    }
}
impl<T> Morlet<T, Vec<T>, usize> for Range<T>
where
    T: Float + FloatConst,
    Vec<T>: NotRange
{
    fn morlet(self, n: usize) -> (Vec<T>, Vec<T>)
    {
        let x: Vec<_> = (0..n).map(|i| {
                let p = T::from(i).unwrap()/T::from(n).unwrap();
                self.start + (self.end - self.start)*p
            }).collect();

        x.morlet(())
    }
}

impl<T, const N: usize> Morlet<T, [T; N], ()> for RangeInclusive<T>
where
    T: Float + FloatConst,
    [T; N]: NotRange
{
    fn morlet(self, (): ()) -> ([T; N], [T; N])
    {
        let x: [_; N] = ArrayOps::fill(|i| {
            let p = T::from(i).unwrap()/T::from(N - 1).unwrap();
            *self.start() + (*self.end() - *self.start())*p
        });
        
        x.morlet(())
    }
}
impl<T> Morlet<T, Vec<T>, usize> for RangeInclusive<T>
where
    T: Float + FloatConst,
    Vec<T>: NotRange
{
    fn morlet(self, n: usize) -> (Vec<T>, Vec<T>)
    {
        let x: Vec<_> = (0..n).map(|i| {
                let p = T::from(i).unwrap()/T::from(n - 1).unwrap();
                *self.start() + (*self.end() - *self.start())*p
            }).collect();

        x.morlet(())
    }
}

#[cfg(test)]
mod test
{
    use array_math::ArrayOps;

    use crate::{plot, Morlet};

    #[test]
    fn test()
    {
        const N: usize = 1024;
        let (psi, t): ([_; N], _) = (-8.0..=8.0).morlet(());

        plot::plot_curves("ψ(t)", "plots/psi_t_morlet.png", [&t.zip(psi)])
            .unwrap()
    }
}