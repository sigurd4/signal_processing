


use ndarray::{Array1, Array2};

use crate::{Lists};

pub trait OwnedLists<T>: Lists<T> + Sized
{
    fn as_mut_slice2<'a>(&'a mut self) -> Vec<&'a mut [T]>
    where
        T: Clone + 'a,
        Self: 'a;
}

impl<T> OwnedLists<T> for Vec<T>
{
    fn as_mut_slice2<'a>(&'a mut self) -> Vec<&'a mut [T]>
    where
        T: Clone + 'a,
        Self: 'a
    {
        vec![self.as_mut_slice()]
    }
}
impl<T, const N: usize> OwnedLists<T> for [T; N]
{
    fn as_mut_slice2<'a>(&'a mut self) -> Vec<&'a mut [T]>
    where
        T: Clone + 'a,
        Self: 'a
    {
        vec![self.as_mut_slice()]
    }
}

impl<T> OwnedLists<T> for Vec<Vec<T>>
{
    fn as_mut_slice2<'a>(&'a mut self) -> Vec<&'a mut [T]>
    where
        T: Clone + 'a,
        Self: 'a
    {
        self.iter_mut()
            .map(|s| s.as_mut_slice())
            .collect()
    }
}
impl<T, const M: usize> OwnedLists<T> for [Vec<T>; M]
{
    fn as_mut_slice2<'a>(&'a mut self) -> Vec<&'a mut [T]>
    where
        T: Clone + 'a,
        Self: 'a
    {
        self.iter_mut()
            .map(|s| s.as_mut_slice())
            .collect()
    }
}

impl<T, const N: usize> OwnedLists<T> for Vec<[T; N]>
{
    fn as_mut_slice2<'a>(&'a mut self) -> Vec<&'a mut [T]>
    where
        T: Clone + 'a,
        Self: 'a
    {
        self.iter_mut()
            .map(|s| s.as_mut_slice())
            .collect()
    }
}
impl<T, const N: usize, const M: usize> OwnedLists<T> for [[T; N]; M]
{
    fn as_mut_slice2<'a>(&'a mut self) -> Vec<&'a mut [T]>
    where
        T: Clone + 'a,
        Self: 'a
    {
        self.iter_mut()
            .map(|s| s.as_mut_slice())
            .collect()
    }
}

impl<T> OwnedLists<T> for Array1<T>
{
    fn as_mut_slice2<'a>(&'a mut self) -> Vec<&'a mut [T]>
    where
        T: Clone + 'a,
        Self: 'a
    {
        vec![self.as_slice_mut().unwrap()]
    }
}

impl<T> OwnedLists<T> for Array2<T>
{
    fn as_mut_slice2<'a>(&'a mut self) -> Vec<&'a mut [T]>
    where
        T: Clone + 'a,
        Self: 'a
    {
        let r_len = self.dim().1;
        if !self.is_standard_layout()
        {
            *self = self.as_standard_layout()
                .try_into_owned_nocopy()
                .unwrap_or_else(|x| x.to_owned())
        }
        self.as_slice_mut()
            .unwrap()
            .chunks_mut(r_len)
            .collect()
    }
}