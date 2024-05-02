

use ndarray::{Array1, ArrayView1};
use option_trait::StaticMaybe;

use crate::quantities::{MatrixOrSingle, OwnedList, ListsOrSingle};

pub trait ListOrSingle<T>: ListsOrSingle<T> + MatrixOrSingle<T>
{
    type Length: StaticMaybe<usize>;
    const LENGTH: usize;
    type Resized<const M: usize>: OwnedList<T>;

    fn length(&self) -> usize;
    fn as_view_slice(&self) -> &'_ [T];
    fn to_vec(&self) -> Vec<T>
    where
        T: Clone;
    fn into_vec(self) -> Vec<T>
    where
        Self: Sized,
        T: Clone;
}

impl<T> ListOrSingle<T> for T
{
    type Length = usize;
    const LENGTH: usize = 1;
    type Resized<const M: usize> = [T; M];

    fn length(&self) -> usize
    {
        1
    }
    fn as_view_slice(&self) -> &'_ [T]
    {
        core::slice::from_ref(self)
    }
    fn to_vec(&self) -> Vec<T>
        where
            T: Clone
    {
        vec![self.clone()]
    }
    fn into_vec(self) -> Vec<T>
    where
        Self: Sized,
        T: Clone
    {
        vec![self]
    }
}

impl<T> ListOrSingle<T> for Vec<T>
{
    type Length = ();
    const LENGTH: usize = usize::MAX;
    type Resized<const M: usize> = Vec<T>;

    fn length(&self) -> usize
    {
        self.len()
    }
    fn as_view_slice(&self) -> &'_ [T]
    {
        self
    }
    fn to_vec(&self) -> Vec<T>
    where
        T: Clone
    {
        self.clone()
    }
    fn into_vec(self) -> Vec<T>
    where
        Self: Sized,
        T: Clone
    {
        self
    }
}
impl<T> ListOrSingle<T> for [T]
{
    type Length = ();
    const LENGTH: usize = usize::MAX;
    type Resized<const M: usize> = Vec<T>;

    fn length(&self) -> usize
    {
        self.len()
    }
    fn as_view_slice(&self) -> &'_ [T]
    {
        self
    }
    fn to_vec(&self) -> Vec<T>
    where
        T: Clone
    {
        self.to_vec()
    }
    fn into_vec(self) -> Vec<T>
    where
        Self: Sized,
        T: Clone
    {
        self.to_vec()
    }
}
impl<T, const N: usize> ListOrSingle<T> for [T; N]
{
    type Length = usize;
    const LENGTH: usize = N;
    type Resized<const M: usize> = [T; M];

    fn length(&self) -> usize
    {
        N
    }
    fn as_view_slice(&self) -> &'_ [T]
    {
        self.as_slice()
    }
    fn to_vec(&self) -> Vec<T>
    where
        T: Clone
    {
        self.as_slice().to_vec()
    }
    fn into_vec(self) -> Vec<T>
    where
        Self: Sized,
        T: Clone
    {
        self.into_iter()
            .collect()
    }
}
impl<T> ListOrSingle<T> for &[T]
{
    type Length = ();
    const LENGTH: usize = usize::MAX;
    type Resized<const M: usize> = Vec<T>;

    fn length(&self) -> usize
    {
        self.len()
    }
    fn as_view_slice(&self) -> &'_ [T]
    {
        self
    }
    fn to_vec(&self) -> Vec<T>
    where
        T: Clone
    {
        (*self).to_vec()
    }
    fn into_vec(self) -> Vec<T>
    where
        Self: Sized,
        T: Clone
    {
        self.to_vec()
    }
}
impl<T, const N: usize> ListOrSingle<T> for &[T; N]
{
    type Length = usize;
    const LENGTH: usize = N;
    type Resized<const M: usize> = [T; M];

    fn length(&self) -> usize
    {
        self.len()
    }
    fn as_view_slice(&self) -> &'_ [T]
    {
        self.as_slice()
    }
    fn to_vec(&self) -> Vec<T>
    where
        T: Clone
    {
        self.as_slice().to_vec()
    }
    fn into_vec(self) -> Vec<T>
    where
        Self: Sized,
        T: Clone
    {
        self.as_slice().to_vec()
    }
}

impl<T> ListOrSingle<T> for Array1<T>
{
    type Length = ();
    const LENGTH: usize = usize::MAX;
    type Resized<const M: usize> = Array1<T>;

    fn length(&self) -> usize
    {
        self.len()
    }
    fn as_view_slice(&self) -> &'_ [T]
    {
        self.as_slice().unwrap()
    }
    fn to_vec(&self) -> Vec<T>
    where
        T: Clone
    {
        self.to_vec()
    }
    fn into_vec(self) -> Vec<T>
    where
        Self: Sized,
        T: Clone
    {
        self.to_vec()
    }
}
impl<'a, T> ListOrSingle<T> for ArrayView1<'a, T>
{
    type Length = ();
    const LENGTH: usize = usize::MAX;
    type Resized<const M: usize> = Array1<T>;

    fn length(&self) -> usize
    {
        self.len()
    }
    fn as_view_slice(&self) -> &'_ [T]
    {
        self.as_slice().unwrap()
    }
    fn to_vec(&self) -> Vec<T>
    where
        T: Clone
    {
        self.to_vec()
    }
    fn into_vec(self) -> Vec<T>
    where
        Self: Sized,
        T: Clone
    {
        self.to_vec()
    }
}