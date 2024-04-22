use core::ops::{AddAssign, Mul, MulAssign};

use num::{complex::ComplexFloat, traits::FloatConst, Complex, One, Zero};
use option_trait::Maybe;
use array_math::SliceMath;

use crate::{window::{Hamming, WindowGen, WindowRange}, ContainerOrSingle, FilterGenError, Fir2, List, ListOrSingle, MaybeList, Polynomial, System, Tf};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Fir1Type
{
    LowPass,
    HighPass,
    BandPass,
    BandStop,
    DC0,
    DC1
}

pub trait Fir1<O, F, W, WW = (), const WWW: bool = false>: System + Sized
where
    O: Maybe<usize>,
    WW: MaybeList<W>,
    F: List<<Self::Domain as ComplexFloat>::Real>,
    W: ComplexFloat<Real = <Self::Domain as ComplexFloat>::Real> + Into<Self::Domain>
{
    fn fir1<FS>(
        order: O,
        frequencies: F,
        filter_type: Fir1Type,
        window: WW,
        scale: bool,
        sampling_frequency: FS
    ) -> Result<Self, FilterGenError>
    where
        FS: Maybe<<Self::Domain as ComplexFloat>::Real>;
}

impl<T, F, W, WW> Fir1<(), F, W, WW, true> for Tf<T, WW::Mapped<T>, ()>
where
    T: ComplexFloat + Into<Complex<T::Real>> + Mul<T::Real, Output = T>,
    T::Real: Into<T>,
    Complex<T::Real>: AddAssign + MulAssign,
    WW: List<W>,
    F: List<T::Real>,
    W: ComplexFloat<Real = T::Real> + Into<T>,
    WW::Mapped<T>: List<T, Mapped<T> = WW::Mapped<T>>,
    Self: Fir2<(), Vec<T::Real>, Vec<T>, W, WW, true> + System<Domain = T>
{
    fn fir1<FS>(
        (): (),
        frequencies: F,
        filter_type: Fir1Type,
        window: WW,
        scale: bool,
        sampling_frequency: FS
    ) -> Result<Self, FilterGenError>
    where
        FS: Maybe<T::Real>
    {
        let ftype = match filter_type
        {
            Fir1Type::LowPass | Fir1Type::BandStop | Fir1Type::DC1 => true,
            Fir1Type::HighPass | Fir1Type::BandPass | Fir1Type::DC0 => false
        };

        let zero: T::Real = Zero::zero();
        let one = One::one();
        let two = one + one;

        let win: &[W] = window.as_view_slice();
        let _n = win.len();

        let w: &[T::Real] = frequencies.as_view_slice();
        let bands = w.len() + 1;
        let mut f = vec![zero; bands*2];
        for (i, &w) in w.iter()
            .enumerate()
        {
            f[1 + i*2] = w;
            f[2 + i*2] = w;
        }
        let nyq = sampling_frequency.as_option()
            .map(|&fs| fs/two)
            .unwrap_or(one);
        *f.last_mut().unwrap() = nyq;
        let mut m = vec![zero.into(); bands*2];
        for i in 0..bands
        {
            m[i*2] = T::from((i + ftype as usize) % 2).unwrap();
            m[i*2 + 1] = m[i*2];
        }

        let w_o = if scale
        {
            if m[0].is_one()
            {
                Some(zero)
            }
            else if let Some(&f4) = f.get(3)
            {
                if f4 == nyq
                {
                    Some(one)
                }
                else
                {
                    Some((f[2] + (f4 - f[2])/two)/nyq)
                }
            }
            else
            {
                None
            }
        }
        else
        {
            None
        };

        /*let ff: Vec<_> = f.iter()
            .map(|&f| <f32 as NumCast>::from(f).unwrap())
            .collect();
        let mf: Vec<_> = m.iter()
            .map(|&m| <f32 as NumCast>::from(m).unwrap())
            .collect();
        println!("{:?}", ff);
        println!("{:?}", mf);*/

        let mut b = Self::fir2((), f, m, (), (), window, sampling_frequency)?;

        if let Some(w_o) = w_o
        {
            let bb = b.b.into_inner();
            let p: &[T] = bb.as_view_slice();
            let renorm = p.dtft(T::Real::PI()*w_o)
                .abs()
                .recip();

            b.b = Polynomial::new(bb.map_into_owned(|b: T| b*renorm))
        }

        Ok(b)
    }
}

impl<T, F> Fir1<usize, F, T::Real, (), false> for Tf<T, Vec<T>, ()>
where
    T: ComplexFloat,
    F: List<T::Real>,
    T::Real: Into<T>,
    Self: Fir1<(), F, T::Real, Vec<T::Real>, true> + System<Domain = T>
{
    fn fir1<FS>(
        order: usize,
        frequencies: F,
        filter_type: Fir1Type,
        (): (),
        scale: bool,
        sampling_frequency: FS
    ) -> Result<Self, FilterGenError>
    where
        FS: Maybe<T::Real>
    {
        Self::fir1(
            (),
            frequencies,
            filter_type,
            Hamming.window_gen(order, WindowRange::Symmetric),
            scale,
            sampling_frequency
        )
    }
}

impl<T, F, const N: usize> Fir1<(), F, T::Real, (), false> for Tf<T, [T; N], ()>
where
    T: ComplexFloat,
    F: List<T::Real>,
    T::Real: Into<T>,
    Self: Fir1<(), F, T::Real, [T::Real; N], true> + System<Domain = T>
{
    fn fir1<FS>(
        (): (),
        frequencies: F,
        filter_type: Fir1Type,
        (): (),
        scale: bool,
        sampling_frequency: FS
    ) -> Result<Self, FilterGenError>
    where
        FS: Maybe<T::Real>
    {
        Self::fir1(
            (),
            frequencies,
            filter_type,
            Hamming.window_gen((), WindowRange::Symmetric),
            scale,
            sampling_frequency
        )
    }
}

#[cfg(test)]
mod test
{
    use array_math::ArrayOps;

    use crate::{plot, Fir1, Fir1Type, Plane, RealFreqZ, Tf, ToZpk, Zpk};

    #[test]
    fn test()
    {
        const N: usize = 49;
        let h: Tf<f64, [_; N]> = Tf::fir1((), [0.35, 0.65], Fir1Type::BandPass, (), true, ())
            .unwrap();

        const M: usize = 1024;
        let (h_f, w): ([_; M], _) = h.real_freqz(());

        plot::plot_curves("H(e^jw)", "plots/h_z_fir1.png", [&w.zip(h_f.map(|h_f| h_f.norm()))])
            .unwrap();

        let h: Zpk<_, Vec<_>, Vec<_>, _> = h.to_zpk((), ());

        plot::plot_pz("H(z)", "plots/pz_z_fir1.png", &h.p, &h.z, Plane::Z)
            .unwrap();
    }
}