use core::borrow::{Borrow, BorrowMut};

use bulks::{AsBulk, Bulk, DoubleEndedBulk, IntoBulk};
use num_complex::{Complex, ComplexFloat};
use num_traits::{Float, FloatConst};
use crate::{Dft, Permute, util::TruncateIm};

pub trait Dht<T>: Permute<T>
where
    T: ComplexFloat
{
    fn dht(&mut self);
}
impl<B, T> Dht<T> for B
where
    for<'a> &'a mut B: IntoBulk<Item: BorrowMut<T>>,
    for<'a> &'a B: IntoBulk<Item: Borrow<T>, IntoBulk: DoubleEndedBulk>,
    B: ?Sized,
    T: Float + FloatConst + 'static
{
    fn dht(&mut self)
    {
        let mut y = (*self).bulk()
            .map(|x| Complex { re: x.borrow().re(), im: x.borrow().im() })
            .collect::<Vec<_>, _>();
        y.dft();

        for (y, mut x) in y.into_iter()
            .zip(self.bulk_mut())
        {
            let y = y.re - y.im;
            *x.borrow_mut() = <T as TruncateIm>::from_real(y)
        }
    }
}

#[cfg(test)]
mod test
{
    use core::f64::consts::TAU;

    use bulks::{Bulk, IntoBulk};
    use linspace::Linspace;

    use crate::{Dht, tests};

    #[test]
    fn plot_dht()
    {
        const N: usize = 1024;
        const T: f64 = 1.0;
        const F: f64 = 220.0;
        
        let x: [_; N] = core::array::from_fn(|i| (TAU*F*i as f64/N as f64*T).sin());

        let mut xf = x;
        xf.dht();

        let w = (0.0..TAU).linspace_array::<N>();

        ezplot::plot_curves("X(e^jw)", "plots/x_z_dht.png", [w.into_bulk().zip(xf).collect_array()])
            .unwrap()
    }

    #[test]
    fn identities()
    {
        let a = [1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11]
            .into_bulk()
            .map(|x| x as f32)
            .collect_array();

        let mut b = a;
        b.dht();
        b.dht();

        assert!(tests::approx_eq(&a, &b, 1e-5))
    }

    #[test]
    fn test_dht()
    {
        let a = [1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11]
            .into_bulk()
            .map(|x| x as f32)
            .collect_array();

        let mut b = a;
        b.dht();

        println!("{b:?}")
    }
}