use array_trait::length::Length;
use num_traits::{Float, FloatConst, Zero};

use crate::{Shape, WindowFn};

#[derive(Clone, Copy)]
pub struct Tukey<T>
where
    T: Float
{
    pub alpha: T
}

impl<L, T> WindowFn<L> for Tukey<T>
where
    L: Length<Elem = T> + ?Sized,
    T: Float + FloatConst
{
    type Functor = impl Fn(usize) -> T;

    fn window_fn(self, len: L::Value, range: Shape) -> Self::Functor
    {
        let m = range.window_len(len);

        let l = T::from(m).unwrap();
        let one = T::one();
        let two = one + one;
        let half = two.recip();

        move |i| {
            if m.is_zero()
            {
                return T::one()
            }

            let i = T::from(if i > m/2 {m - i} else {i}).unwrap();
            if i < self.alpha*l*half
            {
                half*(one - (T::TAU()*i/(l*self.alpha)).cos())
            }
            else
            {
                one
            }
        }
    }
}

#[cfg(test)]
mod test
{
    use crate::{tests, windows::Tukey};

    #[test]
    fn test()
    {
        tests::plot_window(Tukey {
            alpha: 0.5
        })
    }
}