use num::{traits::FloatConst, Complex, Float};
use option_trait::Maybe;

use crate::{IntoList, ListOrSingle};

pub trait ShanWavF<T, L, N>: IntoList<T, L, N>
where
    T: Float,
    L: ListOrSingle<T>,
    N: Maybe<usize>
{
    fn shanwavf(self, numtaps: N, fb: T, fc: T) -> (L::Mapped<Complex<T>>, L);
}

impl<T, L, R, N> ShanWavF<T, L, N> for R
where
    T: Float + FloatConst,
    R: IntoList<T, L, N>,
    N: Maybe<usize>,
    L: ListOrSingle<T>
{
    fn shanwavf(self, n: N, fb: T, fc: T) -> (L::Mapped<Complex<T>>, L)
    {
        let t = self.into_list(n);

        let psi = t.map_to_owned(|&x| {
            let fbx = fb*x;
            let sinc = if fbx.is_zero() || fbx.is_subnormal()
            {
                T::one()
            }
            else
            {
                (fbx*T::PI()).sin()/(fbx*T::PI())
            };
            (Complex::cis(T::TAU()*fc*x)*sinc)/(fb.sqrt())
        });

        (psi, t)
    }
}

#[cfg(test)]
mod test
{
    use array_math::ArrayOps;

    use crate::{plot, ShanWavF};

    #[test]
    fn test()
    {
        const N: usize = 1024;
        let (psi, t): ([_; N], _) = (-8.0..=8.0).shanwavf((), 1.5, 1.0);

        plot::plot_curves("ψ(t)", "plots/psi_t_shanwavf.png", [&t.zip(psi.map(|psi| psi.re)), &t.zip(psi.map(|psi| psi.im))])
            .unwrap()
    }
}