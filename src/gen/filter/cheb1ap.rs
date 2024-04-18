use core::ops::DivAssign;

use num::{complex::ComplexFloat, traits::FloatConst, Complex, Float};
use option_trait::Maybe;

use crate::{ProductSequence, System, Zpk};

pub trait Cheb1AP<O>: System + Sized
where
    Self::Domain: Float,
    O: Maybe<usize>
{
    fn cheb1ap(order: O, ripple: Self::Domain) -> Self;
}

impl<T> Cheb1AP<usize> for Zpk<Complex<T>, (), Vec<Complex<T>>, T>
where
    T: Float + FloatConst + DivAssign
{
    fn cheb1ap(order: usize, mut ripple: T) -> Self
    {
        if order == 0
        {
            return Self::one()
        }
        ripple = ripple.abs();

        let one = T::one();
        let ten = T::from(10u8).unwrap();
        let twenty = ten + ten;
        let n = T::from(order).unwrap();
        let _c = one;
        let epsilon = (ten.powf(ripple/ten) - one).sqrt();
        let v0 = epsilon.recip().asinh()/n;
        let v0s = -v0.sinh();
        let v0c = v0.cosh();
        let ni = (order - 1) as isize;
        let pole: Vec<_> = (-ni..=ni).step_by(2)
            .map(|i| {
                let p = T::from(i).unwrap()/T::from(order*2).unwrap();
                let c = Complex::cis(T::PI()*p);
                Complex::new(c.re*v0s, c.im*v0c)
            }).collect();
        let mut gain = pole.iter()
            .map(|&p| -p)
            .product::<Complex<T>>()
            .abs();
        if order % 2 == 0
        {
            gain /= ten.powf(ripple/twenty)
        }

        Zpk {
            z: ProductSequence::new(()),
            p: ProductSequence::new(pole),
            k: gain
        }
    }
}
impl<T, const N: usize> Cheb1AP<()> for Zpk<Complex<T>, (), [Complex<T>; N], T>
where
    T: Float + FloatConst,
    Zpk<Complex<T>, (), Vec<Complex<T>>, T>: Cheb1AP<usize> + System<Domain = T>
{
    fn cheb1ap((): (), ripple: T) -> Self
    {
        let Zpk { z, p, k } = Zpk::cheb1ap(N, ripple);

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

    use crate::{plot, Bilinear, Cheb1AP, FreqS, Plane, RealFreqZ, Zpk};

    #[test]
    fn test()
    {
        let fs = 2.0;
        let h = Zpk::cheb1ap(6, 0.5);

        plot::plot_pz("H(s)", "plots/pz_s_cheb1ap.png", h.poles(), h.zeros(), Plane::S)
            .unwrap();

        const N: usize = 1024;
        let w: [_; N] = (0.0..fs).linspace_array();
        let h_f = h.freqs(w.map(|w| Complex::new(0.0, w)));

        plot::plot_curves("H(jw)", "plots/h_s_cheb1ap.png", [&w.zip(h_f.map(|h| h.norm())), &w.zip(h_f.map(|h| h.arg()))])
            .unwrap();
        
        let h = h.bilinear(fs)
            .unwrap();

        plot::plot_pz("H(z)", "plots/pz_z_cheb1ap.png", h.poles(), h.zeros(), Plane::Z)
            .unwrap();
        
        let (h_f, w): ([_; N], _) = h.real_freqz(());

        plot::plot_curves("H(e^jw)", "plots/h_z_cheb1ap.png", [&w.zip(h_f.map(|h| h.norm())), &w.zip(h_f.map(|h| h.arg()))])
            .unwrap();
    }
}