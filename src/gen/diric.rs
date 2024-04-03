use core::ops::Range;

use array_math::ArrayOps;
use num::{traits::FloatConst, Float};
use option_trait::Maybe;

use crate::{List, NotRange};

pub trait Diric<'a, T, L, N>
where
    T: Float,
    L: List<T>,
    N: Maybe<usize>
{
    fn diric(&'a self, numtaps: N, order: usize) -> (L::Mapped<T>, L);
}

impl<'a, T, L> Diric<'a, T, L::View<'a>, ()> for L
where
    T: Float + FloatConst,
    L: List<T> + NotRange,
    L::View<'a>: List<T, Mapped<T> = L::Mapped<T>>
{
    fn diric(&'a self, (): (), order: usize) -> (L::Mapped<T>, L::View<'a>)
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

        (y, self.as_view())
    }
}

impl<'a, T, const N: usize> Diric<'a, T, [T; N], ()> for Range<T>
where
    T: Float + FloatConst,
    [T; N]: NotRange
{
    fn diric(&'a self, (): (), order: usize) -> ([T; N], [T; N])
    {
        let x: [_; N] = ArrayOps::fill(|i| {
            let p = T::from(i).unwrap()/T::from(N - 1).unwrap();
            self.start + (self.end - self.start)*p
        });
        
        (x.diric((), order).0, x)
    }
}

impl<'a, T> Diric<'a, T, Vec<T>, usize> for Range<T>
where
    T: Float + FloatConst,
    Vec<T>: NotRange
{
    fn diric(&'a self, n: usize, order: usize) -> (Vec<T>, Vec<T>)
    {
        let x: Vec<_> = (0..n).map(|i| {
                let p = T::from(i).unwrap()/T::from(n - 1).unwrap();
                self.start + (self.end - self.start)*p
            }).collect();

        (x.diric((), order).0, x)
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
        let (y, t): ([_; N], _) = (-8.0..8.0).diric((), 6);

        plot::plot_curves("y(t)", "plots/y_t_diric.png", [&t.zip(y)])
            .unwrap()
    }
}