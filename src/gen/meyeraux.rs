use num::{traits::FloatConst, Float};
use option_trait::Maybe;

use crate::{IntoList, ListOrSingle};

pub trait Meyeraux<T, L, N>: IntoList<T, L, N>
where
    T: Float,
    L: ListOrSingle<T>,
    N: Maybe<usize>
{
    fn meyeraux(self, numtaps: N) -> (L::Mapped<T>, L);
}

impl<T, L, R, N> Meyeraux<T, L, N> for R
where
    T: Float + FloatConst,
    L: ListOrSingle<T>,
    R: IntoList<T, L, N>,
    N: Maybe<usize>
{
    fn meyeraux(self, n: N) -> (L::Mapped<T>, L)
    {
        let t = self.into_list(n);

        let y = t.map_to_owned(|&x| {
            let x4 = x*x*x*x;
            x4*(T::from(35u8).unwrap() + x*(-T::from(84u8).unwrap() + x*(T::from(70u8).unwrap() - x*T::from(20u8).unwrap())))
        });

        (y, t)
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
        let (y, t): (_, [_; N]) = (0.0..=1.0).meyeraux(());

        plot::plot_curves("y(t)", "plots/y_t_meyeraux.png", [&t.zip(y)])
            .unwrap()
    }
}