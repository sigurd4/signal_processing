use num::{traits::FloatConst, Float};
use option_trait::Maybe;

use crate::quantities::{IntoList, ListOrSingle};

pub trait GaussSpline<T, L, N>: IntoList<T, L, N>
where
    T: Float,
    L: ListOrSingle<T>,
    N: Maybe<usize>
{
    fn gauss_spline(self, numtaps: N, order: usize) -> (L::Mapped<T>, L);
}

impl<T, L, R, N> GaussSpline<T, L, N> for R
where
    T: Float + FloatConst,
    L: ListOrSingle<T>,
    R: IntoList<T, L, N>,
    N: Maybe<usize>
{
    fn gauss_spline(self, n: N, order: usize) -> (L::Mapped<T>, L)
    {
        let t = self.into_list(n);

        let one = T::one();
        let two = one + one;

        let sigma = T::from(order + 1).unwrap()/T::from(12u8).unwrap();

        let psi = t.map_to_owned(|&x| {
            (T::TAU()*sigma).sqrt().recip()*(-x*x/two/sigma).exp()
        });

        (psi, t)
    }
}

#[cfg(test)]
mod test
{
    use array_math::ArrayOps;

    use crate::{plot, gen::bspline::GaussSpline};

    #[test]
    fn test()
    {
        const N: usize = 1024;
        let (b, t): (_, [_; N]) = (-1.0..=1.0).gauss_spline((), 3);

        plot::plot_curves("B(t)", "plots/b_t_gauss_spline.png", [&t.zip(b)])
            .unwrap()
    }
}