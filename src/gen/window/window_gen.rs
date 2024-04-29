use num::complex::ComplexFloat;
use option_trait::Maybe;

use crate::quantities::List;

pub enum WindowRange
{
    Symmetric,
    Periodic
}

pub trait WindowGen<T, W, N>
where
    T: ComplexFloat,
    N: Maybe<usize>,
    W: List<T>
{
    type Output: Maybe<W>;

    fn window_gen(&self, numtaps: N, range: WindowRange) -> Self::Output;
}