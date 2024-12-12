use core::ops::{AddAssign, MulAssign};

use ndarray::Array2;
use num::{complex::ComplexFloat, traits::FloatConst, Complex, NumCast};
use option_trait::Maybe;
use array_math::{Array2dOps, ArrayOps, SliceMath};

use crate::{windows::Hamming, gen::window::{WindowGen, WindowRange}, quantities::{List, Matrix, MaybeList, ListOrSingle}};

pub trait SpecGram<T, S, N, O, W, WW, const WWW: bool>: List<T>
where
    T: ComplexFloat + Into<Complex<T::Real>>,
    S: Matrix<Complex<T::Real>>,
    N: Maybe<usize>,
    O: Maybe<usize>,
    W: ComplexFloat<Real = T::Real> + Into<Complex<T::Real>>,
    WW: MaybeList<W>
{
    #[doc(alias = "spectrogram")]
    fn specgram<FS>(
        &self,
        width: N,
        sampling_frequency: FS,
        window: WW,
        overlap: O
    ) -> (S, S::RowsMapped<T::Real>, S::ColsMapped<T::Real>)
    where
        FS: Maybe<T::Real>;
}

impl<T, W, WW, X> SpecGram<T, Array2<Complex<T::Real>>, (), usize, W, WW, true> for X
where
    T: ComplexFloat + Into<Complex<T::Real>>,
    W: ComplexFloat<Real = T::Real> + Into<Complex<T::Real>>,
    WW: List<W>,
    X: List<T>,
    Complex<T::Real>: AddAssign + MulAssign
{
    fn specgram<FS>(
        &self,
        (): (),
        sampling_frequency: FS,
        window: WW,
        overlap: usize
    ) -> (Array2<Complex<T::Real>>, Vec<T::Real>, Vec<T::Real>)
    where
        FS: Maybe<<T as ComplexFloat>::Real>
    {
        let x: &[T] = self.as_view_slice();
        let window: &[W] = window.as_view_slice();
        let n = window.len();
        let step = n - overlap % n;

        let s: Vec<_> = (0..x.len() + 1 - n).step_by(step)
            .map(|i| {
                let mut y: Vec<_> = x[i..].iter()
                    .zip(window.iter())
                    .map(|(&x, &w)| Into::<Complex<T::Real>>::into(x)*w.into())
                    .collect();
                y.fft();
                y
            }).collect();
        let s = Array2::from_shape_fn((n, s.len()), |(r, c)| s[c][r]);

        let fs = sampling_frequency.into_option()
            .unwrap_or_else(FloatConst::TAU);

        let t = (0..x.len() + 1 - n).step_by(step)
            .map(|i| <T::Real as NumCast>::from(i).unwrap()/fs)
            .collect();
        let f = (0..n).map(|i| <T::Real as NumCast>::from(i).unwrap()/NumCast::from(n).unwrap()*fs)
            .collect();

        (s, f, t)
    }
}

impl<T, W, WW, X, const L: usize> SpecGram<T, [[Complex<T::Real>; L]; WW::LENGTH], (), (), W, WW, true> for X
where
    T: ComplexFloat + Into<Complex<T::Real>>,
    W: ComplexFloat<Real = T::Real> + Into<Complex<T::Real>>,
    WW: List<W, Length = usize>,
    X: List<T, Length = usize>,
    Complex<T::Real>: AddAssign + MulAssign,
    [(); WW::LENGTH - (X::LENGTH - WW::LENGTH)/L - 1]:
{
    fn specgram<FS>(
        &self,
        (): (),
        sampling_frequency: FS,
        window: WW,
        (): ()
    ) -> ([[Complex<T::Real>; L]; WW::LENGTH], [T::Real; WW::LENGTH], [T::Real; L])
    where
        FS: Maybe<<T as ComplexFloat>::Real>
    {
        let overlap = WW::LENGTH - (X::LENGTH - WW::LENGTH)/L - 1;

        let x: &[T] = self.as_view_slice();
        let window: &[W] = window.as_view_slice();
        let n = WW::LENGTH;
        let mut step = n - overlap;
        while step*(L - 1) + WW::LENGTH > X::LENGTH
        {
            step -= 1;
        }

        let s = <[_; L]>::fill(|k| {
            let i = k*step;
            let mut y: [Complex<_>; _] = <[_; WW::LENGTH]>::fill(|j| Into::<Complex<T::Real>>::into(x[i + j])*window[j].into());
            y.fft();
            y
        }).transpose();

        let fs = sampling_frequency.into_option()
            .unwrap_or_else(FloatConst::TAU);

        let t = ArrayOps::fill(|k| {
            let i = k*step;
            <T::Real as NumCast>::from(i).unwrap()/fs
        });
        let f = ArrayOps::fill(|i| <T::Real as NumCast>::from(i).unwrap()/NumCast::from(n).unwrap()*fs);

        (s, f, t)
    }
}

impl<T, X> SpecGram<T, Array2<Complex<T::Real>>, usize, usize, T::Real, (), false> for X
where
    T: ComplexFloat + Into<Complex<T::Real>>,
    X: List<T>,
    Complex<T::Real>: AddAssign + MulAssign
{
    fn specgram<FS>(
        &self,
        width: usize,
        sampling_frequency: FS,
        (): (),
        overlap: usize
    ) -> (Array2<Complex<T::Real>>, Vec<T::Real>, Vec<T::Real>)
    where
        FS: Maybe<<T as ComplexFloat>::Real>
    {
        self.specgram(
            (),
            sampling_frequency,
            WindowGen::<T::Real, Vec<T::Real>, _>::window_gen(&Hamming, width, WindowRange::Periodic),
            overlap
        )
    }
}

impl<T, X, const L: usize, const W: usize> SpecGram<T, [[Complex<T::Real>; L]; W], (), (), T::Real, (), false> for X
where
    T: ComplexFloat + Into<Complex<T::Real>>,
    T::Real: ComplexFloat<Real = T::Real> + Into<Complex<T::Real>>,
    X: List<T, Length = usize>,
    [T::Real; W]: List<T::Real, Length = usize>,
    Complex<T::Real>: AddAssign + MulAssign,
    [(); <[T::Real; W]>::LENGTH]:,
    [(); <[T::Real; W]>::LENGTH - (X::LENGTH - <[T::Real; W]>::LENGTH)/L - 1]:
{
    fn specgram<FS>(
        &self,
        (): (),
        sampling_frequency: FS,
        (): (),
        (): ()
    ) -> ([[Complex<T::Real>; L]; W], [T::Real; W], [T::Real; L])
    where
        FS: Maybe<T::Real>
    {
        let (s, f, t) = SpecGram::<_, [[Complex<T::Real>; L]; <[T::Real; W]>::LENGTH], _, _, _, _, _>::specgram(
            self,
            (),
            sampling_frequency,
            WindowGen::<T::Real, [T::Real; W], _>::window_gen(&Hamming, (), WindowRange::Periodic),
            ()
        );
        (
            s.try_reformulate_length()
                .ok()
                .unwrap(),
            f.try_reformulate_length()
                .ok()
                .unwrap(),
            t
        )
    }
}

#[cfg(test)]
mod test
{
    use array_math::ArrayOps;

    use crate::{plot, gen::waveform::{Chirp, ChirpCurve}, analysis::SpecGram};

    #[test]
    fn test()
    {
        const FS: f64 = 1000.0;
        const T: f64 = 2.0;
        const N: usize = (T*FS) as usize;
        let t = 0.0..T;
        let (x, _): (_, [_; N]) = t.chirp((), 10.0..150.0, 0.0..2.0, ChirpCurve::Logarithmic, 0.0);
        const W: usize = ((0.1*FS) as usize).next_power_of_two();
        const O: usize = W/2;
        const L: usize = (N - W)/(W - O);
        let (s, f, t): (_, [_; W], [_; L]) = x.specgram((), N as f64/2.0, (), ());

        plot::plot_parametric_curve_2d("H(e^jw, t)", "plots/h_z_specgram.svg",
            <[_; W]>::fill(|i| i as f64),
            <[_; L]>::fill(|i| i as f64),
            |i, j| [f[i as usize], t[j as usize], s[i as usize][j as usize].norm().log10()*20.0]
        ).unwrap()
    }
}