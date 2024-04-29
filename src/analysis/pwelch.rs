use core::{iter::Sum, ops::{AddAssign, Div, MulAssign, SubAssign}};

use ndarray_linalg::Lapack;
use num::{complex::ComplexFloat, traits::FloatConst, Complex, Float, NumCast, Zero};
use option_trait::{Maybe, NotVoid, StaticMaybe};
use array_math::SliceMath;

use crate::{util::{self, MaybeLenEq}, windows::Hamming, gen::window::{WindowGen, WindowRange}, quantities::{List, MaybeList}};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum PWelchDetrend
{
    None,
    Mean,
    Linear,
    Detrend(usize),
    LongMean,
    LongLinear,
    LongDetrend(usize)
}

pub trait PWelch<T, Y, YY, W, WW, WWW, WL, N, S>: List<T> + MaybeLenEq<YY, true>
where
    T: ComplexFloat,
    W: ComplexFloat<Real = T::Real>,
    Y: ComplexFloat<Real = T::Real>,
    YY: MaybeList<Y, MaybeSome: StaticMaybe<YY::Some, MaybeOr<Y, T> = Y>>,
    WW: List<W>,
    WWW: Maybe<WW>,
    WL: Maybe<usize>,
    N: Maybe<usize>,
    S: Maybe<bool>,
    WW::Mapped<Complex<T::Real>>: StaticMaybe<WW::Mapped<Complex<T::Real>>>,
    WW::Mapped<T::Real>: StaticMaybe<WW::Mapped<T::Real>>
{
    fn pwelch<O, FS, CONF, DT, XPOW, CROSS, TRANS, COHER, YPOW, CONFF, F>(
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
    ) -> (XPOW, CROSS, TRANS, COHER, YPOW, CONFF, F)
    where
        O: Maybe<usize>,
        FS: Maybe<T::Real>,
        CONF: Maybe<T::Real>,
        DT: Maybe<PWelchDetrend>,
        XPOW: StaticMaybe<WW::Mapped<T::Real>>,
        CROSS: StaticMaybe<<YY::MaybeSome as StaticMaybe<YY::Some>>::Maybe<WW::Mapped<Complex<T::Real>>>>,
        TRANS: StaticMaybe<<YY::MaybeSome as StaticMaybe<YY::Some>>::Maybe<WW::Mapped<Complex<T::Real>>>>,
        COHER: StaticMaybe<<YY::MaybeSome as StaticMaybe<YY::Some>>::Maybe<WW::Mapped<T::Real>>>,
        YPOW: StaticMaybe<<YY::MaybeSome as StaticMaybe<YY::Some>>::Maybe<WW::Mapped<T::Real>>>,
        CONFF: StaticMaybe<[WW::Mapped<T::Real>; 2]>,
        F: StaticMaybe<WW::Mapped<T::Real>>;
}

impl<T, L, Y, YY, R, const WL: usize> PWelch<T, Y, YY, R, [R; WL], (), (), (), ()> for L
where
    L: List<T> + MaybeLenEq<YY, true> + PWelch<T, Y, YY, R, [R; WL], [R; WL], (), (), ()>,
    T: ComplexFloat<Real = R>,
    Y: ComplexFloat<Real = T::Real>,
    YY: MaybeList<Y, MaybeSome: StaticMaybe<YY::Some, MaybeOr<Y, T> = Y>>,
    R: Float + FloatConst + NotVoid
{
    fn pwelch<O, FS, CONF, DT, XPOW, CROSS, TRANS, COHER, YPOW, CONFF, F>(
        self,
        y: YY,
        (): (),
        (): (),
        overlap: O,
        (): (),
        sampling_frequency: FS,
        confidence: CONF,
        detrend: DT,
        (): (),
        shift: bool
    ) -> (XPOW, CROSS, TRANS, COHER, YPOW, CONFF, F)
    where
        O: Maybe<usize>,
        FS: Maybe<<T as ComplexFloat>::Real>,
        CONF: Maybe<<T as ComplexFloat>::Real>,
        DT: Maybe<PWelchDetrend>,
        XPOW: StaticMaybe<[R; WL]>,
        CROSS: StaticMaybe<<YY::MaybeSome as StaticMaybe<YY::Some>>::Maybe<[Complex<R>; WL]>>,
        TRANS: StaticMaybe<<YY::MaybeSome as StaticMaybe<YY::Some>>::Maybe<[Complex<R>; WL]>>,
        COHER: StaticMaybe<<YY::MaybeSome as StaticMaybe<YY::Some>>::Maybe<[R; WL]>>,
        YPOW: StaticMaybe<<YY::MaybeSome as StaticMaybe<YY::Some>>::Maybe<[R; WL]>>,
        CONFF: StaticMaybe<[[R; WL]; 2]>,
        F: StaticMaybe<[R; WL]>
    {
        let w = Hamming.window_gen((), WindowRange::Symmetric);

        self.pwelch(y, w, (), overlap, (), sampling_frequency, confidence, detrend, (), shift)
    }
}

impl<T, L, Y, YY, R, WL, N, S> PWelch<T, Y, YY, R, Vec<R>, (), WL, N, S> for L
where
    L: List<T> + MaybeLenEq<YY, true> + PWelch<T, Y, YY, R, Vec<R>, Vec<R>, (), (), ()>,
    T: ComplexFloat<Real = R>,
    Y: ComplexFloat<Real = T::Real>,
    YY: MaybeList<Y, MaybeSome: StaticMaybe<YY::Some, MaybeOr<Y, T> = Y>>,
    WL: Maybe<usize>,
    N: Maybe<usize>,
    S: Maybe<bool>,
    R: Float + FloatConst + NotVoid
{
    fn pwelch<O, FS, CONF, DT, XPOW, CROSS, TRANS, COHER, YPOW, CONFF, F>(
        self,
        y: YY,
        (): (),
        window_length: WL,
        overlap: O,
        nfft: N,
        sampling_frequency: FS,
        confidence: CONF,
        detrend: DT,
        sloppy: S,
        shift: bool
    ) -> (XPOW, CROSS, TRANS, COHER, YPOW, CONFF, F)
    where
        O: Maybe<usize>,
        FS: Maybe<<T as ComplexFloat>::Real>,
        CONF: Maybe<<T as ComplexFloat>::Real>,
        DT: Maybe<PWelchDetrend>,
        XPOW: StaticMaybe<Vec<R>>,
        CROSS: StaticMaybe<<YY::MaybeSome as StaticMaybe<YY::Some>>::Maybe<Vec<Complex<R>>>>,
        TRANS: StaticMaybe<<YY::MaybeSome as StaticMaybe<YY::Some>>::Maybe<Vec<Complex<R>>>>,
        COHER: StaticMaybe<<YY::MaybeSome as StaticMaybe<YY::Some>>::Maybe<Vec<R>>>,
        YPOW: StaticMaybe<<YY::MaybeSome as StaticMaybe<YY::Some>>::Maybe<Vec<R>>>,
        CONFF: StaticMaybe<[Vec<R>; 2]>,
        F: StaticMaybe<Vec<R>>
    {
        let window_length = window_length.into_option()
            .unwrap_or(256);
        let mut w = Hamming.window_gen(window_length, WindowRange::Symmetric);
        let mut wlen = nfft.into_option()
            .map(|nfft| nfft.max(window_length))
            .unwrap_or(window_length);
        if sloppy.into_option().unwrap_or(false)
        {
            wlen = wlen.next_power_of_two()   
        }
        if wlen > window_length
        {
            w.resize(wlen, R::zero())
        }

        self.pwelch(y, w, (), overlap, (), sampling_frequency, confidence, detrend, (), shift)
    }
}

impl<T, L, Y, YY, W, WW, R> PWelch<T, Y, YY, W, WW, WW, (), (), ()> for L
where
    L: List<T> + MaybeLenEq<YY, true>,
    T: ComplexFloat<Real = R> + Sum + SubAssign + Div<R, Output = T> + Into<Complex<R>> + Lapack,
    W: ComplexFloat<Real = R> + Into<Complex<R>>,
    Y: ComplexFloat<Real = R> + Into<Complex<R>> + Sum + SubAssign + Div<R, Output = Y> + Lapack,
    YY: MaybeList<Y, MaybeSome: StaticMaybe<YY::Some, MaybeOr<Y, T> = Y>>,
    WW: List<W>,
    R: Float + FloatConst + Sum + AddAssign + SubAssign,
    Complex<R>: AddAssign + MulAssign,
    WW::Mapped<Complex<R>>: StaticMaybe<WW::Mapped<Complex<R>>>,
    WW::Mapped<R>: StaticMaybe<WW::Mapped<R>>,
    <YY::MaybeSome as StaticMaybe<YY::Some>>::Maybe<WW::Mapped<Complex<R>>>: Sized,
    <YY::MaybeSome as StaticMaybe<YY::Some>>::Maybe<WW::Mapped<R>>: Sized
{
    fn pwelch<O, FS, CONF, DT, XPOW, CROSS, TRANS, COHER, YPOW, CONFF, F>(
        self,
        y: YY,
        window: WW,
        (): (),
        overlap: O,
        (): (),
        sampling_frequency: FS,
        confidence: CONF,
        detrend: DT,
        (): (),
        shift: bool
    ) -> (XPOW, CROSS, TRANS, COHER, YPOW, CONFF, F)
    where
        O: Maybe<usize>,
        FS: Maybe<R>,
        CONF: Maybe<R>,
        DT: Maybe<PWelchDetrend>,
        XPOW: StaticMaybe<WW::Mapped<R>>,
        CROSS: StaticMaybe<<YY::MaybeSome as StaticMaybe<YY::Some>>::Maybe<WW::Mapped<Complex<R>>>>,
        TRANS: StaticMaybe<<YY::MaybeSome as StaticMaybe<YY::Some>>::Maybe<WW::Mapped<Complex<R>>>>,
        COHER: StaticMaybe<<YY::MaybeSome as StaticMaybe<YY::Some>>::Maybe<WW::Mapped<R>>>,
        YPOW: StaticMaybe<<YY::MaybeSome as StaticMaybe<YY::Some>>::Maybe<WW::Mapped<R>>>,
        CONFF: StaticMaybe<[WW::Mapped<R>; 2]>,
        F: StaticMaybe<WW::Mapped<R>>
    {
        let zero = R::zero();
        let one = R::one();
        let two = one + one;

        let fs = sampling_frequency.as_option()
            .map(|&fs| fs)
            .unwrap_or(one);

        let conf = confidence.into_option()
            .map(|conf| conf.min(one).max(zero))
            .unwrap_or_else(|| NumCast::from(0.95).unwrap());

        let need_pxx = XPOW::IS_SOME || TRANS::IS_SOME || COHER::IS_SOME || CONFF::IS_SOME;
        let need_pxy = CROSS::IS_SOME || TRANS::IS_SOME || COHER::IS_SOME;
        let need_pyy = COHER::IS_SOME || YPOW::IS_SOME;

        let seg_len = window.length();
        let w = window.to_vec();
        // seg_len = 2 ^ ceil( log(sqrt(x_len/(1-overlap)))*nearly_one/log_two );

        let max_overlap = self.length()
            .saturating_sub(1);
        let overlap = overlap.into_option()
            .map(|o| o.min(seg_len - 1).min(max_overlap))
            .unwrap_or_else(|| seg_len/2);

        let nfft = seg_len;
        let win_meansq = window.to_vec().into_iter()
            .map(|x| (x.conj()*x).re())
            .sum::<R>()/NumCast::from(seg_len).unwrap();

        let mut x = self.to_vec();
        let mut y_ = y.to_vec_option();
        let mut len = x.len().max(seg_len);
        if let Some(y) = &mut y_
        {
            len = len.max(y.len());
            if y.len() < len
            {
                y.resize(len, Y::zero())
            }
        }
        if x.len() < len
        {
            x.resize(len, T::zero())
        }

        let detrend = detrend.into_option()
            .unwrap_or(PWelchDetrend::LongMean);
        match detrend
        {
            PWelchDetrend::LongMean | PWelchDetrend::LongLinear | PWelchDetrend::LongDetrend(_) => {
                let n_ffts = len.saturating_sub(seg_len)/(seg_len - overlap) + 1;
                len = len.min((seg_len - overlap)*(n_ffts - 1) + seg_len);
                
                match detrend
                {
                    PWelchDetrend::LongMean => {
                        if need_pxx || need_pxy
                        {
                            let x_avg = x[..len].iter()
                                .map(|&x| x)
                                .sum::<T>()/<R as NumCast>::from(len).unwrap();
                            for x in x.iter_mut()
                            {
                                *x -= x_avg
                            }
                        }
                        if let Some(y) = &mut y_
                        {
                            let y_avg = y[..len].iter()
                                .map(|&y| y)
                                .sum::<Y>()/<R as NumCast>::from(len).unwrap();
                            for y in y.iter_mut()
                            {
                                *y -= y_avg
                            }
                        }
                    },
                    PWelchDetrend::LongLinear => {
                        if need_pxx || need_pxy
                        {
                            x[..len].detrend(1);
                        }
                        if let Some(y) = &mut y_
                        {
                            y[..len].detrend(1);
                        }
                    },
                    PWelchDetrend::LongDetrend(d) => {
                        if need_pxx || need_pxy
                        {
                            x[..len].detrend(d);
                        }
                        if let Some(y) = &mut y_
                        {
                            y[..len].detrend(d);
                        }
                    },
                    _ => ()
                }
            },
            _ => ()
        }

        let mut xx = if need_pxx || need_pxy
        {
            Some(vec![Complex::<R>::zero(); nfft])
        }
        else
        {
            None
        };
        let mut yy = if y_.is_some() && (need_pxy || need_pyy)
        {
            Some(vec![Complex::<R>::zero(); nfft])
        }
        else
        {
            None
        };
        let mut pxx = if need_pxx
        {
            Some(vec![zero; nfft])
        }
        else
        {
            None
        };
        let mut pxy = if need_pxy
        {
            Some(vec![Complex::<R>::zero(); nfft])
        }
        else
        {
            None
        };
        let mut pyy = if need_pyy
        {
            Some(vec![zero; nfft])
        }
        else
        {
            None
        };
        let mut vxx = if need_pxx && !conf.is_zero()
        {
            Some(vec![zero; nfft])
        }
        else
        {
            None
        };
        let mut n_ffts = 0;

        for start_seg in (0..len + 1 - seg_len).step_by(seg_len - overlap)
        {
            let end_seg = start_seg + seg_len;
            if let Some(xx) = &mut xx
            {
                xx.fill(Complex::zero());
                match detrend
                {
                    PWelchDetrend::Mean => {
                        let x_avg = x[start_seg..end_seg].iter()
                            .map(|&x| x)
                            .sum::<T>()/<R as NumCast>::from(seg_len).unwrap();
                        for ((xx, &x), &w) in xx.iter_mut()
                            .zip(x[start_seg..end_seg].iter())
                            .zip(w.iter())
                        {
                            *xx = Into::<Complex<_>>::into(x - x_avg)*Into::<Complex<_>>::into(w)
                        }
                    },
                    PWelchDetrend::Linear => {
                        let mut xd = x[start_seg..end_seg].to_vec();
                        xd.detrend(1);
                        for ((xx, x), &w) in xx.iter_mut()
                            .zip(xd.into_iter())
                            .zip(w.iter())
                        {
                            *xx = Into::<Complex<_>>::into(x)*Into::<Complex<_>>::into(w)
                        }
                    },
                    PWelchDetrend::Detrend(d) => {
                        let mut xd = x[start_seg..end_seg].to_vec();
                        xd.detrend(d);
                        for ((xx, x), &w) in xx.iter_mut()
                            .zip(xd.into_iter())
                            .zip(w.iter())
                        {
                            *xx = Into::<Complex<_>>::into(x)*Into::<Complex<_>>::into(w)
                        }
                    },
                    _ => {
                        for ((xx, &x), &w) in xx.iter_mut()
                            .zip(x[start_seg..end_seg].iter())
                            .zip(w.iter())
                        {
                            *xx = Into::<Complex<_>>::into(x)*Into::<Complex<_>>::into(w)
                        }
                    }
                }
                xx.fft();
            }
            if let Some(yy) = &mut yy && let Some(y) = &y_
            {
                yy.fill(Complex::zero());
                match detrend
                {
                    PWelchDetrend::Mean => {
                        let y_avg = y[start_seg..end_seg].iter()
                            .map(|&y| y)
                            .sum::<Y>()/<R as NumCast>::from(seg_len).unwrap();
                        for ((yy, &y), &w) in yy.iter_mut()
                            .zip(y[start_seg..end_seg].iter())
                            .zip(w.iter())
                        {
                            *yy = Into::<Complex<_>>::into(y - y_avg)*Into::<Complex<_>>::into(w)
                        }
                    },
                    PWelchDetrend::Linear => {
                        let mut yd = y[start_seg..end_seg].to_vec();
                        yd.detrend(1);
                        for ((yy, y), &w) in yy.iter_mut()
                            .zip(yd.into_iter())
                            .zip(w.iter())
                        {
                            *yy = Into::<Complex<_>>::into(y)*Into::<Complex<_>>::into(w)
                        }
                    },
                    PWelchDetrend::Detrend(d) => {
                        let mut yd = y[start_seg..end_seg].to_vec();
                        yd.detrend(d);
                        for ((yy, y), &w) in yy.iter_mut()
                            .zip(yd.into_iter())
                            .zip(w.iter())
                        {
                            *yy = Into::<Complex<_>>::into(y)*Into::<Complex<_>>::into(w)
                        }
                    },
                    _ => {
                        for ((yy, &y), &w) in yy.iter_mut()
                            .zip(y[start_seg..end_seg].iter())
                            .zip(w.iter())
                        {
                            *yy = Into::<Complex<_>>::into(y)*Into::<Complex<_>>::into(w)
                        }
                    }
                }
                yy.fft();
            }
            if let Some(xx) = &xx && let Some(pxx) = &mut pxx
            {
                let pgram: Vec<_> = xx.iter()
                    .map(|&xx| (xx.conj()*xx).re)
                    .collect();
                if let Some(vxx) = &mut vxx
                {
                    for (vxx, &p) in vxx.iter_mut()
                        .zip(pgram.iter())
                    {
                        *vxx += p*p
                    }
                }
                for (pxx, p) in pxx.iter_mut()
                    .zip(pgram)
                {
                    *pxx += p
                }
            }
            if let Some(xx) = &xx && let Some(yy) = &yy && let Some(pxy) = &mut pxy
            {
                let pgram = xx.iter()
                    .zip(yy.iter())
                    .map(|(&xx, &yy)| xx.conj()*yy);
                for (pxy, p) in pxy.iter_mut()
                    .zip(pgram)
                {
                    *pxy += p
                }
            }
            if let Some(yy) = &yy && let Some(pyy) = &mut pyy
            {
                let pgram = yy.iter()
                    .map(|&yy| (yy.conj()*yy).re);
                for (pyy, p) in pyy.iter_mut()
                    .zip(pgram)
                {
                    *pyy += p
                }
            }
            n_ffts += 1
        }

        if let Some(pxx) = &pxx && let Some(vxx) = &mut vxx
        {
            if n_ffts < 2
            {
                vxx.fill(Zero::zero());
            }
            else
            {
                // Student-T distribution inverse?
                let a = util::erf_inv(conf)*(R::from(2*n_ffts).unwrap()/R::from(n_ffts - 1).unwrap()).sqrt();
                for (vxx, &pxx) in vxx.iter_mut()
                    .zip(pxx.iter())
                {
                    *vxx = a*(*vxx - pxx*pxx/R::from(n_ffts).unwrap()).sqrt()
                }
            }
        }

        if shift
        {
            if let Some(pxx) = &mut pxx
            {
                pxx.rotate_right(nfft/2)
            }
            if let Some(pxy) = &mut pxy
            {
                pxy.rotate_right(nfft/2)
            }
            if let Some(pyy) = &mut pyy
            {
                pyy.rotate_right(nfft/2)
            }
            if let Some(vxx) = &mut vxx
            {
                vxx.rotate_right(nfft/2)
            }
        }
        
        let scale = R::from(n_ffts*seg_len).unwrap()*(fs*win_meansq);

        (
            XPOW::maybe_from_fn(|| {
                let mut pxx = pxx.as_ref()
                    .unwrap()
                    .iter()
                    .map(|&pxx| pxx/scale);
                window.map_to_owned(|_| pxx.next().unwrap())
            }),
            CROSS::maybe_from_fn(|| StaticMaybe::maybe_from_fn(|| {
                let mut pxy = pxy.as_ref()
                    .unwrap()
                    .iter()
                    .map(|&pxy| pxy/scale);
                window.map_to_owned(|_| pxy.next().unwrap())
            })),
            TRANS::maybe_from_fn(|| StaticMaybe::maybe_from_fn(|| {
                let mut trans = pxy.as_ref()
                    .unwrap()
                    .iter()
                    .zip(pxx.as_ref()
                        .unwrap()
                        .iter()
                    ).map(|(&pxy, &pxx)| pxy/pxx);
                window.map_to_owned(|_| trans.next().unwrap())
            })),
            COHER::maybe_from_fn(|| StaticMaybe::maybe_from_fn(|| {
                let mut coher = pxy.as_ref()
                    .unwrap()
                    .iter()
                    .zip(pxx.as_ref()
                        .unwrap()
                        .iter()
                    ).zip(pyy.as_ref()
                        .unwrap()
                        .iter()
                    ).map(|((&pxy, &pxx), &pyy)| (pxy.conj()*pxy).re/pxx/pyy);
                window.map_to_owned(|_| coher.next().unwrap())
            })),
            YPOW::maybe_from_fn(|| StaticMaybe::maybe_from_fn(|| {
                let mut pyy = pyy.as_ref()
                    .unwrap()
                    .iter()
                    .map(|&pyy| pyy/scale);
                window.map_to_owned(|_| pyy.next().unwrap())
            })),
            CONFF::maybe_from_fn(|| {
                let pxx = pxx.as_ref().unwrap();
                let vxx = vxx.unwrap();
                let mut vxx1 = pxx.iter()
                    .zip(vxx.iter())
                    .map(|(&p, &v)| p - v);
                let mut vxx2 = pxx.iter()
                    .zip(vxx.iter())
                    .map(|(&p, &v)| p + v);
                [
                    window.map_to_owned(|_| vxx1.next().unwrap()),
                    window.map_to_owned(|_| vxx2.next().unwrap())
                ]
            }),
            F::maybe_from_fn(|| {
                let mut i = 0;
                let nfftf = R::from(nfft).unwrap();
                let tau = sampling_frequency.into_option()
                    .unwrap_or(R::TAU());
                window.map_into_owned(|_| {
                    let mut i_f = R::from(i).unwrap();
                    if shift
                    {
                        i_f -= nfftf/two
                    }
                    i += 1;
                    i_f/nfftf*tau
                })
            })
        )
    }
}

#[cfg(test)]
mod test
{
    use core::f64::consts::PI;

    use rand::distributions::uniform::SampleRange;

    use crate::{plot, analysis::RealPWelch};

    #[test]
    fn test()
    {
        const N: usize = 2048;

        let n: [_; N] = core::array::from_fn(|i| i as f64);

        let mut rng = rand::thread_rng();
        let x = n.map(|n| (PI/4.0*n).cos() + (-1.0..1.0).sample_single(&mut rng));

        let (pxx, (), (), (), (), (), f) = x.real_pwelch((), (), 71, (), 256, (), (), (), ());
        let pxx: Vec<_> = pxx;
        let f: Vec<_> = f;

        plot::plot_curves("Pxx(e^jw)", "plots/pxx_z_pwelch.png", [
                &f.into_iter().zip(pxx.into_iter().map(|p| 10.0*p.log10())).collect::<Vec<_>>()
            ]).unwrap()
    }
}