use array_trait::length::Length;
use num_traits::{FloatConst, Float};

use crate::{Shape, WindowFn};

#[derive(Clone, Copy)]
pub struct FlatTop;

impl<L, T> WindowFn<L> for FlatTop
where
    L: Length<Elem = T> + ?Sized,
    T: Float + FloatConst
{
    type Functor = impl Fn(usize) -> T;

    fn window_fn(self, len: L::Value, range: Shape) -> Self::Functor
    {
        let m = range.window_len(len);

        let a0 = T::from(0.21557895).unwrap();
        let a1 = T::from(0.41663158).unwrap();
        let a2 = T::from(0.277263158).unwrap();
        let a3 = T::from(0.083578947).unwrap();
        let a4 = T::from(0.006947368).unwrap();
        
        move |i| {
            let z1 = (T::TAU()*T::from(i).unwrap()/T::from(m).unwrap()).cos();
            let z2 = (T::TAU()*T::from(i*2).unwrap()/T::from(m).unwrap()).cos();
            let z3 = (T::TAU()*T::from(i*3).unwrap()/T::from(m).unwrap()).cos();
            let z4 = (T::TAU()*T::from(i*4).unwrap()/T::from(m).unwrap()).cos();
            a0 - a1*z1 + a2*z2 - a3*z3 + a4*z4
        }
    }
}

#[cfg(test)]
mod test
{
    use crate::{tests, windows::FlatTop};

    #[test]
    fn test()
    {
        tests::plot_window(FlatTop)
    }
}