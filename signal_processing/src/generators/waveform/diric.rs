use num::{traits::FloatConst, Float};
use option_trait::Maybe;

use crate::quantities::{IntoList, ListOrSingle};

pub trait Diric<T, L, N>: IntoList<T, L, N>
where
    T: Float,
    L: ListOrSingle<T>,
    N: Maybe<usize>
{
    fn diric(self, numtaps: N, order: usize) -> (L::Mapped<T>, L);
}

impl<T, L, R, N> Diric<T, L, N> for R
where
    T: Float + FloatConst,
    L: ListOrSingle<T>,
    R: IntoList<T, L, N>,
    N: Maybe<usize>
{
    fn diric(self, n: N, order: usize) -> (L::Mapped<T>, L)
    {
        let t = self.into_list(n);

        let n = T::from(order).unwrap();
        let one = T::one();
        let two = one + one;

        let y = t.map_to_owned(|&x| {
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

        (y, t)
    }
}

#[cfg(test)]
mod test
{
    use array_math::ArrayOps;

    use crate::{plot, gen::waveform::Diric};

    #[test]
    fn test()
    {
        const N: usize = 1024;
        let (y, t): (_, [_; N]) = (-8.0..=8.0).diric((), 6);

        plot::plot_curves("y(t)", "plots/y_t_diric.png", [&t.zip(y)])
            .unwrap()
    }
}