use ndarray::Array1;
use option_trait::StaticMaybe;

use crate::quantities::{MaybeList, OwnedListOrSingle};

use super::{ListOrSingle, MaybeContainer};

pub trait MaybeOwnedList<T>: MaybeList<T> + Sized
{
    fn maybe_from_len_fn<F>(n: <<<Self as MaybeContainer<T>>::Some as ListOrSingle<T>>::Length as StaticMaybe<usize>>::Opposite, f: F) -> Self
    where
        Self: MaybeContainer<T, Some: ListOrSingle<T, Length: StaticMaybe<usize, Opposite: Sized>>>,
        F: FnMut(usize) -> T;

    fn as_mut_slice_option(&mut self) -> Option<&'_ mut [T]>;
}

impl<T> MaybeOwnedList<T> for ()
{
    fn maybe_from_len_fn<F>(_: <<<Self as MaybeContainer<T>>::Some as ListOrSingle<T>>::Length as StaticMaybe<usize>>::Opposite, _: F) -> Self
    where
        Self: MaybeContainer<T, Some: ListOrSingle<T, Length: StaticMaybe<usize, Opposite: Sized>>>,
        F: FnMut(usize) -> T
    {

    }
    fn as_mut_slice_option(&mut self) -> Option<&'_ mut [T]>
    {
        None
    }
}

impl<T> MaybeOwnedList<T> for Vec<T>
{
    fn maybe_from_len_fn<F>(n: <<Self::Some as ListOrSingle<T>>::Length as StaticMaybe<usize>>::Opposite, f: F) -> Self
    where
        F: FnMut(usize) -> T
    {
        Self::from_len_fn(n, f)
    }
    fn as_mut_slice_option(&mut self) -> Option<&'_ mut [T]>
    {
        Some(self.as_mut_slice())
    }
}
impl<T, const N: usize> MaybeOwnedList<T> for [T; N]
{
    fn maybe_from_len_fn<F>(n: <<Self::Some as ListOrSingle<T>>::Length as StaticMaybe<usize>>::Opposite, f: F) -> Self
    where
        F: FnMut(usize) -> T
    {
        Self::from_len_fn(n, f)
    }
    fn as_mut_slice_option(&mut self) -> Option<&'_ mut [T]>
    {
        Some(self.as_mut_slice())
    }
}

impl<T> MaybeOwnedList<T> for Array1<T>
{
    fn maybe_from_len_fn<F>(n: <<Self::Some as ListOrSingle<T>>::Length as StaticMaybe<usize>>::Opposite, f: F) -> Self
    where
        F: FnMut(usize) -> T
    {
        Self::from_len_fn(n, f)
    }
    fn as_mut_slice_option(&mut self) -> Option<&'_ mut [T]>
    {
        Some(self.as_mut_slice())
    }
}