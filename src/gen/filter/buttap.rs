use core::f64::consts::PI;

use num::{traits::FloatConst, Complex, Float, One};
use option_trait::Maybe;

use crate::{ProductSequence, System, Zpk};

pub trait ButtAP<O>: System + Sized
where
    Self::Domain: Float,
    O: Maybe<usize>
{
    fn buttap(order: O) -> Self;
}

impl<T> ButtAP<usize> for Zpk<Complex<T>, (), Vec<Complex<T>>, T>
where
    T: Float + FloatConst
{
    fn buttap(order: usize) -> Self
    {
        if order == 0
        {
            return Self::one()
        }

        let c = One::one();
        let mut pole: Vec<_> = (1..=order).map(|i| Complex::from_polar(c, T::from(PI*(2.0*i as f64 + order as f64 - 1.0)/(2.0*order as f64)).unwrap()))
            .collect();
        if order % 2 == 1
        {
            pole[(order + 1)/2 - 1] = -Complex::one()
        }
        let gain = c.powi(order as i32);
        Zpk {
            z: ProductSequence::new(()),
            p: ProductSequence::new(pole),
            k: gain
        }
    }
}
impl<T, const N: usize> ButtAP<()> for Zpk<Complex<T>, (), [Complex<T>; N], T>
where
    T: Float + FloatConst,
    Zpk<Complex<T>, (), Vec<Complex<T>>, T>: ButtAP<usize> + System<Domain = T>
{
    fn buttap((): ()) -> Self
    {
        let Zpk { z, p, k } = Zpk::buttap(N);

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

    use crate::{plot, ButtAP, Bilinear, FreqS, RealFreqZ, Zpk};

    #[test]
    fn test()
    {
        let fs = 2.0;
        let h = Zpk::buttap(6);

        const N: usize = 1024;
        let w: [_; N] = (0.0..fs).linspace_array();
        let h_f = h.freqs(w.map(|w| Complex::new(0.0, w)));

        plot::plot_curves("H(jw)", "plots/h_s_buttap.png", [&w.zip(h_f.map(|h| h.norm())), &w.zip(h_f.map(|h| h.arg()))]).unwrap();
        
        let (h_f, w): ([_; N], _) = h.bilinear(fs)
            .unwrap()
            .real_freqz(());

        plot::plot_curves("H(e^jw)", "plots/h_z_buttap.png", [&w.zip(h_f.map(|h| h.norm())), &w.zip(h_f.map(|h| h.arg()))]).unwrap();
    }
}