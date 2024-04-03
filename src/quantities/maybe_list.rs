use ndarray::{Array1, ArrayView1};


use crate::MaybeLists;

pub trait MaybeList<T>: MaybeLists<T>
{
    fn as_view_slice_option(&self) -> Option<&'_ [T]>;
}

impl<'a, T> MaybeList<T> for Vec<T>
where
    Self: 'a
{
    fn as_view_slice_option(&self) -> Option<&'_ [T]>
    {
        Some(self.as_slice())
    }
}
impl<'a, T> MaybeList<T> for [T]
where
    Self: 'a
{
    fn as_view_slice_option(&self) -> Option<&'_ [T]>
    {
        Some(self)
    }
}
impl<'a, T, const N: usize> MaybeList<T> for [T; N]
where
    Self: 'a
{
    fn as_view_slice_option(&self) -> Option<&'_ [T]>
    {
        Some(self.as_slice())
    }
}
impl<'a, T> MaybeList<T> for &[T]
where
    Self: 'a
{
    fn as_view_slice_option(&self) -> Option<&'_ [T]>
    {
        Some(*self)
    }
}
impl<'a, T, const N: usize> MaybeList<T> for &[T; N]
where
    Self: 'a
{
    fn as_view_slice_option(&self) -> Option<&'_ [T]>
    {
        Some(self.as_slice())
    }
}
impl<T> MaybeList<T> for ()
{
    fn as_view_slice_option(&self) -> Option<&'_ [T]>
    {
        None
    }
}

impl<'a, T> MaybeList<T> for Array1<T>
where
    Self: 'a
{
    fn as_view_slice_option(&self) -> Option<&'_ [T]>
    {
        self.as_slice()
    }
}
impl<'a, T> MaybeList<T> for ArrayView1<'a, T>
where
    Self: 'a
{
    fn as_view_slice_option(&self) -> Option<&'_ [T]>
    {
        self.as_slice()
    }
}