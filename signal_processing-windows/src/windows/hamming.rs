use array_trait::length::Length;
use num_traits::{Float, FloatConst, Zero};

use crate::{Shape, WindowFn};

#[derive(Clone, Copy)]
pub struct Hamming;

impl<L, T> WindowFn<L> for Hamming
where
    L: Length<Elem = T> + ?Sized,
    T: Float + FloatConst
{
    type Functor = impl Fn(usize) -> T;

    fn window_fn(self, len: L::Value, range: Shape) -> Self::Functor
    {
        let m = range.window_len(len);

        let a0 = T::from(25.0/46.0).unwrap();

        move |i| {
            if m.is_zero()
            {
                return T::one()
            }

            let z = (T::TAU()*T::from(i).unwrap()/T::from(m).unwrap()).cos();
            a0 - (T::one() - a0)*z
        }
    }
}

#[cfg(test)]
mod test
{
    use crate::{tests, windows::Hamming};

    #[test]
    fn test()
    {
        tests::plot_window(Hamming)
    }
}