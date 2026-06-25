use core::{borrow::{Borrow, BorrowMut}, ops::Mul};

use array_trait::length;
use bulks::{AsBulk, Bulk, IntoBulk};
use num_traits::{Float, FloatConst, One, Zero};
use num_complex::{Complex, ComplexFloat};

use crate::{Dft, SpectrumScaling, util::DivAssignSpec};

/// # Chirp z-transform
/// 
/// Computes a chirp-response within the z-transform.
pub trait Czt<T>: Dft<T>
where
    T: Float + FloatConst
{
    /// Computes a chirp-response within the z-transform.
    /// 
    /// `point` is a point on the chirp's curve in the z-domain.
    /// `ratio` is the rate at which the curve grows for each bucket.
    /// 
    /// A spectrum of z-domain response following a complex chirp-curve will be obtained from the time-series signal.
    /// 
    /// # Comparison with the DFT (discrete fourier transform)
    /// 
    /// If we set `ratio` to `e^(j2π/n)`, where `n` is the length of the sequence, and `point` to `1`, we obtain the DFT.
    /// The DFT is therefore a subset of the chirp-z-transform. Using [dft()](crate::Dft::dft) should be preferred in that case.
    /// 
    /// The signal's z-transform is a function across a plane. Just like the DFT, this computes a one-dimensional spectrum of the z-transform along a curve.
    /// The difference is, here: the curve can be configured.
    /// The DFT follows the unit-circle in the z-domain, while the chirp-z-transform follows a configurable chirp-curve.
    fn czt(&mut self, ratio: Complex<T>, point: Complex<T>)
    {
        self.czt_scaled(ratio, point, SpectrumScaling::Balanced);
    }

    fn czt_scaled(&mut self, ratio: Complex<T>, point: Complex<T>, scaling: SpectrumScaling);
}
impl<B, T> Czt<T> for B
where
    for<'a> &'a mut B: IntoBulk<Item: BorrowMut<Complex<T>>>,
    B: ?Sized,
    T: Float + FloatConst + 'static
{
    fn czt_scaled(&mut self, ratio: Complex<T>, point: Complex<T>, scaling: SpectrumScaling)
    {
        let n = self.bulk_mut().length();
        let nfft = length::value::saturating_sub(length::value::mul(n, [(); 2]), [(); 1]);
        let nfft_pow2 = length::value::len(nfft).next_power_of_two();

        let ratio_sqrt = ratio.sqrt();
        let w2: Vec<_> = bulks::range([(); 0], nfft)
            .map(|i| {
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
            .collect::<Vec<_>, _>();
        fw.dft_scaled(SpectrumScaling::Summed);

        let a_recip = point.recip();
        let mut apmk = Complex::one();
        let mut fg: Vec<_> = self.bulk_mut()
            .map(|x| *x.borrow())
            .zip(&w2[length::value::len(n) - 1..])
            .map(|(mut g, &w)| {
                g = g*apmk*w;
                apmk = apmk*a_recip;
                g
            })
            .resize::<[_]>(nfft_pow2, Complex::zero())
            .collect();

        fn mul_tuple<T>((a, b): (T, T)) -> <T as Mul>::Output
        where
            T: Mul
        {
            a*b
        }

        fg.dft_scaled(SpectrumScaling::Summed);
        let mut gg = fg.into_bulk()
            .zip(fw)
            .map(mul_tuple)
            .collect::<Vec<_>, _>();
        gg.idft_scaled(SpectrumScaling::Summed);

        for (y, mut x) in gg.into_bulk()
            .zip(w2)
            .skip(length::value::saturating_sub(n, [(); 1]))
            .map(mul_tuple as fn(_) -> _)
            .into_iter()
            .zip(self.bulk_mut())
        {
            *x.borrow_mut() = y
        }

        let bulk = self.bulk_mut();        
        if let Some(norm) = match scaling
        {
            SpectrumScaling::Summed => None,
            SpectrumScaling::Balanced => Some(Float::sqrt(T::from(bulk.len()).unwrap())),
            SpectrumScaling::Averaged => Some(T::from(bulk.len()).unwrap())
        }
        {
            bulk.for_each(|mut x| x.borrow_mut()._div_assign(norm))
        }
    }
}

#[cfg(test)]
mod test
{
    use core::f64::consts::TAU;

    use bulks::{Bulk, IntoBulk};
    use linspace::Linspace;
    use num_complex::Complex;
    use num_traits::One;

    use crate::{Czt, Dft, tests};

    #[test]
    fn plot_czt()
    {
        const N: usize = 1024;
        const T: f64 = 1.0;
        const F: f64 = 220.0;
        const K: usize = 3;
        
        let x: [_; N] = core::array::from_fn(|i| (TAU*F*i as f64/N as f64*T).sin());

        let w = (0.0..TAU).linspace_array::<N>();
        let xf = core::array::from_fn::<_, K, _>(|i| {
            let mut xf = x.map(Complex::from);
            xf.czt(Complex::cis(-TAU/N as f64), Complex::ONE*2.0f64.powi(i as i32 + 1));
            xf
        });

        ezplot::plot_curves("X(n*e^jw)", "plots/x_z_czt.png", xf.map(|xf| w.into_bulk().zip(xf.map(|xf| xf.norm())))).unwrap()
    }

    #[test]
    fn equals_dft()
    {
        let x = [1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11].map(|x| Complex::from(x as f64));

        let rate = Complex::cis(-TAU/x.len() as f64);
        let point = Complex::one();

        let mut y1 = x;
        let mut y2 = x;
        y1.dft();
        y2.czt(rate, point);

        println!("{y1:?}");
        println!("{y2:?}");
        assert!(tests::approx_eq(&y1, &y2, 1e-5))
    }
}