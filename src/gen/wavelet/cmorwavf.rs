use num::{traits::FloatConst, Complex, Float};
use option_trait::Maybe;

use crate::quantities::{IntoList, ListOrSingle};

pub trait CMorWavF<T, L, N>: IntoList<T, L, N>
where
    T: Float,
    L: ListOrSingle<T>,
    N: Maybe<usize>
{
    fn cmorwavf(self, numtaps: N, fb: T, fc: T) -> (L::Mapped<Complex<T>>, L);
}

impl<T, L, R, N> CMorWavF<T, L, N> for R
where
    T: Float + FloatConst,
    R: IntoList<T, L, N>,
    N: Maybe<usize>,
    L: ListOrSingle<T>
{
    fn cmorwavf(self, n: N, fb: T, fc: T) -> (L::Mapped<Complex<T>>, L)
    {
        let t = self.into_list(n);

        let psi = t.map_to_owned(|&x| {
            Complex::from(T::PI()*fb).inv().sqrt()*Complex::cis(T::TAU()*fc*x)*(-x*x/fb).exp()
        });

        (psi, t)
    }
}

#[cfg(test)]
mod test
{
    use array_math::ArrayOps;

    use crate::{plot, gen::wavelet::CMorWavF};

    #[test]
    fn test()
    {
        const N: usize = 1024;
        let (psi, t): ([_; N], _) = (-8.0..=8.0).cmorwavf((), 1.5, 1.0);

        plot::plot_curves("ψ(t)", "plots/psi_t_cmorwavf.png", [&t.zip(psi.map(|psi| psi.re)), &t.zip(psi.map(|psi| psi.im))])
            .unwrap()
    }
}