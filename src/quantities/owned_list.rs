use ndarray::Array1;

use crate::{List, MaybeOwnedList};

pub trait OwnedList<T>: List<T> + MaybeOwnedList<T> + Sized
{
    fn as_mut_slice(&mut self) -> &'_ mut [T];
}

impl<T> OwnedList<T> for Vec<T>
{
    fn as_mut_slice(&mut self) -> &'_ mut [T]
    {
        self
    }
}
impl<T, const N: usize> OwnedList<T> for [T; N]
{
    fn as_mut_slice(&mut self) -> &'_ mut [T]
    {
        self.as_mut_slice()
    }
}

impl<T> OwnedList<T> for Array1<T>
{
    fn as_mut_slice(&mut self) -> &'_ mut [T]
    {
        self.as_slice_mut().unwrap()
    }
}