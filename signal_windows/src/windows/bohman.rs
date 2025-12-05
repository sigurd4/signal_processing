use array_trait::length::Length;
use num_traits::{Float, FloatConst};

use crate::WindowFn;

#[derive(Clone, Copy)]
pub struct Bohman;

impl<L, T> WindowFn<L> for Bohman
where
    L: Length<Elem = T> + ?Sized,
    T: Float + FloatConst
{
    type Functor = impl Fn(usize) -> T;

    fn window_fn(self, len: usize) -> Self::Functor
    {
        let one = T::one();
        let two = one + one;
        let mf = T::from(len).unwrap();
        move |i| {
            let p = (T::from(i).unwrap() - mf/two).abs()/mf;
            (one - two*p)*(T::TAU()*p).cos() + T::PI().recip()*(T::TAU()*p).sin()
        }
    }
}

#[cfg(test)]
mod test
{
    use crate::tests;

    use super::Bohman;

    #[test]
    fn test()
    {
        tests::plot_window(Bohman)
    }
}