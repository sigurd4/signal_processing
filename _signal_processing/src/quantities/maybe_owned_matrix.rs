use ndarray::{Array1, Array2};

use crate::quantities::{MaybeMatrix, MaybeOwnedLists};

pub trait MaybeOwnedMatrix<T>: MaybeMatrix<T> + MaybeOwnedLists<T>
{
    
}

impl<T> MaybeOwnedMatrix<T> for ()
{
    
}

impl<T> MaybeOwnedMatrix<T> for Vec<T>
{
    
}
impl<T, const N: usize> MaybeOwnedMatrix<T> for [T; N]
{
    
}

impl<T, const N: usize, const M: usize> MaybeOwnedMatrix<T> for [[T; N]; M]
{
    
}
impl<T, const N: usize> MaybeOwnedMatrix<T> for Vec<[T; N]>
{
    
}

impl<T> MaybeOwnedMatrix<T> for Array1<T>
{
    
}

impl<T> MaybeOwnedMatrix<T> for Array2<T>
{
    
}