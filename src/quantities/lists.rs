

use ndarray::{Array1, Array2, ArrayView1, ArrayView2};

use crate::{Container, MaybeContainer, MaybeLists};

pub trait Lists<T>: MaybeLists<T> + Container<T>
{
    fn as_views<'a>(&'a self) -> Vec<Self::IndexView<'a>>
    where
        T: 'a,
        Self: 'a;
    fn as_view_slices<'a>(&'a self) -> Vec<&'a [T]>
    where
        T: 'a,
        Self: 'a;
}

impl<T> Lists<T> for Vec<T>
{
    fn as_views<'a>(&'a self) -> Vec<Self::IndexView<'a>>
    where
        T: 'a,
        Self: 'a
    {
        vec![self.as_slice()]
    }
    fn as_view_slices<'a>(&'a self) -> Vec<&'a [T]>
    where
        T: 'a,
        Self: 'a
    {
        vec![self.as_slice()]
    }
}
impl<T> Lists<T> for [T]
{
    fn as_views<'a>(&'a self) -> Vec<Self::IndexView<'a>>
    where
        T: 'a,
        Self: 'a
    {
        vec![self]
    }
    fn as_view_slices<'a>(&'a self) -> Vec<&'a [T]>
    where
        T: 'a,
        Self: 'a
    {
        vec![self]
    }
}
impl<T, const N: usize> Lists<T> for [T; N]
{
    fn as_views<'a>(&'a self) -> Vec<Self::IndexView<'a>>
    where
        T: 'a,
        Self: 'a
    {
        vec![self]
    }
    fn as_view_slices<'a>(&'a self) -> Vec<&'a [T]>
    where
        T: 'a,
        Self: 'a
    {
        vec![self.as_slice()]
    }
}
impl<T> Lists<T> for &[T]
{
    fn as_views<'a>(&'a self) -> Vec<Self::IndexView<'a>>
    where
        T: 'a,
        Self: 'a
    {
        vec![*self]
    }
    fn as_view_slices<'a>(&'a self) -> Vec<&'a [T]>
    where
        T: 'a,
        Self: 'a
    {
        vec![*self]
    }
}
impl<T, const N: usize> Lists<T> for &[T; N]
{
    fn as_views<'a>(&'a self) -> Vec<Self::IndexView<'a>>
    where
        T: 'a,
        Self: 'a
    {
        vec![self]
    }
    fn as_view_slices<'a>(&'a self) -> Vec<&'a [T]>
    where
        T: 'a,
        Self: 'a
    {
        vec![self.as_slice()]
    }
}

impl<T> Lists<T> for Vec<Vec<T>>
{
    fn as_views<'a>(&'a self) -> Vec<Self::IndexView<'a>>
    where
        T: 'a,
        Self: 'a
    {
        self.iter()
            .map(|s| s.as_slice())
            .collect()
    }
    fn as_view_slices<'a>(&'a self) -> Vec<&'a [T]>
    where
        T: 'a,
        Self: 'a
    {
        self.iter()
            .map(|s| s.as_slice())
            .collect()
    }
}
impl<T, const M: usize> Lists<T> for [Vec<T>; M]
{
    fn as_views<'a>(&'a self) -> Vec<Self::IndexView<'a>>
    where
        T: 'a,
        Self: 'a
    {
        self.iter()
            .map(|s| s.as_slice())
            .collect()
    }
    fn as_view_slices<'a>(&'a self) -> Vec<&'a [T]>
    where
        T: 'a,
        Self: 'a
    {
        self.iter()
            .map(|s| s.as_slice())
            .collect()
    }
}
impl<T> Lists<T> for [Vec<T>]
{
    fn as_views<'a>(&'a self) -> Vec<Self::IndexView<'a>>
    where
        T: 'a,
        Self: 'a
    {
        self.iter()
            .map(|s| s.as_slice())
            .collect()
    }
    fn as_view_slices<'a>(&'a self) -> Vec<&'a [T]>
    where
        T: 'a,
        Self: 'a
    {
        self.iter()
            .map(|s| s.as_slice())
            .collect()
    }
}
impl<T, const M: usize> Lists<T> for &[Vec<T>; M]
{
    fn as_views<'a>(&'a self) -> Vec<Self::IndexView<'a>>
    where
        T: 'a,
        Self: 'a
    {
        self.iter()
            .map(|s| s.as_slice())
            .collect()
    }
    fn as_view_slices<'a>(&'a self) -> Vec<&'a [T]>
    where
        T: 'a,
        Self: 'a
    {
        self.iter()
            .map(|s| s.as_slice())
            .collect()
    }
}
impl<T> Lists<T> for &[Vec<T>]
{
    fn as_views<'a>(&'a self) -> Vec<Self::IndexView<'a>>
    where
        T: 'a,
        Self: 'a
    {
        self.iter()
            .map(|s| s.as_slice())
            .collect()
    }
    fn as_view_slices<'a>(&'a self) -> Vec<&'a [T]>
    where
        T: 'a,
        Self: 'a
    {
        self.iter()
            .map(|s| s.as_slice())
            .collect()
    }
}

impl<T, const N: usize> Lists<T> for Vec<[T; N]>
{
    fn as_views<'a>(&'a self) -> Vec<Self::IndexView<'a>>
    where
        T: 'a,
        Self: 'a
    {
        self.iter()
            .collect()
    }
    fn as_view_slices<'a>(&'a self) -> Vec<&'a [T]>
    where
        T: 'a,
        Self: 'a
    {
        self.iter()
            .map(|s| s.as_slice())
            .collect()
    }
}
impl<T, const N: usize, const M: usize> Lists<T> for [[T; N]; M]
{
    fn as_views<'a>(&'a self) -> Vec<Self::IndexView<'a>>
    where
        T: 'a,
        Self: 'a
    {
        self.iter()
            .collect()
    }
    fn as_view_slices<'a>(&'a self) -> Vec<&'a [T]>
    where
        T: 'a,
        Self: 'a
    {
        self.iter()
            .map(|s| s.as_slice())
            .collect()
    }
}
impl<T, const N: usize> Lists<T> for [[T; N]]
{
    fn as_views<'a>(&'a self) -> Vec<Self::IndexView<'a>>
    where
        T: 'a,
        Self: 'a
    {
        self.iter()
            .collect()
    }
    fn as_view_slices<'a>(&'a self) -> Vec<&'a [T]>
    where
        T: 'a,
        Self: 'a
    {
        self.iter()
            .map(|s| s.as_slice())
            .collect()
    }
}
impl<T, const N: usize, const M: usize> Lists<T> for &[[T; N]; M]
{
    fn as_views<'a>(&'a self) -> Vec<Self::IndexView<'a>>
    where
        T: 'a,
        Self: 'a
    {
        self.iter()
            .collect()
    }
    fn as_view_slices<'a>(&'a self) -> Vec<&'a [T]>
    where
        T: 'a,
        Self: 'a
    {
        self.iter()
            .map(|s| s.as_slice())
            .collect()
    }
}
impl<T, const N: usize> Lists<T> for &[[T; N]]
{
    fn as_views<'a>(&'a self) -> Vec<Self::IndexView<'a>>
    where
        T: 'a,
        Self: 'a
    {
        self.iter()
            .collect()
    }
    fn as_view_slices<'a>(&'a self) -> Vec<&'a [T]>
    where
        T: 'a,
        Self: 'a
    {
        self.iter()
            .map(|s| s.as_slice())
            .collect()
    }
}

impl<T> Lists<T> for Vec<&[T]>
{
    fn as_views<'a>(&'a self) -> Vec<Self::IndexView<'a>>
    where
        T: 'a,
        Self: 'a
    {
        self.to_vec()
    }
    fn as_view_slices<'a>(&'a self) -> Vec<&'a [T]>
    where
        T: 'a,
        Self: 'a
    {
        self.to_vec()
    }
}
impl<T, const M: usize> Lists<T> for [&[T]; M]
{
    fn as_views<'a>(&'a self) -> Vec<Self::IndexView<'a>>
    where
        T: 'a,
        Self: 'a
    {
        self.iter().copied()
            .collect()
    }
    fn as_view_slices<'a>(&'a self) -> Vec<&'a [T]>
    where
        T: 'a,
        Self: 'a
    {
        self.iter().copied()
            .collect()
    }
}
impl<T> Lists<T> for [&[T]]
{
    fn as_views<'a>(&'a self) -> Vec<Self::IndexView<'a>>
    where
        T: 'a,
        Self: 'a
    {
        self.to_vec()
    }
    fn as_view_slices<'a>(&'a self) -> Vec<&'a [T]>
    where
        T: 'a,
        Self: 'a
    {
        self.to_vec()
    }
}
impl<T, const M: usize> Lists<T> for &[&[T]; M]
{
    fn as_views<'a>(&'a self) -> Vec<Self::IndexView<'a>>
    where
        T: 'a,
        Self: 'a
    {
        self.iter().copied()
            .collect()
    }
    fn as_view_slices<'a>(&'a self) -> Vec<&'a [T]>
    where
        T: 'a,
        Self: 'a
    {
        self.iter().copied()
            .collect()
    }
}
impl<T> Lists<T> for &[&[T]]
{
    fn as_views<'a>(&'a self) -> Vec<Self::IndexView<'a>>
    where
        T: 'a,
        Self: 'a
    {
        self.to_vec()
    }
    fn as_view_slices<'a>(&'a self) -> Vec<&'a [T]>
    where
        T: 'a,
        Self: 'a
    {
        self.to_vec()
    }
}

impl<T, const N: usize> Lists<T> for Vec<&[T; N]>
{
    fn as_views<'a>(&'a self) -> Vec<Self::IndexView<'a>>
    where
        T: 'a,
        Self: 'a
    {
        self.to_vec()
    }
    fn as_view_slices<'a>(&'a self) -> Vec<&'a [T]>
    where
        T: 'a,
        Self: 'a
    {
        self.iter()
            .map(|s| s.as_slice())
            .collect()
    }
}
impl<T, const N: usize, const M: usize> Lists<T> for [&[T; N]; M]
{
    fn as_views<'a>(&'a self) -> Vec<Self::IndexView<'a>>
    where
        T: 'a,
        Self: 'a
    {
        self.iter().copied()
            .collect()
    }
    fn as_view_slices<'a>(&'a self) -> Vec<&'a [T]>
    where
        T: 'a,
        Self: 'a
    {
        self.iter()
            .map(|s| s.as_slice())
            .collect()
    }
}
impl<T, const N: usize> Lists<T> for [&[T; N]]
{
    fn as_views<'a>(&'a self) -> Vec<Self::IndexView<'a>>
    where
        T: 'a,
        Self: 'a
    {
        self.to_vec()
    }
    fn as_view_slices<'a>(&'a self) -> Vec<&'a [T]>
    where
        T: 'a,
        Self: 'a
    {
        self.iter()
            .map(|s| s.as_slice())
            .collect()
    }
}
impl<T, const N: usize, const M: usize> Lists<T> for &[&[T; N]; M]
{
    fn as_views<'a>(&'a self) -> Vec<Self::IndexView<'a>>
    where
        T: 'a,
        Self: 'a
    {
        self.iter().copied()
            .collect()
    }
    fn as_view_slices<'a>(&'a self) -> Vec<&'a [T]>
    where
        T: 'a,
        Self: 'a
    {
        self.iter()
            .map(|s| s.as_slice())
            .collect()
    }
}
impl<T, const N: usize> Lists<T> for &[&[T; N]]
{
    fn as_views<'a>(&'a self) -> Vec<Self::IndexView<'a>>
    where
        T: 'a,
        Self: 'a
    {
        self.to_vec()
    }
    fn as_view_slices<'a>(&'a self) -> Vec<&'a [T]>
    where
        T: 'a,
        Self: 'a
    {
        self.iter()
            .map(|s| s.as_slice())
            .collect()
    }
}

impl<T> Lists<T> for Array1<T>
{
    fn as_views<'a>(&'a self) -> Vec<Self::IndexView<'a>>
        where
            T: 'a,
            Self: 'a
    {
        vec![self.as_view()]
    }
    fn as_view_slices<'a>(&'a self) -> Vec<&'a [T]>
    where
        T: 'a,
        Self: 'a
    {
        vec![self.as_slice().unwrap()]
    }
}
impl<'b, T> Lists<T> for ArrayView1<'b, T>
{
    fn as_views<'a>(&'a self) -> Vec<Self::IndexView<'a>>
        where
            T: 'a,
            Self: 'a
    {
        vec![self.as_view().reborrow()]
    }
    fn as_view_slices<'a>(&'a self) -> Vec<&'a [T]>
    where
        T: 'a,
        Self: 'a
    {
        vec![self.as_slice().unwrap()]
    }
}

impl<T> Lists<T> for Array2<T>
{
    fn as_views<'a>(&'a self) -> Vec<Self::IndexView<'a>>
    where
        T: 'a,
        Self: 'a
    {
        self.rows()
            .into_iter()
            .collect()
    }
    fn as_view_slices<'a>(&'a self) -> Vec<&'a [T]>
    where
        T: 'a,
        Self: 'a
    {
        self.as_slice()
            .unwrap()
            .chunks(self.dim().1)
            .collect()
    }
}
impl<'b, 'c, T> Lists<T> for ArrayView2<'c, T>
where
    'b: 'c,
    Self: 'b
{
    fn as_views<'a>(&'a self) -> Vec<Self::IndexView<'a>>
    where
        T: 'a,
        Self: 'a
    {
        self.rows()
            .into_iter()
            .collect()
    }
    fn as_view_slices<'a>(&'a self) -> Vec<&'a [T]>
    where
        T: 'a,
        Self: 'a
    {
        self.as_slice()
            .unwrap()
            .chunks(self.dim().1)
            .collect()
    }
}