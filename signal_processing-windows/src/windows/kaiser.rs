use array_trait::length::Length;
use num_traits::{Float, FloatConst, Zero};

use crate::{Shape, WindowFn, util};

#[derive(Clone, Copy)]
pub struct Kaiser<T>
where
    T: Float
{
    pub beta: T
}

impl<L, T> WindowFn<L> for Kaiser<T>
where
    L: Length<Elem = T> + ?Sized,
    T: Float + FloatConst
{
    type Functor = impl Fn(usize) -> T;

    fn window_fn(self, len: L::Value, range: Shape) -> Self::Functor
    {
        let m = range.window_len(len);

        let i0 = |x| util::i0(x);

        let l = T::from(m).unwrap();
        let one = T::one();
        let two = one + one;
        let d = i0(self.beta);

        move |i| {
            if m.is_zero()
            {
                return T::one()
            }

            let z = two*T::from(i).unwrap()/l - one;
            i0(self.beta*(one - z*z).sqrt())/d
        }
    }
}

#[cfg(test)]
mod test
{
    use core::f64::consts::PI;

use crate::{tests, windows::Kaiser};

    #[test]
    fn test()
    {
        tests::plot_window(Kaiser {
            beta: PI*3.0
        })
    }
}