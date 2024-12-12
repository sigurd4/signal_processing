use ndarray::Array2;
use ndarray_linalg::Lapack;
use num::{complex::ComplexFloat, traits::FloatConst, Complex, NumCast, One, Zero};
use option_trait::Maybe;
use core::{iter::Sum, ops::{AddAssign, Div, MulAssign, Sub}};
use std::ops::Mul;
use array_math::{SliceMath, SliceOps};

use crate::quantities::{List, Matrix, MaybeList};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum StftDetrend
{
    None,
    Linear,
    Constant
}

impl Default for StftDetrend
{
    fn default() -> Self
    {
        Self::None
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum StftBoundary
{
    None,
    Even,
    Odd,
    Constant,
    Zeros
}

impl Default for StftBoundary
{
    fn default() -> Self
    {
        Self::Zeros
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum StftScaling
{
    Spectrum,
    Psd
}

impl Default for StftScaling
{
    fn default() -> Self
    {
        Self::Spectrum
    }
}

pub trait Stft<T, S, N, O, W, WW>: List<T>
where
    T: ComplexFloat,
    S: Matrix<Complex<T::Real>>,
    N: Maybe<usize>,
    O: Maybe<usize>,
    W: ComplexFloat<Real = T::Real>,
    WW: MaybeList<W>
{
    fn stft<FS, D, B, P, SC>(
        self,
        width: N,
        sampling_frequency: FS,
        window: WW,
        overlap: O,
        detrend: D,
        boundary: B,
        padded: P,
        scaling: SC
    ) -> (S, S::RowsMapped<T::Real>, S::ColsMapped<T::Real>)
    where
        FS: Maybe<T::Real>,
        D: Maybe<StftDetrend>,
        B: Maybe<StftBoundary>,
        P: Maybe<bool>,
        SC: Maybe<StftScaling>;
}

impl<T, W, WW, X> Stft<T, Array2<Complex<<T as ComplexFloat>::Real>>, (), usize, W, WW> for X
where
    T: ComplexFloat<Real: Sum + AddAssign> + Into<Complex<<T as ComplexFloat>::Real>> + Lapack + Mul<<T as ComplexFloat>::Real, Output = T> + Div<<T as ComplexFloat>::Real, Output = T>,
    W: ComplexFloat<Real = <T as ComplexFloat>::Real> + Into<Complex<<T as ComplexFloat>::Real>> + Sum,
    Complex<<T as ComplexFloat>::Real>: AddAssign + MulAssign + MulAssign<<T as ComplexFloat>::Real>,
    WW: List<W>,
    X: List<T>
{
    fn stft<FS, D, B, P, SC>(
        self,
        (): (),
        sampling_frequency: FS,
        window: WW,
        overlap: usize,
        detrend: D,
        boundary: B,
        padded: P,
        scaling: SC
    ) -> (Array2<Complex<<T as ComplexFloat>::Real>>, Vec<<T as ComplexFloat>::Real>, Vec<<T as ComplexFloat>::Real>)
    where
        FS: Maybe<<T as ComplexFloat>::Real>,
        D: Maybe<StftDetrend>,
        B: Maybe<StftBoundary>,
        P: Maybe<bool>,
        SC: Maybe<StftScaling>
    {
        let one = <T as ComplexFloat>::Real::one();
        let two = one + one;

        let boundary = boundary.into_option()
            .unwrap_or_default();

        let fs = sampling_frequency.into_option()
            .unwrap_or_else(One::one);

        let mut x: Vec<T> = self.into_vec();
        let nx = x.len();
        let w = window.into_vec();
        let nw = w.len();
        let nwf = <<T as ComplexFloat>::Real as NumCast>::from(nw).unwrap();
        let f = (0..nw).map(|i| {
            let i = <<T as ComplexFloat>::Real as NumCast>::from(i).unwrap();
            i/nwf*<T as ComplexFloat>::Real::TAU()
        }).collect();
        if nx == 0
        {
            return (Array2::from_elem((nw, 0), Zero::zero()), f, vec![]);
        }

        let step = nw - overlap % nw;

        let nb = nw/2;
        if nb >= 1
        {
            match boundary
            {
                StftBoundary::Even => {
                    x = (0..(nx + nw)).map(|mut i| {
                        loop
                        {
                            if i < nb
                            {
                                i = 2*nb - i;
                            }
                            else if i >= nx + nb
                            {
                                i = 2*(nx + nb - 1) - i;
                            }
                            else
                            {
                                break x[i - nb]
                            }
                        }
                    }).collect()
                },
                StftBoundary::Odd => {
                    let le2 = x.first()
                        .copied()
                        .unwrap()*two;
                    let re2 = x.last()
                        .copied()
                        .unwrap()*two;
                    x = (0..(nx + nw)).map(|i| {
                        fn lp<T>(i: usize, x: &[T], le2: &T, re2: &T, nx: usize, nb: usize) -> T
                        where
                            T: Sub<Output = T> + Copy
                        {
                            if i < nb
                            {
                                *le2 - lp(2*nb - i, x, le2, re2, nx, nb)
                            }
                            else if i >= nx + nb
                            {
                                *re2 - lp(2*(nx + nb - 1) - i, x, le2, re2, nx, nb)
                            }
                            else
                            {
                                x[i - nb]
                            }
                        }
                        lp(i, &x, &le2, &re2, nx, nb)
                    }).collect()
                },
                StftBoundary::Constant => {
                    let le = x.first()
                        .copied()
                        .unwrap();
                    let re = x.last()
                        .copied()
                        .unwrap();
                    x = core::iter::repeat(le)
                        .take(nb)
                        .chain(x)
                        .chain(core::iter::repeat(re))
                        .take(nx + nw)
                        .collect()
                },
                StftBoundary::Zeros => {
                    x = core::iter::repeat(T::zero())
                        .take(nb)
                        .chain(x)
                        .chain(core::iter::repeat(T::zero()))
                        .take(nx + nw)
                        .collect()
                },
                StftBoundary::None => (),
            }
        }

        let padded = padded.into_option()
            .unwrap_or(false);

        if padded && step != 0
        {
            let nseg = x.len()/step;
            let nx = nw + nseg.saturating_sub(1)*step;
            x.resize(nx, T::zero());
        }

        let scaling = scaling.into_option()
            .unwrap_or_default();

        let scale = match scaling
        {
            StftScaling::Psd => {
                (fs*w.iter()
                    .map(|&w| (w.conj()*w).re())
                    .sum()
                ).recip()
            },
            StftScaling::Spectrum => {
                let s: W = w.iter()
                    .copied()
                    .sum();
                (s.conj()*s).re()
                    .recip()
            }
        }.sqrt();

        let detrend = detrend.into_option()
            .unwrap_or_default();

        let z: Vec<_> = (0..x.len() - nw.saturating_sub(1)).step_by(step)
            .map(|i| {
                let mut x: Vec<_> = x[i..].iter()
                    .take(nw)
                    .copied()
                    .collect();
                match detrend
                {
                    StftDetrend::None => (),
                    StftDetrend::Linear => x.detrend(1),
                    StftDetrend::Constant => {
                        let xmean = x.iter()
                            .copied()
                            .sum::<T>()/<<T as ComplexFloat>::Real as NumCast>::from(x.len()).unwrap();
                        for x in x.iter_mut()
                        {
                            *x -= xmean;
                        }
                    }
                }
                let mut y: Vec<_> = x.into_iter()
                    .zip(w.iter())
                    .map(|(x, &w)| Into::<Complex<<T as ComplexFloat>::Real>>::into(x)*w.into())
                    .collect();
                y.fft();
                y.mul_assign_all(scale);
                y
            }).collect();
        let z = Array2::from_shape_fn((nw, z.len()), |(r, c)| z[c][r]);
        
        let t = (0..x.len() - nw.saturating_sub(1)).step_by(step)
            .map(|i| {
                let mut n = <<T as ComplexFloat>::Real as NumCast>::from(i).unwrap();
                if boundary == StftBoundary::None
                {
                    n += <<T as ComplexFloat>::Real as NumCast>::from(nw).unwrap()/two
                }
                n/fs
            })
            .collect();

        (z, f, t)
    }
}

#[cfg(test)]
mod test
{
    use core::ops::Sub;

    #[test]
    fn test()
    {
        let x = [1, 2, 3, 4, 5];

        let nx = x.len();
        let nw = 10;
        let nb = nw/2;

        let y: Vec<_> = {
            let le2 = x.first()
                .copied()
                .unwrap()*2;
            let re2 = x.last()
                .copied()
                .unwrap()*2;
            (0..(nx + nw)).map(|i| {
                fn lp<T>(i: usize, x: &[T], le2: &T, re2: &T, nx: usize, nb: usize) -> T
                where
                    T: Sub<Output = T> + Copy
                {
                    if i < nb
                    {
                        *le2 - lp(2*nb - i, x, le2, re2, nx, nb)
                    }
                    else if i >= nx + nb
                    {
                        *re2 - lp(2*(nx + nb - 1) - i, x, le2, re2, nx, nb)
                    }
                    else
                    {
                        x[i - nb]
                    }
                }
                lp(i, &x, &le2, &re2, nx, nb)
            }).collect()
        };

        println!("{:?}", y)
    }
}