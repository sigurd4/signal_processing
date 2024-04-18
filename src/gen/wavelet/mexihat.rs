use num::{traits::FloatConst, Float};
use option_trait::Maybe;

use crate::{IntoList, List};

pub trait Mexihat<T, L, N>: IntoList<T, L, N>
where
    T: Float,
    L: List<T>,
    N: Maybe<usize>
{
    fn mexihat(self, numtaps: N) -> (L::Mapped<T>, L);
}

impl<T, L, R, N> Mexihat<T, L, N> for R
where
    T: Float + FloatConst,
    L: List<T>,
    R: IntoList<T, L, N>,
    N: Maybe<usize>
{
    fn mexihat(self, n: N) -> (L::Mapped<T>, L)
    {
        let t = self.into_list(n);

        let one = T::one();
        let two = one + one;

        let a = two/(T::from(3u8).unwrap()*T::PI().sqrt()).sqrt();

        let psi = t.map_to_owned(|&x| {
            (one - x*x)*a*(-x*x/two).exp()
        });

        (psi, t)
    }
}

#[cfg(test)]
mod test
{
    use array_math::ArrayOps;

    use crate::{plot, Mexihat};

    #[test]
    fn test()
    {
        const N: usize = 1024;
        let (psi, t): ([_; N], _) = (-8.0..=8.0).mexihat(());

        plot::plot_curves("ψ(t)", "plots/psi_t_mexihat.png", [&t.zip(psi)])
            .unwrap()
    }
}