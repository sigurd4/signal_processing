use core::f64::consts::TAU;

use num_traits::{Float, FloatConst, NumCast};

use crate::WindowFn;

#[derive(Clone, Copy)]
pub struct Barthann;

impl<T> WindowFn<T> for Barthann
where
    T: Float + FloatConst
{
    type Functor = impl Fn(usize) -> T;

    fn window_fn(self, length: usize) -> Self::Functor
    {
        move |i| {
            let p = i as f64/length as f64 - 0.5;
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