use ndarray::{Array1, Array2};

use crate::{Matrix, OwnedLists};

pub trait OwnedMatrix<T>: Matrix<T> + OwnedLists<T>
{
    
}

impl<T> OwnedMatrix<T> for Vec<T>
{
    
}
impl<T, const N: usize> OwnedMatrix<T> for [T; N]
{
    
}

impl<T, const N: usize, const M: usize> OwnedMatrix<T> for [[T; N]; M]
{
    
}
impl<T, const N: usize> OwnedMatrix<T> for Vec<[T; N]>
{
    
}

impl<T> OwnedMatrix<T> for Array1<T>
{
    
}

impl<T> OwnedMatrix<T> for Array2<T>
{
    
}