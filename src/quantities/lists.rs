

use ndarray::{Array1, Array2, ArrayBase, ArrayView1, ArrayView2};
use option_trait::StaticMaybe;

use crate::{Container, MaybeContainer, MaybeLists};

pub trait Lists<T>: MaybeLists<T> + Container<T>
{
    type Height: StaticMaybe<usize>;
    type Width: StaticMaybe<usize>;
    const HEIGHT: usize;
    const WIDTH: usize;

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
}

impl<T> Lists<T> for Vec<T>
{
    type Height = usize;
    type Width = ();
    const HEIGHT: usize = 1;
    const WIDTH: usize = usize::MAX;

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
}
impl<T> Lists<T> for [T]
{
    type Height = usize;
    type Width = ();
    const HEIGHT: usize = 1;
    const WIDTH: usize = usize::MAX;

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
}
impl<T, const N: usize> Lists<T> for [T; N]
{
    type Height = usize;
    type Width = usize;
    const HEIGHT: usize = 1;
    const WIDTH: usize = N;

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
}
impl<T> Lists<T> for &[T]
{
    type Height = usize;
    type Width = ();
    const HEIGHT: usize = 1;
    const WIDTH: usize = usize::MAX;

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
}
impl<T, const N: usize> Lists<T> for &[T; N]
{
    type Height = usize;
    type Width = usize;
    const HEIGHT: usize = 1;
    const WIDTH: usize = N;

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
}

impl<T> Lists<T> for Vec<Vec<T>>
{
    type Height = ();
    type Width = ();
    const HEIGHT: usize = usize::MAX;
    const WIDTH: usize = usize::MAX;

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
}
impl<T, const M: usize> Lists<T> for [Vec<T>; M]
{
    type Height = usize;
    type Width = ();
    const HEIGHT: usize = M;
    const WIDTH: usize = usize::MAX;

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
}
impl<T> Lists<T> for [Vec<T>]
{
    type Height = ();
    type Width = ();
    const HEIGHT: usize = usize::MAX;
    const WIDTH: usize = usize::MAX;

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
}
impl<T, const M: usize> Lists<T> for &[Vec<T>; M]
{
    type Height = usize;
    type Width = ();
    const HEIGHT: usize = M;
    const WIDTH: usize = usize::MAX;

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
}
impl<T> Lists<T> for &[Vec<T>]
{
    type Height = ();
    type Width = ();
    const HEIGHT: usize = usize::MAX;
    const WIDTH: usize = usize::MAX;

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
}

impl<T, const N: usize> Lists<T> for Vec<[T; N]>
{
    type Height = ();
    type Width = usize;
    const HEIGHT: usize = usize::MAX;
    const WIDTH: usize = N;

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
}
impl<T, const N: usize, const M: usize> Lists<T> for [[T; N]; M]
{
    type Height = usize;
    type Width = usize;
    const HEIGHT: usize = M;
    const WIDTH: usize = N;

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
}
impl<T, const N: usize> Lists<T> for [[T; N]]
{
    type Height = ();
    type Width = usize;
    const HEIGHT: usize = usize::MAX;
    const WIDTH: usize = N;

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
}
impl<T, const N: usize, const M: usize> Lists<T> for &[[T; N]; M]
{
    type Height = usize;
    type Width = usize;
    const HEIGHT: usize = M;
    const WIDTH: usize = N;

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
}
impl<T, const N: usize> Lists<T> for &[[T; N]]
{
    type Height = ();
    type Width = usize;
    const HEIGHT: usize = usize::MAX;
    const WIDTH: usize = N;

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
}

impl<T> Lists<T> for Vec<&[T]>
{
    type Height = ();
    type Width = ();
    const HEIGHT: usize = usize::MAX;
    const WIDTH: usize = usize::MAX;

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
}
impl<T, const M: usize> Lists<T> for [&[T]; M]
{
    type Height = usize;
    type Width = ();
    const HEIGHT: usize = M;
    const WIDTH: usize = usize::MAX;

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
}
impl<T> Lists<T> for [&[T]]
{
    type Height = ();
    type Width = ();
    const HEIGHT: usize = usize::MAX;
    const WIDTH: usize = usize::MAX;

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
}
impl<T, const M: usize> Lists<T> for &[&[T]; M]
{
    type Height = usize;
    type Width = ();
    const HEIGHT: usize = M;
    const WIDTH: usize = usize::MAX;

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
}
impl<T> Lists<T> for &[&[T]]
{
    type Height = ();
    type Width = ();
    const HEIGHT: usize = usize::MAX;
    const WIDTH: usize = usize::MAX;

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
}

impl<T, const N: usize> Lists<T> for Vec<&[T; N]>
{
    type Height = ();
    type Width = usize;
    const HEIGHT: usize = usize::MAX;
    const WIDTH: usize = N;

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
}
impl<T, const N: usize, const M: usize> Lists<T> for [&[T; N]; M]
{
    type Height = usize;
    type Width = usize;
    const HEIGHT: usize = M;
    const WIDTH: usize = N;

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
}
impl<T, const N: usize> Lists<T> for [&[T; N]]
{
    type Height = ();
    type Width = usize;
    const HEIGHT: usize = usize::MAX;
    const WIDTH: usize = N;

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
}
impl<T, const N: usize, const M: usize> Lists<T> for &[&[T; N]; M]
{
    type Height = usize;
    type Width = usize;
    const HEIGHT: usize = M;
    const WIDTH: usize = N;

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
}
impl<T, const N: usize> Lists<T> for &[&[T; N]]
{
    type Height = ();
    type Width = usize;
    const HEIGHT: usize = usize::MAX;
    const WIDTH: usize = N;

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
}

impl<T> Lists<T> for Array1<T>
{
    type Height = usize;
    type Width = ();
    const HEIGHT: usize = 1;
    const WIDTH: usize = usize::MAX;

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
}
impl<'b, T> Lists<T> for ArrayView1<'b, T>
{
    type Height = usize;
    type Width = ();
    const HEIGHT: usize = 1;
    const WIDTH: usize = usize::MAX;

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
}

impl<T> Lists<T> for Array2<T>
{
    type Height = ();
    type Width = ();
    const HEIGHT: usize = usize::MAX;
    const WIDTH: usize = usize::MAX;

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
}
impl<'b, 'c, T> Lists<T> for ArrayView2<'c, T>
where
    'b: 'c,
    Self: 'b
{
    type Height = ();
    type Width = ();
    const HEIGHT: usize = usize::MAX;
    const WIDTH: usize = usize::MAX;

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
}