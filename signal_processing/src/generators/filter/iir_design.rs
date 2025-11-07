use core::ops::{DivAssign, MulAssign};

use num::complex::ComplexFloat;

use crate::System;

use super::{Butter, Cheby1, Cheby2, Ellip, FilterBandError, FilterGenPlane};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum IirFilterType
{
    Butterworth,
    Chebyshev1,
    Chebyshev2,
    Elliptic
}

pub trait IirDesign: System + Sized
{
    fn iir_design<const F: usize>(
        passband_frequencies: [<Self::Set as ComplexFloat>::Real; F],
        stopband_frequencies: [<Self::Set as ComplexFloat>::Real; F],
        passband_ripple: <Self::Set as ComplexFloat>::Real,
        stopband_attenuation: <Self::Set as ComplexFloat>::Real,
        plane: FilterGenPlane<<Self::Set as ComplexFloat>::Real>,
        filter_type: IirFilterType
    ) -> Result<Self, FilterBandError>
    where
        [(); F - 1]:,
        [(); 2 - F]:;
}

impl<S, T> IirDesign for S
where
    T: ComplexFloat<Real: MulAssign + DivAssign>,
    S: System<Set = T> + Butter<usize> + Cheby1<usize> + Cheby2<usize> + Ellip<usize>
{
    fn iir_design<const F: usize>(
        passband_frequencies: [T::Real; F],
        stopband_frequencies: [T::Real; F],
        passband_ripple: T::Real,
        stopband_attenuation: T::Real,
        plane: FilterGenPlane<T::Real>,
        filter_type: IirFilterType
    ) -> Result<Self, FilterBandError>
    where
        [(); F - 1]:,
        [(); 2 - F]:
    {
        let plane_no_fs = match plane
        {
            FilterGenPlane::S => FilterGenPlane::S,
            FilterGenPlane::Z { .. } => FilterGenPlane::Z {sampling_frequency: None},
        };
        Ok(match filter_type
        {
            IirFilterType::Butterworth => {
                let (n, wp, _ws, t) = crate::generators::filter::buttord(
                    passband_frequencies,
                    stopband_frequencies,
                    passband_ripple,
                    stopband_attenuation,
                    plane
                )?;
                S::butter(n, wp, t, plane_no_fs)
                    .unwrap()
            },
            IirFilterType::Chebyshev1 => {
                let (n, wp, _ws, rp, t) = crate::generators::filter::cheb1ord(
                    passband_frequencies,
                    stopband_frequencies,
                    passband_ripple,
                    stopband_attenuation,
                    plane
                )?;
                S::cheby1(n, rp, wp, t, plane_no_fs)
                    .unwrap()
            },
            IirFilterType::Chebyshev2 => {
                let (n, wp, _ws, rs, t) = crate::generators::filter::cheb2ord(
                    passband_frequencies,
                    stopband_frequencies,
                    passband_ripple,
                    stopband_attenuation,
                    plane
                )?;
                S::cheby2(n, rs, wp, t, plane_no_fs)
                    .unwrap()
            },
            IirFilterType::Elliptic => {
                let (n, wp, _ws, rp, rs, t) = crate::generators::filter::ellipord(
                    passband_frequencies,
                    stopband_frequencies,
                    passband_ripple,
                    stopband_attenuation,
                    plane
                )?;
                S::ellip(n, rp, rs, wp, t, plane_no_fs)
                    .unwrap()
            },
        })
    }
}