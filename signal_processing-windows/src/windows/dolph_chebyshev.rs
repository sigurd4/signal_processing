use core::borrow::{BorrowMut};

use array_trait::length::{self, Length};
use bulks::{AsBulk, Bulk, CollectNearest};
use num_complex::Complex;
use num_traits::{Float, FloatConst, Zero};
use signal_processing_fourier::DftInplace;

use crate::{Shape, WindowFn};

#[derive(Clone, Copy)]
pub struct DolphChebyshev<T>
where
    T: Float
{
    pub alpha: T
}

impl<L, T> WindowFn<L> for DolphChebyshev<T>
where
    L: Length<Elem = T> + ?Sized,
    T: Float + FloatConst
{
    type Functor = impl Fn(usize) -> T;

    fn window_fn(self, len: L::Value, range: Shape) -> Self::Functor
    {
        let m = range.window_len(len);
        let one = T::one();
        let two = one + one;
        let ten = T::from(10u8).unwrap();
        let l = T::from(m).unwrap();
        let t = |x: T| {
            if x <= -one
            {
                let s = one - two*T::from(m % 2).unwrap();
                s*(l*(-x).acosh()).cosh()
            }
            else if x >= one
            {
                (l*x.acosh()).cosh()
            }
            else
            {
                (l*x.acos()).cos()
            }
        };

        let gamma = ten.powf(-self.alpha);
        let beta = (gamma.recip().acosh()/l).cosh();

        let mut w = bulks::repeat_n((), len)
            .enumerate()
            .map(|(i, ())| {
                let i = T::from(i).unwrap();
                let x = beta*(T::PI()*i/(l + one)).cos();
                Complex::from(t(x))
            }).collect_nearest();
        let wr = if m % 2 == 0
        {
            w.borrow_mut().bulk_mut().dft_inplace();
            let mut wr = bulks::repeat_n(T::zero(), len)
                .collect_nearest();
            let mm = (m + 2)/2;
            for k in 0..mm
            {
                let ww = (w[k]/w[0]).re;
                wr[mm - k - 1] = ww;
                if k + m + 1 - mm < length::value::len(len)
                {
                    wr[k + m + 1 - mm] = ww;
                }
            }
            wr
        }
        else
        {
            for (k, w) in w.borrow_mut().iter_mut()
                .enumerate()
            {
                *w = *w * Complex::cis(T::PI()*T::from(k).unwrap()/(l + one))
            }
            w.borrow_mut().bulk_mut().dft_inplace();
            let mut wr = bulks::repeat_n(T::zero(), len)
                .collect_nearest();
            let mm = (m + 1)/2 + 1;
            for k in 1..mm
            {
                let ww = (w[k]/w[1]).re;
                wr[mm - k - 1] = ww;
                if k + m + 1 - mm < length::value::len(len)
                {
                    wr[k + m + 1 - mm] = ww;
                }
            }
            wr
        };
        move |i| {
            if m.is_zero()
            {
                return T::one()
            }
            
            wr[i]
        }
    }
}

#[cfg(test)]
mod test
{
    use crate::tests;

    use super::DolphChebyshev;

    #[test]
    fn test()
    {
        tests::plot_window(DolphChebyshev {
            alpha: 5.0
        })
    }
}