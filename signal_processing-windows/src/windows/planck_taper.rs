use array_trait::length::Length;
use num_traits::{Float, FloatConst, Zero};

use crate::{Shape, WindowFn};

#[derive(Clone, Copy)]
pub struct PlanckTaper<T>
where
    T: Float
{
    pub epsilon: T
}

impl<L, T> WindowFn<L> for PlanckTaper<T>
where
    L: Length<Elem = T> + ?Sized,
    T: Float + FloatConst
{
    type Functor = impl Fn(usize) -> T;

    fn window_fn(self, len: L::Value, range: Shape) -> Self::Functor
    {
        let m = range.window_len(len);

        let l = T::from(m).unwrap();
        let zero = T::zero();
        let one = T::one();

        move |i| {
            if m.is_zero()
            {
                return T::one()
            }

            let i = T::from(if i > m/2 {m - i} else {i}).unwrap();
            if i.is_zero()
            {
                zero
            }
            else if i < self.epsilon*l
            {
                (one + (self.epsilon*l*(i.recip() - (self.epsilon*l - i).recip())).exp()).recip()
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
    use crate::{tests, windows::PlanckTaper};

    #[test]
    fn test()
    {
        tests::plot_window(PlanckTaper {
            epsilon: 0.1
        })
    }
}