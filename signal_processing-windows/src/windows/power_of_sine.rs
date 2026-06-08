use array_trait::length::Length;
use num_traits::{FloatConst, Float};

use crate::{Shape, WindowFn};

#[derive(Clone, Copy)]
pub struct PowerOfSine<T>
where
    T: Float
{
    pub power: T
}

impl<L, T> WindowFn<L> for PowerOfSine<T>
where
    L: Length<Elem = T> + ?Sized,
    T: Float + FloatConst
{
    type Functor = impl Fn(usize) -> T;

    fn window_fn(self, len: L::Value, range: Shape) -> Self::Functor
    {
        let m = range.window_len(len);

        move |i| {
            (T::PI()*T::from(i).unwrap()/T::from(m).unwrap()).sin()
                .powf(self.power)
        }
    }
}

#[cfg(test)]
mod test
{
    use crate::{tests, windows::PowerOfSine};

    #[test]
    fn test()
    {
        tests::plot_window(PowerOfSine {
            power: 4.0
        })
    }
}