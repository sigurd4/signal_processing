use array_trait::length::Length;
use num_traits::{Float, FloatConst, Zero};

use crate::{Shape, WindowFn};

#[derive(Clone, Copy)]
pub struct Bohman;

impl<L, T> WindowFn<L> for Bohman
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
        let mf = T::from(m).unwrap();
        move |i| {
            if m.is_zero()
            {
                return T::one()
            }

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