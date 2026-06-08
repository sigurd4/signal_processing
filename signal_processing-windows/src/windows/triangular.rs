use array_trait::length::Length;
use num_traits::{Float, FloatConst, Zero};

use crate::{Shape, WindowFn};

#[derive(Clone, Copy)]
#[doc(alias = "Bartlett")]
pub struct Triangular;

impl<L, T> WindowFn<L> for Triangular
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

        move |i| {
            if m.is_zero()
            {
                return T::one()
            }

            one - ((T::from(i).unwrap()*two - T::from(m - 1).unwrap())/T::from(m).unwrap()).abs()
        }
    }
}

#[cfg(test)]
mod test
{
    use crate::{tests, windows::Triangular};

    #[test]
    fn test()
    {
        tests::plot_window(Triangular)
    }
}