use ndarray::Array1;

use crate::{MaybeList, OwnedListOrSingle};

pub trait MaybeOwnedList<T>: MaybeList<T> + Sized
{
    fn as_mut_slice_option(&mut self) -> Option<&'_ mut [T]>;
}

impl<T> MaybeOwnedList<T> for ()
{
    fn as_mut_slice_option(&mut self) -> Option<&'_ mut [T]>
    {
        None
    }
}

impl<T> MaybeOwnedList<T> for Vec<T>
{
    fn as_mut_slice_option(&mut self) -> Option<&'_ mut [T]>
    {
        Some(self.as_mut_slice())
    }
}
impl<T, const N: usize> MaybeOwnedList<T> for [T; N]
{
    fn as_mut_slice_option(&mut self) -> Option<&'_ mut [T]>
    {
        Some(self.as_mut_slice())
    }
}

impl<T> MaybeOwnedList<T> for Array1<T>
{
    fn as_mut_slice_option(&mut self) -> Option<&'_ mut [T]>
    {
        Some(self.as_mut_slice())
    }
}