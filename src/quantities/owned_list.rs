use ndarray::Array1;

use crate::quantities::{List, OwnedListOrSingle};

use super::OwnedLists;

pub trait OwnedList<T>: List<T> + OwnedListOrSingle<T> + OwnedLists<T>
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