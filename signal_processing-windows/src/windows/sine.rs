use array_trait::length::Length;
use num_traits::{Float, FloatConst, Zero};

use crate::{Shape, WindowFn};

#[derive(Clone, Copy)]
pub struct Sine;

impl<L, T> WindowFn<L> for Sine
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

            (T::PI()*T::from(i).unwrap()/T::from(m).unwrap()).sin()
        }
    }
}

#[cfg(test)]
mod test
{
    use crate::{tests, windows::Sine};

    #[test]
    fn test()
    {
        tests::plot_window(Sine)
    }
}