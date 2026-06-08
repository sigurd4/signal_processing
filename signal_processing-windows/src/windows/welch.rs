use array_trait::length::Length;
use num_traits::{Float, FloatConst, Zero};

use crate::{Shape, WindowFn};

#[derive(Clone, Copy)]
pub struct Welch;

impl<L, T> WindowFn<L> for Welch
where
    L: Length<Elem = T> + ?Sized,
    T: Float + FloatConst
{
    type Functor = impl Fn(usize) -> T;

    fn window_fn(self, len: L::Value, range: Shape) -> Self::Functor
    {
        let m = range.window_len(len);

        let ld2 = T::from(m).unwrap()/T::from(2u8).unwrap();

        move |i| {
            if m.is_zero()
            {
                return T::one()
            }

            let z = (T::from(i).unwrap() - ld2)/ld2;
            T::one() - z*z
        }
    }
}

#[cfg(test)]
mod test
{
    use crate::{tests, windows::Welch};

    #[test]
    fn test()
    {
        tests::plot_window(Welch)
    }
}