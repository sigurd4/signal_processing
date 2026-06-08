use array_trait::length::Length;
use num_traits::{Float, FloatConst, Zero};

use crate::{Shape, WindowFn};

#[derive(Clone, Copy)]
pub struct Nuttall;

impl<L, T> WindowFn<L> for Nuttall
where
    L: Length<Elem = T> + ?Sized,
    T: Float + FloatConst
{
    type Functor = impl Fn(usize) -> T;

    fn window_fn(self, len: L::Value, range: Shape) -> Self::Functor
    {
        let m = range.window_len(len);

        let a0 = T::from(0.355768).unwrap();
        let a1 = T::from(0.487396).unwrap();
        let a2 = T::from(0.144232).unwrap();
        let a3 = T::from(0.012604).unwrap();

        move |i| {
            if m.is_zero()
            {
                return T::one()
            }

            let z1 = (T::TAU()*T::from(i).unwrap()/T::from(m).unwrap()).cos();
            let z2 = (T::TAU()*T::from(i*2).unwrap()/T::from(m).unwrap()).cos();
            let z3 = (T::TAU()*T::from(i*3).unwrap()/T::from(m).unwrap()).cos();
            a0 - a1*z1 + a2*z2 - a3*z3
        }
    }
}

#[cfg(test)]
mod test
{
    use crate::{tests, windows::Nuttall};

    #[test]
    fn test()
    {
        tests::plot_window(Nuttall)
    }
}