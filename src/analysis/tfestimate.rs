use num::{complex::ComplexFloat, traits::FloatConst, Complex, Float};
use option_trait::{Maybe, StaticMaybe};

use crate::{List, MaybeLenEq, PWelch, PWelchDetrend};

pub trait TfEstimate<X, Y, YY, W, WW, WWW, WL, N, S>: List<X> + MaybeLenEq<YY, true>
where
    X: ComplexFloat,
    Y: ComplexFloat<Real = X::Real>,
    YY: List<Y>,
    W: ComplexFloat<Real = X::Real>,
    WW: List<W>,
    WWW: Maybe<WW>,
    WL: Maybe<usize>,
    N: Maybe<usize>,
    S: Maybe<bool>
{
    #[doc(alias = "tfe")]
    fn tfestimate<O, FS, CONF, DT, F>(
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
    ) -> (WW::Mapped<Complex<X::Real>>, F)
    where
        O: Maybe<usize>,
        FS: Maybe<X::Real>,
        CONF: Maybe<X::Real>,
        DT: Maybe<PWelchDetrend>,
        F: StaticMaybe<WW::Mapped<X::Real>>;
}

impl<T, X, XX, Y, YY, W, WW, WWW, WL, N, S> TfEstimate<X, Y, YY, W, WW, WWW, WL, N, S> for XX
where
    T: FloatConst + Float,
    X: ComplexFloat<Real = T>,
    XX: List<X> + MaybeLenEq<YY, true>,
    Y: ComplexFloat<Real = T>,
    YY: List<Y, MaybeSome = YY, Some = YY> + StaticMaybe<YY, MaybeOr<Y, X> = Y>,
    W: ComplexFloat<Real = T>,
    WW: List<W>,
    WWW: Maybe<WW>,
    WL: Maybe<usize>,
    N: Maybe<usize>,
    S: Maybe<bool>,
    XX: PWelch<X, Y, YY, W, WW, WWW, WL, N, S>,
    (): StaticMaybe<YY::Maybe<WW::Mapped<Complex<T>>>>,
    (): StaticMaybe<YY::Maybe<WW::Mapped<T>>>,
    WW::Mapped<Complex<T>>: StaticMaybe<YY::Maybe<WW::Mapped<Complex<T>>>>
{
    fn tfestimate<O, FS, CONF, DT, F>(
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
    ) -> (WW::Mapped<Complex<T>>, F)
    where
        O: Maybe<usize>,
        FS: Maybe<T>,
        CONF: Maybe<T>,
        DT: Maybe<PWelchDetrend>,
        F: StaticMaybe<WW::Mapped<T>>
    {
        let ((), (), trans, (), (), (), f) = self.pwelch(
            y,
            window,
            window_length,
            overlap,
            nfft,
            sampling_frequency,
            confidence,
            detrend,
            sloppy,
            shift
        );

        (trans, f)
    }
}

#[cfg(test)]
mod test
{
    use array_math::ArrayOps;
    use rand::distributions::uniform::SampleRange;

    use crate::{plot, window::{Boxcar, WindowGen, WindowRange}, Filter, Fir1, Fir1Type, FreqZ, Tf, TfEstimate};

    #[test]
    fn test()
    {
        let w: [f64; 31] = Boxcar.window_gen((), WindowRange::Symmetric);
        let h = Tf::<f64, [_; _]>::fir1((), [0.2], Fir1Type::LowPass, w, true, ())
            .unwrap();

        const N: usize = 1024;
        let mut rng = rand::thread_rng();
        let x: [_; N] = core::array::from_fn(|_| (-1.0..1.0).sample_single(&mut rng));
        let y = h.filter(x, ());
        let (h_f, _) = h.freqz((), true);

        const M: usize = 1024;
        let fs = 500.0;
        let (he, f): ([_; M], [_; M]) = TfEstimate::<_, _, _, f64, [_; _], (), (), (), ()>::
            tfestimate(x, y, (), (), (), (), fs, (), (), (), true);

        plot::plot_curves("H(e^jw)", "plots/h_z_tfestimate.png", [
                &f.zip(h_f.map(|h| 20.0*h.norm().log10())),
                &f.zip(he.map(|h| 20.0*h.norm().log10()))
            ]).unwrap()
    }
}