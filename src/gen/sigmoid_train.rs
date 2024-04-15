use core::ops::{Range, RangeInclusive};

use array_math::ArrayOps;
use num::{traits::FloatConst, Float};
use option_trait::Maybe;

use crate::{List, ListOrSingle, NotRange};

pub trait SigmoidTrain<T, L, N>
where
    T: Float,
    L: List<T>,
    N: Maybe<usize>
{
    fn sigmoid_train<TR>(self, numtaps: N, train: TR) -> (TR::Mapped<L::Mapped<T>>, L::Mapped<T>, L)
    where
        TR: List<(Range<T>, T, T)>,
        TR::Mapped<L::Mapped<T>>: List<L::Mapped<T>>;
}

impl<T, L> SigmoidTrain<T, L, ()> for L
where
    T: Float + FloatConst,
    L: List<T> + NotRange,
    L::Mapped<T>: List<T>
{
    fn sigmoid_train<TR>(self, (): (), train: TR) -> (TR::Mapped<L::Mapped<T>>, L::Mapped<T>, L)
    where
        TR: List<(Range<T>, T, T)>,
        TR::Mapped<L::Mapped<T>>: List<L::Mapped<T>>
    {
        let one = T::one();

        let s = train.map_into_owned(|(t0t1, rt, ft)| self.map_to_owned(|&t| {
            let t0 = t0t1.start;
            let t1 = t0t1.end;

            let a_up = (t0 - t)/rt;
            let a_dw = (t1 - t)/ft;

            (one + a_up.exp()).recip()*(one - (one + a_dw.exp()).recip())
        }));

        let ss = s.as_view_slice();

        let mut i = 0;
        let y = self.map_to_owned(|_| {
            let y = ss.iter()
                .map(|s| s.as_view_slice()[i])
                .reduce(Float::max)
                .unwrap_or_else(T::zero);
            i += 1;
            y
        });

        (s, y, self)
    }
}

impl<T, const N: usize> SigmoidTrain<T, [T; N], ()> for Range<T>
where
    T: Float + FloatConst,
    [T; N]: NotRange
{
    fn sigmoid_train<TR>(self, (): (), train: TR) -> (TR::Mapped<[T; N]>, [T; N], [T; N])
    where
        TR: List<(Range<T>, T, T)>,
        TR::Mapped<[T; N]>: List<[T; N]>
    {
        let x: [_; N] = ArrayOps::fill(|i| {
            let p = T::from(i).unwrap()/T::from(N).unwrap();
            self.start + (self.end - self.start)*p
        });
        
        x.sigmoid_train((), train)
    }
}

impl<T> SigmoidTrain<T, Vec<T>, usize> for Range<T>
where
    T: Float + FloatConst,
    Vec<T>: NotRange
{
    fn sigmoid_train<TR>(self, n: usize, train: TR) -> (TR::Mapped<Vec<T>>, Vec<T>, Vec<T>)
    where
        TR: List<(Range<T>, T, T)>,
        TR::Mapped<Vec<T>>: List<Vec<T>>
    {
        let x: Vec<_> = (0..n).map(|i| {
                let p = T::from(i).unwrap()/T::from(n).unwrap();
                self.start + (self.end - self.start)*p
            }).collect();

        x.sigmoid_train((), train)
    }
}

impl<T, const N: usize> SigmoidTrain<T, [T; N], ()> for RangeInclusive<T>
where
    T: Float + FloatConst,
    [T; N]: NotRange
{
    fn sigmoid_train<TR>(self, (): (), train: TR) -> (TR::Mapped<[T; N]>, [T; N], [T; N])
    where
        TR: List<(Range<T>, T, T)>,
        TR::Mapped<[T; N]>: List<[T; N]>
    {
        let x: [_; N] = ArrayOps::fill(|i| {
            let p = T::from(i).unwrap()/T::from(N - 1).unwrap();
            *self.start() + (*self.end() - *self.start())*p
        });
        
        x.sigmoid_train((), train)
    }
}

impl<T> SigmoidTrain<T, Vec<T>, usize> for RangeInclusive<T>
where
    T: Float + FloatConst,
    Vec<T>: NotRange
{
    fn sigmoid_train<TR>(self, n: usize, train: TR) -> (TR::Mapped<Vec<T>>, Vec<T>, Vec<T>)
    where
        TR: List<(Range<T>, T, T)>,
        TR::Mapped<Vec<T>>: List<Vec<T>>
    {
        let x: Vec<_> = (0..n).map(|i| {
                let p = T::from(i).unwrap()/T::from(n - 1).unwrap();
                *self.start() + (*self.end() - *self.start())*p
            }).collect();

        x.sigmoid_train((), train)
    }
}

#[cfg(test)]
mod test
{
    use array_math::ArrayOps;

    use crate::{plot, SigmoidTrain};

    #[test]
    fn test()
    {
        const N: usize = 1024;
        let (_s, y, t): (_, [_; N], _) = (0.0..=8.0).sigmoid_train((), [
            (0.2..0.8, 0.01, 0.1),
            (1.5..3.0, 0.05, 0.01),
            (5.0..7.0, 0.1, 0.1)
        ]);

        plot::plot_curves("y(t)", "plots/y_t_sigmoid_train.png", [&t.zip(y)])
            .unwrap()
    }
}