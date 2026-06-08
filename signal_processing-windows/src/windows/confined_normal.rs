use array_trait::length::Length;
use num_traits::{Float, FloatConst, Zero};

use crate::{Shape, WindowFn};

#[derive(Clone, Copy)]
pub struct ConfinedNormal<T>
where
    T: Float
{
    pub sigma: T,
    pub exp: T
}

impl<L, T> WindowFn<L> for ConfinedNormal<T>
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
        let l = T::from(m).unwrap();
        let g = move |x| {
            if m.is_zero()
            {
                return T::one()
            }

            let z: T = (x - half*T::from(m - 1).unwrap())/(self.sigma*two*l);
            (-z.abs().powf(self.exp)).exp()
        };
        let gmul = g(-half)/(g(-half + l) + g(-half - l));
        move |i| {
            if m.is_zero()
            {
                return T::one()
            }

            let i = T::from(i).unwrap();
            g(i) - (g(i + l) + g(i - l))*gmul
        }
    }
}

#[cfg(test)]
mod test
{
    use crate::tests;

    use super::ConfinedNormal;

    #[test]
    fn test()
    {
        tests::plot_window(ConfinedNormal {
            sigma: 0.1,
            exp: 1.5
        })
    }
}