

use ndarray::{Array1, Array2, ArrayBase, ArrayView1, ArrayView2};
use option_trait::StaticMaybe;

use crate::quantities::{Container, ListOrSingle, MaybeContainer, MaybeLists, OwnedMatrix, ListsOrSingle};

pub trait Lists<T>: MaybeLists<T> + Container<T> + ListsOrSingle<T>
{
    type CoercedMatrix: OwnedMatrix<T>;

    fn as_views<'a>(&'a self) -> Vec<Self::RowView<'a>>
    where
        T: 'a,
        Self: 'a;
    fn resize_to_owned<F>(self, size: (<Self::Height as StaticMaybe<usize>>::Opposite, <Self::Width as StaticMaybe<usize>>::Opposite), fill: F) -> Self::Owned
    where
        T: Clone,
        Self: Sized,
        F: FnMut() -> T,
        <Self::Height as StaticMaybe<usize>>::Opposite: Sized,
        <Self::Width as StaticMaybe<usize>>::Opposite: Sized;
    fn coerce_into_matrix<F>(self, or: F) -> Self::CoercedMatrix
    where
        Self: Sized,
        T: Clone,
        F: FnMut() -> T;
}

impl<T> Lists<T> for Vec<T>
{
    type CoercedMatrix = Vec<T>;

    fn as_views<'a>(&'a self) -> Vec<Self::RowView<'a>>
    where
        T: 'a,
        Self: 'a
    {
        vec![self.as_slice()]
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
    fn coerce_into_matrix<F>(self, _or: F) -> Self::CoercedMatrix
    where
        Self: Sized,
        T: Clone,
        F: FnMut() -> T
    {
        self
    }
}
impl<T> Lists<T> for [T]
{
    type CoercedMatrix = Vec<T>;

    fn as_views<'a>(&'a self) -> Vec<Self::RowView<'a>>
    where
        T: 'a,
        Self: 'a
    {
        vec![self]
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
    fn coerce_into_matrix<F>(self, _or: F) -> Self::CoercedMatrix
    where
        Self: Sized,
        T: Clone,
        F: FnMut() -> T
    {
        self.to_vec()
    }
}
impl<T, const N: usize> Lists<T> for [T; N]
{
    type CoercedMatrix = [T; N];

    fn as_views<'a>(&'a self) -> Vec<Self::RowView<'a>>
    where
        T: 'a,
        Self: 'a
    {
        vec![self]
    }
    fn resize_to_owned<F>(self, ((), ()): (<Self::Height as StaticMaybe<usize>>::Opposite, <Self::Width as StaticMaybe<usize>>::Opposite), _: F) -> Self::Owned
    where
        T: Clone,
        Self: Sized,
        F: FnMut() -> T
    {
        self
    }
    fn coerce_into_matrix<F>(self, _or: F) -> Self::CoercedMatrix
    where
        Self: Sized,
        T: Clone,
        F: FnMut() -> T
    {
        self
    }
}
impl<T> Lists<T> for &[T]
{
    type CoercedMatrix = Vec<T>;

    fn as_views<'a>(&'a self) -> Vec<Self::RowView<'a>>
    where
        T: 'a,
        Self: 'a
    {
        vec![*self]
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
    fn coerce_into_matrix<F>(self, _or: F) -> Self::CoercedMatrix
    where
        Self: Sized,
        T: Clone,
        F: FnMut() -> T
    {
        self.to_vec()
    }
}
impl<T, const N: usize> Lists<T> for &[T; N]
{
    type CoercedMatrix = [T; N];

    fn as_views<'a>(&'a self) -> Vec<Self::RowView<'a>>
    where
        T: 'a,
        Self: 'a
    {
        vec![self]
    }
    fn resize_to_owned<F>(self, ((), ()): (<Self::Height as StaticMaybe<usize>>::Opposite, <Self::Width as StaticMaybe<usize>>::Opposite), _: F) -> Self::Owned
    where
        T: Clone,
        Self: Sized,
        F: FnMut() -> T
    {
        self.clone()
    }
    fn coerce_into_matrix<F>(self, _or: F) -> Self::CoercedMatrix
    where
        Self: Sized,
        T: Clone,
        F: FnMut() -> T
    {
        self.clone()
    }
}

impl<T> Lists<T> for Vec<Vec<T>>
{
    type CoercedMatrix = Array2<T>;

    fn as_views<'a>(&'a self) -> Vec<Self::RowView<'a>>
    where
        T: 'a,
        Self: 'a
    {
        self.iter()
            .map(|s| s.as_slice())
            .collect()
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
    fn coerce_into_matrix<F>(self, mut or: F) -> Self::CoercedMatrix
    where
        Self: Sized,
        T: Clone,
        F: FnMut() -> T
    {
        let m = self.len();
        let n = self.iter()
            .map(|v| v.len())
            .max()
            .unwrap_or(0);
        Array2::from_shape_fn((m, n), |(i, j)| self[i].get(j)
            .cloned()
            .unwrap_or_else(&mut or)
        )
    }
}
impl<T, const M: usize> Lists<T> for [Vec<T>; M]
{
    type CoercedMatrix = Array2<T>;

    fn as_views<'a>(&'a self) -> Vec<Self::RowView<'a>>
    where
        T: 'a,
        Self: 'a
    {
        self.iter()
            .map(|s| s.as_slice())
            .collect()
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
    fn coerce_into_matrix<F>(self, mut or: F) -> Self::CoercedMatrix
    where
        Self: Sized,
        T: Clone,
        F: FnMut() -> T
    {
        let n = self.iter()
            .map(|v| v.len())
            .max()
            .unwrap_or(0);
        Array2::from_shape_fn((M, n), |(i, j)| self[i].get(j)
            .cloned()
            .unwrap_or_else(&mut or)
        )
    }
}
impl<T> Lists<T> for [Vec<T>]
{
    type CoercedMatrix = Array2<T>;

    fn as_views<'a>(&'a self) -> Vec<Self::RowView<'a>>
    where
        T: 'a,
        Self: 'a
    {
        self.iter()
            .map(|s| s.as_slice())
            .collect()
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
    fn coerce_into_matrix<F>(self, mut or: F) -> Self::CoercedMatrix
    where
        Self: Sized,
        T: Clone,
        F: FnMut() -> T
    {
        let m = self.len();
        let n = self.iter()
            .map(|v| v.len())
            .max()
            .unwrap_or(0);
        Array2::from_shape_fn((m, n), |(i, j)| self[i].get(j)
            .cloned()
            .unwrap_or_else(&mut or)
        )
    }
}
impl<T, const M: usize> Lists<T> for &[Vec<T>; M]
{
    type CoercedMatrix = Array2<T>;

    fn as_views<'a>(&'a self) -> Vec<Self::RowView<'a>>
    where
        T: 'a,
        Self: 'a
    {
        self.iter()
            .map(|s| s.as_slice())
            .collect()
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
    fn coerce_into_matrix<F>(self, mut or: F) -> Self::CoercedMatrix
    where
        Self: Sized,
        T: Clone,
        F: FnMut() -> T
    {
        let n = self.iter()
            .map(|v| v.len())
            .max()
            .unwrap_or(0);
        Array2::from_shape_fn((M, n), |(i, j)| self[i].get(j)
            .cloned()
            .unwrap_or_else(&mut or)
        )
    }
}
impl<T> Lists<T> for &[Vec<T>]
{
    type CoercedMatrix = Array2<T>;

    fn as_views<'a>(&'a self) -> Vec<Self::RowView<'a>>
    where
        T: 'a,
        Self: 'a
    {
        self.iter()
            .map(|s| s.as_slice())
            .collect()
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
    fn coerce_into_matrix<F>(self, mut or: F) -> Self::CoercedMatrix
    where
        Self: Sized,
        T: Clone,
        F: FnMut() -> T
    {
        let m = self.len();
        let n = self.iter()
            .map(|v| v.len())
            .max()
            .unwrap_or(0);
        Array2::from_shape_fn((m, n), |(i, j)| self[i].get(j)
            .cloned()
            .unwrap_or_else(&mut or)
        )
    }
}

impl<T, const N: usize> Lists<T> for Vec<[T; N]>
{
    type CoercedMatrix = Vec<[T; N]>;

    fn as_views<'a>(&'a self) -> Vec<Self::RowView<'a>>
    where
        T: 'a,
        Self: 'a
    {
        self.iter()
            .collect()
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
    fn coerce_into_matrix<F>(self, _or: F) -> Self::CoercedMatrix
    where
        Self: Sized,
        T: Clone,
        F: FnMut() -> T
    {
        self
    }
}
impl<T, const N: usize, const M: usize> Lists<T> for [[T; N]; M]
{
    type CoercedMatrix = [[T; N]; M];

    fn as_views<'a>(&'a self) -> Vec<Self::RowView<'a>>
    where
        T: 'a,
        Self: 'a
    {
        self.iter()
            .collect()
    }
    fn resize_to_owned<F>(self, ((), ()): (<Self::Height as StaticMaybe<usize>>::Opposite, <Self::Width as StaticMaybe<usize>>::Opposite), _: F) -> Self::Owned
    where
        T: Clone,
        Self: Sized,
        F: FnMut() -> T
    {
        self
    }
    fn coerce_into_matrix<F>(self, _or: F) -> Self::CoercedMatrix
    where
        Self: Sized,
        T: Clone,
        F: FnMut() -> T
    {
        self
    }
}
impl<T, const N: usize> Lists<T> for [[T; N]]
{
    type CoercedMatrix = Vec<[T; N]>;

    fn as_views<'a>(&'a self) -> Vec<Self::RowView<'a>>
    where
        T: 'a,
        Self: 'a
    {
        self.iter()
            .collect()
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
    fn coerce_into_matrix<F>(self, _or: F) -> Self::CoercedMatrix
    where
        Self: Sized,
        T: Clone,
        F: FnMut() -> T
    {
        self.to_vec()
    }
}
impl<T, const N: usize, const M: usize> Lists<T> for &[[T; N]; M]
{
    type CoercedMatrix = [[T; N]; M];

    fn as_views<'a>(&'a self) -> Vec<Self::RowView<'a>>
    where
        T: 'a,
        Self: 'a
    {
        self.iter()
            .collect()
    }
    fn resize_to_owned<F>(self, ((), ()): (<Self::Height as StaticMaybe<usize>>::Opposite, <Self::Width as StaticMaybe<usize>>::Opposite), _: F) -> Self::Owned
    where
        T: Clone,
        Self: Sized,
        F: FnMut() -> T
    {
        self.clone()
    }
    fn coerce_into_matrix<F>(self, _or: F) -> Self::CoercedMatrix
    where
        Self: Sized,
        T: Clone,
        F: FnMut() -> T
    {
        self.clone()
    }
}
impl<T, const N: usize> Lists<T> for &[[T; N]]
{
    type CoercedMatrix = Vec<[T; N]>;

    fn as_views<'a>(&'a self) -> Vec<Self::RowView<'a>>
    where
        T: 'a,
        Self: 'a
    {
        self.iter()
            .collect()
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
    fn coerce_into_matrix<F>(self, _or: F) -> Self::CoercedMatrix
    where
        Self: Sized,
        T: Clone,
        F: FnMut() -> T
    {
        self.to_vec()
    }
}

impl<T> Lists<T> for Vec<&[T]>
{
    type CoercedMatrix = Array2<T>;

    fn as_views<'a>(&'a self) -> Vec<Self::RowView<'a>>
    where
        T: 'a,
        Self: 'a
    {
        self.to_vec()
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
    fn coerce_into_matrix<F>(self, mut or: F) -> Self::CoercedMatrix
    where
        Self: Sized,
        T: Clone,
        F: FnMut() -> T
    {
        let m = self.len();
        let n = self.iter()
            .map(|v| v.len())
            .max()
            .unwrap_or(0);
        Array2::from_shape_fn((m, n), |(i, j)| self[i].get(j)
            .cloned()
            .unwrap_or_else(&mut or)
        )
    }
}
impl<T, const M: usize> Lists<T> for [&[T]; M]
{
    type CoercedMatrix = Array2<T>;

    fn as_views<'a>(&'a self) -> Vec<Self::RowView<'a>>
    where
        T: 'a,
        Self: 'a
    {
        self.iter().copied()
            .collect()
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
    fn coerce_into_matrix<F>(self, mut or: F) -> Self::CoercedMatrix
    where
        Self: Sized,
        T: Clone,
        F: FnMut() -> T
    {
        let n = self.iter()
            .map(|v| v.len())
            .max()
            .unwrap_or(0);
        Array2::from_shape_fn((M, n), |(i, j)| self[i].get(j)
            .cloned()
            .unwrap_or_else(&mut or)
        )
    }
}
impl<T> Lists<T> for [&[T]]
{
    type CoercedMatrix = Array2<T>;

    fn as_views<'a>(&'a self) -> Vec<Self::RowView<'a>>
    where
        T: 'a,
        Self: 'a
    {
        self.to_vec()
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
    fn coerce_into_matrix<F>(self, mut or: F) -> Self::CoercedMatrix
    where
        Self: Sized,
        T: Clone,
        F: FnMut() -> T
    {
        let m = self.len();
        let n = self.iter()
            .map(|v| v.len())
            .max()
            .unwrap_or(0);
        Array2::from_shape_fn((m, n), |(i, j)| self[i].get(j)
            .cloned()
            .unwrap_or_else(&mut or)
        )
    }
}
impl<T, const M: usize> Lists<T> for &[&[T]; M]
{
    type CoercedMatrix = Array2<T>;

    fn as_views<'a>(&'a self) -> Vec<Self::RowView<'a>>
    where
        T: 'a,
        Self: 'a
    {
        self.iter().copied()
            .collect()
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
    fn coerce_into_matrix<F>(self, mut or: F) -> Self::CoercedMatrix
    where
        Self: Sized,
        T: Clone,
        F: FnMut() -> T
    {
        let n = self.iter()
            .map(|v| v.len())
            .max()
            .unwrap_or(0);
        Array2::from_shape_fn((M, n), |(i, j)| self[i].get(j)
            .cloned()
            .unwrap_or_else(&mut or)
        )
    }
}
impl<T> Lists<T> for &[&[T]]
{
    type CoercedMatrix = Array2<T>;

    fn as_views<'a>(&'a self) -> Vec<Self::RowView<'a>>
    where
        T: 'a,
        Self: 'a
    {
        self.to_vec()
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
    fn coerce_into_matrix<F>(self, mut or: F) -> Self::CoercedMatrix
    where
        Self: Sized,
        T: Clone,
        F: FnMut() -> T
    {
        let m = self.len();
        let n = self.iter()
            .map(|v| v.len())
            .max()
            .unwrap_or(0);
        Array2::from_shape_fn((m, n), |(i, j)| self[i].get(j)
            .cloned()
            .unwrap_or_else(&mut or)
        )
    }
}

impl<T, const N: usize> Lists<T> for Vec<&[T; N]>
{
    type CoercedMatrix = Vec<[T; N]>;

    fn as_views<'a>(&'a self) -> Vec<Self::RowView<'a>>
    where
        T: 'a,
        Self: 'a
    {
        self.to_vec()
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
    fn coerce_into_matrix<F>(self, _or: F) -> Self::CoercedMatrix
    where
        Self: Sized,
        T: Clone,
        F: FnMut() -> T
    {
        self.into_iter()
            .map(|v| v.clone())
            .collect()
    }
}
impl<T, const N: usize, const M: usize> Lists<T> for [&[T; N]; M]
{
    type CoercedMatrix = [[T; N]; M];

    fn as_views<'a>(&'a self) -> Vec<Self::RowView<'a>>
    where
        T: 'a,
        Self: 'a
    {
        self.iter().copied()
            .collect()
    }
    fn resize_to_owned<F>(self, ((), ()): (<Self::Height as StaticMaybe<usize>>::Opposite, <Self::Width as StaticMaybe<usize>>::Opposite), _: F) -> Self::Owned
    where
        T: Clone,
        Self: Sized,
        F: FnMut() -> T
    {
        self.map(|r| r.clone())
    }
    fn coerce_into_matrix<F>(self, _or: F) -> Self::CoercedMatrix
    where
        Self: Sized,
        T: Clone,
        F: FnMut() -> T
    {
        self.map(|v| v.clone())
    }
}
impl<T, const N: usize> Lists<T> for [&[T; N]]
{
    type CoercedMatrix = Vec<[T; N]>;

    fn as_views<'a>(&'a self) -> Vec<Self::RowView<'a>>
    where
        T: 'a,
        Self: 'a
    {
        self.to_vec()
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
    fn coerce_into_matrix<F>(self, _or: F) -> Self::CoercedMatrix
    where
        Self: Sized,
        T: Clone,
        F: FnMut() -> T
    {
        self.iter()
            .map(|&v| v.clone())
            .collect()
    }
}
impl<T, const N: usize, const M: usize> Lists<T> for &[&[T; N]; M]
{
    type CoercedMatrix = [[T; N]; M];

    fn as_views<'a>(&'a self) -> Vec<Self::RowView<'a>>
    where
        T: 'a,
        Self: 'a
    {
        self.iter().copied()
            .collect()
    }
    fn resize_to_owned<F>(self, ((), ()): (<Self::Height as StaticMaybe<usize>>::Opposite, <Self::Width as StaticMaybe<usize>>::Opposite), _: F) -> Self::Owned
    where
        T: Clone,
        Self: Sized,
        F: FnMut() -> T
    {
        (*self).map(|r| r.clone())
    }
    fn coerce_into_matrix<F>(self, _or: F) -> Self::CoercedMatrix
    where
        Self: Sized,
        T: Clone,
        F: FnMut() -> T
    {
        self.map(|v| v.clone())
    }
}
impl<T, const N: usize> Lists<T> for &[&[T; N]]
{
    type CoercedMatrix = Vec<[T; N]>;

    fn as_views<'a>(&'a self) -> Vec<Self::RowView<'a>>
    where
        T: 'a,
        Self: 'a
    {
        self.to_vec()
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
    fn coerce_into_matrix<F>(self, _or: F) -> Self::CoercedMatrix
    where
        Self: Sized,
        T: Clone,
        F: FnMut() -> T
    {
        self.iter()
            .map(|&v| v.clone())
            .collect()
    }
}

impl<T> Lists<T> for Array1<T>
{
    type CoercedMatrix = Array1<T>;

    fn as_views<'a>(&'a self) -> Vec<Self::RowView<'a>>
        where
            T: 'a,
            Self: 'a
    {
        vec![self.as_view()]
    }
    fn resize_to_owned<F>(self, size: (<Self::Height as StaticMaybe<usize>>::Opposite, <Self::Width as StaticMaybe<usize>>::Opposite), mut fill: F) -> Self::Owned
    where
        T: Clone,
        Self: Sized,
        F: FnMut() -> T
    {
        ArrayBase::from_shape_fn(size.1, |i| self.get(i).map(|x| x.clone()).unwrap_or_else(&mut fill))
    }
    fn coerce_into_matrix<F>(self, _or: F) -> Self::CoercedMatrix
    where
        Self: Sized,
        T: Clone,
        F: FnMut() -> T
    {
        self
    }
}
impl<'b, T> Lists<T> for ArrayView1<'b, T>
{
    type CoercedMatrix = Array1<T>;

    fn as_views<'a>(&'a self) -> Vec<Self::RowView<'a>>
        where
            T: 'a,
            Self: 'a
    {
        vec![self.as_view().reborrow()]
    }
    fn resize_to_owned<F>(self, size: (<Self::Height as StaticMaybe<usize>>::Opposite, <Self::Width as StaticMaybe<usize>>::Opposite), mut fill: F) -> Self::Owned
    where
        T: Clone,
        Self: Sized,
        F: FnMut() -> T
    {
        ArrayBase::from_shape_fn(size.1, |i| self.get(i).map(|x| x.clone()).unwrap_or_else(&mut fill))
    }
    fn coerce_into_matrix<F>(self, _or: F) -> Self::CoercedMatrix
    where
        Self: Sized,
        T: Clone,
        F: FnMut() -> T
    {
        self.into_owned()
    }
}

impl<T> Lists<T> for Array2<T>
{
    type CoercedMatrix = Array2<T>;

    fn as_views<'a>(&'a self) -> Vec<Self::RowView<'a>>
    where
        T: 'a,
        Self: 'a
    {
        self.rows()
            .into_iter()
            .collect()
    }
    fn resize_to_owned<F>(self, size: (<Self::Height as StaticMaybe<usize>>::Opposite, <Self::Width as StaticMaybe<usize>>::Opposite), mut fill: F) -> Self::Owned
    where
        T: Clone,
        Self: Sized,
        F: FnMut() -> T
    {
        ArrayBase::from_shape_fn(size, |i| self.get(i).map(|x| x.clone()).unwrap_or_else(&mut fill))
    }
    fn coerce_into_matrix<F>(self, _or: F) -> Self::CoercedMatrix
    where
        Self: Sized,
        T: Clone,
        F: FnMut() -> T
    {
        self
    }
}
impl<'b, 'c, T> Lists<T> for ArrayView2<'c, T>
where
    'b: 'c,
    Self: 'b
{
    type CoercedMatrix = Array2<T>;

    fn as_views<'a>(&'a self) -> Vec<Self::RowView<'a>>
    where
        T: 'a,
        Self: 'a
    {
        self.rows()
            .into_iter()
            .collect()
    }
    fn resize_to_owned<F>(self, size: (<Self::Height as StaticMaybe<usize>>::Opposite, <Self::Width as StaticMaybe<usize>>::Opposite), mut fill: F) -> Self::Owned
    where
        T: Clone,
        Self: Sized,
        F: FnMut() -> T
    {
        ArrayBase::from_shape_fn(size, |i| self.get(i).map(|x| x.clone()).unwrap_or_else(&mut fill))
    }
    fn coerce_into_matrix<F>(self, _or: F) -> Self::CoercedMatrix
    where
        Self: Sized,
        T: Clone,
        F: FnMut() -> T
    {
        self.into_owned()
    }
}