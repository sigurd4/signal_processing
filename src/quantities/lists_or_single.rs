

use ndarray::{Array1, Array2, ArrayView1, ArrayView2};

use crate::quantities::{ListOrSingle, ContainerOrSingle};

pub trait ListsOrSingle<T>: ContainerOrSingle<T>
{
    fn as_view_slices<'a>(&'a self) -> Vec<&'a [T]>
    where
        T: 'a,
        Self: 'a;
    fn height(&self) -> usize;
    fn to_vecs(&self) -> Vec<Vec<T>>
    where
        T: Clone;
    fn into_vecs(self) -> Vec<Vec<T>>
    where
        Self: Sized,
        T: Clone;
}

impl<T> ListsOrSingle<T> for T
{
    fn as_view_slices<'a>(&'a self) -> Vec<&'a [T]>
    where
        T: 'a,
        Self: 'a
    {
        vec![core::slice::from_ref(self)]
    }
    fn height(&self) -> usize
    {
        1
    }
    fn to_vecs(&self) -> Vec<Vec<T>>
    where
        T: Clone
    {
        vec![vec![self.clone()]]
    }
    fn into_vecs(self) -> Vec<Vec<T>>
    where
        Self: Sized,
        T: Clone
    {
        vec![vec![self]]
    }
}

impl<T> ListsOrSingle<T> for Vec<T>
{
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
impl<T> ListsOrSingle<T> for [T]
{
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
impl<T, const N: usize> ListsOrSingle<T> for [T; N]
{
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
impl<T> ListsOrSingle<T> for &[T]
{
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
impl<T, const N: usize> ListsOrSingle<T> for &[T; N]
{
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

impl<T> ListsOrSingle<T> for Vec<Vec<T>>
{
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
impl<T, const M: usize> ListsOrSingle<T> for [Vec<T>; M]
{
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
impl<T> ListsOrSingle<T> for [Vec<T>]
{
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
impl<T, const M: usize> ListsOrSingle<T> for &[Vec<T>; M]
{
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
impl<T> ListsOrSingle<T> for &[Vec<T>]
{
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

impl<T, const N: usize> ListsOrSingle<T> for Vec<[T; N]>
{
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
impl<T, const N: usize, const M: usize> ListsOrSingle<T> for [[T; N]; M]
{
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
impl<T, const N: usize> ListsOrSingle<T> for [[T; N]]
{
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
impl<T, const N: usize, const M: usize> ListsOrSingle<T> for &[[T; N]; M]
{
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
impl<T, const N: usize> ListsOrSingle<T> for &[[T; N]]
{
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

impl<T> ListsOrSingle<T> for Vec<&[T]>
{
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
impl<T, const M: usize> ListsOrSingle<T> for [&[T]; M]
{
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
impl<T> ListsOrSingle<T> for [&[T]]
{
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
impl<T, const M: usize> ListsOrSingle<T> for &[&[T]; M]
{
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
impl<T> ListsOrSingle<T> for &[&[T]]
{
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

impl<T, const N: usize> ListsOrSingle<T> for Vec<&[T; N]>
{
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
impl<T, const N: usize, const M: usize> ListsOrSingle<T> for [&[T; N]; M]
{
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
impl<T, const N: usize> ListsOrSingle<T> for [&[T; N]]
{
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
impl<T, const N: usize, const M: usize> ListsOrSingle<T> for &[&[T; N]; M]
{
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
impl<T, const N: usize> ListsOrSingle<T> for &[&[T; N]]
{
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

impl<T> ListsOrSingle<T> for Array1<T>
{
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
impl<'b, T> ListsOrSingle<T> for ArrayView1<'b, T>
{
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

impl<T> ListsOrSingle<T> for Array2<T>
{
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
impl<'b, 'c, T> ListsOrSingle<T> for ArrayView2<'c, T>
where
    'b: 'c,
    Self: 'b
{
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