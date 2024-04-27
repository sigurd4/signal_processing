use num::{traits::FloatConst, Float};
use option_trait::Maybe;

use crate::{IntoList, ListOrSingle};

pub trait GMonoPuls<T, L, N>: IntoList<T, L, N>
where
    T: Float,
    L: ListOrSingle<T>,
    N: Maybe<usize>
{
    fn gmonopuls(self, numtaps: N, fc: T) -> (L::Mapped<T>, L);
}

impl<T, L, R, N> GMonoPuls<T, L, N> for R
where
    T: Float + FloatConst,
    L: ListOrSingle<T>,
    R: IntoList<T, L, N>,
    N: Maybe<usize>
{
    fn gmonopuls(self, n: N, fc: T) -> (L::Mapped<T>, L)
    {
        let t = self.into_list(n);

        let scale = T::E().sqrt()*T::TAU()*fc;
        let y = t.map_to_owned(|&t| {
            if t.is_finite()
            {
                scale*t*(-T::TAU()*T::PI()*t*t*fc*fc).exp()
            }
            else
            {
                T::zero()
            }
        });
        (y, t)
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