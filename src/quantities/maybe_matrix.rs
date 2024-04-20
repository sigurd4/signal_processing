use array_math::{Array2dOps, ArrayOps};
use ndarray::{Array1, Array2, ArrayView1, ArrayView2};

use crate::{ListOrSingle, Matrix, MaybeLists, MaybeOwnedMatrix};

pub trait MaybeMatrix<T>: MaybeLists<T>
{
    type Transpose: MaybeOwnedMatrix<T>;
    type ColsMapped<M>: ListOrSingle<M> = <Self::Transpose as MaybeLists<T>>::RowsMapped<M>;

    fn to_array2_option(&self) -> Option<Array2<T>>
    where
        T: Clone;

    fn matrix_transpose(&self) -> Self::Transpose
    where
        T: Clone;
}

impl<T> MaybeMatrix<T> for ()
{
    type Transpose = ();

    fn to_array2_option(&self) -> Option<Array2<T>>
    where
        T: Clone
    {
        None
    }

    fn matrix_transpose(&self) -> Self::Transpose
    where
        T: Clone
    {
        
    }
}

impl<T> MaybeMatrix<T> for Vec<T>
{
    type Transpose = Array2<T>;

    fn to_array2_option(&self) -> Option<Array2<T>>
    where
        T: Clone
    {
        Some(self.to_array2())
    }

    fn matrix_transpose(&self) -> Self::Transpose
    where
        T: Clone
    {
        Array2::from_shape_fn((self.len(), 1), |(i, _)| self[i].clone())
    }
}
impl<T> MaybeMatrix<T> for [T]
{
    type Transpose = Array2<T>;

    fn to_array2_option(&self) -> Option<Array2<T>>
    where
        T: Clone
    {
        Some(self.to_array2())
    }

    fn matrix_transpose(&self) -> Self::Transpose
    where
        T: Clone
    {
        Array2::from_shape_fn((self.len(), 1), |(i, _)| self[i].clone())
    }
}
impl<T, const N: usize> MaybeMatrix<T> for [T; N]
{
    type Transpose = [[T; 1]; N];

    fn to_array2_option(&self) -> Option<Array2<T>>
    where
        T: Clone
    {
        Some(self.to_array2())
    }
    
    fn matrix_transpose(&self) -> Self::Transpose
    where
        T: Clone
    {
        self.as_collumn().clone()
    }
}
impl<T> MaybeMatrix<T> for &[T]
{
    type Transpose = Array2<T>;

    fn to_array2_option(&self) -> Option<Array2<T>>
    where
        T: Clone
    {
        Some(self.to_array2())
    }
    
    fn matrix_transpose(&self) -> Self::Transpose
    where
        T: Clone
    {
        Array2::from_shape_fn((self.len(), 1), |(i, _)| self[i].clone())
    }
}
impl<T, const N: usize> MaybeMatrix<T> for &[T; N]
{
    type Transpose = [[T; 1]; N];

    fn to_array2_option(&self) -> Option<Array2<T>>
    where
        T: Clone
    {
        Some(self.to_array2())
    }
    
    fn matrix_transpose(&self) -> Self::Transpose
    where
        T: Clone
    {
        self.as_collumn().clone()
    }
}

impl<T, const N: usize, const M: usize> MaybeMatrix<T> for [[T; N]; M]
{
    type Transpose = [[T; M]; N];

    fn to_array2_option(&self) -> Option<Array2<T>>
    where
        T: Clone
    {
        Some(self.to_array2())
    }

    fn matrix_transpose(&self) -> Self::Transpose
    where
        T: Clone
    {
        self.clone().transpose()
    }
}
impl<T, const N: usize, const M: usize> MaybeMatrix<T> for &[[T; N]; M]
{
    type Transpose = [[T; M]; N];

    fn to_array2_option(&self) -> Option<Array2<T>>
    where
        T: Clone
    {
        Some(self.to_array2())
    }

    fn matrix_transpose(&self) -> Self::Transpose
    where
        T: Clone
    {
        (*self).clone().transpose()
    }
}

impl<T, const N: usize, const M: usize> MaybeMatrix<T> for [&[T; N]; M]
{
    type Transpose = [[T; M]; N];

    fn to_array2_option(&self) -> Option<Array2<T>>
    where
        T: Clone
    {
        Some(self.to_array2())
    }

    fn matrix_transpose(&self) -> Self::Transpose
    where
        T: Clone
    {
        self.clone().map(|r| r.clone()).transpose()
    }
}
impl<T, const N: usize, const M: usize> MaybeMatrix<T> for &[&[T; N]; M]
{
    type Transpose = [[T; M]; N];

    fn to_array2_option(&self) -> Option<Array2<T>>
    where
        T: Clone
    {
        Some(self.to_array2())
    }

    fn matrix_transpose(&self) -> Self::Transpose
    where
        T: Clone
    {
        (*self).clone().map(|r| r.clone()).transpose()
    }
}

impl<T, const N: usize> MaybeMatrix<T> for Vec<[T; N]>
{
    type Transpose = Array2<T>;

    fn to_array2_option(&self) -> Option<Array2<T>>
    where
        T: Clone
    {
        Some(self.to_array2())
    }

    fn matrix_transpose(&self) -> Self::Transpose
    where
        T: Clone
    {
        Array2::from_shape_fn((N, self.len()), |(i, j)| self[i][j].clone())
    }
}
impl<T, const N: usize> MaybeMatrix<T> for &[[T; N]]
{
    type Transpose = Array2<T>;

    fn to_array2_option(&self) -> Option<Array2<T>>
    where
        T: Clone
    {
        Some(self.to_array2())
    }

    fn matrix_transpose(&self) -> Self::Transpose
    where
        T: Clone
    {
        Array2::from_shape_fn((N, self.len()), |(i, j)| self[i][j].clone())
    }
}
impl<T, const N: usize> MaybeMatrix<T> for [[T; N]]
{
    type Transpose = Array2<T>;

    fn to_array2_option(&self) -> Option<Array2<T>>
    where
        T: Clone
    {
        Some(self.to_array2())
    }

    fn matrix_transpose(&self) -> Self::Transpose
    where
        T: Clone
    {
        Array2::from_shape_fn((N, self.len()), |(i, j)| self[i][j].clone())
    }
}

impl<T, const N: usize> MaybeMatrix<T> for Vec<&[T; N]>
{
    type Transpose = Array2<T>;

    fn to_array2_option(&self) -> Option<Array2<T>>
    where
        T: Clone
    {
        Some(self.to_array2())
    }

    fn matrix_transpose(&self) -> Self::Transpose
    where
        T: Clone
    {
        Array2::from_shape_fn((N, self.len()), |(i, j)| self[i][j].clone())
    }
}
impl<T, const N: usize> MaybeMatrix<T> for &[&[T; N]]
{
    type Transpose = Array2<T>;

    fn to_array2_option(&self) -> Option<Array2<T>>
    where
        T: Clone
    {
        Some(self.to_array2())
    }

    fn matrix_transpose(&self) -> Self::Transpose
    where
        T: Clone
    {
        Array2::from_shape_fn((N, self.len()), |(i, j)| self[i][j].clone())
    }
}
impl<T, const N: usize> MaybeMatrix<T> for [&[T; N]]
{
    type Transpose = Array2<T>;

    fn to_array2_option(&self) -> Option<Array2<T>>
    where
        T: Clone
    {
        Some(self.to_array2())
    }

    fn matrix_transpose(&self) -> Self::Transpose
    where
        T: Clone
    {
        Array2::from_shape_fn((N, self.len()), |(i, j)| self[i][j].clone())
    }
}

impl<T> MaybeMatrix<T> for Array1<T>
{
    type Transpose = Array2<T>;

    fn to_array2_option(&self) -> Option<Array2<T>>
    where
        T: Clone
    {
        Some(self.to_array2())
    }

    fn matrix_transpose(&self) -> Self::Transpose
    where
        T: Clone
    {
        Array2::from_shape_fn((self.len(), 1), |(j, _)| self[j].clone())
    }
}
impl<'b, T> MaybeMatrix<T> for ArrayView1<'b, T>
{
    type Transpose = Array2<T>;

    fn to_array2_option(&self) -> Option<Array2<T>>
    where
        T: Clone
    {
        Some(self.to_array2())
    }
    
    fn matrix_transpose(&self) -> Self::Transpose
    where
        T: Clone
    {
        Array2::from_shape_fn((self.len(), 1), |(j, _)| self[j].clone())
    }
}

impl<T> MaybeMatrix<T> for Array2<T>
{
    type Transpose = Array2<T>;

    fn to_array2_option(&self) -> Option<Array2<T>>
    where
        T: Clone
    {
        Some(self.to_array2())
    }

    fn matrix_transpose(&self) -> Self::Transpose
    where
        T: Clone
    {
        self.t()
            .to_owned()
    }
}
impl<'b, T> MaybeMatrix<T> for ArrayView2<'b, T>
{
    type Transpose = Array2<T>;

    fn to_array2_option(&self) -> Option<Array2<T>>
    where
        T: Clone
    {
        Some(self.to_array2())
    }

    fn matrix_transpose(&self) -> Self::Transpose
    where
        T: Clone
    {
        self.t()
            .to_owned()
    }
}