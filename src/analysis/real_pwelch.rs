use num::{traits::FloatConst, Complex, Float, Zero};
use option_trait::{Maybe, StaticMaybe};

use crate::{quantities::{List, Lists, MaybeList, ListOrSingle}, util::MaybeLenEq, analysis::{PWelch, PWelchDetrend}};

pub trait RealPWelch<T, YY, WW, WWW, WL, N, S>: Lists<T> + MaybeLenEq<YY, true>
where
    T: Float + FloatConst,
    YY: MaybeList<T>,
    WW: List<T>,
    WWW: Maybe<WW>,
    WL: Maybe<usize>,
    N: Maybe<usize>,
    S: Maybe<bool>,
    WW::Mapped<T>: List<T>,
    WW::Mapped<Complex<T>>: List<Complex<T>>,
    <YY::MaybeSome as StaticMaybe<YY::Some>>::Maybe<WW::Mapped<T>>: StaticMaybe<<YY::MaybeSome as StaticMaybe<YY::Some>>::Maybe<WW::Mapped<T>>>,
    <YY::MaybeSome as StaticMaybe<YY::Some>>::Maybe<WW::Mapped<Complex<T>>>: StaticMaybe<<YY::MaybeSome as StaticMaybe<YY::Some>>::Maybe<WW::Mapped<Complex<T>>>>,
{
    #[doc(alias = "real_welch")]
    fn real_pwelch<O, FS, CONF, DT, XPOW, CROSS, TRANS, COHER, YPOW, CONFF, F>(
        self,
        y: YY,
        window: WWW,
        window_length: WL,
        overlap: O,
        nfft: N,
        sampling_frequency: FS,
        confidence: CONF,
        detrend: DT,
        sloppy: S
    ) -> (XPOW, CROSS, TRANS, COHER, YPOW, CONFF, F)
    where
        O: Maybe<usize>,
        FS: Maybe<T>,
        CONF: Maybe<T>,
        DT: Maybe<PWelchDetrend>,
        XPOW: StaticMaybe<<WW::Mapped<T> as List<T>>::ResizedList<{WW::WIDTH/2 + 1}>>,
        CROSS: StaticMaybe<<YY::MaybeSome as StaticMaybe<YY::Some>>::Maybe<<WW::Mapped<Complex<T>> as List<Complex<T>>>::ResizedList<{WW::WIDTH/2 + 1}>>>,
        TRANS: StaticMaybe<<YY::MaybeSome as StaticMaybe<YY::Some>>::Maybe<<WW::Mapped<Complex<T>> as List<Complex<T>>>::ResizedList<{WW::WIDTH/2 + 1}>>>,
        COHER: StaticMaybe<<YY::MaybeSome as StaticMaybe<YY::Some>>::Maybe<<WW::Mapped<T> as List<T>>::ResizedList<{WW::WIDTH/2 + 1}>>>,
        YPOW: StaticMaybe<<YY::MaybeSome as StaticMaybe<YY::Some>>::Maybe<<WW::Mapped<T> as List<T>>::ResizedList<{WW::WIDTH/2 + 1}>>>,
        CONFF: StaticMaybe<[<WW::Mapped<T> as List<T>>::ResizedList<{WW::WIDTH/2 + 1}>; 2]>,
        F: StaticMaybe<<WW::Mapped<T> as List<T>>::ResizedList<{WW::WIDTH/2 + 1}>>,
        XPOW::Maybe<WW::Mapped<T>>: Sized,
        CROSS::Maybe<<YY::MaybeSome as StaticMaybe<YY::Some>>::Maybe<WW::Mapped<Complex<T>>>>: Sized,
        TRANS::Maybe<<YY::MaybeSome as StaticMaybe<YY::Some>>::Maybe<WW::Mapped<Complex<T>>>>: Sized,
        COHER::Maybe<<YY::MaybeSome as StaticMaybe<YY::Some>>::Maybe<WW::Mapped<T>>>: Sized,
        YPOW::Maybe<<YY::MaybeSome as StaticMaybe<YY::Some>>::Maybe<WW::Mapped<T>>>: Sized,
        CONFF::Maybe<[WW::Mapped<T>; 2]>: Sized,
        F::Maybe<WW::Mapped<T>>: Sized;
}

impl<T, L, YY, WW, WWW, WL, N, S> RealPWelch<T, YY, WW, WWW, WL, N, S> for L
where
    L: Lists<T> + MaybeLenEq<YY, true>,
    T: Float + FloatConst,
    YY: MaybeList<T, MaybeSome: StaticMaybe<YY::Some, MaybeOr<T, T> = T>>,
    WW: List<T>,
    WWW: Maybe<WW>,
    WL: Maybe<usize>,
    N: Maybe<usize>,
    S: Maybe<bool>,
    WW::Mapped<T>: List<T>,
    WW::Mapped<Complex<T>>: List<Complex<T>>,
    L: PWelch<T, T, YY, T, WW, WWW, WL, N, S>,
    <YY::MaybeSome as StaticMaybe<YY::Some>>::Maybe<WW::Mapped<T>>: StaticMaybe<WW::Mapped<T>> + StaticMaybe<<YY::MaybeSome as StaticMaybe<YY::Some>>::Maybe<WW::Mapped<T>>> + Sized,
    <YY::MaybeSome as StaticMaybe<YY::Some>>::Maybe<WW::Mapped<Complex<T>>>: StaticMaybe<WW::Mapped<Complex<T>>> + StaticMaybe<<YY::MaybeSome as StaticMaybe<YY::Some>>::Maybe<WW::Mapped<Complex<T>>>> + Sized,
    <YY::MaybeSome as StaticMaybe<YY::Some>>::Maybe<<WW::Mapped<T> as List<T>>::ResizedList<{WW::WIDTH/2 + 1}>>: Sized,
    <YY::MaybeSome as StaticMaybe<YY::Some>>::Maybe<<WW::Mapped<Complex<T>> as List<Complex<T>>>::ResizedList<{WW::WIDTH/2 + 1}>>: Sized
{
    fn real_pwelch<O, FS, CONF, DT, XPOW, CROSS, TRANS, COHER, YPOW, CONFF, F>(
        self,
        y: YY,
        window: WWW,
        window_length: WL,
        overlap: O,
        nfft: N,
        sampling_frequency: FS,
        confidence: CONF,
        detrend: DT,
        sloppy: S
    ) -> (XPOW, CROSS, TRANS, COHER, YPOW, CONFF, F)
    where
        O: Maybe<usize>,
        FS: Maybe<T>,
        CONF: Maybe<T>,
        DT: Maybe<PWelchDetrend>,
        XPOW: StaticMaybe<<WW::Mapped<T> as List<T>>::ResizedList<{WW::WIDTH/2 + 1}>>,
        CROSS: StaticMaybe<<YY::MaybeSome as StaticMaybe<YY::Some>>::Maybe<<WW::Mapped<Complex<T>> as List<Complex<T>>>::ResizedList<{WW::WIDTH/2 + 1}>>>,
        TRANS: StaticMaybe<<YY::MaybeSome as StaticMaybe<YY::Some>>::Maybe<<WW::Mapped<Complex<T>> as List<Complex<T>>>::ResizedList<{WW::WIDTH/2 + 1}>>>,
        COHER: StaticMaybe<<YY::MaybeSome as StaticMaybe<YY::Some>>::Maybe<<WW::Mapped<T> as List<T>>::ResizedList<{WW::WIDTH/2 + 1}>>>,
        YPOW: StaticMaybe<<YY::MaybeSome as StaticMaybe<YY::Some>>::Maybe<<WW::Mapped<T> as List<T>>::ResizedList<{WW::WIDTH/2 + 1}>>>,
        CONFF: StaticMaybe<[<WW::Mapped<T> as List<T>>::ResizedList<{WW::WIDTH/2 + 1}>; 2]>,
        F: StaticMaybe<<WW::Mapped<T> as List<T>>::ResizedList<{WW::WIDTH/2 + 1}>>,
        XPOW::Maybe<WW::Mapped<T>>: Sized,
        CROSS::Maybe<<YY::MaybeSome as StaticMaybe<YY::Some>>::Maybe<WW::Mapped<Complex<T>>>>: Sized,
        TRANS::Maybe<<YY::MaybeSome as StaticMaybe<YY::Some>>::Maybe<WW::Mapped<Complex<T>>>>: Sized,
        COHER::Maybe<<YY::MaybeSome as StaticMaybe<YY::Some>>::Maybe<WW::Mapped<T>>>: Sized,
        YPOW::Maybe<<YY::MaybeSome as StaticMaybe<YY::Some>>::Maybe<WW::Mapped<T>>>: Sized,
        CONFF::Maybe<[WW::Mapped<T>; 2]>: Sized,
        F::Maybe<WW::Mapped<T>>: Sized
    {
        let (pxx, cross, trans, coher, ypp, conf, f) = self.pwelch::<O, FS, CONF, DT, XPOW::Maybe<WW::Mapped<T>>, CROSS::Maybe<<YY::MaybeSome as StaticMaybe<YY::Some>>::Maybe<WW::Mapped<Complex<T>>>>, TRANS::Maybe<<YY::MaybeSome as StaticMaybe<YY::Some>>::Maybe<WW::Mapped<Complex<T>>>>, COHER::Maybe<<YY::MaybeSome as StaticMaybe<YY::Some>>::Maybe<WW::Mapped<T>>>, YPOW::Maybe<<YY::MaybeSome as StaticMaybe<YY::Some>>::Maybe<WW::Mapped<T>>>, CONFF::Maybe<[WW::Mapped<T>; 2]>, F::Maybe<WW::Mapped<T>>>(
            y, window, window_length, overlap, nfft, sampling_frequency, confidence, detrend, sloppy, false
        );

        (
            StaticMaybe::maybe_from_fn(|| {
                let pxx = pxx.into_option().unwrap();
                let l = pxx.length();
                pxx.static_resize_list::<{WW::WIDTH/2 + 1}>(l/2 + 1, Zero::zero)
            }),
            StaticMaybe::maybe_from_fn(|| StaticMaybe::maybe_from_fn(|| {
                let cross: WW::Mapped<Complex<T>> = cross.into_option().unwrap().into_option().unwrap();
                let l = cross.length();
                cross.static_resize_list::<{WW::WIDTH/2 + 1}>(l/2 + 1, Zero::zero)
            })),
            StaticMaybe::maybe_from_fn(|| StaticMaybe::maybe_from_fn(|| {
                let trans: WW::Mapped<Complex<T>> = trans.into_option().unwrap().into_option().unwrap();
                let l = trans.length();
                trans.static_resize_list::<{WW::WIDTH/2 + 1}>(l.length()/2 + 1, Zero::zero)
            })),
            StaticMaybe::maybe_from_fn(|| StaticMaybe::maybe_from_fn(|| {
                let coher: WW::Mapped<T> = coher.into_option().unwrap().into_option().unwrap();
                let l = coher.length();
                coher.static_resize_list::<{WW::WIDTH/2 + 1}>(l/2 + 1, Zero::zero)
            })),
            StaticMaybe::maybe_from_fn(|| StaticMaybe::maybe_from_fn(|| {
                let ypp: WW::Mapped<T> = ypp.into_option().unwrap().into_option().unwrap();
                let l = ypp.length();
                ypp.static_resize_list::<{WW::WIDTH/2 + 1}>(l/2 + 1, Zero::zero)
            })),
            StaticMaybe::maybe_from_fn(|| {
                let conf = conf.into_option().unwrap();
                conf.map(|conf| {
                    let l = conf.length();
                    conf.static_resize_list::<{WW::WIDTH/2 + 1}>(l/2 + 1, Zero::zero)
                })
            }),
            StaticMaybe::maybe_from_fn(|| {
                let f = f.into_option().unwrap();
                let l = f.length();
                f.static_resize_list::<{WW::WIDTH/2 + 1}>(l/2 + 1, Zero::zero)
            })
        )
    }
}