use core::borrow::BorrowMut;

use array_trait::{length::{self, LengthValue}, same::Same};
use bulks::{AsBulk, IntoBulk};

pub const trait ScratchLength<T = <Self as IntoIterator>::Item>: LengthValue
where
    T: Copy
{
    type ScratchSpace: ~const BorrowMut<[T]> + Sized + IntoBulk<Item = T> + AsBulk;

    fn scratch_space(len: Self, fill: T) -> Self::ScratchSpace;
}
impl<I, T> ScratchLength<T> for I
where
    I: LengthValue,
    T: Copy
{
    default type ScratchSpace = Vec<T>;

    default fn scratch_space(len: Self, fill: T) -> Self::ScratchSpace
    {
        vec![fill; length::value::len(len)].same().ok().unwrap()
    }
}
impl<T, const N: usize> const ScratchLength<T> for [(); N]
where
    T: Copy
{
    type ScratchSpace = [T; N];

    fn scratch_space(_len: Self, fill: T) -> Self::ScratchSpace
    {
        [fill; _]
    }
}