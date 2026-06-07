use array_trait::length::LengthValue;
use num::complex::ComplexFloat;
use option_trait::Maybe;

pub enum WindowRange
{
    Symmetric,
    Periodic
}

pub const trait WindowGen<T, W, N>
where
    T: ComplexFloat,
    N: Maybe<usize>
{
    type Output: Maybe<W>;

    fn window_gen(&self, numtaps: N, range: WindowRange) -> Self::Output;
}