use array_trait::length::Length;
use num_traits::One;

use crate::WindowFn;

#[derive(Clone, Copy)]
pub struct Boxcar;

impl<L, T> WindowFn<L> for Boxcar
where
    L: Length<Elem = T> + ?Sized,
    T: One
{
    type Functor = impl Fn(usize) -> T;

    fn window_fn(self, _len: usize) -> Self::Functor
    {
        move |_| T::one()
    }
}

#[cfg(test)]
mod test
{
    use crate::tests;

    use super::Boxcar;

    #[test]
    fn test()
    {
        tests::plot_window(Boxcar)
    }
}