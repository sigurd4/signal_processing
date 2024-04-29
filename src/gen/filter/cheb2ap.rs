use core::ops::DivAssign;

use num::{complex::ComplexFloat, traits::FloatConst, Complex, Float};
use option_trait::Maybe;

use crate::{quantities::ProductSequence, System, systems::Zpk};

pub trait Cheb2AP<O>: System + Sized
where
    Self::Set: Float,
    O: Maybe<usize>
{
    fn cheb2ap(order: O, ripple: Self::Set) -> Self;
}

impl<T> Cheb2AP<usize> for Zpk<Complex<T>, Vec<Complex<T>>, Vec<Complex<T>>, T>
where
    T: Float + FloatConst + DivAssign
{
    fn cheb2ap(order: usize, mut ripple: T) -> Self
    {
        if order == 0
        {
            return Self::one()
        }
        ripple = ripple.abs();

        let zero = T::zero();
        let one = T::one();
        let two = one + one;
        let half = two.recip();
        let ten = T::from(10u8).unwrap();
        let twenty = ten + ten;

        let n = T::from(order).unwrap();
        let c = one;
        let lambda = ten.powf(ripple/twenty);
        let phi = ((Complex::from(lambda*lambda) - one).sqrt() + lambda).ln()/n;
        let theta: Vec<_> = (1..=order).map(|i| T::PI()*(T::from(i).unwrap() - half)/n)
            .collect();
        let alpha: Vec<_> = theta.iter()
            .map(|theta| -phi.sinh()*theta.sin())
            .collect();
        let beta: Vec<_> = theta.iter()
            .map(|theta| phi.cosh()*theta.cos())
            .collect();
        let zero: Vec<_> = if order % 2 == 1
        {
            theta[..(order - 1)/2].iter()
                .chain(theta[(order + 1)/2..].iter()).copied()
                .collect::<Vec<_>>()
        }
        else
        {
            theta
        }.into_iter()
            .map(|theta| Complex::new(zero, c)/theta.cos())
            .collect();
        let pole: Vec<_> = alpha.into_iter()
            .zip(beta)
            .map(|(alpha, beta)| Complex::new(alpha.re + beta.im, alpha.im - beta.re)*c/(alpha*alpha + beta*beta))
            .collect();
        let gain = (
                pole.iter().copied()
                    .product::<Complex<T>>()
                /zero.iter().copied()
                    .product::<Complex<T>>()
            ).abs();

        Zpk {
            z: ProductSequence::new(zero),
            p: ProductSequence::new(pole),
            k: gain
        }
    }
}
impl<T, const N: usize> Cheb2AP<()> for Zpk<Complex<T>, (), [Complex<T>; N], T>
where
    T: Float + FloatConst,
    Zpk<Complex<T>, (), Vec<Complex<T>>, T>: Cheb2AP<usize> + System<Set = T>
{
    fn cheb2ap((): (), ripple: T) -> Self
    {
        let Zpk { z, p, k } = Zpk::cheb2ap(N, ripple);

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

    use crate::{plot, gen::filter::Cheb2AP, transforms::domain::Bilinear, analysis::{FreqS, RealFreqZ}, systems::Zpk, Plane};

    #[test]
    fn test()
    {
        let fs = 2.0;
        let h = Zpk::cheb2ap(6, 20.0);

        plot::plot_pz("H(s)", "plots/pz_s_cheb2ap.png", h.poles(), h.zeros(), Plane::S)
            .unwrap();

        const N: usize = 1024;
        let w: [_; N] = (0.0..fs).linspace_array();
        let h_f = h.freqs(w.map(|w| Complex::new(0.0, w)));

        plot::plot_curves("H(jw)", "plots/h_s_cheb2ap.png", [&w.zip(h_f.map(|h| h.norm())), &w.zip(h_f.map(|h| h.arg()))])
            .unwrap();
        
        let h = h.bilinear(fs)
            .unwrap();

        plot::plot_pz("H(z)", "plots/pz_z_cheb2ap.png", h.poles(), h.zeros(), Plane::Z)
            .unwrap();
        
        let (h_f, w): ([_; N], _) = h.real_freqz(());

        plot::plot_curves("H(e^jw)", "plots/h_z_cheb2ap.png", [&w.zip(h_f.map(|h| h.norm())), &w.zip(h_f.map(|h| h.arg()))])
            .unwrap();
    }
}