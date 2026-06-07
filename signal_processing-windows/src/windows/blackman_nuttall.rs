use array_trait::length::Length;
use num_traits::{Float, FloatConst};

use crate::WindowFn;

#[derive(Clone, Copy)]
pub struct BlackmanNuttall;

impl<L, T> WindowFn<L> for BlackmanNuttall
where
    L: Length<Elem = T> + ?Sized,
    T: Float + FloatConst
{
    type Functor = impl Fn(usize) -> T;

    fn window_fn(self, len: usize) -> Self::Functor
    {
        let a0 = T::from(0.3635819).unwrap();
        let a1 = T::from(0.4891775).unwrap();
        let a2 = T::from(0.1365995).unwrap();
        let a3 = T::from(0.0106411).unwrap();
        move |i| {
            let z1 = (T::TAU()*T::from(i).unwrap()/T::from(len).unwrap()).cos();
            let z2 = (T::TAU()*T::from(i*2).unwrap()/T::from(len).unwrap()).cos();
            let z3 = (T::TAU()*T::from(i*3).unwrap()/T::from(len).unwrap()).cos();
            a0 - a1*z1 + a2*z2 - a3*z3
        }
    }
}

#[cfg(test)]
mod test
{
    use crate::tests;

    use super::BlackmanNuttall;

    #[test]
    fn test()
    {
        tests::plot_window(BlackmanNuttall)
    }
}