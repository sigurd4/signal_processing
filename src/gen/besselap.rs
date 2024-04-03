use core::ops::{AddAssign, DivAssign, MulAssign, SubAssign};


use num::{complex::ComplexFloat, traits::FloatConst, Complex, Float, NumCast};
use option_trait::Maybe;

use array_math::SliceMath;

use crate::{Chain, Polynomial, ProductSequence, System, Zpk};

pub trait BesselAP<O>: System + Sized
where
    Self::Domain: Float,
    O: Maybe<usize>
{
    fn besselap(order: O) -> Self;
}

impl<T> BesselAP<usize> for Zpk<Complex<T>, (), Vec<Complex<T>>, T>
where
    T: Float + FloatConst + AddAssign + MulAssign + Into<Complex<T>> + ComplexFloat<Real = T> + ndarray_linalg::Lapack<Complex = Complex<T>>,
    Complex<T>: From<T> + AddAssign + SubAssign + MulAssign + DivAssign + DivAssign<T>,
{
    fn besselap(order: usize) -> Self
    {
        if order == 0
        {
            return Self::one()
        }

        let zero = T::zero();
        let one = T::one();

        let mut p0 = Polynomial::new(vec![one]);
        let mut p1 = Polynomial::new(vec![one, one]);
        for nn in 2..=order
        {
            let x = <T as NumCast>::from(2*nn - 1).unwrap();
            let px = p1.as_view()*Polynomial::new([x]);
            let py = Polynomial::new(p0.into_inner().chain([zero, zero]));
            p0 = p1;
            p1 = px + py;
        }

        let l = p1.len();
        let w = *p1.last().unwrap();
        for (i, p) in p1.iter_mut()
            .enumerate()
        {
            let j = l - i - 1;
            //*p *= w;
            *p *= Float::powf(w, <T as NumCast>::from(j).unwrap()/NumCast::from(l - 1).unwrap())
        }

        let p = p1.rpolynomial_roots();

        Zpk {
            z: ProductSequence::new(()),
            p: ProductSequence::new(p),
            k: T::one()
        }
    }
}
impl<T, const N: usize> BesselAP<()> for Zpk<Complex<T>, (), [Complex<T>; N], T>
where
    T: Float + FloatConst,
    Zpk<Complex<T>, (), Vec<Complex<T>>, T>: BesselAP<usize> + System<Domain = T>
{
    fn besselap((): ()) -> Self
    {
        let Zpk { z, p, k } = Zpk::besselap(N);

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

    use crate::{plot, BesselAP, Bilinear, FreqS, RealFreqZ, Zpk};

    #[test]
    fn test()
    {
        let fs = 2.0;
        let h = Zpk::besselap(6);

        const N: usize = 1024;
        let w: [_; N] = (0.0..fs).linspace_array();
        let h_f = h.freqs(&w.map(|w| Complex::new(0.0, w)));

        plot::plot_curves("H(jw)", "plots/h_s_besselap.png", [&w.zip(h_f.map(|h| h.norm())), &w.zip(h_f.map(|h| h.arg()))]).unwrap();
        
        let (h_f, w): ([_; N], _) = h.bilinear(fs)
            .unwrap()
            .real_freqz(());

        plot::plot_curves("H(e^jw)", "plots/h_z_besselap.png", [&w.zip(h_f.map(|h| h.norm())), &w.zip(h_f.map(|h| h.arg()))]).unwrap();
    }
}