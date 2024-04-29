use core::ops::{Range, RangeInclusive};

use ndarray::{Array1, ArrayView1};
use num::complex::ComplexFloat;
use option_trait::Maybe;

use crate::quantities::ListOrSingle;

pub trait IntoList<T, L, N>
where
    L: ListOrSingle<T> + ?Sized,
    N: Maybe<usize>
{
    fn into_list(self, n: N) -> L
    where
        T: Clone,
        L: Sized,
        Self: Sized;
}

impl<T> IntoList<T, T, ()> for T
{
    fn into_list(self, (): ()) -> T
    where
        T: Clone,
        T: Sized,
        Self: Sized
    {
        self
    }
}

impl<T, const N: usize> IntoList<T, [T; N], ()> for Range<T>
where
    T: ComplexFloat
{
    fn into_list(self, (): ()) -> [T; N]
    where
        T: Clone,
        [T; N]: Sized,
        Self: Sized
    {
        let nf = T::from(N).unwrap();
        core::array::from_fn(|i| {
            let i = T::from(i).unwrap();
            self.start + (self.end - self.start)*i/nf
        })
    }
}
impl<T, const N: usize> IntoList<T, [T; N], ()> for RangeInclusive<T>
where
    T: ComplexFloat
{
    fn into_list(self, (): ()) -> [T; N]
    where
        T: Clone,
        [T; N]: Sized,
        Self: Sized
    {
        let nm1 = T::from(N.saturating_sub(1)).unwrap();
        if nm1.is_zero()
        {
            let one = T::one();
            let two = one + one;
            return core::array::from_fn(|_| {
                (*self.start() + *self.end())/two
            })
        }
        core::array::from_fn(|i| {
            let i = T::from(i).unwrap();
            *self.start() + (*self.end() - *self.start())*i/nm1
        })
    }
}

impl<T> IntoList<T, Vec<T>, usize> for Range<T>
where
    T: ComplexFloat
{
    fn into_list(self, n: usize) -> Vec<T>
    where
        T: Clone,
        Vec<T>: Sized,
        Self: Sized
    {
        let nf = T::from(n).unwrap();
        (0..n).map(|i| {
            let i = T::from(i).unwrap();
            self.start + (self.end - self.start)*i/nf
        }).collect()
    }
}
impl<T> IntoList<T, Vec<T>, usize> for RangeInclusive<T>
where
    T: ComplexFloat
{
    fn into_list(self, n: usize) -> Vec<T>
    where
        T: Clone,
        Vec<T>: Sized,
        Self: Sized
    {
        let nm1 = T::from(n.saturating_sub(1)).unwrap();
        if nm1.is_zero()
        {
            let one = T::one();
            let two = one + one;
            return (0..n).map(|_| {
                (*self.start() + *self.end())/two
            }).collect()
        }
        (0..n).map(|i| {
            let i = T::from(i).unwrap();
            *self.start() + (*self.end() - *self.start())*i/nm1
        }).collect()
    }
}

impl<T> IntoList<T, Vec<T>, ()> for Range<T>
where
    Self: Iterator<Item = T>
{
    fn into_list(self, (): ()) -> Vec<T>
    where
        T: Clone,
        Vec<T>: Sized,
        Self: Sized
    {
        self.collect()
    }
}
impl<T> IntoList<T, Vec<T>, ()> for RangeInclusive<T>
where
    Self: Iterator<Item = T>
{
    fn into_list(self, (): ()) -> Vec<T>
    where
        T: Clone,
        Vec<T>: Sized,
        Self: Sized
    {
        self.collect()
    }
}

impl<T, const N: usize> IntoList<T, Self, ()> for [T; N]
{
    fn into_list(self, (): ()) -> Self
    where
        T: Clone,
        Self: Sized
    {
        self
    }
}
impl<T, const N: usize> IntoList<T, Vec<T>, ()> for [T; N]
{
    fn into_list(self, (): ()) -> Vec<T>
    where
        T: Clone,
        Self: Sized
    {
        self.into_iter()
            .collect()
    }
}
impl<T, const N: usize> IntoList<T, [T; N], ()> for &[T; N]
{
    fn into_list(self, (): ()) -> [T; N]
    where
        T: Clone,
        Self: Sized
    {
        self.clone()
    }
}
impl<T, const N: usize> IntoList<T, Self, ()> for &[T; N]
{
    fn into_list(self, (): ()) -> Self
    where
        T: Clone,
        Self: Sized
    {
        self
    }
}
impl<T, const N: usize> IntoList<T, Vec<T>, ()> for &[T; N]
{
    fn into_list(self, (): ()) -> Vec<T>
    where
        T: Clone,
        Self: Sized
    {
        self.to_vec()
    }
}
impl<'a, T, const N: usize> IntoList<T, &'a [T], ()> for &'a [T; N]
{
    fn into_list(self, (): ()) -> &'a [T]
    where
        T: Clone,
        Self: Sized
    {
        self.as_slice()
    }
}
impl<T> IntoList<T, Self, ()> for Vec<T>
{
    fn into_list(self, (): ()) -> Self
    where
        T: Clone,
        Self: Sized
    {
        self
    }
}
impl<T> IntoList<T, Vec<T>, ()> for [T]
{
    fn into_list(self, (): ()) -> Vec<T>
    where
        T: Clone,
        Self: Sized
    {
        self.to_vec()
    }
}
impl<T> IntoList<T, Self, ()> for [T]
{
    fn into_list(self, (): ()) -> Self
    where
        T: Clone,
        Self: Sized
    {
        self
    }
}
impl<T> IntoList<T, Vec<T>, ()> for &[T]
{
    fn into_list(self, (): ()) -> Vec<T>
    where
        T: Clone,
        Self: Sized
    {
        self.to_vec()
    }
}
impl<T> IntoList<T, Self, ()> for &[T]
{
    fn into_list(self, (): ()) -> Self
    where
        T: Clone,
        Self: Sized
    {
        self
    }
}

impl<T> IntoList<T, Self, ()> for Array1<T>
{
    fn into_list(self, (): ()) -> Array1<T>
    where
        T: Clone,
        Self: Sized
    {
        self
    }
}
impl<T> IntoList<T, Vec<T>, ()> for Array1<T>
{
    fn into_list(self, (): ()) -> Vec<T>
    where
        T: Clone,
        Self: Sized
    {
        self.to_vec()
    }
}
impl<'a, T> IntoList<T, Self, ()> for ArrayView1<'a, T>
{
    fn into_list(self, (): ()) -> Self
    where
        T: Clone,
        Self: Sized
    {
        self
    }
}
impl<'a, T> IntoList<T, Vec<T>, ()> for ArrayView1<'a, T>
{
    fn into_list(self, (): ()) -> Vec<T>
    where
        T: Clone,
        Self: Sized
    {
        self.to_vec()
    }
}