

use ndarray::{Array1, Array2, ArrayBase, ArrayView1, ArrayView2};
use option_trait::StaticMaybe;

use crate::{Container, ListOrSingle, MaybeContainer, MaybeLists};

pub trait Lists<T>: MaybeLists<T> + Container<T>
{
    fn as_views<'a>(&'a self) -> Vec<Self::RowView<'a>>
    where
        T: 'a,
        Self: 'a;
    fn as_view_slices<'a>(&'a self) -> Vec<&'a [T]>
    where
        T: 'a,
        Self: 'a;
    fn height(&self) -> usize;
    fn resize_to_owned<F>(self, size: (<Self::Height as StaticMaybe<usize>>::Opposite, <Self::Width as StaticMaybe<usize>>::Opposite), fill: F) -> Self::Owned
    where
        T: Clone,
        Self: Sized,
        F: FnMut() -> T,
        <Self::Height as StaticMaybe<usize>>::Opposite: Sized,
        <Self::Width as StaticMaybe<usize>>::Opposite: Sized;
    fn to_vecs(&self) -> Vec<Vec<T>>
    where
        T: Clone;
    fn into_vecs(self) -> Vec<Vec<T>>
    where
        Self: Sized,
        T: Clone;
}

impl<T> Lists<T> for Vec<T>
{
    fn as_views<'a>(&'a self) -> Vec<Self::RowView<'a>>
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
    fn height(&self) -> usize
    {
        1
    }
    fn resize_to_owned<F>(mut self, size: (<Self::Height as StaticMaybe<usize>>::Opposite, <Self::Width as StaticMaybe<usize>>::Opposite), fill: F) -> Self::Owned
    where
        T: Clone,
        Self: Sized,
        F: FnMut() -> T
    {
        self.resize_with(size.1, fill);
        self
    }
    fn to_vecs(&self) -> Vec<Vec<T>>
    where
        T: Clone
    {
        vec![self.clone()]
    }
    fn into_vecs(self) -> Vec<Vec<T>>
    where
        Self: Sized,
        T: Clone
    {
        vec![self]
    }
}
impl<T> Lists<T> for [T]
{
    fn as_views<'a>(&'a self) -> Vec<Self::RowView<'a>>
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
    fn height(&self) -> usize
    {
        1
    }
    fn resize_to_owned<F>(self, size: (<Self::Height as StaticMaybe<usize>>::Opposite, <Self::Width as StaticMaybe<usize>>::Opposite), fill: F) -> Self::Owned
    where
        T: Clone,
        Self: Sized,
        F: FnMut() -> T
    {
        let mut v = self.to_vec();
        v.resize_with(size.1, fill);
        v
    }
    fn to_vecs(&self) -> Vec<Vec<T>>
    where
        T: Clone
    {
        vec![self.to_vec()]
    }
    fn into_vecs(self) -> Vec<Vec<T>>
    where
        Self: Sized,
        T: Clone
    {
        vec![self.to_vec()]
    }
}
impl<T, const N: usize> Lists<T> for [T; N]
{
    fn as_views<'a>(&'a self) -> Vec<Self::RowView<'a>>
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
    fn height(&self) -> usize
    {
        1
    }
    fn resize_to_owned<F>(self, ((), ()): (<Self::Height as StaticMaybe<usize>>::Opposite, <Self::Width as StaticMaybe<usize>>::Opposite), _: F) -> Self::Owned
    where
        T: Clone,
        Self: Sized,
        F: FnMut() -> T
    {
        self
    }
    fn to_vecs(&self) -> Vec<Vec<T>>
    where
        T: Clone
    {
        vec![self.as_slice().to_vec()]
    }
    fn into_vecs(self) -> Vec<Vec<T>>
    where
        Self: Sized,
        T: Clone
    {
        vec![self.into_iter().collect()]
    }
}
impl<T> Lists<T> for &[T]
{
    fn as_views<'a>(&'a self) -> Vec<Self::RowView<'a>>
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
    fn height(&self) -> usize
    {
        1
    }
    fn resize_to_owned<F>(self, size: (<Self::Height as StaticMaybe<usize>>::Opposite, <Self::Width as StaticMaybe<usize>>::Opposite), fill: F) -> Self::Owned
    where
        T: Clone,
        Self: Sized,
        F: FnMut() -> T
    {
        let mut v = self.to_vec();
        v.resize_with(size.1, fill);
        v
    }
    fn to_vecs(&self) -> Vec<Vec<T>>
    where
        T: Clone
    {
        vec![self.to_vec()]
    }
    fn into_vecs(self) -> Vec<Vec<T>>
    where
        Self: Sized,
        T: Clone
    {
        vec![self.to_vec()]
    }
}
impl<T, const N: usize> Lists<T> for &[T; N]
{
    fn as_views<'a>(&'a self) -> Vec<Self::RowView<'a>>
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
    fn height(&self) -> usize
    {
        1
    }
    fn resize_to_owned<F>(self, ((), ()): (<Self::Height as StaticMaybe<usize>>::Opposite, <Self::Width as StaticMaybe<usize>>::Opposite), _: F) -> Self::Owned
    where
        T: Clone,
        Self: Sized,
        F: FnMut() -> T
    {
        self.clone()
    }
    fn to_vecs(&self) -> Vec<Vec<T>>
    where
        T: Clone
    {
        vec![self.as_slice().to_vec()]
    }
    fn into_vecs(self) -> Vec<Vec<T>>
    where
        Self: Sized,
        T: Clone
    {
        vec![self.as_slice().to_vec()]
    }
}

impl<T> Lists<T> for Vec<Vec<T>>
{
    fn as_views<'a>(&'a self) -> Vec<Self::RowView<'a>>
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
    fn height(&self) -> usize
    {
        self.len()
    }
    fn resize_to_owned<F>(mut self, size: (<Self::Height as StaticMaybe<usize>>::Opposite, <Self::Width as StaticMaybe<usize>>::Opposite), mut fill: F) -> Self::Owned
    where
        Self: Sized,
        F: FnMut() -> T
    {
        self.resize_with(size.0, || vec![]);
        for v in self.iter_mut()
        {
            v.resize_with(size.1, &mut fill)
        }
        self
    }
    fn to_vecs(&self) -> Vec<Vec<T>>
    where
        T: Clone
    {
        self.clone()
    }
    fn into_vecs(self) -> Vec<Vec<T>>
    where
        Self: Sized,
        T: Clone
    {
        self
    }
}
impl<T, const M: usize> Lists<T> for [Vec<T>; M]
{
    fn as_views<'a>(&'a self) -> Vec<Self::RowView<'a>>
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
    fn height(&self) -> usize
    {
        M
    }
    fn resize_to_owned<F>(mut self, size: (<Self::Height as StaticMaybe<usize>>::Opposite, <Self::Width as StaticMaybe<usize>>::Opposite), mut fill: F) -> Self::Owned
    where
        T: Clone,
        Self: Sized,
        F: FnMut() -> T
    {
        for v in self.iter_mut()
        {
            v.resize_with(size.1, &mut fill);
        }
        self
    }
    fn to_vecs(&self) -> Vec<Vec<T>>
    where
        T: Clone
    {
        self.to_vec()
    }
    fn into_vecs(self) -> Vec<Vec<T>>
    where
        Self: Sized,
        T: Clone
    {
        self.into_vec()
    }
}
impl<T> Lists<T> for [Vec<T>]
{
    fn as_views<'a>(&'a self) -> Vec<Self::RowView<'a>>
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
    fn height(&self) -> usize
    {
        self.len()
    }
    fn resize_to_owned<F>(self, size: (<Self::Height as StaticMaybe<usize>>::Opposite, <Self::Width as StaticMaybe<usize>>::Opposite), mut fill: F) -> Self::Owned
    where
        T: Clone,
        Self: Sized,
        F: FnMut() -> T
    {
        let mut v = self.to_vec();
        v.resize_with(size.0, || vec![]);
        for v in v.iter_mut()
        {
            v.resize_with(size.1, &mut fill)
        }
        v
    }
    fn to_vecs(&self) -> Vec<Vec<T>>
    where
        T: Clone
    {
        self.to_vec()
    }
    fn into_vecs(self) -> Vec<Vec<T>>
    where
        Self: Sized,
        T: Clone
    {
        self.into_vec()
    }
}
impl<T, const M: usize> Lists<T> for &[Vec<T>; M]
{
    fn as_views<'a>(&'a self) -> Vec<Self::RowView<'a>>
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
    fn height(&self) -> usize
    {
        M
    }
    fn resize_to_owned<F>(self, size: (<Self::Height as StaticMaybe<usize>>::Opposite, <Self::Width as StaticMaybe<usize>>::Opposite), mut fill: F) -> Self::Owned
    where
        T: Clone,
        Self: Sized,
        F: FnMut() -> T
    {
        let mut v = self.clone();
        for v in v.iter_mut()
        {
            v.resize_with(size.1, &mut fill)
        }
        v
    }
    fn to_vecs(&self) -> Vec<Vec<T>>
    where
        T: Clone
    {
        self.to_vec()
    }
    fn into_vecs(self) -> Vec<Vec<T>>
    where
        Self: Sized,
        T: Clone
    {
        self.into_vec()
    }
}
impl<T> Lists<T> for &[Vec<T>]
{
    fn as_views<'a>(&'a self) -> Vec<Self::RowView<'a>>
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
    fn height(&self) -> usize
    {
        self.len()
    }
    fn resize_to_owned<F>(self, size: (<Self::Height as StaticMaybe<usize>>::Opposite, <Self::Width as StaticMaybe<usize>>::Opposite), mut fill: F) -> Self::Owned
    where
        T: Clone,
        Self: Sized,
        F: FnMut() -> T
    {
        let mut v = self.to_vec();
        v.resize_with(size.0, || vec![]);
        for v in v.iter_mut()
        {
            v.resize_with(size.1, &mut fill)
        }
        v
    }
    fn to_vecs(&self) -> Vec<Vec<T>>
    where
        T: Clone
    {
        self.to_vec()
    }
    fn into_vecs(self) -> Vec<Vec<T>>
    where
        Self: Sized,
        T: Clone
    {
        self.into_vec()
    }
}

impl<T, const N: usize> Lists<T> for Vec<[T; N]>
{
    fn as_views<'a>(&'a self) -> Vec<Self::RowView<'a>>
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
    fn height(&self) -> usize
    {
        self.len()
    }
    fn resize_to_owned<F>(mut self, size: (<Self::Height as StaticMaybe<usize>>::Opposite, <Self::Width as StaticMaybe<usize>>::Opposite), mut fill: F) -> Self::Owned
    where
        T: Clone,
        Self: Sized,
        F: FnMut() -> T
    {
        self.resize_with(size.0, || core::array::from_fn(|_| fill()));
        self
    }
    fn to_vecs(&self) -> Vec<Vec<T>>
    where
        T: Clone
    {
        self.iter()
            .map(|r| r.to_vec())
            .collect()
    }
    fn into_vecs(self) -> Vec<Vec<T>>
    where
        Self: Sized,
        T: Clone
    {
        self.into_iter()
            .map(|r| r.into_vec())
            .collect()
    }
}
impl<T, const N: usize, const M: usize> Lists<T> for [[T; N]; M]
{
    fn as_views<'a>(&'a self) -> Vec<Self::RowView<'a>>
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
    fn height(&self) -> usize
    {
        M
    }
    fn resize_to_owned<F>(self, ((), ()): (<Self::Height as StaticMaybe<usize>>::Opposite, <Self::Width as StaticMaybe<usize>>::Opposite), _: F) -> Self::Owned
    where
        T: Clone,
        Self: Sized,
        F: FnMut() -> T
    {
        self
    }
    fn to_vecs(&self) -> Vec<Vec<T>>
    where
        T: Clone
    {
        self.iter()
            .map(|r| r.to_vec())
            .collect()
    }
    fn into_vecs(self) -> Vec<Vec<T>>
    where
        Self: Sized,
        T: Clone
    {
        self.into_iter()
            .map(|r| r.into_vec())
            .collect()
    }
}
impl<T, const N: usize> Lists<T> for [[T; N]]
{
    fn as_views<'a>(&'a self) -> Vec<Self::RowView<'a>>
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
    fn height(&self) -> usize
    {
        self.len()
    }
    fn resize_to_owned<F>(self, size: (<Self::Height as StaticMaybe<usize>>::Opposite, <Self::Width as StaticMaybe<usize>>::Opposite), mut fill: F) -> Self::Owned
    where
        T: Clone,
        Self: Sized,
        F: FnMut() -> T
    {
        let mut v = self.to_vec();
        v.resize_with(size.0, || core::array::from_fn(|_| fill()));
        v
    }
    fn to_vecs(&self) -> Vec<Vec<T>>
    where
        T: Clone
    {
        self.iter()
            .map(|r| r.to_vec())
            .collect()
    }
    fn into_vecs(self) -> Vec<Vec<T>>
    where
        Self: Sized,
        T: Clone
    {
        self.iter()
            .map(|r| r.to_vec())
            .collect()
    }
}
impl<T, const N: usize, const M: usize> Lists<T> for &[[T; N]; M]
{
    fn as_views<'a>(&'a self) -> Vec<Self::RowView<'a>>
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
    fn height(&self) -> usize
    {
        M
    }
    fn resize_to_owned<F>(self, ((), ()): (<Self::Height as StaticMaybe<usize>>::Opposite, <Self::Width as StaticMaybe<usize>>::Opposite), _: F) -> Self::Owned
    where
        T: Clone,
        Self: Sized,
        F: FnMut() -> T
    {
        self.clone()
    }
    fn to_vecs(&self) -> Vec<Vec<T>>
    where
        T: Clone
    {
        self.iter()
            .map(|r| r.to_vec())
            .collect()
    }
    fn into_vecs(self) -> Vec<Vec<T>>
    where
        Self: Sized,
        T: Clone
    {
        self.iter()
            .map(|r| r.to_vec())
            .collect()
    }
}
impl<T, const N: usize> Lists<T> for &[[T; N]]
{
    fn as_views<'a>(&'a self) -> Vec<Self::RowView<'a>>
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
    fn height(&self) -> usize
    {
        self.len()
    }
    fn resize_to_owned<F>(self, size: (<Self::Height as StaticMaybe<usize>>::Opposite, <Self::Width as StaticMaybe<usize>>::Opposite), mut fill: F) -> Self::Owned
    where
        T: Clone,
        Self: Sized,
        F: FnMut() -> T
    {
        let mut v = self.to_vec();
        v.resize_with(size.0, || core::array::from_fn(|_| fill()));
        v
    }
    fn to_vecs(&self) -> Vec<Vec<T>>
    where
        T: Clone
    {
        self.iter()
            .map(|r| r.to_vec())
            .collect()
    }
    fn into_vecs(self) -> Vec<Vec<T>>
    where
        Self: Sized,
        T: Clone
    {
        self.iter()
            .map(|r| r.to_vec())
            .collect()
    }
}

impl<T> Lists<T> for Vec<&[T]>
{
    fn as_views<'a>(&'a self) -> Vec<Self::RowView<'a>>
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
    fn height(&self) -> usize
    {
        self.len()
    }
    fn resize_to_owned<F>(self, size: (<Self::Height as StaticMaybe<usize>>::Opposite, <Self::Width as StaticMaybe<usize>>::Opposite), mut fill: F) -> Self::Owned
    where
        T: Clone,
        Self: Sized,
        F: FnMut() -> T
    {
        let mut v: Vec<_> = self.into_iter()
            .map(|r| r.to_vec())
            .collect();
        v.resize_with(size.0, || vec![]);
        for v in v.iter_mut()
        {
            v.resize_with(size.1, &mut fill)
        }
        v
    }
    fn to_vecs(&self) -> Vec<Vec<T>>
    where
        T: Clone
    {
        self.iter()
            .map(|r| r.to_vec())
            .collect()
    }
    fn into_vecs(self) -> Vec<Vec<T>>
    where
        Self: Sized,
        T: Clone
    {
        self.into_iter()
            .map(|r| r.to_vec())
            .collect()
    }
}
impl<T, const M: usize> Lists<T> for [&[T]; M]
{
    fn as_views<'a>(&'a self) -> Vec<Self::RowView<'a>>
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
    fn height(&self) -> usize
    {
        M
    }
    fn resize_to_owned<F>(self, size: (<Self::Height as StaticMaybe<usize>>::Opposite, <Self::Width as StaticMaybe<usize>>::Opposite), mut fill: F) -> Self::Owned
    where
        T: Clone,
        Self: Sized,
        F: FnMut() -> T
    {
        let mut v = self.map(|r| r.to_vec());
        for v in v.iter_mut()
        {
            v.resize_with(size.1, &mut fill)
        }
        v
    }
    fn to_vecs(&self) -> Vec<Vec<T>>
    where
        T: Clone
    {
        self.iter()
            .map(|r| r.to_vec())
            .collect()
    }
    fn into_vecs(self) -> Vec<Vec<T>>
    where
        Self: Sized,
        T: Clone
    {
        self.into_iter()
            .map(|r| r.to_vec())
            .collect()
    }
}
impl<T> Lists<T> for [&[T]]
{
    fn as_views<'a>(&'a self) -> Vec<Self::RowView<'a>>
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
    fn height(&self) -> usize
    {
        self.len()
    }
    fn resize_to_owned<F>(self, size: (<Self::Height as StaticMaybe<usize>>::Opposite, <Self::Width as StaticMaybe<usize>>::Opposite), mut fill: F) -> Self::Owned
    where
        T: Clone,
        Self: Sized,
        F: FnMut() -> T
    {
        let mut v: Vec<_> = self.iter()
            .map(|r| r.to_vec())
            .collect();
        v.resize_with(size.0, || vec![]);
        for v in v.iter_mut()
        {
            v.resize_with(size.1, &mut fill)
        }
        v
    }
    fn to_vecs(&self) -> Vec<Vec<T>>
    where
        T: Clone
    {
        self.iter()
            .map(|r| r.to_vec())
            .collect()
    }
    fn into_vecs(self) -> Vec<Vec<T>>
    where
        Self: Sized,
        T: Clone
    {
        self.iter()
            .map(|r| r.to_vec())
            .collect()
    }
}
impl<T, const M: usize> Lists<T> for &[&[T]; M]
{
    fn as_views<'a>(&'a self) -> Vec<Self::RowView<'a>>
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
    fn height(&self) -> usize
    {
        M
    }
    fn resize_to_owned<F>(self, size: (<Self::Height as StaticMaybe<usize>>::Opposite, <Self::Width as StaticMaybe<usize>>::Opposite), mut fill: F) -> Self::Owned
    where
        T: Clone,
        Self: Sized,
        F: FnMut() -> T
    {
        let mut v = (*self).map(|r| r.to_vec());
        for v in v.iter_mut()
        {
            v.resize_with(size.1, &mut fill)
        }
        v
    }
    fn to_vecs(&self) -> Vec<Vec<T>>
    where
        T: Clone
    {
        self.iter()
            .map(|r| r.to_vec())
            .collect()
    }
    fn into_vecs(self) -> Vec<Vec<T>>
    where
        Self: Sized,
        T: Clone
    {
        self.iter()
            .map(|r| r.to_vec())
            .collect()
    }
}
impl<T> Lists<T> for &[&[T]]
{
    fn as_views<'a>(&'a self) -> Vec<Self::RowView<'a>>
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
    fn height(&self) -> usize
    {
        self.len()
    }
    fn resize_to_owned<F>(self, size: (<Self::Height as StaticMaybe<usize>>::Opposite, <Self::Width as StaticMaybe<usize>>::Opposite), mut fill: F) -> Self::Owned
    where
        T: Clone,
        Self: Sized,
        F: FnMut() -> T
    {
        let mut v: Vec<_> = self.iter()
            .map(|r| r.to_vec())
            .collect();
        v.resize_with(size.0, || vec![]);
        for v in v.iter_mut()
        {
            v.resize_with(size.1, &mut fill)
        }
        v
    }
    fn to_vecs(&self) -> Vec<Vec<T>>
    where
        T: Clone
    {
        self.iter()
            .map(|r| r.to_vec())
            .collect()
    }
    fn into_vecs(self) -> Vec<Vec<T>>
    where
        Self: Sized,
        T: Clone
    {
        self.iter()
            .map(|r| r.to_vec())
            .collect()
    }
}

impl<T, const N: usize> Lists<T> for Vec<&[T; N]>
{
    fn as_views<'a>(&'a self) -> Vec<Self::RowView<'a>>
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
    fn height(&self) -> usize
    {
        self.len()
    }
    fn resize_to_owned<F>(self, size: (<Self::Height as StaticMaybe<usize>>::Opposite, <Self::Width as StaticMaybe<usize>>::Opposite), mut fill: F) -> Self::Owned
    where
        T: Clone,
        Self: Sized,
        F: FnMut() -> T
    {
        let mut v: Vec<_> = self.into_iter()
            .map(|r| r.clone())
            .collect();
        v.resize_with(size.0, || core::array::from_fn(|_| fill()));
        v
    }
    fn to_vecs(&self) -> Vec<Vec<T>>
    where
        T: Clone
    {
        self.iter()
            .map(|r| r.to_vec())
            .collect()
    }
    fn into_vecs(self) -> Vec<Vec<T>>
    where
        Self: Sized,
        T: Clone
    {
        self.into_iter()
            .map(|r| r.to_vec())
            .collect()
    }
}
impl<T, const N: usize, const M: usize> Lists<T> for [&[T; N]; M]
{
    fn as_views<'a>(&'a self) -> Vec<Self::RowView<'a>>
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
    fn height(&self) -> usize
    {
        M
    }
    fn resize_to_owned<F>(self, ((), ()): (<Self::Height as StaticMaybe<usize>>::Opposite, <Self::Width as StaticMaybe<usize>>::Opposite), _: F) -> Self::Owned
    where
        T: Clone,
        Self: Sized,
        F: FnMut() -> T
    {
        self.map(|r| r.clone())
    }
    fn to_vecs(&self) -> Vec<Vec<T>>
    where
        T: Clone
    {
        self.iter()
            .map(|r| r.to_vec())
            .collect()
    }
    fn into_vecs(self) -> Vec<Vec<T>>
    where
        Self: Sized,
        T: Clone
    {
        self.into_iter()
            .map(|r| r.to_vec())
            .collect()
    }
}
impl<T, const N: usize> Lists<T> for [&[T; N]]
{
    fn as_views<'a>(&'a self) -> Vec<Self::RowView<'a>>
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
    fn height(&self) -> usize
    {
        self.len()
    }
    fn resize_to_owned<F>(self, size: (<Self::Height as StaticMaybe<usize>>::Opposite, <Self::Width as StaticMaybe<usize>>::Opposite), mut fill: F) -> Self::Owned
    where
        T: Clone,
        Self: Sized,
        F: FnMut() -> T
    {
        let mut v: Vec<_> = self.iter()
            .map(|&r| r.clone())
            .collect();
        v.resize_with(size.0, || core::array::from_fn(|_| fill()));
        v
    }
    fn to_vecs(&self) -> Vec<Vec<T>>
    where
        T: Clone
    {
        self.iter()
            .map(|r| r.to_vec())
            .collect()
    }
    fn into_vecs(self) -> Vec<Vec<T>>
    where
        Self: Sized,
        T: Clone
    {
        self.iter()
            .map(|r| r.to_vec())
            .collect()
    }
}
impl<T, const N: usize, const M: usize> Lists<T> for &[&[T; N]; M]
{
    fn as_views<'a>(&'a self) -> Vec<Self::RowView<'a>>
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
    fn height(&self) -> usize
    {
        M
    }
    fn resize_to_owned<F>(self, ((), ()): (<Self::Height as StaticMaybe<usize>>::Opposite, <Self::Width as StaticMaybe<usize>>::Opposite), _: F) -> Self::Owned
    where
        T: Clone,
        Self: Sized,
        F: FnMut() -> T
    {
        (*self).map(|r| r.clone())
    }
    fn to_vecs(&self) -> Vec<Vec<T>>
    where
        T: Clone
    {
        self.iter()
            .map(|r| r.to_vec())
            .collect()
    }
    fn into_vecs(self) -> Vec<Vec<T>>
    where
        Self: Sized,
        T: Clone
    {
        self.iter()
            .map(|r| r.to_vec())
            .collect()
    }
}
impl<T, const N: usize> Lists<T> for &[&[T; N]]
{
    fn as_views<'a>(&'a self) -> Vec<Self::RowView<'a>>
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
    fn height(&self) -> usize
    {
        self.len()
    }
    fn resize_to_owned<F>(self, size: (<Self::Height as StaticMaybe<usize>>::Opposite, <Self::Width as StaticMaybe<usize>>::Opposite), mut fill: F) -> Self::Owned
    where
        T: Clone,
        Self: Sized,
        F: FnMut() -> T
    {
        let mut v: Vec<_> = self.iter()
            .map(|&r| r.clone())
            .collect();
        v.resize_with(size.0, || core::array::from_fn(|_| fill()));
        v
    }
    fn to_vecs(&self) -> Vec<Vec<T>>
    where
        T: Clone
    {
        self.iter()
            .map(|r| r.to_vec())
            .collect()
    }
    fn into_vecs(self) -> Vec<Vec<T>>
    where
        Self: Sized,
        T: Clone
    {
        self.iter()
            .map(|r| r.to_vec())
            .collect()
    }
}

impl<T> Lists<T> for Array1<T>
{
    fn as_views<'a>(&'a self) -> Vec<Self::RowView<'a>>
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
    fn height(&self) -> usize
    {
        1
    }
    fn resize_to_owned<F>(self, size: (<Self::Height as StaticMaybe<usize>>::Opposite, <Self::Width as StaticMaybe<usize>>::Opposite), mut fill: F) -> Self::Owned
    where
        T: Clone,
        Self: Sized,
        F: FnMut() -> T
    {
        ArrayBase::from_shape_fn(size.1, |i| self.get(i).map(|x| x.clone()).unwrap_or_else(&mut fill))
    }
    fn to_vecs(&self) -> Vec<Vec<T>>
    where
        T: Clone
    {
        vec![self.to_vec()]
    }
    fn into_vecs(self) -> Vec<Vec<T>>
    where
        Self: Sized,
        T: Clone
    {
        vec![self.to_vec()]
    }
}
impl<'b, T> Lists<T> for ArrayView1<'b, T>
{
    fn as_views<'a>(&'a self) -> Vec<Self::RowView<'a>>
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
    fn height(&self) -> usize
    {
        1
    }
    fn resize_to_owned<F>(self, size: (<Self::Height as StaticMaybe<usize>>::Opposite, <Self::Width as StaticMaybe<usize>>::Opposite), mut fill: F) -> Self::Owned
    where
        T: Clone,
        Self: Sized,
        F: FnMut() -> T
    {
        ArrayBase::from_shape_fn(size.1, |i| self.get(i).map(|x| x.clone()).unwrap_or_else(&mut fill))
    }
    fn to_vecs(&self) -> Vec<Vec<T>>
    where
        T: Clone
    {
        vec![self.to_vec()]
    }
    fn into_vecs(self) -> Vec<Vec<T>>
    where
        Self: Sized,
        T: Clone
    {
        vec![self.to_vec()]
    }
}

impl<T> Lists<T> for Array2<T>
{
    fn as_views<'a>(&'a self) -> Vec<Self::RowView<'a>>
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
    fn height(&self) -> usize
    {
        self.dim().0
    }
    fn resize_to_owned<F>(self, size: (<Self::Height as StaticMaybe<usize>>::Opposite, <Self::Width as StaticMaybe<usize>>::Opposite), mut fill: F) -> Self::Owned
    where
        T: Clone,
        Self: Sized,
        F: FnMut() -> T
    {
        ArrayBase::from_shape_fn(size, |i| self.get(i).map(|x| x.clone()).unwrap_or_else(&mut fill))
    }
    fn to_vecs(&self) -> Vec<Vec<T>>
    where
        T: Clone
    {
        self.rows()
            .into_iter()
            .map(|r| r.to_vec())
            .collect()
    }
    fn into_vecs(self) -> Vec<Vec<T>>
    where
        Self: Sized,
        T: Clone
    {
        self.rows()
            .into_iter()
            .map(|r| r.to_vec())
            .collect()
    }
}
impl<'b, 'c, T> Lists<T> for ArrayView2<'c, T>
where
    'b: 'c,
    Self: 'b
{
    fn as_views<'a>(&'a self) -> Vec<Self::RowView<'a>>
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
    fn height(&self) -> usize
    {
        self.dim().0
    }
    fn resize_to_owned<F>(self, size: (<Self::Height as StaticMaybe<usize>>::Opposite, <Self::Width as StaticMaybe<usize>>::Opposite), mut fill: F) -> Self::Owned
    where
        T: Clone,
        Self: Sized,
        F: FnMut() -> T
    {
        ArrayBase::from_shape_fn(size, |i| self.get(i).map(|x| x.clone()).unwrap_or_else(&mut fill))
    }
    fn to_vecs(&self) -> Vec<Vec<T>>
    where
        T: Clone
    {
        self.rows()
            .into_iter()
            .map(|r| r.to_vec())
            .collect()
    }
    fn into_vecs(self) -> Vec<Vec<T>>
    where
        Self: Sized,
        T: Clone
    {
        self.rows()
            .into_iter()
            .map(|r| r.to_vec())
            .collect()
    }
}