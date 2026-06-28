use core::borrow::{Borrow, BorrowMut};

use array_trait::length;
use bulks::{AsBulk, Bulk, CollectNearest, IntoBulk};
use num_complex::{Complex, ComplexFloat};
use crate::{Dft, Permute, SpectrumScaling, util::{IntoComplex, MulAssignSpec, TruncateIm}};

/// Discrete Hilbert transform
pub trait Hilbert<T>: Permute<T>
where
    T: ComplexFloat
{
    /// Discrete Hilbert transform
    fn hilbert(&mut self);
}
impl<B, T> Hilbert<T> for B
where
    for<'a> &'a mut B: IntoBulk<Item: BorrowMut<T>>,
    B: ?Sized,
    T: ComplexFloat + 'static
{
    fn hilbert(&mut self)
    {
        let n = self.bulk_mut().length();
        let n_half = length::value::div(n, [(); 2]);
        
        let mut y = unsafe {(self as *mut Self).as_mut_unchecked()}
            .bulk_mut()
            .map(|x| x.borrow().into_complex())
            .collect_nearest();
        let y: &mut [_] = y.borrow_mut();
        
        y.dft_scaled(SpectrumScaling::Summed);

        let (y1, y2) = y.bulk_mut()
            .skip([(); 1])
            .split_at(n_half);
        y1.for_each(|y| y._mul_assign(-Complex::i()));
        y2.for_each(|y| y._mul_assign(Complex::i()));
        
        y.idft_scaled(SpectrumScaling::Summed);
        
        bulks::zip(y, self.bulk_mut())
            .for_each(|(y, mut x)| *x.borrow_mut() = T::truncate_im(*y));
    }
}

#[cfg(test)]
mod test
{
    use core::f64::consts::TAU;

    use bulks::{Bulk, IntoBulk};
    use linspace::Linspace;

    use crate::Hilbert;

    #[test]
    fn plot_hilbert()
    {
        const N: usize = 1024;
        const F: f64 = 220.0;
        const T: f64 = 4.0/F;
        
        let t = (0.0..T).linspace_array::<N>();
        let x: [_; N] = core::array::from_fn(|i| (TAU*F*i as f64/N as f64*T).sin()*(N - i - 1) as f64/N as f64);

        let mut h = x;
        h.hilbert();

        let e: [_; N] = x.into_bulk().zip(h).map(|(x, h)| x.hypot(h)).collect_array();

        ezplot::plot_curves("x(t), y(t)", "plots/xy_t_hilbert.png", [
                t.into_bulk().zip(x).collect_array(),
                t.into_bulk().zip(h).collect_array(),
                t.into_bulk().zip(e).collect_array()
            ])
            .unwrap()
    }
}