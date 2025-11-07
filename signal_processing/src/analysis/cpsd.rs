use num::{complex::ComplexFloat, Complex};
use option_trait::{Maybe, NotVoid, StaticMaybe};

use crate::{quantities::List, util::MaybeLenEq, analysis::{PWelch, PWelchDetrend}};

/// A trait for computing the cross power spectral density of two sequences.
pub trait CPsd<T, Y, YY, W, WW, WWW, WL, N, S>: List<T> + MaybeLenEq<YY, true>
where
    T: ComplexFloat,
    W: ComplexFloat<Real = T::Real>,
    Y: ComplexFloat<Real = T::Real>,
    YY: List<Y>,
    WW: List<W>,
    WWW: Maybe<WW>,
    WL: Maybe<usize>,
    N: Maybe<usize>,
    S: Maybe<bool>
{
    /// Computes the cross power spectral density of two sequences using Welch's averaged periodogram method.
    /// 
    /// # Arguments
    /// 
    /// * `y` - The other sequence.
    /// * `window` - A window sequence. If none, a [Hamming](crate::windows::Hamming) window with lenght `window_length` will be used.
    /// * `window_length` - A window length if no `window` is given.
    /// * `overlap` - Overlap length in samples. If none, defaults to `window_length/2`.
    /// * `nfft` - Length used for FFT if no `window` is given.
    /// * `sampling_frequency` - Sampling frequency. If none, defaults to `1.0`.
    /// * `confidence` - Probability in the range `[0.0, 1.0]` for the confidence intervals for the PSD estimate. If none, defaults to `0.95`.
    /// * `detrend` - Method of detrending the signals before or after analysis. If none, [LongMean](PWelchDetrend::LongMean) is used.
    /// * `sloppy` - If true, `nfft` will be set to the next power of two. Only applicable if no `window` is given.
    /// * `shift` - If true, data will be shifted to center-DC.
    /// 
    /// # Returns
    /// 
    /// * `pxy` - Cross power spectral density.
    /// * `frequencies` - Frequencies of the cross power spectral density.
    #[doc(alias = "csd")]
    fn cpsd<O, FS, CONF, DT, F>(
        self,
        y: YY,
        window: WWW,
        window_length: WL,
        overlap: O,
        nfft: N,
        sampling_frequency: FS,
        confidence: CONF,
        detrend: DT,
        sloppy: S,
        shift: bool
    ) -> (WW::Mapped<Complex<<T as ComplexFloat>::Real>>, F)
    where
        O: Maybe<usize>,
        FS: Maybe<T::Real>,
        CONF: Maybe<T::Real>,
        DT: Maybe<PWelchDetrend>,
        F: StaticMaybe<WW::Mapped<T::Real>>;
}

impl<T, L, Y, YY, W, WW, WWW, WL, N, S> CPsd<T, Y, YY, W, WW, WWW, WL, N, S> for L
where
    L: List<T> + MaybeLenEq<YY, true>,
    T: ComplexFloat,
    W: ComplexFloat<Real = T::Real>,
    Y: ComplexFloat<Real = T::Real>,
    YY: List<Y, MaybeSome: StaticMaybe<YY::Some, MaybeOr<Y, T> = Y>>,
    WW: List<W>,
    WWW: Maybe<WW>,
    WL: Maybe<usize>,
    N: Maybe<usize>,
    S: Maybe<bool>,
    Self: PWelch<T, Y, YY, W, WW, WWW, WL, N, S>,
    WW::Mapped<T::Real>: StaticMaybe<WW::Mapped<T::Real>>,
    WW::Mapped<Complex<T::Real>>: StaticMaybe<WW::Mapped<Complex<T::Real>>> + StaticMaybe<<YY::MaybeSome as StaticMaybe<YY::Some>>::Maybe<WW::Mapped<Complex<T::Real>>>>,
    <YY::MaybeSome as StaticMaybe<YY::Some>>::Maybe<WW::Mapped<Complex<T::Real>>>: NotVoid,
    <YY::MaybeSome as StaticMaybe<YY::Some>>::Maybe<WW::Mapped<T::Real>>: NotVoid,
    (): StaticMaybe<WW::Mapped<T::Real>>,
{
    fn cpsd<O, FS, CONF, DT, F>(
        self,
        y: YY,
        window: WWW,
        window_length: WL,
        overlap: O,
        nfft: N,
        sampling_frequency: FS,
        confidence: CONF,
        detrend: DT,
        sloppy: S,
        shift: bool
    ) -> (<WW>::Mapped<Complex<<T as ComplexFloat>::Real>>, F)
    where
        O: Maybe<usize>,
        FS: Maybe<<T as ComplexFloat>::Real>,
        CONF: Maybe<<T as ComplexFloat>::Real>,
        DT: Maybe<PWelchDetrend>,
        F: StaticMaybe<<WW>::Mapped<<T as ComplexFloat>::Real>>
    {
        let ((), cross, (), (), (), (), f) = self.pwelch(y, window, window_length, overlap, nfft, sampling_frequency, confidence, detrend, sloppy, shift);
        (cross, f)
    }
}

#[cfg(test)]
mod test
{
    use array_math::ArrayOps;
    use rand::distributions::uniform::SampleRange;

    use crate::{
        plot,
        windows::{Boxcar, Triangular},
        operations::filtering::Filter,
        gen::{window::{WindowGen, WindowRange}, filter::{Fir1, Fir1Type}},
        analysis::RealCPsd,
        systems::Tf
    };

    #[test]
    fn test()
    {
        const N: usize = 16384;
        let mut rng = rand::thread_rng();
        let r: Vec<_> = (0..N).map(|_| (-1.0..1.0).sample_single(&mut rng)).collect();

        const B: usize = 31;
        let w: [f64; B] = Boxcar.window_gen((), WindowRange::Symmetric);
        let hx = Tf::<f64, [_; B]>::fir1((), [0.2], Fir1Type::LowPass, w, true, ())
            .unwrap();
        let x = hx.filter(r.as_slice(), ());

        let hy = Tf::new([1.0; 10], ());
        let y = hy.filter(r, ());

        const M: usize = 500;
        let w: [_; M] = Triangular.window_gen((), WindowRange::Symmetric);
        let noverlap = 250;

        let (pxy, f): (_, [_; M/2 + 1]) = x.real_cpsd(y, w, (), noverlap, (), (), (), (), ());

        plot::plot_curves("P_xy(e^jw)", "plots/pxy_z_cpsd.png", [
            &f.zip(pxy.map(|p| 10.0*p.norm().log10()))
        ]).unwrap()
    }
}