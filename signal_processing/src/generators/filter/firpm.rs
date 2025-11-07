use core::ops::{AddAssign, DivAssign, MulAssign, SubAssign};

use ndarray::Array2;
use num::{complex::ComplexFloat, traits::FloatConst, Complex, Float, NumCast};
use option_trait::{Maybe, StaticMaybe};
use rand::distributions::uniform::SampleUniform;
use thiserror::Error;

use crate::{systems::{Ss, SsAMatrix, SsBMatrix, SsCMatrix, SsDMatrix, Tf, Zpk}, transforms::system::{ToSs, ToZpk}, System};

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
    Self::Set: Float
{
    fn firpm<FS, RES, const B2: usize, const R: usize, const W: usize>(
        order: usize,
        bands: [Self::Set; B2],
        response: [Self::Set; R],
        weight: [Self::Set; W],
        filter_type: FirPmType,
        sampling_frequency: FS,
        accuracy: Self::Set,
        persistence: Self::Set,
        robustness: Self::Set,
        target: Self::Set
    ) -> Result<(Self, Self::Set, RES), FirPmError>
    where
        FS: Maybe<Self::Set>,
        RES: StaticMaybe<FirPmReport<Self::Set>>,
        [(); 0 - B2%2]:,
        [(); B2/2 - 1]:,
        [(); B2 - R]:,
        [(); 0 - R % (B2/2)]:,
        [(); B2 - W]:,
        [(); 0 - W % (B2/2)]:;
}

impl<T> FirPm for Tf<T, Vec<T>, ()>
where
    T: Float + FloatConst + AddAssign + SubAssign + MulAssign + DivAssign + Default + SampleUniform + 'static,
    Complex<T>: MulAssign + AddAssign + MulAssign<T>
{
    fn firpm<FS, RES, const B2: usize, const R: usize, const W: usize>(
        order: usize,
        bands: [Self::Set; B2],
        response: [Self::Set; R],
        weight: [Self::Set; W],
        filter_type: FirPmType,
        sampling_frequency: FS,
        accuracy: Self::Set,
        persistence: Self::Set,
        robustness: Self::Set,
        target: Self::Set
    ) -> Result<(Self, Self::Set, RES), FirPmError>
    where
        FS: Maybe<Self::Set>,
        RES: StaticMaybe<FirPmReport<Self::Set>>,
        [(); 0 - B2%2]:,
        [(); B2/2 - 1]:,
        [(); B2 - R]:,
        [(); 0 - R % (B2/2)]:,
        [(); B2 - W]:,
        [(); 0 - W % (B2/2)]:
    {
        let mut h = Err(FirPmError::NumericalError);
        let mut n = 1;

        let sampling_frequency = sampling_frequency.into_option();
    
        for _ in 0..128
        {
            let h_ = mmfir::mmfir::<T, RES, B2, R, W>(
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
                    if let Some(r) = r.as_option_mut() && let Some(r_) = r_.into_option()
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
            if let Some(r) = r.as_option_mut()
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
    
        h.map(|(h, m, r)| (Tf::new(
            h.b.into_inner().into_iter().map(Into::into).collect(),
            ()
        ), m, r))
    }
}

impl<T> FirPm for Zpk<Complex<T>, Vec<Complex<T>>, Vec<Complex<T>>, T>
where
    T: Float + FloatConst,
    Complex<T>: ComplexFloat<Real = T>,
    Tf<T, Vec<T>, ()>: FirPm + ToZpk<Complex<T>, Vec<Complex<T>>, Vec<Complex<T>>, T, (), ()> + System<Set = T>
{
    fn firpm<FS, RES, const B2: usize, const R: usize, const W: usize>(
        order: usize,
        bands: [Self::Set; B2],
        response: [Self::Set; R],
        weight: [Self::Set; W],
        filter_type: FirPmType,
        sampling_frequency: FS,
        accuracy: Self::Set,
        persistence: Self::Set,
        robustness: Self::Set,
        target: Self::Set
    ) -> Result<(Self, Self::Set, RES), FirPmError>
    where
        FS: Maybe<Self::Set>,
        RES: StaticMaybe<FirPmReport<Self::Set>>,
        [(); 0 - B2%2]:,
        [(); B2/2 - 1]:,
        [(); B2 - R]:,
        [(); 0 - R % (B2/2)]:,
        [(); B2 - W]:,
        [(); 0 - W % (B2/2)]:
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
    Tf<T, Vec<T>, ()>: FirPm + ToSs<T, Array2<T>, Array2<T>, Array2<T>, Array2<T>> + System<Set = T>,
    Array2<T>: SsAMatrix<T, Array2<T>, Array2<T>, Array2<T>> + SsBMatrix<T, Array2<T>, Array2<T>, Array2<T>> + SsCMatrix<T, Array2<T>, Array2<T>, Array2<T>>+ SsDMatrix<T, Array2<T>, Array2<T>, Array2<T>>
{
    fn firpm<FS, RES, const B2: usize, const R: usize, const W: usize>(
        order: usize,
        bands: [Self::Set; B2],
        response: [Self::Set; R],
        weight: [Self::Set; W],
        filter_type: FirPmType,
        sampling_frequency: FS,
        accuracy: Self::Set,
        persistence: Self::Set,
        robustness: Self::Set,
        target: Self::Set
    ) -> Result<(Self, Self::Set, RES), FirPmError>
    where
        FS: Maybe<Self::Set>,
        RES: StaticMaybe<FirPmReport<Self::Set>>,
        [(); 0 - B2%2]:,
        [(); B2/2 - 1]:,
        [(); B2 - R]:,
        [(); 0 - R % (B2/2)]:,
        [(); B2 - W]:,
        [(); 0 - W % (B2/2)]:
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

        Ok((h.to_ss(), m, r))
    }
}

#[cfg(test)]
mod test
{
    use array_math::ArrayOps;

    use crate::{plot, gen::filter::{FirPm, FirPmType}, Plane, analysis::RealFreqZ, transforms::system::ToZpk, systems::{Tf, Zpk}};

    #[test]
    fn test()
    {
        let fs: f64 = 8000.0;
        let (n, f, a, w) = crate::generators::filter::firpmord([1900.0, 2000.0], [1.0, 0.0], [0.0001, 0.0001], fs)
            .unwrap();
        println!("{}", n);
        let (h, _, ()) = Tf::firpm(
            n,
            f,
            a,
            w,
            FirPmType::Symmetric,
            (),
            3.0,
            3.0,
            3.0,
            3.0
        ).unwrap();
    
        const N: usize = 1024;
        let (h_f, w): ([_; N], _) = h.real_freqz(());

        plot::plot_curves("H(e^jw)", "plots/h_z_firpm.png", [&w.zip(h_f.map(|h| h.norm())), &w.zip(h_f.map(|h| h.arg()))])
            .unwrap();

        let h: Zpk<_, Vec<_>, Vec<_>, _> = h.to_zpk((), ());

        plot::plot_pz("H(z)", "plots/pz_z_firpm.png", &h.p, &h.z, Plane::Z)
            .unwrap();
    }
}