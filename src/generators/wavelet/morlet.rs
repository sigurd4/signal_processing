use num::{traits::FloatConst, Float};
use option_trait::Maybe;

use crate::quantities::{IntoList, ListOrSingle};

pub trait Morlet<T, L, N>: IntoList<T, L, N>
where
    T: Float,
    L: ListOrSingle<T>,
    N: Maybe<usize>
{
    fn morlet(self, numtaps: N) -> (L::Mapped<T>, L);
}

impl<T, L, R, N> Morlet<T, L, N> for R
where
    T: Float + FloatConst,
    L: ListOrSingle<T>,
    R: IntoList<T, L, N>,
    N: Maybe<usize>
{
    fn morlet(self, n: N) -> (L::Mapped<T>, L)
    {
        let t = self.into_list(n);

        let one = T::one();
        let two = one + one;
        let five = T::from(5u8).unwrap();

        let psi = t.map_to_owned(|&x| {
            (five*x).cos()*(-x*x/two).exp()
        });

        (psi, t)
    }
}

#[cfg(test)]
mod test
{
    use array_math::ArrayOps;

    use crate::{plot, gen::wavelet::Morlet};

    #[test]
    fn test()
    {
        const N: usize = 1024;
        let (psi, t): (_, [_; N]) = (-8.0..=8.0).morlet(());

        plot::plot_curves("ψ(t)", "plots/psi_t_morlet.png", [&t.zip(psi)])
            .unwrap()
    }
}