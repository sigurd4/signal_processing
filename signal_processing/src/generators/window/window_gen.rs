use array_trait::length::LengthValue;
use bulks::Bulk;
use num::complex::ComplexFloat;

pub enum WindowRange
{
    Symmetric,
    Periodic
}

pub const trait WindowGen<T, N>
where
    T: ComplexFloat,
    N: LengthValue
{
    type Output: Bulk<Item = T>;

    fn window_gen(&self, numtaps: N, range: WindowRange) -> Self::Output;
}