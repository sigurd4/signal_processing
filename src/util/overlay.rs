use array_math::max_len;
use ndarray::{Array1, ArrayView1};

pub trait Overlay<T, Rhs>
{
    type Output;

    fn overlay(self, rhs: Rhs) -> Self::Output;
}

impl<T, const N1: usize, const N2: usize> Overlay<T, [T; N1]> for [T; N2]
where

    [(); max_len(N1, N2)]:
{
    type Output = [T; max_len(N1, N2)];

    fn overlay(self, rhs: [T; N1]) -> Self::Output
    {
        let mut lhs = self.into_iter();
        let mut rhs = rhs.into_iter();
        core::array::from_fn(|_| lhs.next().or(rhs.next()).unwrap())
    }
}
impl<T, const N1: usize, const N2: usize> Overlay<T, &[T; N1]> for [T; N2]
where
    T: Clone,
    [(); max_len(N1, N2)]:
{
    type Output = [T; max_len(N1, N2)];

    fn overlay(self, rhs: &[T; N1]) -> Self::Output
    {
        let mut lhs = self.into_iter();
        let mut rhs = rhs.into_iter();
        core::array::from_fn(|_| {
            let b = rhs.next();
            if let Some(a) = lhs.next()
            {
                a
            }
            else
            {
                b.unwrap().clone()
            }
        })
    }
}
impl<T, const N2: usize> Overlay<T, Vec<T>> for [T; N2]
{
    type Output = Vec<T>;

    fn overlay(self, rhs: Vec<T>) -> Self::Output
    {
        let n = N2.max(rhs.len());
        let mut lhs = self.into_iter();
        let mut rhs = rhs.into_iter();
        (0..n).map(|_| lhs.next().or(rhs.next()).unwrap()).collect()
    }
}
impl<T, const N2: usize> Overlay<T, &[T]> for [T; N2]
where
    T: Clone
{
    type Output = Vec<T>;

    fn overlay(self, rhs: &[T]) -> Self::Output
    {
        let n = N2.max(rhs.len());
        let mut lhs = self.into_iter();
        let mut rhs = rhs.into_iter();
        (0..n).map(|_| {
                let b = rhs.next();
                if let Some(a) = lhs.next()
                {
                    a
                }
                else
                {
                    b.unwrap().clone()
                }
            }).collect()
    }
}
impl<T, const N2: usize> Overlay<T, Array1<T>> for [T; N2]
{
    type Output = Vec<T>;

    fn overlay(self, rhs: Array1<T>) -> Self::Output
    {
        let n = N2.max(rhs.len());
        let mut lhs = self.into_iter();
        let mut rhs = rhs.into_iter();
        (0..n).map(|_| lhs.next().or(rhs.next()).unwrap()).collect()
    }
}
impl<'a, T, const N2: usize> Overlay<T, ArrayView1<'a, T>> for [T; N2]
where
    T: Clone
{
    type Output = Vec<T>;

    fn overlay(self, rhs: ArrayView1<T>) -> Self::Output
    {
        let n = N2.max(rhs.len());
        let mut lhs = self.into_iter();
        let mut rhs = rhs.into_iter();
        (0..n).map(|_| {
            let b = rhs.next();
            if let Some(a) = lhs.next()
            {
                a
            }
            else
            {
                b.unwrap().clone()
            }
        }).collect()
    }
}

impl<T, const N2: usize, U> Overlay<T, U> for &[T; N2]
where
    T: Clone,
    [T; N2]: Overlay<T, U>
{
    type Output = <[T; N2] as Overlay<T, U>>::Output;

    fn overlay(self, rhs: U) -> Self::Output
    {
        self.clone()
            .overlay(rhs)
    }
}

impl<T, const N1: usize> Overlay<T, [T; N1]> for Vec<T>
{
    type Output = Vec<T>;

    fn overlay(self, rhs: [T; N1]) -> Self::Output
    {
        let n = self.len().max(N1);
        let mut lhs = self.into_iter();
        let mut rhs = rhs.into_iter();
        (0..n).map(|_| lhs.next().or(rhs.next()).unwrap())
            .collect()
    }
}
impl<T, const N1: usize> Overlay<T, &[T; N1]> for Vec<T>
where
    T: Clone
{
    type Output = Vec<T>;

    fn overlay(self, rhs: &[T; N1]) -> Self::Output
    {
        self.overlay(rhs.as_slice())
    }
}
impl<T> Overlay<T, Vec<T>> for Vec<T>
{
    type Output = Vec<T>;

    fn overlay(self, rhs: Vec<T>) -> Self::Output
    {
        let n = self.len().max(rhs.len());
        let mut lhs = self.into_iter();
        let mut rhs = rhs.into_iter();
        (0..n).map(|_| lhs.next().or(rhs.next()).unwrap())
            .collect()
    }
}
impl<T> Overlay<T, &[T]> for Vec<T>
where
    T: Clone
{
    type Output = Vec<T>;

    fn overlay(self, rhs: &[T]) -> Self::Output
    {
        let n = self.len().max(rhs.len());
        let mut lhs = self.into_iter();
        let mut rhs = rhs.iter();
        (0..n).map(|_| {
                let b = rhs.next();
                if let Some(a) = lhs.next()
                {
                    a
                }
                else
                {
                    b.unwrap().clone()
                }
            }).collect()
    }
}
impl<T> Overlay<T, Array1<T>> for Vec<T>
{
    type Output = Vec<T>;

    fn overlay(self, rhs: Array1<T>) -> Self::Output
    {
        let n = self.len().max(rhs.len());
        let mut lhs = self.into_iter();
        let mut rhs = rhs.into_iter();
        (0..n).map(|_| lhs.next().or(rhs.next()).unwrap())
            .collect()
    }
}
impl<'a, T> Overlay<T, ArrayView1<'a, T>> for Vec<T>
where
    T: Clone
{
    type Output = Vec<T>;

    fn overlay(self, rhs: ArrayView1<'a, T>) -> Self::Output
    {
        let n = self.len().max(rhs.len());
        let mut lhs = self.into_iter();
        let mut rhs = rhs.iter();
        (0..n).map(|_| {
                let b = rhs.next();
                if let Some(a) = lhs.next()
                {
                    a
                }
                else
                {
                    b.unwrap().clone()
                }
            }).collect()
    }
}

impl<T, U> Overlay<T, U> for &[T]
where
    T: Clone,
    Vec<T>: Overlay<T, U>
{
    type Output = <Vec<T> as Overlay<T, U>>::Output;

    fn overlay(self, rhs: U) -> Self::Output
    {
        self.to_vec().overlay(rhs)
    }
}

impl<T, const N1: usize> Overlay<T, [T; N1]> for Array1<T>
{
    type Output = Vec<T>;

    fn overlay(self, rhs: [T; N1]) -> Self::Output
    {
        let n = self.len().max(N1);
        let mut lhs = self.into_iter();
        let mut rhs = rhs.into_iter();
        (0..n).map(|_| lhs.next().or(rhs.next()).unwrap())
            .collect()
    }
}
impl<T, const N1: usize> Overlay<T, &[T; N1]> for Array1<T>
where
    T: Clone
{
    type Output = Vec<T>;

    fn overlay(self, rhs: &[T; N1]) -> Self::Output
    {
        self.overlay(rhs.as_slice())
    }
}
impl<T> Overlay<T, Vec<T>> for Array1<T>
{
    type Output = Vec<T>;

    fn overlay(self, rhs: Vec<T>) -> Self::Output
    {
        let n = self.len().max(rhs.len());
        let mut lhs = self.into_iter();
        let mut rhs = rhs.into_iter();
        (0..n).map(|_| lhs.next().or(rhs.next()).unwrap())
            .collect()
    }
}
impl<T> Overlay<T, &[T]> for Array1<T>
where
    T: Clone
{
    type Output = Vec<T>;

    fn overlay(self, rhs: &[T]) -> Self::Output
    {
        let n = self.len().max(rhs.len());
        let mut lhs = self.into_iter();
        let mut rhs = rhs.iter();
        (0..n).map(|_| {
                let b = rhs.next();
                if let Some(a) = lhs.next()
                {
                    a
                }
                else
                {
                    b.unwrap().clone()
                }
            }).collect()
    }
}
impl<T> Overlay<T, Array1<T>> for Array1<T>
{
    type Output = Vec<T>;

    fn overlay(self, rhs: Array1<T>) -> Self::Output
    {
        let n = self.len().max(rhs.len());
        let mut lhs = self.into_iter();
        let mut rhs = rhs.into_iter();
        (0..n).map(|_| lhs.next().or(rhs.next()).unwrap())
            .collect()
    }
}
impl<'a, T> Overlay<T, ArrayView1<'a, T>> for Array1<T>
where
    T: Clone
{
    type Output = Vec<T>;

    fn overlay(self, rhs: ArrayView1<'a, T>) -> Self::Output
    {
        let n = self.len().max(rhs.len());
        let mut lhs = self.into_iter();
        let mut rhs = rhs.iter();
        (0..n).map(|_| {
                let b = rhs.next();
                if let Some(a) = lhs.next()
                {
                    a
                }
                else
                {
                    b.unwrap().clone()
                }
            }).collect()
    }
}

impl<'a, T, U> Overlay<T, U> for ArrayView1<'a, T>
where
    T: Clone,
    Array1<T>: Overlay<T, U>
{
    type Output = <Array1<T> as Overlay<T, U>>::Output;

    fn overlay(self, rhs: U) -> Self::Output
    {
        self.to_owned().overlay(rhs)
    }
}