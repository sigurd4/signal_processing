use core::{iter::Sum, ops::Mul};

use num::{complex::ComplexFloat, traits::FloatConst, Float, NumCast, One, Zero};
use option_trait::Maybe;

use crate::quantities::{IntoList, List, ListOrSingle};

pub trait SampledToContinuous<T, L, R, N>: List<T>
where
    T: ComplexFloat,
    L: ListOrSingle<T::Real>,
    R: IntoList<T::Real, L, N>,
    N: Maybe<usize>
{
    fn sampled_to_continuous(self, t: R, numtaps: N, sampling_rate: T::Real) -> (L::Mapped<T>, L);
}

impl<T, LL, L, R, N> SampledToContinuous<T, L, R, N> for LL
where
    LL: List<T>,
    T: ComplexFloat + Mul<T::Real, Output = T> + Sum,
    L: ListOrSingle<T::Real>,
    R: IntoList<T::Real, L, N>,
    N: Maybe<usize>
{
    fn sampled_to_continuous(self, t: R, numtaps: N, sampling_rate: T::Real) -> (L::Mapped<T>, L)
    {
        let t = t.into_list(numtaps);

        let xt = t.map_to_owned(|&t| {
            self.to_vec()
                .into_iter()
                .enumerate()
                .map(|(i, x)| {
                    let n = t*sampling_rate - <T::Real as NumCast>::from(i).unwrap();
                    let sinc = if n.is_zero() || n.is_subnormal()
                    {
                        T::Real::one()
                    }
                    else
                    {
                        Float::sin(T::Real::PI()*n)/(T::Real::PI()*n)
                    };

                    x*sinc
                }).sum::<T>()
        });

        (xt, t)
    }
}

#[cfg(test)]
mod test
{
    use array_math::ArrayOps;

    use crate::{plot, transforms::SampledToContinuous};

    #[test]
    fn test()
    {
        let xn = [1.0, 2.0, 3.0, 4.0, 5.0];
        let n = core::array::from_fn(|i| i as f64);

        const N: usize = 1024;
        let (xt, t): (_, [_; N]) = xn.sampled_to_continuous(0.0..5.0, (), 1.0);

        plot::plot_curves("x(t)", "plots/x_t_sampled_to_continuous.png", [&t.zip(xt), &n.zip(xn)])
            .unwrap();
    }
}