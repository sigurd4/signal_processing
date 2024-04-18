

use array_math::ArrayOps;
use ndarray::{Array1, ArrayView1};


use crate::{IntoList, ListOrSingle, Matrix, MaybeList, OwnedList};

pub trait List<T>: MaybeList<T> + Matrix<T> + ListOrSingle<T> + IntoList<T, Self, ()>
{
    type ResizedList<const L: usize>: OwnedList<T>;

    fn static_resize_list<const L: usize>(self, dyn_length: usize, fill: impl FnMut() -> T) -> Self::ResizedList<L>
    where
        T: Clone,
        Self: Sized;
}

impl<T> List<T> for Vec<T>
{
    type ResizedList<const L: usize> = Vec<T>;
    
    fn static_resize_list<const L: usize>(mut self, dyn_length: usize, fill: impl FnMut() -> T) -> Self::ResizedList<L>
    where
        T: Clone,
        Self: Sized
    {
        self.resize_with(dyn_length, fill);
        self
    }
}
impl<T> List<T> for [T]
{
    type ResizedList<const L: usize> = Vec<T>;
    
    fn static_resize_list<const L: usize>(self, dyn_length: usize, fill: impl FnMut() -> T) -> Self::ResizedList<L>
    where
        T: Clone,
        Self: Sized
    {
        let mut v = self.to_vec();
        v.resize_with(dyn_length, fill);
        v
    }
}
impl<T, const N: usize> List<T> for [T; N]
{
    type ResizedList<const L: usize> = [T; L];
    
    fn static_resize_list<const L: usize>(self, _: usize, mut fill: impl FnMut() -> T) -> Self::ResizedList<L>
    where
        T: Clone,
        Self: Sized
    {
        self.resize(|_| fill())
    }
}
impl<T> List<T> for &[T]
{
    type ResizedList<const L: usize> = Vec<T>;
    
    fn static_resize_list<const L: usize>(self, dyn_length: usize, fill: impl FnMut() -> T) -> Self::ResizedList<L>
    where
        T: Clone,
        Self: Sized
    {
        let mut v = self.to_vec();
        v.resize_with(dyn_length, fill);
        v
    }
}
impl<T, const N: usize> List<T> for &[T; N]
{
    type ResizedList<const L: usize> = [T; L];
    
    fn static_resize_list<const L: usize>(self, _: usize, mut fill: impl FnMut() -> T) -> Self::ResizedList<L>
    where
        T: Clone,
        Self: Sized
    {
        self.clone().resize(|_| fill())
    }
}

impl<T> List<T> for Array1<T>
{
    type ResizedList<const L: usize> = Vec<T>;

    fn static_resize_list<const L: usize>(self, dyn_length: usize, fill: impl FnMut() -> T) -> Self::ResizedList<L>
    where
        T: Clone,
        Self: Sized
    {
        let mut v = self.to_vec();
        v.resize_with(dyn_length, fill);
        v
    }
}
impl<'a, T> List<T> for ArrayView1<'a, T>
{
    type ResizedList<const L: usize> = Vec<T>;

    fn static_resize_list<const L: usize>(self, dyn_length: usize, fill: impl FnMut() -> T) -> Self::ResizedList<L>
    where
        T: Clone,
        Self: Sized
    {
        let mut v = self.to_vec();
        v.resize_with(dyn_length, fill);
        v
    }
}