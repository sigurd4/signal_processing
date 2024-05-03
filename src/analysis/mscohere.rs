use num::{complex::ComplexFloat, Complex};
use option_trait::{Maybe, NotVoid, StaticMaybe};

use crate::{quantities::List, util::MaybeLenEq, analysis::{PWelch, PWelchDetrend}};

pub trait MsCohere<T, Y, YY, W, WW, WWW, WL, N, S>: List<T> + MaybeLenEq<YY, true>
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
    #[doc(alias = "cohere", alias = "coherence")]
    fn mscohere<O, FS, CONF, DT, F>(
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
    ) -> (WW::Mapped<T::Real>, F)
    where
        O: Maybe<usize>,
        FS: Maybe<T::Real>,
        CONF: Maybe<T::Real>,
        DT: Maybe<PWelchDetrend>,
        F: StaticMaybe<WW::Mapped<T::Real>>;
}

impl<T, L, Y, YY, W, WW, WWW, WL, N, S> MsCohere<T, Y, YY, W, WW, WWW, WL, N, S> for L
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
    WW::Mapped<T::Real>: StaticMaybe<WW::Mapped<T::Real>> + StaticMaybe<<YY::MaybeSome as StaticMaybe<YY::Some>>::Maybe<WW::Mapped<T::Real>>>,
    WW::Mapped<Complex<T::Real>>: StaticMaybe<WW::Mapped<Complex<T::Real>>>,
    <YY::MaybeSome as StaticMaybe<YY::Some>>::Maybe<WW::Mapped<Complex<T::Real>>>: NotVoid,
    <YY::MaybeSome as StaticMaybe<YY::Some>>::Maybe<WW::Mapped<T::Real>>: NotVoid,
    (): StaticMaybe<WW::Mapped<T::Real>>
{
    fn mscohere<O, FS, CONF, DT, F>(
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
    ) -> (<WW>::Mapped<<T as ComplexFloat>::Real>, F)
    where
        O: Maybe<usize>,
        FS: Maybe<<T as ComplexFloat>::Real>,
        CONF: Maybe<<T as ComplexFloat>::Real>,
        DT: Maybe<PWelchDetrend>,
        F: StaticMaybe<<WW>::Mapped<<T as ComplexFloat>::Real>>
    {
        let ((), (), (), coher, (), (), f) = self.pwelch(y, window, window_length, overlap, nfft, sampling_frequency, confidence, detrend, sloppy, shift);
        (coher, f)
    }
}

#[cfg(test)]
mod test
{
    use array_math::ArrayOps;
    use rand::distributions::uniform::SampleRange;

    use crate::{plot, gen::filter::{Cheby1, Cheby2, FilterGenPlane}, operations::filtering::Filter, analysis::{RealMsCohere, RealFreqZ}, systems::Tf};

    #[test]
    fn test()
    {
        const N: usize = 16384;
        let mut rng = rand::thread_rng();
        let r: Vec<_> = (0..N).map(|_| (-1.0..1.0).sample_single(&mut rng)).collect();

        let (n, wp, _ws, rs, t) = crate::gen::filter::cheb2ord([0.2, 0.4], [0.15, 0.45], 0.1, 60.0, FilterGenPlane::Z { sampling_frequency: None })
            .unwrap();
        let dx: Tf::<f64, _, _> = Tf::cheby2(n, rs, wp, t, FilterGenPlane::Z { sampling_frequency: None })
            .unwrap();
        
        let (n, wp, _ws, rp, t) = crate::gen::filter::cheb1ord([0.6, 0.8], [0.65, 0.75], 0.1, 60.0, FilterGenPlane::Z { sampling_frequency: None })
            .unwrap();
        let dy: Tf::<f64, _, _> = Tf::cheby1(n, rp, wp, t, FilterGenPlane::Z { sampling_frequency: None })
            .unwrap();

        let x = dx.as_view().filter(r.as_slice(), ());
        let y = dy.as_view().filter(r, ());

        let (cxy, fc): (Vec<_>, Vec<_>) = x.real_mscohere(y, (), 512, 500, 2048, (), (), (), ());

        const M: usize = 1024;
        let (qx, f): ([_; M], _) = dx.real_freqz(());
        let (qy, _): ([_; M], _) = dy.real_freqz(());

        plot::plot_curves("C_xy(e^jw)", "plots/cxy_z_cohere.png", [
            &fc.into_iter().zip(cxy).collect::<Vec<_>>(),
            &f.zip(qx.map(|qx| qx.norm())),
            &f.zip(qy.map(|qy| qy.norm()))
        ]).unwrap()
    }
}