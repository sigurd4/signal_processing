use array_trait::length::Length;
use num_traits::{Float, FloatConst, Zero};

use crate::{Shape, WindowFn};

#[derive(Clone, Copy)]
#[doc(alias = "Hanning")]
pub struct Hann;

impl<L, T> WindowFn<L> for Hann
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

            let s = (T::PI()*T::from(i).unwrap()/T::from(m).unwrap()).sin();
            s*s
        }
    }
}

#[cfg(test)]
mod test
{
    use crate::{tests, windows::Hann};

    #[test]
    fn test()
    {
        tests::plot_window(Hann)
    }
}