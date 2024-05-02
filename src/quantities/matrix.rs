use ndarray::{Array1, Array2, ArrayView1, ArrayView2};

use crate::quantities::{Lists, MaybeMatrix, MatrixOrSingle};

pub trait Matrix<T>: Lists<T> + MaybeMatrix<T> + MatrixOrSingle<T>
{
    
}

impl<T> Matrix<T> for Vec<T>
{
    
}
impl<T> Matrix<T> for [T]
{

}
impl<T, const N: usize> Matrix<T> for [T; N]
{
    
}
impl<T> Matrix<T> for &[T]
{
    
}
impl<T, const N: usize> Matrix<T> for &[T; N]
{
    
}

impl<T, const N: usize, const M: usize> Matrix<T> for [[T; N]; M]
{
    
}
impl<T, const N: usize, const M: usize> Matrix<T> for &[[T; N]; M]
{
    
}

impl<T, const N: usize, const M: usize> Matrix<T> for [&[T; N]; M]
{
    
}
impl<T, const N: usize, const M: usize> Matrix<T> for &[&[T; N]; M]
{
    
}

impl<T, const N: usize> Matrix<T> for Vec<[T; N]>
{
    
}
impl<T, const N: usize> Matrix<T> for &[[T; N]]
{
    
}
impl<T, const N: usize> Matrix<T> for [[T; N]]
{
    
}

impl<T, const N: usize> Matrix<T> for Vec<&[T; N]>
{
    
}
impl<T, const N: usize> Matrix<T> for &[&[T; N]]
{
    
}
impl<T, const N: usize> Matrix<T> for [&[T; N]]
{
    
}

impl<T> Matrix<T> for Array1<T>
{
    
}
impl<'b, T> Matrix<T> for ArrayView1<'b, T>
{
    
}

impl<T> Matrix<T> for Array2<T>
{
    
}
impl<'b, T> Matrix<T> for ArrayView2<'b, T>
{
    
}