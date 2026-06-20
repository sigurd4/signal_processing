use bulks::{AsBulk, Bulk, IntoBulk};
use num_complex::Complex;
use num_traits::{Float, FloatConst};
use crate::{Permute, util::{DivAssignSpec, fft}};

pub trait Dft: Permute
{
    type ItemReal: Float + FloatConst;

    #[doc(alias = "fft")]
    fn dft(&mut self);

    #[doc(alias = "ifft")]
    fn idft(&mut self);
}
impl<B, T> Dft for B
where
    for<'a> &'a mut B: IntoBulk<Item = &'a mut Complex<T>>,
    B: ?Sized,
    T: Float + FloatConst + 'static
{
    type ItemReal = T;

    fn dft(&mut self)
    {
        fft::fft_unscaled::<_, _, false>(self, None);
    }
    
    fn idft(&mut self)
    {
        fft::fft_unscaled::<_, _, false>(self, None);
        let bulk = self.bulk_mut();
        let norm = T::from(bulk.len()).unwrap();
        bulk.for_each(|x| x._div_assign(norm))
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
    fn it_works_dft()
    {
        const N: usize = 1024;
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
    fn it_works_idft()
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