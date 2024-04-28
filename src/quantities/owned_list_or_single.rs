use ndarray::Array1;
use option_trait::StaticMaybe;

use crate::ListOrSingle;

pub trait OwnedListOrSingle<T>: ListOrSingle<T> + Sized
{
    fn from_len_fn<F>(n: <Self::Length as StaticMaybe<usize>>::Opposite, f: F) -> Self
    where
        F: FnMut(usize) -> T;
    fn as_mut_slice(&mut self) -> &'_ mut [T];
}

impl<T> OwnedListOrSingle<T> for T
{
    fn from_len_fn<F>((): <Self::Length as StaticMaybe<usize>>::Opposite, mut f: F) -> Self
    where
        F: FnMut(usize) -> T
    {
        f(0)
    }
    fn as_mut_slice(&mut self) -> &'_ mut [T]
    {
        core::slice::from_mut(self)
    }
}

impl<T> OwnedListOrSingle<T> for Vec<T>
{
    fn from_len_fn<F>(n: <Self::Length as StaticMaybe<usize>>::Opposite, f: F) -> Self
    where
        F: FnMut(usize) -> T
    {
        (0..n).map(f)
            .collect()
    }
    fn as_mut_slice(&mut self) -> &'_ mut [T]
    {
        self
    }
}
impl<T, const N: usize> OwnedListOrSingle<T> for [T; N]
{
    fn from_len_fn<F>((): <Self::Length as StaticMaybe<usize>>::Opposite, f: F) -> Self
    where
        F: FnMut(usize) -> T
    {
        core::array::from_fn(f)
    }
    fn as_mut_slice(&mut self) -> &'_ mut [T]
    {
        self.as_mut_slice()
    }
}

impl<T> OwnedListOrSingle<T> for Array1<T>
{
    fn from_len_fn<F>(n: <Self::Length as StaticMaybe<usize>>::Opposite, f: F) -> Self
    where
        F: FnMut(usize) -> T
    {
        Self::from_shape_fn(n, f)
    }
    fn as_mut_slice(&mut self) -> &'_ mut [T]
    {
        self.as_slice_mut().unwrap()
    }
}