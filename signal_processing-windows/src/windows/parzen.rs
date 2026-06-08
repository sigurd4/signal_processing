use array_trait::length::Length;
use num_traits::{Float, FloatConst, Zero};

use crate::{Shape, WindowFn};

#[derive(Clone, Copy)]
pub struct Parzen;

impl<L, T> WindowFn<L> for Parzen
where
    L: Length<Elem = T> + ?Sized,
    T: Float + FloatConst
{
    type Functor = impl Fn(usize) -> T;

    fn window_fn(self, len: L::Value, range: Shape) -> Self::Functor
    {
        let m = range.window_len(len);

        let ld2 = T::from(m).unwrap()/T::from(2.0).unwrap();
        let ld4 = ld2/T::from(2.0).unwrap();

        move |i| {
            if m.is_zero()
            {
                return T::one()
            }

            let m = T::from(i).unwrap() - T::from(m - 1).unwrap()/T::from(2.0).unwrap();
            let z1 = T::one() - m.abs()/ld2;
            if m.abs() <= ld4
            {
                let z2 = m/ld2;
                T::one() - T::from(6.0).unwrap()*z2*z2*z1
            }
            else
            {
                T::from(2.0).unwrap()*z1*z1*z1
            }
        }
    }
}

#[cfg(test)]
mod test
{
    use crate::{tests, windows::Parzen};

    #[test]
    fn test()
    {
        tests::plot_window(Parzen)
    }
}