use core::ops::{AddAssign, MulAssign, SubAssign};

use num::{complex::ComplexFloat, traits::float::FloatConst, Complex, Float, NumCast, One, Zero};
use option_trait::Maybe;
use array_math::SliceMath;

use crate::{systems::Ar, quantities::{ContainerOrSingle, List, ListOrSingle, Lists, MaybeList}, System};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum PsdMethod
{
    Fft,
    Polynomial
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum PsdRange
{
    Whole,
    Shift,
    Half
}

pub trait Psd<'a, P, F, N, FF, R, M>: System
where
    P: Lists<<Self::Domain as ComplexFloat>::Real>,
    F: List<<Self::Domain as ComplexFloat>::Real>,
    N: Maybe<usize>,
    FF: Maybe<F>,
    M: Maybe<PsdMethod>,
    R: Maybe<PsdRange>
{
    fn psd<FS>(&'a self, numtaps: N, frequencies: FF, sampling_frequency: FS, range: R, method: M) -> (P, F)
    where
        FS: Maybe<<Self::Domain as ComplexFloat>::Real>;
}

impl<'a, T, A, AV, N, M, R> Psd<'a, AV::Mapped<Vec<T::Real>>, Vec<T::Real>, N, (), R, M> for Ar<T, A, AV>
where
    T: ComplexFloat<Real: SubAssign + AddAssign> + Into<Complex<T::Real>>,
    A: MaybeList<T>,
    AV: ListOrSingle<(A, T::Real)>,
    AV::Mapped<Vec<T::Real>>: Lists<T::Real>,
    N: Maybe<usize>,
    M: Maybe<PsdMethod>,
    R: Maybe<PsdRange>,
    Complex<T::Real>: MulAssign + AddAssign
{
    fn psd<FS>(&'a self, numtaps: N, (): (), sampling_frequency: FS, range: R, method: M) -> (AV::Mapped<Vec<T::Real>>, Vec<T::Real>)
    where
        FS: Maybe<<Self::Domain as ComplexFloat>::Real>
    {
        let one = T::Real::one();
        let two = one + one;

        let freq_len = numtaps.into_option()
            .unwrap_or(256);
        let freq_lenf = <T::Real as NumCast>::from(freq_len).unwrap();
        let fs = sampling_frequency.into_option()
            .unwrap_or(T::Real::TAU());
        let range = range.into_option()
            .unwrap_or(PsdRange::Whole);

        let w = fs/freq_lenf;
        let freq: Vec<_> = (0..if range == PsdRange::Half {freq_len/2 + 1} else {freq_len}).map(|i| {
            let mut i = <T::Real as NumCast>::from(i).unwrap();
            if range == PsdRange::Shift
            {
                i -= Float::floor(freq_lenf/two)
            }
            w*i
        }).collect();

        let method = method.into_option()
            .unwrap_or_else(|| if freq_len.is_power_of_two()
            {
                PsdMethod::Fft
            }
            else
            {
                PsdMethod::Polynomial
            });
        
        if method == PsdMethod::Polynomial
        {
            return self.psd((), freq, fs, (), ())
        }

        let psd = self.av.map_to_owned(|(a, v)| {
            let a = a.to_vec_option()
                .unwrap_or_else(|| vec![]);

            let mut fft_out: Vec<_> = a.into_iter()
                .map(|a| a.into())
                .collect();
            fft_out.resize(freq_len, Complex::zero());
            fft_out.fft();

            let vdfs = *v/fs;

            let mut psd: Vec<_> = fft_out.into_iter()
                .map(|o| vdfs/(o.conj()*o).re)
                .collect();
            match range
            {
                PsdRange::Shift => psd.rotate_right(freq_len/2),
                PsdRange::Half => {
                    let psd2 = psd.split_off(freq_len/2 + 1);
                    let psd0 = psd[0];
                    for (p, o) in psd.iter_mut()
                        .zip(core::iter::once(psd0)
                            .chain(psd2.into_iter()
                                .rev()
                            )
                        )
                    {
                        *p += o
                    }
                },
                PsdRange::Whole => ()
            }

            psd
        });

        (psd, freq)
    }
}

impl<'a, T, A, AV, M, R, const N: usize> Psd<'a, AV::Mapped<[T::Real; N]>, [T::Real; N], (), (), R, M> for Ar<T, A, AV>
where
    T: ComplexFloat,
    A: MaybeList<T>,
    AV: ListOrSingle<(A, T::Real)>,
    AV::Mapped<[T::Real; N]>: Lists<T::Real>,
    M: Maybe<PsdMethod>,
    R: Maybe<PsdRange>,
    Self: Psd<'a, AV::Mapped<Vec<T::Real>>, Vec<T::Real>, usize, (), PsdRange, M> + System<Domain = T>,
    AV::Mapped<Vec<T::Real>>: Lists<T::Real> + ListOrSingle<Vec<T::Real>, Mapped<[T::Real; N]> = AV::Mapped<[T::Real; N]>>
{
    fn psd<FS>(&'a self, (): (), (): (), sampling_frequency: FS, range: R, method: M) -> (AV::Mapped<[T::Real; N]>, [T::Real; N])
    where
        FS: Maybe<<Self::Domain as ComplexFloat>::Real>
    {
        if N == 0
        {
            return (self.av.map_to_owned(|_| [T::Real::zero(); N]), [T::Real::zero(); N])
        }
        
        let range = range.into_option()
            .unwrap_or(PsdRange::Whole);

        let numtaps = if range == PsdRange::Half
        {
            (N - 1)*2
        }
        else
        {
            N
        };

        let (psd, f) = self.psd(numtaps, (), sampling_frequency, range, method);

        (
            psd.map_into_owned(|psd: Vec<T::Real>| TryInto::<[T::Real; N]>::try_into(psd).ok().unwrap()),
            f.try_into().ok().unwrap()
        )
    }
}

impl<'a, T, A, AV, FF> Psd<'a, AV::Mapped<FF::Mapped<T::Real>>, FF, (), FF, (), ()> for Ar<T, A, AV>
where
    T: ComplexFloat + Into<Complex<T::Real>>,
    A: MaybeList<T>,
    AV: ListOrSingle<(A, T::Real)>,
    FF: List<T::Real>,
    AV::Mapped<FF::Mapped<T::Real>>: Lists<T::Real>,
    Complex<T::Real>: AddAssign + MulAssign
{
    fn psd<FS>(&'a self, (): (), frequencies: FF, sampling_frequency: FS, (): (), (): ()) -> (AV::Mapped<FF::Mapped<T::Real>>, FF)
    where
        FS: Maybe<<Self::Domain as ComplexFloat>::Real>
    {
        let fs = sampling_frequency.into_option()
            .unwrap_or(T::Real::TAU());
        
        let w = T::Real::TAU()/fs;
        
        let psd = self.av.map_to_owned(|(a, v)| {
            let a: Vec<_> = a.to_vec_option()
                .unwrap_or_else(|| vec![])
                .into_iter()
                .map(|a| a.into())
                .collect();

            let vdfs = *v/fs;

            frequencies.map_to_owned(|&f| {
                let o = a.polynomial(Complex::cis(-w*f));
                vdfs/(o.conj()*o).re
            })
        });

        (psd, frequencies)
    }
}

#[cfg(test)]
mod test
{
    use array_math::ArrayOps;

    use crate::{plot, systems::Ar, analysis::Psd};

    #[test]
    fn test()
    {
        let ar = Ar::new(([1.0, 2.0, 3.0], 4.0));

        const N: usize = 1024;
        let (psd, f): ([_; N], _) = ar.psd((), (), (), (), ());

        plot::plot_curves("P(e^jw)", "plots/p_z_psd.png", [&f.zip(psd)])
            .unwrap();
    }
}