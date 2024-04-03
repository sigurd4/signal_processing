

use ndarray::{Array1, ArrayView1};


use crate::{ListOrSingle, Matrix, MaybeList};

pub trait List<T>: MaybeList<T> + Matrix<T> + ListOrSingle<T>
{
    
}

impl<T> List<T> for Vec<T>
{

}
impl<T> List<T> for [T]
{
    
}
impl<T, const N: usize> List<T> for [T; N]
{
    
}
impl<T> List<T> for &[T]
{
    
}
impl<T, const N: usize> List<T> for &[T; N]
{
    
}

impl<T> List<T> for Array1<T>
{
    
}
impl<'a, T> List<T> for ArrayView1<'a, T>
{
    
}