use array_trait::length::Length;
use num_traits::{Float, FloatConst};

use crate::WindowFn;

#[derive(Clone, Copy)]
pub struct Blackman;

impl<L, T> WindowFn<L> for Blackman
where
    L: Length<Elem = T> + ?Sized,
    T: Float + FloatConst
{
    type Functor = impl Fn(usize) -> T;

    fn window_fn(self, len: usize) -> Self::Functor
    {
        let a0 = T::from(7938.0/18608.0).unwrap();
        let a1 = T::from(9240.0/18608.0).unwrap();
        let a2 = T::from(1430.0/18608.0).unwrap();
        move |i| {
            let z1 = (T::TAU()*T::from(i).unwrap()/T::from(len).unwrap()).cos();
            let z2 = (T::TAU()*T::from(i*2).unwrap()/T::from(len).unwrap()).cos();
            a0 - a1*z1 + a2*z2
        }
    }
}

#[cfg(test)]
mod test
{
    use crate::tests;

    use super::Blackman;

    #[test]
    fn test()
    {
        tests::plot_window(Blackman)
    }
}