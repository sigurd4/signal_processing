use ndarray::Array1;

use crate::quantities::{List, MaybeOwnedList, OwnedListOrSingle};

pub trait OwnedList<T>: List<T> + OwnedListOrSingle<T> + MaybeOwnedList<T>
{
    
}

impl<T> OwnedList<T> for Vec<T>
{
    
}
impl<T, const N: usize> OwnedList<T> for [T; N]
{
    
}

impl<T> OwnedList<T> for Array1<T>
{
    
}