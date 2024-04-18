use core::{iter::Sum, ops::{AddAssign, DivAssign, MulAssign}};

use num::{complex::ComplexFloat, traits::FloatConst, Complex, Float, NumCast, Zero};
use array_math::SliceMath;

use crate::{window::{Kaiser, WindowGen, WindowRange}, Conv, Idft, System, Tf, Hilbert};

pub trait QpKaiser: System
{
    fn qp_kaiser(num_bands: usize, attenuation: <Self::Domain as ComplexFloat>::Real, linear: bool) -> Self;
}

impl<T> QpKaiser for Tf<T, Vec<T>>
where
    T: Float + FloatConst + MulAssign + DivAssign + Sum + Into<Complex<T>> + 'static,
    Vec<T>: Conv<T, T, Vec<T>, Output = Vec<T>>,
    Complex<T>: AddAssign + MulAssign
{
    fn qp_kaiser(num_bands: usize, mut attenuation: T, linear: bool) -> Self
    {
        let nb = T::from(num_bands).unwrap();

        let bandwidth = T::PI()/nb;

        let zero = T::zero();
        let one = T::one();
        let two = one + one;
        let twenty = T::from(20u8).unwrap();

        let corr = (T::from(1.4).unwrap() + T::from(0.6).unwrap()*(attenuation - twenty)/T::from(80u8).unwrap())
            .powf(twenty/attenuation);
        attenuation *= corr;

        let n = (attenuation - T::from(8u8).unwrap())/(T::from(2.285).unwrap()*bandwidth);
        let m = <usize as NumCast>::from((n/two).floor()).unwrap();
        let n = 2*m + 1;

        let beta = if attenuation > T::from(50u8).unwrap()
        {
            T::from(0.1102).unwrap()*(attenuation - T::from(8.7).unwrap())
        }
        else
        {
            let am21 = attenuation - T::from(21u8).unwrap();
            if am21 > zero
            {
                T::from(0.5842).unwrap()*am21.powf(T::from(0.4).unwrap())
                    + T::from(0.07886).unwrap()*am21
            }
            else
            {
                zero
            }
        };
        let w = Kaiser {beta}.window_gen(n, WindowRange::Symmetric);
        let wsqr = w.clone().conv(w);

        let nb_recip = T::from(num_bands)
            .unwrap()
            .recip();
        let mut hcomp: Vec<_> = (1..n*2).zip(wsqr)
            .map(|(i, w)| {
            let x = T::from(i).unwrap() - T::from(n).unwrap();
            ((if x.is_subnormal() || x.is_zero()
            {
                nb_recip
            }
            else
            {
                (x*nb_recip*T::PI()).sin()/(x*T::PI())
            })*w).into()
        }).collect();

        const NDFT: usize = 1 << 15;
        hcomp.resize_with(hcomp.len().max(NDFT), Zero::zero);
        hcomp.fft();
        let hsqr: Vec<_> = hcomp.into_iter()
            .map(|h| h.abs().sqrt())
            .collect();

        let mut h: Vec<_> = if linear
        {
            let h: Vec<_> = hsqr.idft()
                .into_iter()
                .take(n)
                .skip(1)
                .map(|h| h.re())
                .collect();
            h.iter()
                .rev()
                .chain(h.first())
                .map(|&h| h)
                .collect::<Vec<_>>()
                .into_iter()
                .chain(h)
                .collect()
        }
        else
        {
            let h: Vec<_> = hsqr.iter()
                .map(|&h| h.ln())
                .collect();
            let h = h.hilbert();
            let mut h: Vec<_> = hsqr.into_iter()
                .zip(h)
                .map(|(a, h)| Complex::cis(-h)*a)
                .collect();
            h.ifft();

            h.into_iter()
                .take(n)
                .map(|h| h.re())
                .collect()
        };

        let h_sum = h.iter()
            .map(|&h| h)
            .sum();
        for h in h.iter_mut()
        {
            *h /= h_sum
        }

        Tf::new(h, ())
    }
}

#[cfg(test)]
mod test
{
    use array_math::ArrayOps;

    use crate::{plot, QpKaiser, RealFreqZ, Tf};

    #[test]
    fn test()
    {
        let h = Tf::qp_kaiser(6, 20.0, false);

        const N: usize = 1024;
        let (h_f, w): ([_; N], _) = h.real_freqz(());

        plot::plot_curves("H(e^jw)", "plots/h_z_qp_kaiser.png", [&w.zip(h_f.map(|h| h.norm()))]).unwrap();
    }
}