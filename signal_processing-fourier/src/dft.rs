use core::borrow::BorrowMut;

use bulks::{AsBulk, Bulk, IntoBulk};
use num_complex::Complex;
use num_traits::{Float, FloatConst};
use crate::{Permute, util::{DivAssignSpec, fft}};

#[derive(Clone, Copy, Debug)]
pub enum SpectrumScaling
{
    Summed,
    Balanced,
    Averaged
}

pub trait Dft<T>: Permute<Complex<T>>
where
    T: Float + FloatConst
{
    #[doc(alias = "fft")]
    fn dft(&mut self)
    {
        self.dft_scaled(SpectrumScaling::Balanced);
    }
    #[doc(alias = "ifft")]
    fn idft(&mut self)
    {
        self.idft_scaled(SpectrumScaling::Balanced);
    }

    #[doc(alias = "fft_scaled")]
    fn dft_scaled(&mut self, scaling: SpectrumScaling);

    #[doc(alias = "ifft_scaled")]
    fn idft_scaled(&mut self, scaling: SpectrumScaling);
}
impl<B, T> Dft<T> for B
where
    for<'a> &'a mut B: IntoBulk<Item: BorrowMut<Complex<T>>>,
    B: ?Sized,
    T: Float + FloatConst + 'static
{
    fn dft_scaled(&mut self, scaling: SpectrumScaling)
    {
        fft::fft_unscaled::<_, _, false>(self, None);

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
    
    fn idft_scaled(&mut self, scaling: SpectrumScaling)
    {
        fft::fft_unscaled::<_, _, true>(self, None);

        let bulk = self.bulk_mut();
        if let Some(norm) = match scaling
        {
            SpectrumScaling::Summed => Some(T::from(bulk.len()).unwrap()),
            SpectrumScaling::Balanced => Some(Float::sqrt(T::from(bulk.len()).unwrap())),
            SpectrumScaling::Averaged => None
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

    use crate::Dft;

    #[test]
    fn plot_dft()
    {
        const N: usize = 2048;
        const T: f64 = 1.0;
        const F: f64 = 220.0;
        
        let x: [_; N] = core::array::from_fn(|i| (TAU*F*i as f64/N as f64*T).sin());

        let w = (0.0..TAU).linspace_array::<N>();
        let mut xf = x.map(Complex::from);
        xf.dft();

        ezplot::plot_curves("X(e^jw)", "plots/x_z_dft.png", [w.into_bulk().zip(xf.map(|xf| xf.norm()))])
            .unwrap()
    }

    #[test]
    fn plot_idft()
    {
        const N: usize = 1024;
        const T: f64 = 0.1;
        const F: f64 = 220.0;
        
        let x: [_; N] = core::array::from_fn(|i| (TAU*F*i as f64/N as f64*T).sin());

        let t: [_; N] = (0.0..T).linspace_array::<N>();
        let mut y = x.map(Complex::from);
        y.dft();
        y.idft();

        ezplot::plot_curves("x(t)", "plots/x_t_idft.png", [t.into_bulk().zip(y.map(|y| y.re)), t.into_bulk().zip(x)])
            .unwrap()
    }
}