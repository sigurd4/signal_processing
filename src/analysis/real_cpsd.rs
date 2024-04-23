use num::{traits::FloatConst, Complex, Float, Zero};
use option_trait::{Maybe, StaticMaybe};

use crate::{CPsd, List, ListOrSingle, MaybeLenEq, PWelchDetrend};

pub trait RealCPsd<T, YY, WW, WWW, WL, N, S>: List<T> + MaybeLenEq<YY, true>
where
    T: Float + FloatConst,
    YY: List<T>,
    WW: List<T>,
    WWW: Maybe<WW>,
    WL: Maybe<usize>,
    N: Maybe<usize>,
    S: Maybe<bool>,
    WW::Mapped<Complex<T>>: List<Complex<T>>,
    WW::Mapped<T>: List<T>
{
    #[doc(alias = "real_csd")]
    fn real_cpsd<O, FS, CONF, DT, F>(
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
    ) -> (<WW::Mapped<Complex<T>> as List<Complex<T>>>::ResizedList<{WW::WIDTH/2 + 1}>, F)
    where
        O: Maybe<usize>,
        FS: Maybe<T>,
        CONF: Maybe<T>,
        DT: Maybe<PWelchDetrend>,
        F: StaticMaybe<<WW::Mapped<T> as List<T>>::ResizedList<{WW::WIDTH/2 + 1}>>,
        F::Maybe<WW::Mapped<T>>: Sized;
}

impl<T, L, YY, WW, WWW, WL, N, S> RealCPsd<T, YY, WW, WWW, WL, N, S> for L
where
    L: List<T> + MaybeLenEq<YY, true>,
    T: Float + FloatConst,
    YY: List<T>,
    WW: List<T>,
    WWW: Maybe<WW>,
    WL: Maybe<usize>,
    N: Maybe<usize>,
    S: Maybe<bool>,
    WW::Mapped<Complex<T>>: List<Complex<T>>,
    WW::Mapped<T>: List<T>,
    Self: CPsd<T, T, YY, T, WW, WWW, WL, N, S>
{
    fn real_cpsd<O, FS, CONF, DT, F>(
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
    ) -> (<WW::Mapped<Complex<T>> as List<Complex<T>>>::ResizedList<{WW::WIDTH/2 + 1}>, F)
    where
        O: Maybe<usize>,
        FS: Maybe<T>,
        CONF: Maybe<T>,
        DT: Maybe<PWelchDetrend>,
        F: StaticMaybe<<WW::Mapped<T> as List<T>>::ResizedList<{WW::WIDTH/2 + 1}>>,
        F::Maybe<WW::Mapped<T>>: Sized
    {
        let (cross, f) = self.cpsd::<_, _, _, _, F::Maybe<WW::Mapped<T>>>(
            y, window, window_length, overlap, nfft, sampling_frequency, confidence, detrend, sloppy, false
        );

        (
            {
                let l = cross.length();
                cross.static_resize_list::<{WW::WIDTH/2 + 1}>(l/2 + 1, Zero::zero)
            },
            StaticMaybe::maybe_from_fn(|| {
                let f = f.into_option().unwrap();
                let l = f.length();
                f.static_resize_list::<{WW::WIDTH/2 + 1}>(l/2 + 1, Zero::zero)
            })
        )
    }
}