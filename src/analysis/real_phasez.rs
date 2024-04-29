use core::{iter::Sum, ops::{AddAssign, SubAssign}};

use num::{traits::FloatConst, Complex, Float, NumCast};
use option_trait::Maybe;

use crate::{quantities::{ContainerOrSingle, List, ListOrSingle, Lists, MaybeLists, OwnedList, OwnedListOrSingle}, analysis::{PhaseUnwrapReference, RealFreqZ}, System};

pub trait RealPhaseZ<'a, H, W, N>: System<Domain: Float + FloatConst>
where
    H: Lists<Self::Domain>,
    W: List<Self::Domain>,
    N: Maybe<usize>,
{
    fn real_phasez(&'a self, n: N, reference: PhaseUnwrapReference) -> (H, W);
}

impl<'a, S, T, H, W, N> RealPhaseZ<'a, H, W, N> for S
where
    T: Float + FloatConst + Sum + AddAssign + SubAssign,
    S: RealFreqZ<'a, H::Mapped<Complex<T>>, W, N> + System<Domain = T>,
    H: Lists<T>,
    H::Mapped<Complex<T>>: Lists<Complex<T>, RowOwned: OwnedList<Complex<T>, Mapped<T>: OwnedList<T>>, RowsMapped<<<H::Mapped<Complex<T>> as MaybeLists<Complex<T>>>::RowOwned as ContainerOrSingle<Complex<T>>>::Mapped<T>>: Into<H>>,
    W: List<T>,
    N: Maybe<usize>,
{
    fn real_phasez(&'a self, n: N, reference: PhaseUnwrapReference) -> (H, W)
    {
        let (h, w) = self.real_freqz(n);

        let theta = h.map_rows_into_owned(|mut z| {
            let mut prev_theta = T::zero();

            let l = z.as_view_slice().len();

            if reference == PhaseUnwrapReference::Nyq
            {
                z.as_mut_slice()
                    .reverse()
            }

            let mut theta = z.map_into_owned(|z| {
                let mut theta = (z.arg() - prev_theta) % T::TAU() + prev_theta;
                while theta - prev_theta > T::PI()
                {
                    theta -= T::TAU()
                }
                while theta - prev_theta < -T::PI()
                {
                    theta += T::TAU()
                }
                prev_theta = theta;
                theta
            });

            match reference
            {
                PhaseUnwrapReference::Zero => (),
                PhaseUnwrapReference::Nyq => {
                    theta.as_mut_slice()
                        .reverse()
                },
                PhaseUnwrapReference::Mean => {
                    let mean = theta.as_view_slice()
                        .iter()
                        .map(|&theta| theta)
                        .sum::<T>()/NumCast::from(l).unwrap();

                    let o = (mean/T::TAU()).round()*T::TAU();
                    if !o.is_zero()
                    {
                        for theta in theta.as_mut_slice()
                            .iter_mut()
                        {
                            *theta -= o
                        }
                    }
                }
            }

            theta
        }).into();

        (theta, w)
    }
}