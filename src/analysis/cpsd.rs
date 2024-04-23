use num::{complex::ComplexFloat, Complex};
use option_trait::{Maybe, NotVoid, StaticMaybe};

use crate::{List, MaybeLenEq, PWelch, PWelchDetrend};

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

    use crate::{plot, window::{Boxcar, Triangular, WindowGen, WindowRange}, Filter, Fir1, Fir1Type, RealCPsd, Tf};

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
        let nov = 250;

        let (pxy, f): (_, [_; M/2 + 1]) = x.real_cpsd(y, w, (), nov, (), (), (), (), ());

        plot::plot_curves("P_xy(e^jw)", "plots/pxy_z_cpsd.png", [
            &f.zip(pxy.map(|p| 10.0*p.norm().log10()))
        ]).unwrap()
    }
}