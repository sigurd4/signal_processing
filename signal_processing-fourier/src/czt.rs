use core::ops::Mul;

use array_trait::length;
use bulks::{AsBulk, Bulk, IntoBulk, Map, Resize, Skip, Zip};
use num_traits::{One, Zero};
use num_complex::{Complex, ComplexFloat};

use crate::DftInplace;

/// Computes a chirp-response within the z-transform.
pub trait Czt: Bulk<Item: ComplexFloat>
{
    type Output: Bulk<Item = Complex<<Self::Item as ComplexFloat>::Real>, Length = Self::Length>;

    /// Computes a chirp-response within the z-transform.
    /// 
    /// `point` is a point on the chirp's curve in the z-domain.
    /// `ratio` is the rate at which the curve grows for each bucket.
    /// 
    /// A spectrum of z-domain response following a complex chirp-curve will be obtained from the time-series signal.
    fn czt(self, ratio: Complex<<Self::Item as ComplexFloat>::Real>, point: Complex<<Self::Item as ComplexFloat>::Real>) -> Self::Output;
}
impl<B, T> Czt for B
where
    B: Bulk<Item = T>,
    T: ComplexFloat + 'static
{
    type Output = Resize<Map<Skip<Zip<bulks::vec::IntoBulk<Complex<T::Real>>, bulks::vec::IntoBulk<Complex<T::Real>>>, length::value::Length<length::value::SaturatingSub<length::Value<B::Length>, [(); 1]>, ()>>, fn((Complex<T::Real>, Complex<T::Real>)) -> Complex<T::Real>>, B::Length>;

    fn czt(self, ratio: Complex<T::Real>, point: Complex<T::Real>) -> Self::Output
    {
        let n = self.length();
        let nfft = length::value::saturating_sub(length::value::mul(n, [(); 2]), [(); 1]);
        let nfft_pow2 = length::value::len(nfft).next_power_of_two();

        let ratio_sqrt = ratio.sqrt();
        let w2: Vec<_> = bulks::repeat_n((), nfft)
            .enumerate()
            .map(|(i, ())| {
                let p = i as i32 + 1 - length::value::len(n) as i32;
                if let Some(pp) = p.checked_mul(p)
                {
                    ratio_sqrt.powi(pp)
                }
                else
                {
                    ratio_sqrt.powi(p).powi(p)
                }
            }).collect();
        let mut fw = w2.bulk()
            .map(Complex::inv)
            .resize::<[_]>(nfft_pow2, Complex::zero())
            .collect::<Vec<_>, _>()
            .into_bulk();
        fw.dft_inplace();

        let mut fg: Vec<_> = self.map(|x| Complex { re: x.re(), im: x.im() })
            .collect();
        let a_recip = point.recip();
        let mut apmk = Complex::one();
        for (i, g)  in fg.iter_mut()
            .enumerate()
        {
            *g = *g*apmk*w2[i + length::value::len(n) - 1];
            apmk = apmk*a_recip;
        }
        fg.resize(nfft_pow2, Complex::zero());

        fn mul_tuple<T>((a, b): (T, T)) -> <T as Mul>::Output
        where
            T: Mul
        {
            a*b
        }

        let mut fg = fg.into_bulk();
        fg.dft_inplace();
        let mut gg = fg.zip(fw)
            .map(mul_tuple)
            .collect::<Vec<_>, _>()
            .into_bulk();
        gg.idft_inplace();
        gg.zip(w2)
            .skip(length::value::saturating_sub(n, [(); 1]))
            .map(mul_tuple as fn(_) -> _)
            .resize(n, Complex::zero())
    }
}

#[cfg(test)]
mod test
{
    use core::f64::consts::TAU;

    use bulks::{Bulk, CollectNearest, IntoBulk};
use linspace::Linspace;
use num_complex::Complex;

    use crate::Czt;

    #[test]
    fn test()
    {
        const N: usize = 1024;
        const T: f64 = 1.0;
        const F: f64 = 220.0;
        const K: usize = 3;
        
        let x: [_; N] = core::array::from_fn(|i| (TAU*F*i as f64/N as f64*T).sin());

        let w = (0.0..TAU).linspace_array::<N>();
        let xf = core::array::from_fn::<_, K, _>(|i| {
            x.into_bulk()
                .czt(Complex::cis(-TAU/N as f64), Complex::ONE*2.0f64.powi(i as i32 + 1))
                .collect_nearest()
        });

        ezplot::plot_curves("X(n*e^jw)", "plots/x_z_czt.png", xf.map(|xf| w.into_bulk().zip(xf.map(|xf| xf.norm())))).unwrap()
    }
}