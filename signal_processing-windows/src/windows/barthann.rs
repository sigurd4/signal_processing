use core::f64::consts::TAU;

use array_trait::length::Length;
use num_traits::{Float, FloatConst, NumCast, Zero};

use crate::{Shape, WindowFn};

#[derive(Clone, Copy)]
pub struct Barthann;

impl<L, T> WindowFn<L> for Barthann
where
    L: Length<Elem = T> + ?Sized,
    T: Float + FloatConst
{
    type Functor = impl Fn(usize) -> T;

    fn window_fn(self, len: L::Value, range: Shape) -> Self::Functor
    {
        let m = range.window_len(len);
        move |i| {
            if m.is_zero()
            {
                return T::one()
            }

            let p = i as f64/m as f64 - 0.5;
            let g = 0.62 - 0.48*p.abs() + 0.38*(TAU*p).cos();
            NumCast::from(g).unwrap()
        }
    }
}

#[cfg(test)]
mod test
{
    use crate::tests;

    use super::Barthann;

    #[test]
    fn test()
    {
        tests::plot_window(Barthann)
    }
}