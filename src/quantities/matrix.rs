use array_math::{Array2dOps, ArrayOps};
use ndarray::{Array1, Array2, ArrayView1, ArrayView2};
use option_trait::Maybe;

use crate::{ListOrSingle, Lists, MaybeLists, OwnedMatrix};

pub trait Matrix<T>: Lists<T>
{
    type Height: Maybe<usize>;
    type Width: Maybe<usize>;
    const HEIGHT: usize;
    const WIDTH: usize;
    type Transpose: OwnedMatrix<T>;
    type ColsMapped<M>: ListOrSingle<M> = <Self::Transpose as MaybeLists<T>>::RowsMapped<M>;

    fn matrix_dim(&self) -> (usize, usize);

    fn to_array2(&self) -> Array2<T>
    where
        T: Clone;

    fn matrix_transpose(&self) -> Self::Transpose
    where
        T: Clone;
}

impl<T> Matrix<T> for Vec<T>
{
    type Height = usize;
    type Width = ();
    const HEIGHT: usize = 1;
    const WIDTH: usize = usize::MAX;
    type Transpose = Array2<T>;

    fn matrix_dim(&self) -> (usize, usize)
    {
        (1, self.len())
    }

    fn to_array2(&self) -> Array2<T>
    where
        T: Clone
    {
        Array2::from_shape_fn(self.matrix_dim(), |(_, i)| self[i].clone())
    }

    fn matrix_transpose(&self) -> Self::Transpose
    where
        T: Clone
    {
        Array2::from_shape_fn((self.len(), 1), |(i, _)| self[i].clone())
    }
}
impl<T> Matrix<T> for [T]
{
    type Height = usize;
    type Width = ();
    const HEIGHT: usize = 1;
    const WIDTH: usize = usize::MAX;
    type Transpose = Array2<T>;

    fn matrix_dim(&self) -> (usize, usize)
    {
        (1, self.len())
    }

    fn to_array2(&self) -> Array2<T>
    where
        T: Clone
    {
        Array2::from_shape_fn(self.matrix_dim(), |(_, i)| self[i].clone())
    }

    fn matrix_transpose(&self) -> Self::Transpose
    where
        T: Clone
    {
        Array2::from_shape_fn((self.len(), 1), |(i, _)| self[i].clone())
    }
}
impl<T, const N: usize> Matrix<T> for [T; N]
{
    type Height = usize;
    type Width = usize;
    const HEIGHT: usize = 1;
    const WIDTH: usize = N;
    type Transpose = [[T; 1]; N];

    fn matrix_dim(&self) -> (usize, usize)
    {
        (1, N)
    }

    fn to_array2(&self) -> Array2<T>
    where
        T: Clone
    {
        Array2::from_shape_fn(self.matrix_dim(), |(_, i)| self[i].clone())
    }
    
    fn matrix_transpose(&self) -> Self::Transpose
    where
        T: Clone
    {
        self.as_collumn().clone()
    }
}
impl<T> Matrix<T> for &[T]
{
    type Height = usize;
    type Width = ();
    const HEIGHT: usize = 1;
    const WIDTH: usize = usize::MAX;
    type Transpose = Array2<T>;

    fn matrix_dim(&self) -> (usize, usize)
    {
        (1, self.len())
    }

    fn to_array2(&self) -> Array2<T>
    where
        T: Clone
    {
        Array2::from_shape_fn(self.matrix_dim(), |(_, i)| self[i].clone())
    }
    
    fn matrix_transpose(&self) -> Self::Transpose
    where
        T: Clone
    {
        Array2::from_shape_fn((self.len(), 1), |(i, _)| self[i].clone())
    }
}
impl<T, const N: usize> Matrix<T> for &[T; N]
{
    type Height = usize;
    type Width = usize;
    const HEIGHT: usize = 1;
    const WIDTH: usize = N;
    type Transpose = [[T; 1]; N];

    fn matrix_dim(&self) -> (usize, usize)
    {
        (1, N)
    }

    fn to_array2(&self) -> Array2<T>
    where
        T: Clone
    {
        Array2::from_shape_fn(self.matrix_dim(), |(_, i)| self[i].clone())
    }
    
    fn matrix_transpose(&self) -> Self::Transpose
    where
        T: Clone
    {
        self.as_collumn().clone()
    }
}

impl<T, const N: usize, const M: usize> Matrix<T> for [[T; N]; M]
{
    type Height = usize;
    type Width = usize;
    const HEIGHT: usize = M;
    const WIDTH: usize = N;
    type Transpose = [[T; M]; N];

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

    fn matrix_transpose(&self) -> Self::Transpose
    where
        T: Clone
    {
        self.clone().transpose()
    }
}
impl<T, const N: usize, const M: usize> Matrix<T> for &[[T; N]; M]
{
    type Height = usize;
    type Width = usize;
    const HEIGHT: usize = M;
    const WIDTH: usize = N;
    type Transpose = [[T; M]; N];

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

    fn matrix_transpose(&self) -> Self::Transpose
    where
        T: Clone
    {
        (*self).clone().transpose()
    }
}

impl<T, const N: usize, const M: usize> Matrix<T> for [&[T; N]; M]
{
    type Height = usize;
    type Width = usize;
    const HEIGHT: usize = M;
    const WIDTH: usize = N;
    type Transpose = [[T; M]; N];

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

    fn matrix_transpose(&self) -> Self::Transpose
    where
        T: Clone
    {
        self.clone().map(|r| r.clone()).transpose()
    }
}
impl<T, const N: usize, const M: usize> Matrix<T> for &[&[T; N]; M]
{
    type Height = usize;
    type Width = usize;
    const HEIGHT: usize = M;
    const WIDTH: usize = N;
    type Transpose = [[T; M]; N];

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

    fn matrix_transpose(&self) -> Self::Transpose
    where
        T: Clone
    {
        (*self).clone().map(|r| r.clone()).transpose()
    }
}

impl<T, const N: usize> Matrix<T> for Vec<[T; N]>
{
    type Height = ();
    type Width = usize;
    const HEIGHT: usize = usize::MAX;
    const WIDTH: usize = N;
    type Transpose = Array2<T>;

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

    fn matrix_transpose(&self) -> Self::Transpose
    where
        T: Clone
    {
        Array2::from_shape_fn((N, self.len()), |(i, j)| self[i][j].clone())
    }
}
impl<T, const N: usize> Matrix<T> for &[[T; N]]
{
    type Height = ();
    type Width = usize;
    const HEIGHT: usize = usize::MAX;
    const WIDTH: usize = N;
    type Transpose = Array2<T>;

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

    fn matrix_transpose(&self) -> Self::Transpose
    where
        T: Clone
    {
        Array2::from_shape_fn((N, self.len()), |(i, j)| self[i][j].clone())
    }
}
impl<T, const N: usize> Matrix<T> for [[T; N]]
{
    type Height = ();
    type Width = usize;
    const HEIGHT: usize = usize::MAX;
    const WIDTH: usize = N;
    type Transpose = Array2<T>;

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

    fn matrix_transpose(&self) -> Self::Transpose
    where
        T: Clone
    {
        Array2::from_shape_fn((N, self.len()), |(i, j)| self[i][j].clone())
    }
}

impl<T, const N: usize> Matrix<T> for Vec<&[T; N]>
{
    type Height = ();
    type Width = usize;
    const HEIGHT: usize = usize::MAX;
    const WIDTH: usize = N;
    type Transpose = Array2<T>;

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

    fn matrix_transpose(&self) -> Self::Transpose
    where
        T: Clone
    {
        Array2::from_shape_fn((N, self.len()), |(i, j)| self[i][j].clone())
    }
}
impl<T, const N: usize> Matrix<T> for &[&[T; N]]
{
    type Height = ();
    type Width = usize;
    const HEIGHT: usize = usize::MAX;
    const WIDTH: usize = N;
    type Transpose = Array2<T>;

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

    fn matrix_transpose(&self) -> Self::Transpose
    where
        T: Clone
    {
        Array2::from_shape_fn((N, self.len()), |(i, j)| self[i][j].clone())
    }
}
impl<T, const N: usize> Matrix<T> for [&[T; N]]
{
    type Height = ();
    type Width = usize;
    const HEIGHT: usize = usize::MAX;
    const WIDTH: usize = N;
    type Transpose = Array2<T>;

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

    fn matrix_transpose(&self) -> Self::Transpose
    where
        T: Clone
    {
        Array2::from_shape_fn((N, self.len()), |(i, j)| self[i][j].clone())
    }
}

impl<T> Matrix<T> for Array1<T>
{
    type Height = usize;
    type Width = ();
    const HEIGHT: usize = 1;
    const WIDTH: usize = usize::MAX;
    type Transpose = Array2<T>;

    fn matrix_dim(&self) -> (usize, usize)
    {
        (1, self.len())
    }

    fn to_array2(&self) -> Array2<T>
        where
            T: Clone
    {
        Array2::from_shape_fn(self.matrix_dim(), |(_, j)| self[j].clone())
    }

    fn matrix_transpose(&self) -> Self::Transpose
    where
        T: Clone
    {
        Array2::from_shape_fn((self.len(), 1), |(j, _)| self[j].clone())
    }
}
impl<'b, T> Matrix<T> for ArrayView1<'b, T>
{
    type Height = ();
    type Width = ();
    const HEIGHT: usize = usize::MAX;
    const WIDTH: usize = usize::MAX;
    type Transpose = Array2<T>;

    fn matrix_dim(&self) -> (usize, usize)
    {
        (1, self.len())
    }

    fn to_array2(&self) -> Array2<T>
        where
            T: Clone
    {
        Array2::from_shape_fn(self.matrix_dim(), |(_, j)| self[j].clone())
    }
    
    fn matrix_transpose(&self) -> Self::Transpose
    where
        T: Clone
    {
        Array2::from_shape_fn((self.len(), 1), |(j, _)| self[j].clone())
    }
}

impl<T> Matrix<T> for Array2<T>
{
    type Height = ();
    type Width = ();
    const HEIGHT: usize = usize::MAX;
    const WIDTH: usize = usize::MAX;
    type Transpose = Array2<T>;

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

    fn matrix_transpose(&self) -> Self::Transpose
    where
        T: Clone
    {
        self.t()
            .to_owned()
    }
}
impl<'b, T> Matrix<T> for ArrayView2<'b, T>
{
    type Height = ();
    type Width = ();
    const HEIGHT: usize = usize::MAX;
    const WIDTH: usize = usize::MAX;
    type Transpose = Array2<T>;

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
    fn matrix_transpose(&self) -> Self::Transpose
    where
        T: Clone
    {
        self.t()
            .to_owned()
    }
}