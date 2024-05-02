use ndarray::{Array1, Array2, ArrayView1, ArrayView2};

use crate::quantities::ListsOrSingle;

pub trait MatrixOrSingle<T>: ListsOrSingle<T>
{
    fn matrix_dim(&self) -> (usize, usize);

    fn to_array2(&self) -> Array2<T>
    where
        T: Clone;
}

impl<T> MatrixOrSingle<T> for T
{
    fn matrix_dim(&self) -> (usize, usize)
    {
        (1, 1)
    }

    fn to_array2(&self) -> Array2<T>
    where
        T: Clone
    {
        Array2::from_elem((1, 1), self.clone())
    }
}

impl<T> MatrixOrSingle<T> for Vec<T>
{
    fn matrix_dim(&self) -> (usize, usize)
    {
        (1, self.len())
    }

    fn to_array2(&self) -> Array2<T>
    where
        T: Clone
    {
        Array2::from_shape_fn(MatrixOrSingle::<T>::matrix_dim(self), |(_, i)| self[i].clone())
    }
}
impl<T> MatrixOrSingle<T> for [T]
{
    fn matrix_dim(&self) -> (usize, usize)
    {
        (1, self.len())
    }

    fn to_array2(&self) -> Array2<T>
    where
        T: Clone
    {
        Array2::from_shape_fn(MatrixOrSingle::<T>::matrix_dim(self), |(_, i)| self[i].clone())
    }
}
impl<T, const N: usize> MatrixOrSingle<T> for [T; N]
{
    fn matrix_dim(&self) -> (usize, usize)
    {
        (1, N)
    }

    fn to_array2(&self) -> Array2<T>
    where
        T: Clone
    {
        Array2::from_shape_fn(MatrixOrSingle::<T>::matrix_dim(self), |(_, i)| self[i].clone())
    }
}
impl<T> MatrixOrSingle<T> for &[T]
{
    fn matrix_dim(&self) -> (usize, usize)
    {
        (1, self.len())
    }

    fn to_array2(&self) -> Array2<T>
    where
        T: Clone
    {
        Array2::from_shape_fn(MatrixOrSingle::<T>::matrix_dim(self), |(_, i)| self[i].clone())
    }
}
impl<T, const N: usize> MatrixOrSingle<T> for &[T; N]
{
    fn matrix_dim(&self) -> (usize, usize)
    {
        (1, N)
    }

    fn to_array2(&self) -> Array2<T>
    where
        T: Clone
    {
        Array2::from_shape_fn(MatrixOrSingle::<T>::matrix_dim(self), |(_, i)| self[i].clone())
    }
}

impl<T, const N: usize, const M: usize> MatrixOrSingle<T> for [[T; N]; M]
{
    fn matrix_dim(&self) -> (usize, usize)
    {
        (M, N)
    }

    fn to_array2(&self) -> Array2<T>
    where
        T: Clone
    {
        Array2::from_shape_fn((M, N), |(i, j)| self[i][j].clone())
    }
}
impl<T, const N: usize, const M: usize> MatrixOrSingle<T> for &[[T; N]; M]
{
    fn matrix_dim(&self) -> (usize, usize)
    {
        (M, N)
    }

    fn to_array2(&self) -> Array2<T>
    where
        T: Clone
    {
        Array2::from_shape_fn((M, N), |(i, j)| self[i][j].clone())
    }
}

impl<T, const N: usize, const M: usize> MatrixOrSingle<T> for [&[T; N]; M]
{
    fn matrix_dim(&self) -> (usize, usize)
    {
        (M, N)
    }

    fn to_array2(&self) -> Array2<T>
    where
        T: Clone
    {
        Array2::from_shape_fn((M, N), |(i, j)| self[i][j].clone())
    }
}
impl<T, const N: usize, const M: usize> MatrixOrSingle<T> for &[&[T; N]; M]
{
    fn matrix_dim(&self) -> (usize, usize)
    {
        (M, N)
    }

    fn to_array2(&self) -> Array2<T>
    where
        T: Clone
    {
        Array2::from_shape_fn((M, N), |(i, j)| self[i][j].clone())
    }
}

impl<T, const N: usize> MatrixOrSingle<T> for Vec<[T; N]>
{
    fn matrix_dim(&self) -> (usize, usize)
    {
        (self.len(), N)
    }

    fn to_array2(&self) -> Array2<T>
    where
        T: Clone
    {
        Array2::from_shape_fn((self.len(), N), |(i, j)| self[i][j].clone())
    }
}
impl<T, const N: usize> MatrixOrSingle<T> for &[[T; N]]
{
    fn matrix_dim(&self) -> (usize, usize)
    {
        (self.len(), N)
    }

    fn to_array2(&self) -> Array2<T>
    where
        T: Clone
    {
        Array2::from_shape_fn((self.len(), N), |(i, j)| self[i][j].clone())
    }
}
impl<T, const N: usize> MatrixOrSingle<T> for [[T; N]]
{
    fn matrix_dim(&self) -> (usize, usize)
    {
        (self.len(), N)
    }

    fn to_array2(&self) -> Array2<T>
    where
        T: Clone
    {
        Array2::from_shape_fn((self.len(), N), |(i, j)| self[i][j].clone())
    }
}

impl<T, const N: usize> MatrixOrSingle<T> for Vec<&[T; N]>
{
    fn matrix_dim(&self) -> (usize, usize)
    {
        (self.len(), N)
    }

    fn to_array2(&self) -> Array2<T>
    where
        T: Clone
    {
        Array2::from_shape_fn((self.len(), N), |(i, j)| self[i][j].clone())
    }
}
impl<T, const N: usize> MatrixOrSingle<T> for &[&[T; N]]
{
    fn matrix_dim(&self) -> (usize, usize)
    {
        (self.len(), N)
    }

    fn to_array2(&self) -> Array2<T>
    where
        T: Clone
    {
        Array2::from_shape_fn((self.len(), N), |(i, j)| self[i][j].clone())
    }
}
impl<T, const N: usize> MatrixOrSingle<T> for [&[T; N]]
{
    fn matrix_dim(&self) -> (usize, usize)
    {
        (self.len(), N)
    }

    fn to_array2(&self) -> Array2<T>
    where
        T: Clone
    {
        Array2::from_shape_fn((self.len(), N), |(i, j)| self[i][j].clone())
    }
}

impl<T> MatrixOrSingle<T> for Array1<T>
{
    fn matrix_dim(&self) -> (usize, usize)
    {
        (1, self.len())
    }

    fn to_array2(&self) -> Array2<T>
        where
            T: Clone
    {
        Array2::from_shape_fn(MatrixOrSingle::<T>::matrix_dim(self), |(_, j)| self[j].clone())
    }
}
impl<'b, T> MatrixOrSingle<T> for ArrayView1<'b, T>
{
    fn matrix_dim(&self) -> (usize, usize)
    {
        (1, self.len())
    }

    fn to_array2(&self) -> Array2<T>
        where
            T: Clone
    {
        Array2::from_shape_fn(MatrixOrSingle::<T>::matrix_dim(self), |(_, j)| self[j].clone())
    }
}

impl<T> MatrixOrSingle<T> for Array2<T>
{
    fn matrix_dim(&self) -> (usize, usize)
    {
        self.dim()
    }

    fn to_array2(&self) -> Array2<T>
    where
        T: Clone
    {
        self.clone()
    }
}
impl<'b, T> MatrixOrSingle<T> for ArrayView2<'b, T>
{
    fn matrix_dim(&self) -> (usize, usize)
    {
        self.dim()
    }

    fn to_array2(&self) -> Array2<T>
    where
        T: Clone
    {
        self.to_owned()
    }
}