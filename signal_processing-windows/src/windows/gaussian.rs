use array_trait::length::Length;
use num_traits::{Float, FloatConst, Zero};

use crate::{Shape, WindowFn};

#[derive(Clone, Copy)]
pub struct Gaussian<T>
where
    T: Float
{
    pub sigma: T
}

impl<L, T> WindowFn<L> for Gaussian<T>
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
        let half = two.recip();

        move |i| {
            if m.is_zero()
            {
                return T::one()
            }

            let z = (T::from(i).unwrap() - half*T::from(m - 1).unwrap())/(self.sigma*half*T::from(m).unwrap());
            (-half*z*z).exp()
        }
    }
}

#[cfg(test)]
mod test
{
    use crate::{tests, windows::Gaussian};

    #[test]
    fn test()
    {
        tests::plot_window(Gaussian {
            sigma: 0.4
        })
    }
}