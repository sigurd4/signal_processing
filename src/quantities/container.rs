

use ndarray::{prelude::ArrayView, ArrayBase, Dimension, NdIndex, OwnedRepr};
use option_trait::NotVoid;

use crate::quantities::{ContainerOrSingle, MaybeContainer};

pub trait Container<T>: MaybeContainer<T> + ContainerOrSingle<T> + NotVoid
{
    
}

impl<T> !Container<T> for ()
where
    T: NotVoid {}

impl<T> Container<T> for Vec<T>
{
    
}
impl<T> Container<T> for [T]
{
    
}
impl<T, const N: usize> Container<T> for [T; N]
{
    
}
impl<'c, T> Container<T> for &'c [T]
{
    
}
impl<'b, T, const N: usize> Container<T> for &'b [T; N]
{
    
}

impl<T> Container<T> for Vec<Vec<T>>
{
    
}
impl<T, const M: usize> Container<T> for [Vec<T>; M]
{
    
}
impl<T> Container<T> for [Vec<T>]
{
    
}
impl<'b, T, const M: usize> Container<T> for &'b [Vec<T>; M]
{
    
}
impl<'b, T> Container<T> for &'b [Vec<T>]
{
    
}

impl<T, const N: usize> Container<T> for Vec<[T; N]>
{
    
}
impl<T, const N: usize, const M: usize> Container<T> for [[T; N]; M]
{
    
}
impl<T, const N: usize> Container<T> for [[T; N]]
{
    
}
impl<'b, T, const N: usize, const M: usize> Container<T> for &'b [[T; N]; M]
{
    
}
impl<'b, T, const N: usize> Container<T> for &'b [[T; N]]
{
    
}

impl<'b, T> Container<T> for Vec<&'b [T]>
{
    
}
impl<'b, T, const M: usize> Container<T> for [&'b [T]; M]
{
    
}
impl<'b, T> Container<T> for [&'b [T]]
{
    
}
impl<'b, 'c, T, const M: usize> Container<T> for &'b [&'c [T]; M]
{
    
}
impl<'b, 'c, T> Container<T> for &'b [&'c [T]]
{
    
}

impl<'b, T, const N: usize> Container<T> for Vec<&'b [T; N]>
{
    
}
impl<'b, T, const N: usize, const M: usize> Container<T> for [&'b [T; N]; M]
{
    
}
impl<'b, T, const N: usize> Container<T> for [&'b [T; N]]
{
    
}
impl<'b, 'c, T, const N: usize, const M: usize> Container<T> for &'b [&'c [T; N]; M]
{
    
}
impl<'b, 'c, T, const N: usize> Container<T> for &'b [&'c [T; N]]
{
    
}

impl<T, D> Container<T> for ArrayBase<OwnedRepr<T>, D>
where
    D: Dimension + NotVoid,
    <D as Dimension>::Pattern: NdIndex<D>
{
    
}

impl<'c, T, D> Container<T> for ArrayView<'c, T, D>
where
    D: Dimension + NotVoid,
    <D as Dimension>::Pattern: NdIndex<D>
{
    
}