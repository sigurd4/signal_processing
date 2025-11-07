use core::ops::Range;

use num::{traits::FloatConst, Float};
use option_trait::Maybe;

use crate::quantities::{IntoList, ListOrSingle};

pub trait SigmoidTrain<T, L, N>: IntoList<T, L, N>
where
    T: Float,
    L: ListOrSingle<T>,
    N: Maybe<usize>
{
    fn sigmoid_train<TR>(self, numtaps: N, train: TR) -> (TR::Mapped<L::Mapped<T>>, L::Mapped<T>, L)
    where
        TR: ListOrSingle<(Range<T>, T, T)>,
        TR::Mapped<L::Mapped<T>>: ListOrSingle<L::Mapped<T>>;
}

impl<T, L, R, N> SigmoidTrain<T, L, N> for R
where
    T: Float + FloatConst,
    L: ListOrSingle<T>,
    R: IntoList<T, L, N>,
    N: Maybe<usize>,
    L::Mapped<T>: ListOrSingle<T>
{
    fn sigmoid_train<TR>(self, n: N, train: TR) -> (TR::Mapped<L::Mapped<T>>, L::Mapped<T>, L)
    where
        TR: ListOrSingle<(Range<T>, T, T)>,
        TR::Mapped<L::Mapped<T>>: ListOrSingle<L::Mapped<T>>
    {
        let t = self.into_list(n);

        let one = T::one();

        let s = train.map_into_owned(|(t0t1, rt, ft)| t.map_to_owned(|&t| {
            let t0 = t0t1.start;
            let t1 = t0t1.end;

            let a_up = (t0 - t)/rt;
            let a_dw = (t1 - t)/ft;

            (one + a_up.exp()).recip()*(one - (one + a_dw.exp()).recip())
        }));

        let ss = s.as_view_slice();

        let mut i = 0;
        let y = t.map_to_owned(|_| {
            let y = ss.iter()
                .map(|s| s.as_view_slice()[i])
                .reduce(Float::max)
                .unwrap_or_else(T::zero);
            i += 1;
            y
        });

        (s, y, t)
    }
}

#[cfg(test)]
mod test
{
    use array_math::ArrayOps;

    use crate::{plot, gen::pulse::SigmoidTrain};

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