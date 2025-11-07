use num::{traits::FloatConst, Float};
use option_trait::Maybe;

use crate::quantities::{IntoList, ListOrSingle};

pub trait HaarWavF<T, L, N>: IntoList<T, L, N>
where
    T: Float,
    L: ListOrSingle<T>,
    N: Maybe<usize>
{
    fn haarwavf(self, numtaps: N) -> (L::Mapped<T>, L);
}

impl<T, L, R, N> HaarWavF<T, L, N> for R
where
    T: Float + FloatConst,
    L: ListOrSingle<T>,
    R: IntoList<T, L, N>,
    N: Maybe<usize>
{
    fn haarwavf(self, n: N) -> (L::Mapped<T>, L)
    {
        let t = self.into_list(n);

        let zero = T::zero();
        let one = T::one();
        let two = one + one;
        let half = two.recip();

        let psi = t.map_to_owned(|&x| {
            if x >= zero && x < half
            {
                one
            }
            else if x >= half && x < one
            {
                -one
            }
            else
            {
                zero
            }
        });

        (psi, t)
    }
}

#[cfg(test)]
mod test
{
    use array_math::ArrayOps;

    use crate::{plot, gen::wavelet::HaarWavF};

    #[test]
    fn test()
    {
        const N: usize = 1024;
        let (psi, t): (_, [_; N]) = (-0.5..=1.5).haarwavf(());

        plot::plot_curves("ψ(t)", "plots/psi_t_haarwavf.png", [&t.zip(psi)])
            .unwrap()
    }
}