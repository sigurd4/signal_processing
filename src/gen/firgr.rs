use core::ops::{AddAssign, DivAssign, MulAssign};


use ndarray::Array2;
use num::{complex::ComplexFloat, traits::FloatConst, Complex, Float};
use option_trait::Maybe;
use thiserror::Error;

use crate::{FilterClassType, Polynomial, Ss, SsAMatrix, SsBMatrix, SsCMatrix, SsDMatrix, System, Tf, ToSs, ToZpk, Zpk};

moddef::moddef!(
    mod {
        remez
    }
);


#[derive(Debug, Clone, Copy, PartialEq, Error)]
pub enum FirGrError
{
    #[error("Bands must be monotonic starting at zero.")]
    EdgesNotNondecreasing,
    #[error("Band edges should be less than 1/2 the sampling frequency, or if no sampling frequency is specified, between 0 and 1.")]
    EdgesOutOfRange,
    #[error("Failure to converge at iteration {niter}, try reducing transition band width.")]
    FailureToConverge{
        niter: usize,
        dev: f64
    },
    #[error("Grid too dense.")]
    TooDense,
    #[error("Sampling frequency must be a positive number.")]
    InvalidSamplingFrequency
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum FirGrOrder
{
    Exact(usize),
    MinEven(f64),
    MinOdd(f64),
    Min(f64)
}

pub trait FirGr<O>: System + Sized
where
    Self::Domain: Float,
    O: Maybe<usize>
{
    fn firgr<FS, MI, GD, const B: usize>(
        order: O,
        bands: [Self::Domain; B*2],
        response: [Self::Domain; B],
        weight: [Self::Domain; B],
        filter_type: FilterClassType,
        sampling_frequency: FS,
        max_iter: MI,
        grid_density: GD
    ) -> Result<Self, FirGrError>
    where
        FS: Maybe<Self::Domain>,
        MI: Maybe<usize>,
        GD: Maybe<usize>;
}

impl<T> FirGr<usize> for Tf<T, Vec<T>, ()>
where
    T: Float + FloatConst + AddAssign + MulAssign + DivAssign
{
    fn firgr<FS, MI, GD, const B: usize>(
        order: usize,
        bands: [T; B*2],
        response: [T; B],
        weight: [T; B],
        filter_type: FilterClassType,
        sampling_frequency: FS,
        max_iter: MI,
        grid_density: GD
    ) -> Result<Self, FirGrError>
    where
        FS: Maybe<T>,
        MI: Maybe<usize>,
        GD: Maybe<usize>
    {
        remez::pre_remez(
            order + 1,
            bands,
            response,
            weight,
            filter_type,
            sampling_frequency.into_option(),
            max_iter.into_option().unwrap_or(40),
            grid_density.into_option().unwrap_or(16),
            T::infinity()
        ).map(|(h, _)| Tf {
            b: Polynomial::new(h.into_iter().map(Into::into).collect()),
            a: Polynomial::new(())
        })
    }
}

impl<T, const N: usize> FirGr<()> for Tf<T, [T; N], ()>
where
    T: Float + FloatConst + AddAssign + MulAssign + DivAssign
{
    fn firgr<FS, MI, GD, const B: usize>(
        (): (),
        bands: [T; B*2],
        response: [T; B],
        weight: [T; B],
        filter_type: FilterClassType,
        sampling_frequency: FS,
        max_iter: MI,
        grid_density: GD
    ) -> Result<Self, FirGrError>
    where
        FS: Maybe<T>,
        MI: Maybe<usize>,
        GD: Maybe<usize>
    {
        remez::pre_remez(
            N,
            bands,
            response,
            weight,
            filter_type,
            sampling_frequency.into_option(),
            max_iter.into_option().unwrap_or(40),
            grid_density.into_option().unwrap_or(16),
            T::infinity()
        ).map(|(h, _)| Tf {
            b: Polynomial::new(h.into_iter()
                .map(Into::into)
                .collect::<Vec<_>>()
                .try_into()
                .ok()
                .unwrap()
            ),
            a: Polynomial::new(())
        })
    }
}

impl<T> FirGr<usize> for Zpk<Complex<T>, Vec<Complex<T>>, Vec<Complex<T>>, T>
where
    T: Float + FloatConst,
    Complex<T>: ComplexFloat<Real = T>,
    Tf<T, Vec<T>, ()>: FirGr<usize> + ToZpk<Complex<T>, Vec<Complex<T>>, Vec<Complex<T>>, T, (), ()> + System<Domain = T>
{
    fn firgr<FS, MI, GD, const B: usize>(
        order: usize,
        bands: [T; B*2],
        response: [T; B],
        weight: [T; B],
        filter_type: FilterClassType,
        sampling_frequency: FS,
        max_iter: MI,
        grid_density: GD
    ) -> Result<Self, FirGrError>
    where
        FS: Maybe<T>,
        MI: Maybe<usize>,
        GD: Maybe<usize>
    {
        let h = Tf::firgr(
            order,
            bands,
            response,
            weight,
            filter_type,
            sampling_frequency,
            max_iter,
            grid_density
        )?;

        Ok(h.to_zpk((), ()))
    }
}

impl<T> FirGr<usize> for Ss<T, Array2<T>, Array2<T>, Array2<T>, Array2<T>>
where
    T: Float + FloatConst,
    Tf<T, Vec<T>, ()>: FirGr<usize> + ToSs<T, Array2<T>, Array2<T>, Array2<T>, Array2<T>> + System<Domain = T>,
    Array2<T>: SsAMatrix<T, Array2<T>, Array2<T>, Array2<T>> + SsBMatrix<T, Array2<T>, Array2<T>, Array2<T>> + SsCMatrix<T, Array2<T>, Array2<T>, Array2<T>>+ SsDMatrix<T, Array2<T>, Array2<T>, Array2<T>>
{
    fn firgr<FS, MI, GD, const B: usize>(
        order: usize,
        bands: [T; B*2],
        response: [T; B],
        weight: [T; B],
        filter_type: FilterClassType,
        sampling_frequency: FS,
        max_iter: MI,
        grid_density: GD
    ) -> Result<Self, FirGrError>
    where
        FS: Maybe<T>,
        MI: Maybe<usize>,
        GD: Maybe<usize>
    {
        let h = Tf::firgr(
            order,
            bands,
            response,
            weight,
            filter_type,
            sampling_frequency,
            max_iter,
            grid_density
        )?;

        Ok(h.to_ss().unwrap())
    }
}

#[cfg(test)]
mod test
{
    use array_math::ArrayOps;
    use crate::{plot, FirGr, FilterClassType, RealFreqZ, Tf};

    #[test]
    fn test()
    {
        let h: Tf<_, [_; 25]> = Tf::firgr(
            (),
            [0.0, 0.4, 0.5, 0.7, 0.8, 1.0],
            [1.0, 0.0, 1.0],
            [1.0, 1.0, 1.0],
            FilterClassType::Symmetric,
            (),
            (),
            ()
        ).unwrap();
    
        const N: usize = 1024;
        let (h_f, w): ([_; N], _) = h.real_freqz(());

        plot::plot_curves("H(e^jw)", "plots/h_z_firgr.png", [&w.zip(h_f.map(|h| h.norm())), /*&w.zip(h_f.map(|h| h.arg()))*/]).unwrap();
    }
}