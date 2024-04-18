use core::{iter::Sum, ops::{AddAssign, DivAssign, MulAssign, SubAssign}};

use array_math::ArrayMath;
use num::{complex::ComplexFloat, traits::FloatConst, Complex, Float};
use option_trait::Maybe;

use crate::{ProductSequence, System, Zpk};

pub trait EllipAP<O>: System + Sized
where
    Self::Domain: Float,
    O: Maybe<usize>
{
    #[doc(alias = "ncauer")]
    fn ellipap(order: O, passband_ripple: Self::Domain, stopband_ripple: Self::Domain) -> Self;
}

impl<T> EllipAP<usize> for Zpk<Complex<T>, Vec<Complex<T>>, Vec<Complex<T>>, T>
where
    T: Float + FloatConst + DivAssign + Sum + SubAssign + MulAssign + AddAssign,
    Complex<T>: MulAssign<T>
{
    fn ellipap(order: usize, mut passband_ripple: T, mut stopband_ripple: T) -> Self
    {
        if order == 0
        {
            return Self::one()
        }
        passband_ripple = passband_ripple.abs();
        stopband_ripple = stopband_ripple.abs();

        let one = T::one();
        let two = one + one;
        let half = two.recip();
        let ten = T::from(10u8).unwrap();
        let fifteen = ten + ten*half;
        let one_hundred_and_fifty = fifteen*ten;
        let twenty = ten + ten;
        let eps = T::epsilon();
        let n = T::from(order).unwrap();

        let wp = one;

        fn ellip_ws_min<T>(kl: T, x: T) -> T
        where
            T: Float + FloatConst + AddAssign + MulAssign
        {
            let ([ql, q], _) = [kl, T::one() - kl].ellipke(None)
                .unwrap();
            (ql/q - x).abs()
        }

        fn fminbnd<T, F>(mut f: F, x_min: T, x_max: T, i: usize) -> T
        where
            T: Float,
            F: FnMut(T) -> T
        {
            let one = T::one();
            let two = one + one;
            let mid = (x_min + x_max)/two;
            if i == 0 || x_min == x_max
            {
                mid
            }
            else if f((x_min + mid)/two) < f((mid + x_max)/two)
            {
                fminbnd(f, x_min, mid, i - 1)
            }
            else
            {
                fminbnd(f, mid, x_max, i - 1)
            }
        }

        let kl0 = (ten.powf(passband_ripple/ten) - one)/(ten.powf(stopband_ripple/ten) - one);
        let k0 = one - kl0;
        let ([ql0, q0], _) = [kl0, k0].ellipke(None)
            .unwrap();
        let x = n*ql0/q0;
        let kl = fminbnd(|kl| ellip_ws_min(kl, x), eps, one - eps, 1024);
        let ws = kl.recip().sqrt();

        let k = wp/ws;
        let k1 = (one - k*k).sqrt();
        let q0 = half*((one - k1.sqrt())/(one + k1.sqrt()));
        let q0_p4 = q0*q0*q0*q0;
        let q = q0*(one + q0_p4*(two + q0_p4*(fifteen + q0_p4*one_hundred_and_fifty)));
        //let d = (ten.powf(stopband_ripple/ten) - one)/(ten.powf(passband_ripple/ten) - one);

        let l = (two*n).recip()*((ten.powf(passband_ripple/twenty) + one)/(ten.powf(passband_ripple/twenty) - one)).ln();

        let sig01 = (0..=30).map(|m| {
            let s = T::from(1 - (m % 2) as i8*2).unwrap();
            s*q.powi(m*(m + 1))*(T::from(2*m + 1).unwrap()*l).sinh()
        }).sum::<T>();
        let sig02 = (1..=30).map(|m| {
            let s = T::from(1 - (m % 2) as i8*2).unwrap();
            s*q.powi(m*m)*(T::from(2*m).unwrap()*l).cosh()
        }).sum::<T>();
        let sig0 = ((Complex::from(q).sqrt().sqrt()*(two*sig01))/(one + two*sig02)).abs();

        let w = Complex::from((one + k*sig0*sig0)*(one + sig0*sig0/k)).sqrt();
        let r = order/2;
        let wi: Vec<_> = (1..=r).map(|ii| {
            let mut mu = T::from(ii).unwrap();
            if order % 2 == 0
            {
                mu -= half;
            }
            let soma1 = (0..=30).map(|m| {
                let s = T::from(1 - (m % 2) as i8*2).unwrap();
                Complex::from(q).sqrt().sqrt()*two*(s*q.powi(m*(m + 1))*((T::from(2*m + 1).unwrap()*T::PI()*mu)/n).sin())
            }).sum::<Complex<T>>();
            let soma2 = (1..=30).map(|m| {
                let s = T::from(1 - (m % 2) as i8*2).unwrap();
                two*s*q.powi(m*m)*((T::from(2*m).unwrap()*T::PI()*mu)/n).cos()
            }).sum::<T>();

            soma1/(one + soma2)
        }).collect();

        let vi: Vec<_> = wi.iter()
            .map(|&wi| ((-(wi*wi)*k + one)*(-(wi*wi)/k + one)).sqrt())
            .collect();
        let a0i: Vec<_> = wi.iter()
            .map(|&wi| (wi*wi).inv())
            .collect();
        let sqra0i: Vec<_> = wi.iter()
            .map(|&wi| wi.inv())
            .collect();
        let b0i: Vec<_> = wi.iter()
            .zip(vi.iter())
            .map(|(&wi, &vi)| {
                let sig0vi = vi*sig0;
                let wwi = w*wi;
                let sig0wi = wi*sig0;
                let d = sig0wi*sig0wi + one;
                (sig0vi*sig0vi + wwi*wwi)/(d*d)
            }).collect();
        /*let b1i: Vec<_> = wi.iter()
            .zip(vi.iter())
            .map(|(&wi, &vi)| {
                vi*(two*sig0)/((wi*wi)*(sig0*sig0) + one)
            }).collect();*/

        let pab = b0i.iter()
            .zip(a0i.iter())
            .map(|(&b0i, &a0i)| b0i/a0i)
            .product::<Complex<T>>();
            
        let t0 = if order % 2 == 1
        {
            pab*sig0*Complex::from(ws).sqrt()
        }
        else
        {
            pab*ten.powf(-passband_ripple/twenty)
        }.abs();

        let mut zeros = sqra0i.clone()
            .into_iter()
            .chain(sqra0i.into_iter()
                .map(|s| -s)
            ).map(|s| s*Complex::i())
            .collect();
        let mut poles: Vec<_> = vi.iter()
            .zip(wi.iter())
            .map(|(&vi, &wi)| (-vi*sig0 + wi*w*Complex::i())*two/((wi*wi*sig0*sig0 + one)*two))
            .collect::<Vec<_>>()
            .into_iter()
            .chain(vi.into_iter()
                .zip(wi)
                .map(|(vi, wi)| (-vi*sig0 - wi*w*Complex::i())*two/((wi*wi*sig0*sig0 + one)*two))
            ).collect();

        if order % 2 == 1
        {
            poles.push(Complex::from(-sig0))
        }

        let ws_sqrt = ws.sqrt();
        for z in [&mut zeros, &mut poles]
        {
            for z in z.iter_mut()
            {
                *z *= ws_sqrt
            }
        }

        Zpk {
            z: ProductSequence::new(zeros),
            p: ProductSequence::new(poles),
            k: t0
        }
    }
}
impl<T, const N: usize> EllipAP<()> for Zpk<Complex<T>, Vec<Complex<T>>, [Complex<T>; N], T>
where
    T: Float + FloatConst,
    Zpk<Complex<T>, Vec<Complex<T>>, Vec<Complex<T>>, T>: EllipAP<usize> + System<Domain = T>
{
    fn ellipap((): (), passband_ripple: T, stopband_ripple: T) -> Self
    {
        let Zpk { z, p, k } = Zpk::ellipap(N, passband_ripple, stopband_ripple);

        Zpk {
            z,
            p: p.try_into().map_err(|_| ()).unwrap(),
            k
        }
    }
}

#[cfg(test)]
mod test
{
    use array_math::ArrayOps;
    use linspace::LinspaceArray;
    use num::Complex;

    use crate::{plot, Bilinear, EllipAP, FreqS, Plane, RealFreqZ, Zpk};

    #[test]
    fn test()
    {
        let fs = 2.0;
        let h = Zpk::ellipap(6, 5.0, 50.0);

        plot::plot_pz("H(s)", "plots/pz_s_ellipap.png", h.poles(), h.zeros(), Plane::S)
            .unwrap();

        const N: usize = 1024;
        let w: [_; N] = (0.0..fs).linspace_array();
        let h_f = h.freqs(w.map(|w| Complex::new(0.0, w)));

        plot::plot_curves("H(jw)", "plots/h_s_ellipap.png", [&w.zip(h_f.map(|h| h.norm())), &w.zip(h_f.map(|h| h.arg()))])
            .unwrap();
        
        let h = h.bilinear(fs)
            .unwrap();

        plot::plot_pz("H(z)", "plots/pz_z_ellipap.png", h.poles(), h.zeros(), Plane::Z)
            .unwrap();
        
        let (h_f, w): ([_; N], _) = h.real_freqz(());

        plot::plot_curves("H(e^jw)", "plots/h_z_ellipap.png", [&w.zip(h_f.map(|h| h.norm())), &w.zip(h_f.map(|h| h.arg()))])
            .unwrap();
    }
}