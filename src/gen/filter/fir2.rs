use core::ops::{AddAssign, Div, Mul, MulAssign, SubAssign};

use array_math::{SliceOps, SliceMath};
use num::{complex::ComplexFloat, traits::float::TotalOrder, Complex, Float, NumCast, One, Zero};
use option_trait::Maybe;

use crate::{windows::Hamming, gen::{filter::FilterGenError, window::{WindowGen, WindowRange}}, quantities::{List, MaybeList, Polynomial}, System, systems::Tf, util::TruncateIm};

pub trait Fir2<O, F, M, W, WW = (), const WWW: bool = false>: System + Sized
where
    O: Maybe<usize>,
    WW: MaybeList<W>,
    F: List<<Self::Set as ComplexFloat>::Real>,
    M: List<Self::Set, Length = F::Length>,
    W: ComplexFloat<Real = <Self::Set as ComplexFloat>::Real> + Into<Self::Set>
{
    fn fir2<NPT, LAP, FS>(
        order: O,
        frequencies: F,
        magnitudes: M,
        npt: NPT,
        lap: LAP,
        window: WW,
        sampling_frequency: FS
    ) -> Result<Self, FilterGenError>
    where
        NPT: Maybe<usize>,
        LAP: Maybe<usize>,
        FS: Maybe<<Self::Set as ComplexFloat>::Real>;
}

impl<T, F, M, W, WW>  Fir2<(), F, M, W, WW, true> for Tf<T, WW::Mapped<T>, ()>
where
    T: ComplexFloat<Real: TotalOrder + MulAssign + AddAssign + SubAssign> + Div<T::Real, Output = T> + Mul<T::Real, Output = T> + Into<Complex<T::Real>> + 'static,
    Complex<T::Real>: AddAssign + MulAssign + MulAssign<T::Real>,
    T::Real: Into<T>,
    F: List<T::Real>,
    M: List<T, Length = F::Length>,
    W: ComplexFloat<Real = T::Real> + Into<T>,
    WW: List<W>,
    WW::Mapped<T>: List<T>,
    [(); F::LENGTH - M::LENGTH]:,
    [(); M::LENGTH - F::LENGTH]:,
    [(); F::LENGTH - 1]:
{
    fn fir2<NPT, LAP, FS>(
        (): (),
        frequencies: F,
        magnitudes: M,
        npt: NPT,
        lap: LAP,
        window: WW,
        sampling_frequency: FS
    ) -> Result<Self, FilterGenError>
    where
        NPT: Maybe<usize>,
        LAP: Maybe<usize>,
        FS: Maybe<T::Real>
    {
        let w: &[W] = window.as_view_slice();
        let n = w.len().saturating_sub(1);

        let mut grid_n = npt.into_option()
            .unwrap_or(512.max(n + 1));
        let ramp_n = lap.into_option()
            .unwrap_or(grid_n/25);

        let f = frequencies.as_view_slice();
        let m = magnitudes.as_view_slice();
        if f.len() != m.len()
        {
            return Err(FilterGenError::FrequenciesAndMagnitudesDifferentLength)
        }
        let mut fm: Vec<(T::Real, T)> = f.iter()
            .zip(m.iter())
            .map(|(&f, &m)| (f, m))
            .collect();
        fm.sort_by(|a, b| a.0.total_cmp(&b.0));
        let mut f: Vec<_> = fm.iter()
            .map(|(f, _)| *f)
            .collect();
        let mut m: Vec<_> = fm.into_iter()
            .map(|(_, m)| m)
            .collect();
        
        let zero = T::Real::zero();
        let one = T::Real::one();
        let two = one + one;
        if let Some(fs) = sampling_frequency.into_option()
        {
            for wc in f.iter_mut()
            {
                if *wc > fs/two
                {
                    return Err(FilterGenError::FrequenciesOutOfRange)
                }
                *wc *= two/fs;
            }
        }
        if f.iter()
            .any(|f| *f < zero || *f > one)
        {
            return Err(FilterGenError::FrequenciesOutOfRange)
        }

        grid_n = grid_n.max((n + 1)/2).next_power_of_two();

        if ramp_n > 0
        {
            let basef = f.clone();
            let basem = m.clone();

            let mut df = f.clone();
            df.differentiate();

            let idx: Vec<_> = df[1..].iter()
                .enumerate()
                .filter_map(|(i, &df)| if df.is_zero() {Some(i)} else {None})
                .collect();
            let ramp_offset = <T::Real as NumCast>::from(ramp_n).unwrap()/NumCast::from(grid_n).unwrap()/two;
            for &idx in idx.iter()
            {
                f[idx] -= ramp_offset;
                f[idx + 1] += ramp_offset;
                f.push(basef[idx]);
                m.push((basem[idx] + basem[idx + 1])/two)
            }
            for f in f.iter_mut()
            {
                *f = (*f).max(zero).min(one)
            }
            
            let mut fm: Vec<(T::Real, T)> = frequencies.as_view_slice()
                .iter()
                .zip(magnitudes.as_view_slice()
                    .iter()
                ).map(|(&f, &m)| (f, m))
                .collect();
            fm.sort_by(|a, b| a.0.total_cmp(&b.0));
            f = fm.iter()
                .map(|(f, _)| *f)
                .collect();
            m = fm.into_iter()
                .map(|(_, m)| m)
                .collect();
        }

        let nbands = f.len();
        let mut grid = vec![T::zero(); grid_n];
        let grid_nf = <T::Real as NumCast>::from(grid_n - 1).unwrap();
        // Interpolate
        for i in 0..=nbands
        {
            let f0 = if i == 0 {zero} else {f[i - 1]};
            let f1 = if i == nbands {one} else {f[i]};
            let m0 = if i == 0 {m[0]} else {m[i - 1]};
            let m1 = if i == nbands {m[nbands - 1]} else {m[i]};

            let k0 = f0*grid_nf;
            let k1 = f1*grid_nf;
            let k0i: usize = NumCast::from(k0.ceil()).unwrap();
            let k1i: usize = NumCast::from(k1.floor()).unwrap();

            for (k, g) in grid.iter_mut().enumerate().take(k1i + 1).skip(k0i)
            {
                let m = if !(k1 - k0).is_zero()
                {
                    (m1 - m0)*(<T::Real as NumCast>::from(k).unwrap() - k0)/(k1 - k0) + m0
                }
                else
                {
                    m0
                };
                *g = m;
            }
        }

        /*let gridf: Vec<_> = grid.iter()
            .map(|&m| <f32 as NumCast>::from(m).unwrap())
            .collect();
        println!("{:?}", gridf);*/
        
        let gridc: Vec<_> = grid.into_iter()
            .map(|m| Into::<Complex<T::Real>>::into(m))
            .collect();
        let mut gridcrev: Vec<_> = gridc.iter()
            .rev().copied()
            .collect();
        gridcrev.pop();

        let b: Vec<_> = if n % 2 == 0
        {
            let mut b: Vec<_> = gridc.into_iter()
                .chain(core::iter::once(gridcrev[0]))
                .chain(gridcrev)
                .collect();
            b.ifft();

            let mid = (n as f32 + 1.0)/2.0;
            b[b.len() - mid.floor() as usize..].iter()
                .chain(b[..mid.ceil() as usize].iter())
                .map(|&b| b.truncate_im::<T>())
                .collect()
        }
        else
        {
            let mut b: Vec<_> = gridc.into_iter()
                .chain(vec![Zero::zero(); grid_n*2 + 1])
                .chain(gridcrev)
                .collect();
            b.ifft();

            b[b.len() - n..].iter()
                .step_by(2)
                .chain(b[1..n + 1].iter()
                    .step_by(2)
                ).map(|&b| b.truncate_im::<T>()*two)
                .collect()
        };

        let mut b = b.into_iter();
        let bw = window.map_into_owned(|w: W| b.next()
            .unwrap_or(zero.into())
            *w.into()
        );

        Ok(Tf {
            b: Polynomial::new(bw),
            a: Polynomial::new(())
        })
    }
}

impl<T, F, M> Fir2<usize, F, M, T::Real, (), false> for Tf<T, Vec<T>, ()>
where
    T: ComplexFloat,
    T::Real: Into<T>,
    F: List<T::Real>,
    M: List<T, Length = F::Length>,
    Self: Fir2<(), F, M, T::Real, Vec<T::Real>, true> + System<Set = T>,
    [(); F::LENGTH - M::LENGTH]:,
    [(); M::LENGTH - F::LENGTH]:,
    [(); F::LENGTH - 1]:
{
    fn fir2<NPT, LAP, FS>(
        order: usize,
        frequencies: F,
        magnitudes: M,
        npt: NPT,
        lap: LAP,
        (): (),
        sampling_frequency: FS
    ) -> Result<Self, FilterGenError>
    where
        NPT: Maybe<usize>,
        LAP: Maybe<usize>,
        FS: Maybe<<Self::Set as ComplexFloat>::Real>
    {
        Self::fir2(
            (),
            frequencies,
            magnitudes,
            npt,
            lap,
            Hamming.window_gen(order, WindowRange::Symmetric),
            sampling_frequency
        )
    }
}

impl<T, F, M, const N: usize> Fir2<(), F, M, T::Real, (), false> for Tf<T, [T; N], ()>
where
    T: ComplexFloat,
    T::Real: Into<T>,
    F: List<T::Real>,
    M: List<T, Length = F::Length>,
    Self: Fir2<(), F, M, T::Real, [T::Real; N], true> + System<Set = T>,
    [(); F::LENGTH - M::LENGTH]:,
    [(); M::LENGTH - F::LENGTH]:,
    [(); F::LENGTH - 1]:
{
    fn fir2<NPT, LAP, FS>(
        (): (),
        frequencies: F,
        magnitudes: M,
        npt: NPT,
        lap: LAP,
        (): (),
        sampling_frequency: FS
    ) -> Result<Self, FilterGenError>
    where
        NPT: Maybe<usize>,
        LAP: Maybe<usize>,
        FS: Maybe<<Self::Set as ComplexFloat>::Real>
    {
        Self::fir2(
            (),
            frequencies,
            magnitudes,
            npt,
            lap,
            Hamming.window_gen((), WindowRange::Symmetric),
            sampling_frequency
        )
    }
}

#[cfg(test)]
mod test
{
    use core::f64::consts::{PI, TAU};

    use array_math::ArrayOps;
    
    use crate::{plot, gen::filter::Fir2, analysis::FreqZ, Plane, systems::Tf, transforms::system::ToZpk, systems::Zpk};

    #[test]
    fn test()
    {
        let f = [0.1, 0.2, 0.3, 0.6, 0.8];
        let m = [1.0, 1.5, 0.0, 0.0, 1.0];

        const N: usize = 63;
        //let w: [f32; N] = Hamming.window_gen((), WindowRange::Symmetric);
        let h: Tf<_, [_; N], _> = Tf::fir2((), f, m, (), (), (), ())
            .unwrap();

        const M: usize = 1024;
        let (h_f, w): ([_; M], _) = h.freqz((), false);

        let wg = f.map(|f| f*PI);
        let mut wg_rev = wg.map(|f| TAU - f);
        wg_rev.reverse();
        let wg = [0.0].chain(wg).chain(wg_rev).chain([TAU]);
        let mut m_rev = m;
        m_rev.reverse();
        let g = [m[0]].chain(m).chain(m_rev).chain([m[0]]);

        plot::plot_curves("H(e^jw)", "plots/h_z_fir2.png", [&w.zip(h_f.map(|h_f| h_f.norm())), &wg.zip(g)])
            .unwrap();

        let h: Zpk<_, Vec<_>, Vec<_>, _> = h.to_zpk((), ());

        plot::plot_pz("H(z)", "plots/pz_z_fir2.png", &h.p, &h.z, Plane::Z)
            .unwrap();
    }
}