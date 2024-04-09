use ndarray::{Array1, Array2};

use crate::{MaybeLists, OwnedLists};

pub trait MaybeOwnedLists<T>: MaybeLists<T> + Sized
{
    fn as_mut_slices_option<'a>(&'a mut self) -> Option<Vec<&'a mut [T]>>
    where
        T: Clone + 'a,
        Self: 'a;
}

impl<T> MaybeOwnedLists<T> for ()
{
    fn as_mut_slices_option<'a>(&'a mut self) -> Option<Vec<&'a mut [T]>>
    where
        T: Clone + 'a,
        Self: 'a
    {
        None
    }
}

impl<T> MaybeOwnedLists<T> for Vec<T>
{
    fn as_mut_slices_option<'a>(&'a mut self) -> Option<Vec<&'a mut [T]>>
    where
        T: Clone + 'a,
        Self: 'a
    {
        Some(self.as_mut_slices())
    }
}
impl<T, const N: usize> MaybeOwnedLists<T> for [T; N]
{
    fn as_mut_slices_option<'a>(&'a mut self) -> Option<Vec<&'a mut [T]>>
    where
        T: Clone + 'a,
        Self: 'a
    {
        Some(self.as_mut_slices())
    }
}

impl<T> MaybeOwnedLists<T> for Vec<Vec<T>>
{
    fn as_mut_slices_option<'a>(&'a mut self) -> Option<Vec<&'a mut [T]>>
    where
        T: Clone + 'a,
        Self: 'a
    {
        Some(self.as_mut_slices())
    }
}
impl<T, const M: usize> MaybeOwnedLists<T> for [Vec<T>; M]
{
    fn as_mut_slices_option<'a>(&'a mut self) -> Option<Vec<&'a mut [T]>>
    where
        T: Clone + 'a,
        Self: 'a
    {
        Some(self.as_mut_slices())
    }
}

impl<T, const N: usize> MaybeOwnedLists<T> for Vec<[T; N]>
{
    fn as_mut_slices_option<'a>(&'a mut self) -> Option<Vec<&'a mut [T]>>
    where
        T: Clone + 'a,
        Self: 'a
    {
        Some(self.as_mut_slices())
    }
}
impl<T, const N: usize, const M: usize> MaybeOwnedLists<T> for [[T; N]; M]
{
    fn as_mut_slices_option<'a>(&'a mut self) -> Option<Vec<&'a mut [T]>>
    where
        T: Clone + 'a,
        Self: 'a
    {
        Some(self.as_mut_slices())
    }
}

impl<T> MaybeOwnedLists<T> for Array1<T>
{
    fn as_mut_slices_option<'a>(&'a mut self) -> Option<Vec<&'a mut [T]>>
    where
        T: Clone + 'a,
        Self: 'a
    {
        Some(self.as_mut_slices())
    }
}

impl<T> MaybeOwnedLists<T> for Array2<T>
{
    fn as_mut_slices_option<'a>(&'a mut self) -> Option<Vec<&'a mut [T]>>
    where
        T: Clone + 'a,
        Self: 'a
    {
        Some(self.as_mut_slices())
    }
}