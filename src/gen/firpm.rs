use core::ops::{AddAssign, DivAssign, MulAssign, SubAssign};

use ndarray::Array2;
use num::{complex::ComplexFloat, traits::FloatConst, Complex, Float, NumCast};
use option_trait::MaybeCell;
use rand::distributions::uniform::SampleUniform;
use thiserror::Error;

use crate::{Polynomial, Ss, System, Tf, ToSs, ToZpk, Zpk};

moddef::moddef!(
    mod {
        mmfir
    }
);

#[derive(Debug, Clone, Copy, PartialEq, Error)]
pub enum FirPmError
{
    #[error("Bands must be monotonic starting at zero.")]
    EdgesNotNondecreasing,
    #[error("Band edges should be less than 1/2 the sampling frequency, or if no sampling frequency is specified, between 0 and 1.")]
    EdgesOutOfRange,
    #[error("Band edges are too close together.")]
    BandTooNarrow,
    #[error("Filter order must be at least 1.")]
    ZeroOrder,
    #[error("Weighting function out-of-range.")]
    WeightsOutOfRange,
    #[error("Amplitude function out-of-range.")]
    AmplitudeOutOfRange,
    #[error("Type III/IV DC amplitude response must be 0.")]
    NonZeroDC,
    #[error("Type II/III Nyquist amplitude response must be 0.")]
    NonZeroNyq,
    #[error("Numerical error.")]
    NumericalError,
    #[error("Too many extremal points, likely caused by numerical error.")]
    TooManyPeaks,
    #[error("Too few extremal points, likely caused by numerical error.")]
    TooFewPeaks,
    #[error("Missed target.")]
    MissedTarget,
    #[error("Sampling frequency must be a positive number.")]
    InvalidSamplingFrequency
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum FirPmType
{
    Symmetric,
    Antisymmetric,
    Differentiator
}

#[derive(Debug, Clone, PartialEq)]
pub struct FirPmReport<T>
where
    T: Float
{
    fgrid: Vec<T>,
    des: Vec<T>,
    wt: Vec<T>,
    h: Vec<Complex<T>>,
    error: Vec<T>,
    iextr: Vec<usize>,
    fextr: Vec<T>
}

pub trait FirPm: System + Sized
where
    Self::Domain: Float
{
    fn firpm<const B2: usize, const R: usize, const W: usize, const RES: bool>(
        order: usize,
        bands: [Self::Domain; B2],
        response: [Self::Domain; R],
        weight: [Self::Domain; W],
        filter_type: FirPmType,
        sampling_frequency: Option<Self::Domain>,
        accuracy: Self::Domain,
        persistence: Self::Domain,
        robustness: Self::Domain,
        target: Self::Domain
    ) -> Result<(Self, Self::Domain, MaybeCell<FirPmReport<Self::Domain>, RES>), FirPmError>
    where
        [(); 0 - B2%2]:,
        [(); B2/2 - 1]:,
        [(); B2 - R]:,
        [(); 0 - R % (B2/2)]:,
        [(); B2 - W]:,
        [(); 0 - W % (B2/2)]:,
        [(); RES as usize]:;
}

impl<T> FirPm for Tf<T, Vec<T>, ()>
where
    T: Float + FloatConst + AddAssign + SubAssign + MulAssign + DivAssign + Default + SampleUniform + 'static,
    Complex<T>: MulAssign + AddAssign + MulAssign<T>
{
    fn firpm<'a, const B2: usize, const R: usize, const W: usize, const RES: bool>(
        order: usize,
        bands: [T; B2],
        response: [T; R],
        weight: [T; W],
        filter_type: FirPmType,
        sampling_frequency: Option<T>,
        accuracy: T,
        persistence: T,
        robustness: T,
        target: T
    ) -> Result<(Self, T, MaybeCell<FirPmReport<T>, RES>), FirPmError>
    where
        [(); 0 - B2%2]:,
        [(); B2/2 - 1]:,
        [(); B2 - R]:,
        [(); 0 - R % (B2/2)]:,
        [(); B2 - W]:,
        [(); 0 - W % (B2/2)]:,
        [(); RES as usize]:
    {
        let mut h = Err(FirPmError::NumericalError);
        let mut n = 1;
    
        for _ in 0..64
        {
            let h_ = mmfir::mmfir::<T, B2, R, W, RES>(
                order + 1,
                bands,
                mmfir::Response::Bands { response, weight },
                filter_type,
                sampling_frequency,
                accuracy,
                persistence,
                robustness,
                target
            );
    
            match (&mut h, h_)
            {
                (Err(_), h_) => h = h_,
                (Ok(_), Err(_)) => (),
                (Ok((h, m, r)), Ok((h_, m_, r_))) => {
                    for e in h.b.iter_mut()
                        .zip(h_.b.into_inner())
                    {
                        *e.0 += e.1
                    }
                    *m += m_;
                    if let Some(r) = r.get_mut() && let Some(r_) = r_.into_option()
                    {
                        for e in r.des.iter_mut()
                            .zip(r_.des)
                        {
                            *e.0 += e.1
                        }
                        for e in r.error.iter_mut()
                            .zip(r_.error)
                        {
                            *e.0 += e.1
                        }
                        for e in r.fgrid.iter_mut()
                            .zip(r_.fgrid)
                        {
                            *e.0 += e.1
                        }
                        for e in r.h.iter_mut()
                            .zip(r_.h)
                        {
                            *e.0 += e.1
                        }
                    }
                    n += 1;
                }
            }
        }
    
        if let Ok((h, m, r)) = &mut h
        {
            let s = Float::recip(<T as NumCast>::from(n).unwrap());
            for e in h.b.iter_mut()
            {
                *e *= s;
            }
            *m *= s;
            if let Some(r) = r.get_mut()
            {
                for e in r.des.iter_mut()
                {
                    *e *= s;
                }
                for e in r.error.iter_mut()
                {
                    *e *= s;
                }
                for e in r.fgrid.iter_mut()
                {
                    *e *= s;
                }
                for e in r.h.iter_mut()
                {
                    *e *= s;
                }
            }
        }
    
        h.map(|(h, m, r)| (Tf {
            b: Polynomial::new(h.b.into_inner().into_iter().map(Into::into).collect()),
            a: Polynomial::new(())
        }, m, r))
    }
}

impl<T> FirPm for Zpk<Complex<T>, Vec<Complex<T>>, Vec<Complex<T>>, T>
where
    T: Float + FloatConst,
    Complex<T>: ComplexFloat<Real = T>,
    Tf<T, Vec<T>, ()>: FirPm + ToZpk<Complex<T>, Vec<Complex<T>>, Vec<Complex<T>>, T, (), ()> + System<Domain = T>
{
    fn firpm<'a, const B2: usize, const R: usize, const W: usize, const RES: bool>(
        order: usize,
        bands: [T; B2],
        response: [T; R],
        weight: [T; W],
        filter_type: FirPmType,
        sampling_frequency: Option<T>,
        accuracy: T,
        persistence: T,
        robustness: T,
        target: T
    ) -> Result<(Self, T, MaybeCell<FirPmReport<T>, RES>), FirPmError>
    where
        [(); 0 - B2%2]:,
        [(); B2/2 - 1]:,
        [(); B2 - R]:,
        [(); 0 - R % (B2/2)]:,
        [(); B2 - W]:,
        [(); 0 - W % (B2/2)]:,
        [(); RES as usize]:
    {
        let (h, m, r) = Tf::<T, Vec<T>, ()>::firpm(
            order,
            bands,
            response,
            weight,
            filter_type,
            sampling_frequency,
            accuracy,
            persistence,
            robustness,
            target
        )?;

        Ok((h.to_zpk((), ()), m, r))
    }
}

impl<T> FirPm for Ss<T, Array2<T>, Array2<T>, Array2<T>, Array2<T>>
where
    T: Float + FloatConst,
    Tf<T, Vec<T>, ()>: FirPm + ToSs<T, Array2<T>, Array2<T>, Array2<T>, Array2<T>> + System<Domain = T>
{
    fn firpm<'a, const B2: usize, const R: usize, const W: usize, const RES: bool>(
        order: usize,
        bands: [T; B2],
        response: [T; R],
        weight: [T; W],
        filter_type: FirPmType,
        sampling_frequency: Option<T>,
        accuracy: T,
        persistence: T,
        robustness: T,
        target: T
    ) -> Result<(Self, T, MaybeCell<FirPmReport<T>, RES>), FirPmError>
    where
        [(); 0 - B2%2]:,
        [(); B2/2 - 1]:,
        [(); B2 - R]:,
        [(); 0 - R % (B2/2)]:,
        [(); B2 - W]:,
        [(); 0 - W % (B2/2)]:,
        [(); RES as usize]:
    {
        let (h, m, r) = Tf::<T, Vec<T>, ()>::firpm(
            order,
            bands,
            response,
            weight,
            filter_type,
            sampling_frequency,
            accuracy,
            persistence,
            robustness,
            target
        )?;

        Ok((h.to_ss().unwrap(), m, r))
    }
}

#[cfg(test)]
mod test
{
    use array_math::ArrayOps;

    use crate::{plot, FirPm, FirPmType, RealFreqZ, Tf};

    #[test]
    fn test()
    {
        let fs: f64 = 8000.0;
        let (n, f, a, w) = crate::firpmord([1900.0, 2000.0], [1.0, 0.0], [0.0001, 0.0001], Some(fs))
            .unwrap();
        println!("{}", n);
        let (h, _, _) = Tf::firpm::<_, _, _, false>(
            n,
            f,
            a,
            w,
            FirPmType::Symmetric,
            None,
            3.0,
            3.0,
            3.0,
            3.0
        ).unwrap();
    
        const N: usize = 1024;
        let (h_f, w): ([_; N], _) = h.real_freqz(());

        plot::plot_curves("H(e^jw)", "plots/h_z_firpm.png", [&w.zip(h_f.map(|h| h.norm())), &w.zip(h_f.map(|h| h.arg()))]).unwrap();
    }
}