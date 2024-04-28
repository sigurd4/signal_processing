use core::{iter::Sum, ops::{AddAssign, SubAssign}};

use num::{complex::ComplexFloat, traits::FloatConst, Complex, Float, NumCast, Zero};
use option_trait::Maybe;

use crate::{ContainerOrSingle, FreqZ, List, ListOrSingle, Lists, MaybeLists, OwnedList, System, OwnedListOrSingle};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum PhaseUnwrapReference
{
    Zero,
    Nyq,
    Mean
}

pub trait PhaseZ<'a, H, W, N>: System
where
    H: Lists<<Self::Domain as ComplexFloat>::Real>,
    W: List<<Self::Domain as ComplexFloat>::Real>,
    N: Maybe<usize>,
{
    fn phasez(&'a self, n: N, reference: PhaseUnwrapReference, shift: bool) -> (H, W);
}

impl<'a, S, T, H, W, N> PhaseZ<'a, H, W, N> for S
where
    T: ComplexFloat<Real: Sum + AddAssign + SubAssign>,
    S: FreqZ<'a, H::Mapped<Complex<T::Real>>, W, N> + System<Domain = T>,
    H: Lists<T::Real>,
    H::Mapped<Complex<T::Real>>: Lists<Complex<T::Real>, RowOwned: OwnedList<Complex<T::Real>, Mapped<T::Real>: OwnedList<T::Real>>, RowsMapped<<<H::Mapped<Complex<T::Real>> as MaybeLists<Complex<T::Real>>>::RowOwned as ContainerOrSingle<Complex<T::Real>>>::Mapped<T::Real>>: Into<H>>,
    W: List<T::Real>,
    N: Maybe<usize>,
{
    fn phasez(&'a self, n: N, reference: PhaseUnwrapReference, shift: bool) -> (H, W)
    {
        let (h, w) = self.freqz(n, shift);

        let theta = h.map_rows_into_owned(|z| {
            let mut prev_theta = T::Real::zero();

            let l = z.as_view_slice().len();

            let mut theta = z.map_into_owned(|z| {
                let mut theta = (z.arg() - prev_theta) % T::Real::TAU() + prev_theta;
                while theta - prev_theta > T::Real::PI()
                {
                    theta -= T::Real::TAU()
                }
                while theta - prev_theta < -T::Real::PI()
                {
                    theta += T::Real::TAU()
                }
                prev_theta = theta;
                theta
            });

            match reference
            {
                PhaseUnwrapReference::Nyq | PhaseUnwrapReference::Zero => if shift == (reference == PhaseUnwrapReference::Zero)
                {
                    let o = Float::round(theta.as_view_slice()[l/2]/T::Real::TAU())*T::Real::TAU();
                    if !o.is_zero()
                    {
                        for theta in theta.as_mut_slice()
                            .iter_mut()
                        {
                            *theta -= o
                        }
                    }
                },
                PhaseUnwrapReference::Mean => {
                    let mean = theta.as_view_slice()
                        .iter()
                        .map(|&theta| theta)
                        .sum::<T::Real>()/NumCast::from(l).unwrap();

                    let o = (mean/T::Real::TAU()).round()*T::Real::TAU();
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

#[cfg(test)]
mod test
{
    use array_math::ArrayOps;

    use crate::{plot, Butter, FilterGenPlane, FreqZ, PhaseUnwrapReference, PhaseZ, Tf};

    #[test]
    fn test()
    {
        let fs = 1000.0;

        let (n, wp, _ws, t) = crate::buttord(
            [40.0],
            [150.0],
            3.0,
            60.0,
            FilterGenPlane::Z { sampling_frequency: Some(fs) }
        ).unwrap();

        let h = Tf::butter(n, wp, t, FilterGenPlane::Z { sampling_frequency: None })
            .unwrap();

        const N: usize = 1024;
        let (h_f, w): ([_; N], _) = h.freqz((), true);
        let (hp_f, _) = h.phasez((), PhaseUnwrapReference::Mean, true);

        plot::plot_curves("H(e^jw)", "plots/h_z_phasez.png", [&w.zip(h_f.map(|h| h.norm())), &w.zip(hp_f)]).unwrap();
    }
}