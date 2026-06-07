use ndarray::{Array1, ArrayView1};


use crate::quantities::{ListOrSingle, MaybeLists};

pub trait MaybeList<T>: MaybeLists<T>
{
    fn as_view_slice_option(&self) -> Option<&'_ [T]>;
    fn to_vec_option(&self) -> Option<Vec<T>>
    where
        T: Clone;
    fn into_vec_option(self) -> Option<Vec<T>>
    where
        T: Clone,
        Self: Sized;
}

impl<T> MaybeList<T> for Vec<T>
{
    fn as_view_slice_option(&self) -> Option<&'_ [T]>
    {
        Some(self.as_slice())
    }
    fn to_vec_option(&self) -> Option<Vec<T>>
    where
        T: Clone
    {
        Some(self.to_vec())
    }
    fn into_vec_option(self) -> Option<Vec<T>>
    where
        T: Clone,
        Self: Sized
    {
        Some(self.into_vec())
    }
}
impl<T> MaybeList<T> for [T]
{
    fn as_view_slice_option(&self) -> Option<&'_ [T]>
    {
        Some(self)
    }
    fn to_vec_option(&self) -> Option<Vec<T>>
    where
        T: Clone
    {
        Some(self.to_vec())
    }
    fn into_vec_option(self) -> Option<Vec<T>>
    where
        T: Clone,
        Self: Sized
    {
        Some(self.into_vec())
    }
}
impl<T, const N: usize> MaybeList<T> for [T; N]
{
    fn as_view_slice_option(&self) -> Option<&'_ [T]>
    {
        Some(self.as_slice())
    }
    fn to_vec_option(&self) -> Option<Vec<T>>
    where
        T: Clone
    {
        Some(self.to_vec())
    }
    fn into_vec_option(self) -> Option<Vec<T>>
    where
        T: Clone,
        Self: Sized
    {
        Some(self.into_vec())
    }
}
impl<'a, T> MaybeList<T> for &'a [T]
{
    fn as_view_slice_option(&self) -> Option<&'_ [T]>
    {
        Some(*self)
    }
    fn to_vec_option(&self) -> Option<Vec<T>>
    where
        T: Clone
    {
        Some(self.to_vec())
    }
    fn into_vec_option(self) -> Option<Vec<T>>
    where
        T: Clone,
        Self: Sized
    {
        Some(self.into_vec())
    }
}
impl<'a, T, const N: usize> MaybeList<T> for &'a [T; N]
{
    fn as_view_slice_option(&self) -> Option<&'_ [T]>
    {
        Some(self.as_slice())
    }
    fn to_vec_option(&self) -> Option<Vec<T>>
    where
        T: Clone
    {
        Some(self.to_vec())
    }
    fn into_vec_option(self) -> Option<Vec<T>>
    where
        T: Clone,
        Self: Sized
    {
        Some(self.into_vec())
    }
}
impl<T> MaybeList<T> for ()
{
    fn as_view_slice_option(&self) -> Option<&'_ [T]>
    {
        None
    }
    fn to_vec_option(&self) -> Option<Vec<T>>
    where
        T: Clone
    {
        None
    }
    fn into_vec_option(self) -> Option<Vec<T>>
    where
        T: Clone,
        Self: Sized
    {
        None
    }
}

impl<T> MaybeList<T> for Array1<T>
{
    fn as_view_slice_option(&self) -> Option<&'_ [T]>
    {
        self.as_slice()
    }
    fn to_vec_option(&self) -> Option<Vec<T>>
    where
        T: Clone
    {
        Some(self.to_vec())
    }
    fn into_vec_option(self) -> Option<Vec<T>>
    where
        T: Clone,
        Self: Sized
    {
        Some(self.into_vec())
    }
}
impl<'a, T> MaybeList<T> for ArrayView1<'a, T>
{
    fn as_view_slice_option(&self) -> Option<&'_ [T]>
    {
        self.as_slice()
    }
    fn to_vec_option(&self) -> Option<Vec<T>>
    where
        T: Clone
    {
        Some(self.to_vec())
    }
    fn into_vec_option(self) -> Option<Vec<T>>
    where
        T: Clone,
        Self: Sized
    {
        Some(self.into_vec())
    }
}