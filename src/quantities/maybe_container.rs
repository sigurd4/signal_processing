


use ndarray::{ArrayBase, ArrayView, Dimension, OwnedRepr};




pub trait MaybeContainer<T>
{
    type View<'a>: MaybeContainer<T> + 'a
    where
        Self: 'a;
    type Owned: MaybeContainer<T> + Sized;

    fn as_view(&self) -> Self::View<'_>;
    fn to_owned(&self) -> Self::Owned
    where
        T: Clone;
    fn into_owned(self) -> Self::Owned
    where
        Self: Sized,
        T: Clone;
}

impl<T> MaybeContainer<T> for ()
{
    type View<'a> = ();
    type Owned = ();

    fn as_view(&self) -> Self::View<'_>
    {
        
    }
    fn to_owned(&self) -> Self::Owned
    {
        
    }
    fn into_owned(self) -> Self::Owned
    where
        Self: Sized,
        T: Clone
    {

    }
}

impl<T> MaybeContainer<T> for Vec<T>
{
    type View<'a> = &'a [T]
    where
        Self: 'a;
    type Owned = Vec<T>;

    fn as_view(&self) -> Self::View<'_>
    {
        self.as_slice()
    }
    fn to_owned(&self) -> Self::Owned
    where
        T: Clone
    {
        self.clone()
    }
    fn into_owned(self) -> Self::Owned
    where
        Self: Sized,
        T: Clone
    {
        self
    }
}
impl<T> MaybeContainer<T> for [T]
{
    type View<'a> = &'a [T]
    where
        Self: 'a;
    type Owned = Vec<T>;

    fn as_view(&self) -> Self::View<'_>
    {
        self
    }
    fn to_owned(&self) -> Self::Owned
    where
        T: Clone
    {
        self.to_vec()
    }
    fn into_owned(self) -> Self::Owned
    where
        Self: Sized,
        T: Clone
    {
        self.to_vec()
    }
}
impl<T, const N: usize> MaybeContainer<T> for [T; N]
{
    type View<'a> = &'a [T; N]
    where
        Self: 'a;
    type Owned = [T; N];

    fn as_view(&self) -> Self::View<'_>
    {
        self
    }
    fn to_owned(&self) -> Self::Owned
    where
        T: Clone
    {
        self.clone()
    }
    fn into_owned(self) -> Self::Owned
    where
        Self: Sized,
        T: Clone
    {
        self
    }
}
impl<T> MaybeContainer<T> for &[T]
{
    type View<'a> = &'a [T]
    where
        Self: 'a;
    type Owned = Vec<T>;

    fn as_view(&self) -> Self::View<'_>
    {
        self
    }
    fn to_owned(&self) -> Self::Owned
    where
        T: Clone
    {
        self.to_vec()
    }
    fn into_owned(self) -> Self::Owned
    where
        Self: Sized,
        T: Clone
    {
        self.to_vec()
    }
}
impl<T, const N: usize> MaybeContainer<T> for &[T; N]
{
    type View<'a> = &'a [T; N]
    where
        Self: 'a;
    type Owned = [T; N];

    fn as_view(&self) -> Self::View<'_>
    {
        self
    }
    fn to_owned(&self) -> Self::Owned
    where
        T: Clone
    {
        (*self).clone()
    }
    fn into_owned(self) -> Self::Owned
    where
        Self: Sized,
        T: Clone
    {
        self.clone()
    }
}

impl<T> MaybeContainer<T> for Vec<Vec<T>>
{
    type View<'a> = Vec<&'a [T]>
    where
        Self: 'a;
    type Owned = Vec<Vec<T>>;

    fn as_view(&self) -> Self::View<'_>
    {
        self.iter()
            .map(|s| s.as_slice())
            .collect()
    }
    fn to_owned(&self) -> Self::Owned
    where
        T: Clone
    {
        self.to_vec()
    }
    fn into_owned(self) -> Self::Owned
    where
        Self: Sized,
        T: Clone
    {
        self
    }
}
impl<T, const M: usize> MaybeContainer<T> for [Vec<T>; M]
{
    type View<'a> = [&'a [T]; M]
    where
        Self: 'a;
    type Owned = [Vec<T>; M];

    fn as_view(&self) -> Self::View<'_>
    {
        self.each_ref()
            .map(|s| s.as_slice())
    }
    fn to_owned(&self) -> Self::Owned
    where
        T: Clone
    {
        self.each_ref()
            .map(|s| s.clone())
    }
    fn into_owned(self) -> Self::Owned
    where
        Self: Sized,
        T: Clone
    {
        self
    }
}
impl<T> MaybeContainer<T> for [Vec<T>]
{
    type View<'a> = Vec<&'a [T]>
    where
        Self: 'a;
    type Owned = Vec<Vec<T>>;

    fn as_view(&self) -> Self::View<'_>
    {
        self.iter()
            .map(|s| s.as_slice())
            .collect()
    }
    fn to_owned(&self) -> Self::Owned
    where
        T: Clone
    {
        self.to_vec()
    }
    fn into_owned(self) -> Self::Owned
    where
        Self: Sized,
        T: Clone
    {
        self.to_vec()
    }
}
impl<T, const M: usize> MaybeContainer<T> for &[Vec<T>; M]
{
    type View<'a> = [&'a [T]; M]
    where
        Self: 'a;
    type Owned = [Vec<T>; M];

    fn as_view(&self) -> Self::View<'_>
    {
        self.each_ref()
            .map(|s| s.as_slice())
    }
    fn to_owned(&self) -> Self::Owned
    where
        T: Clone
    {
        (*self).clone()
    }
    fn into_owned(self) -> Self::Owned
    where
        Self: Sized,
        T: Clone
    {
        self.clone()
    }
}
impl<T> MaybeContainer<T> for &[Vec<T>]
{
    type View<'a> = Vec<&'a [T]>
    where
        Self: 'a;
    type Owned = Vec<Vec<T>>;

    fn as_view(&self) -> Self::View<'_>
    {
        self.iter()
            .map(|s| s.as_slice())
            .collect()
    }
    fn to_owned(&self) -> Self::Owned
    where
        T: Clone
    {
        self.to_vec()
    }
    fn into_owned(self) -> Self::Owned
    where
        Self: Sized,
        T: Clone
    {
        self.to_vec()
    }
}

impl<T, const N: usize> MaybeContainer<T> for Vec<[T; N]>
{
    type View<'a> = Vec<&'a [T; N]>
    where
        Self: 'a;
    type Owned = Vec<[T; N]>;

    fn as_view(&self) -> Self::View<'_>
    {
        self.iter()
            .collect()
    }
    fn to_owned(&self) -> Self::Owned
    where
        T: Clone
    {
        self.to_vec()
    }
    fn into_owned(self) -> Self::Owned
    where
        Self: Sized,
        T: Clone
    {
        self
    }
}
impl<T, const N: usize, const M: usize> MaybeContainer<T> for [[T; N]; M]
{
    type View<'a> = &'a [[T; N]; M]
    where
        Self: 'a;
    type Owned = [[T; N]; M];

    fn as_view(&self) -> Self::View<'_>
    {
        self
    }
    fn to_owned(&self) -> Self::Owned
    where
        T: Clone
    {
        self.each_ref()
            .map(|s| s.clone())
    }
    fn into_owned(self) -> Self::Owned
    where
        Self: Sized,
        T: Clone
    {
        self
    }
}
impl<T, const N: usize> MaybeContainer<T> for [[T; N]]
{
    type View<'a> = Vec<&'a [T; N]>
    where
        Self: 'a;
    type Owned = Vec<[T; N]>;

    fn as_view(&self) -> Self::View<'_>
    {
        self.iter()
            .collect()
    }
    fn to_owned(&self) -> Self::Owned
    where
        T: Clone
    {
        self.to_vec()
    }
    fn into_owned(self) -> Self::Owned
    where
        Self: Sized,
        T: Clone
    {
        self.to_vec()
    }
}
impl<T, const N: usize, const M: usize> MaybeContainer<T> for &[[T; N]; M]
{
    type View<'a> = &'a [[T; N]; M]
    where
        Self: 'a;
    type Owned = [[T; N]; M];

    fn as_view(&self) -> Self::View<'_>
    {
        *self
    }
    fn to_owned(&self) -> Self::Owned
    where
        T: Clone
    {
        (*self).clone()
    }
    fn into_owned(self) -> Self::Owned
    where
        Self: Sized,
        T: Clone
    {
        self.clone()
    }
}
impl<T, const N: usize> MaybeContainer<T> for &[[T; N]]
{
    type View<'a> = &'a [[T; N]]
    where
        Self: 'a;
    type Owned = Vec<[T; N]>;

    fn as_view(&self) -> Self::View<'_>
    {
        *self
    }
    fn to_owned(&self) -> Self::Owned
    where
        T: Clone
    {
        self.to_vec()
    }
    fn into_owned(self) -> Self::Owned
    where
        Self: Sized,
        T: Clone
    {
        self.to_vec()
    }
}

impl<T> MaybeContainer<T> for Vec<&[T]>
{
    type View<'a> = &'a [&'a [T]]
    where
        Self: 'a;
    type Owned = Vec<Vec<T>>;

    fn as_view(&self) -> Self::View<'_>
    {
        self.as_slice()
    }
    fn to_owned(&self) -> Self::Owned
    where
        T: Clone
    {
        self.iter()
            .map(|&s| s.to_vec())
            .collect()
    }
    fn into_owned(self) -> Self::Owned
    where
        Self: Sized,
        T: Clone
    {
        self.into_iter()
            .map(|s| s.to_vec())
            .collect()
    }
}
impl<T, const M: usize> MaybeContainer<T> for [&[T]; M]
{
    type View<'a> = &'a [&'a [T]; M]
    where
        Self: 'a;
    type Owned = [Vec<T>; M];

    fn as_view(&self) -> Self::View<'_>
    {
        self
    }
    fn to_owned(&self) -> Self::Owned
    where
        T: Clone
    {
        self.map(|s| s.to_vec())
    }
    fn into_owned(self) -> Self::Owned
    where
        Self: Sized,
        T: Clone
    {
        self.map(|s| s.to_vec())
    }
}
impl<T> MaybeContainer<T> for [&[T]]
{
    type View<'a> = &'a [&'a [T]]
    where
        Self: 'a;
    type Owned = Vec<Vec<T>>;

    fn as_view(&self) -> Self::View<'_>
    {
        self
    }
    fn to_owned(&self) -> Self::Owned
    where
        T: Clone
    {
        self.iter()
            .map(|&s| s.to_vec())
            .collect()
    }
    fn into_owned(self) -> Self::Owned
    where
        Self: Sized,
        T: Clone
    {
        self.iter()
            .map(|&s| s.to_vec())
            .collect()
    }
}
impl<T, const M: usize> MaybeContainer<T> for &[&[T]; M]
{
    type View<'a> = &'a [&'a [T]; M]
    where
        Self: 'a;
    type Owned = [Vec<T>; M];

    fn as_view(&self) -> Self::View<'_>
    {
        *self
    }
    fn to_owned(&self) -> Self::Owned
    where
        T: Clone
    {
        self.map(|s| s.to_vec())
    }
    fn into_owned(self) -> Self::Owned
    where
        Self: Sized,
        T: Clone
    {
        self.map(|s| s.to_vec())
    }
}
impl<T> MaybeContainer<T> for &[&[T]]
{
    type View<'a> = &'a [&'a [T]]
    where
        Self: 'a;
    type Owned = Vec<Vec<T>>;

    fn as_view(&self) -> Self::View<'_>
    {
        *self
    }
    fn to_owned(&self) -> Self::Owned
    where
        T: Clone
    {
        self.iter()
            .map(|&s| s.to_vec())
            .collect()
    }
    fn into_owned(self) -> Self::Owned
    where
        Self: Sized,
        T: Clone
    {
        self.iter()
            .map(|&s| s.to_vec())
            .collect()
    }
}

impl<T, const N: usize> MaybeContainer<T> for Vec<&[T; N]>
{
    type View<'a> = &'a [&'a [T; N]]
    where
        Self: 'a;
    type Owned = Vec<[T; N]>;

    fn as_view(&self) -> Self::View<'_>
    {
        self.as_slice()
    }
    fn to_owned(&self) -> Self::Owned
    where
        T: Clone
    {
        self.iter()
            .map(|&s| s.clone())
            .collect()
    }
    fn into_owned(self) -> Self::Owned
    where
        Self: Sized,
        T: Clone
    {
        self.iter()
            .map(|&s| s.clone())
            .collect()
    }
}
impl<T, const N: usize, const M: usize> MaybeContainer<T> for [&[T; N]; M]
{
    type View<'a> = &'a [&'a [T; N]; M]
    where
        Self: 'a;
    type Owned = [[T; N]; M];

    fn as_view(&self) -> Self::View<'_>
    {
        self
    }
    fn to_owned(&self) -> Self::Owned
    where
        T: Clone
    {
        self.map(|s| s.clone())
    }
    fn into_owned(self) -> Self::Owned
    where
        Self: Sized,
        T: Clone
    {
        self.map(|s| s.clone())
    }
}
impl<T, const N: usize> MaybeContainer<T> for [&[T; N]]
{
    type View<'a> = &'a [&'a [T; N]]
    where
        Self: 'a;
    type Owned = Vec<[T; N]>;

    fn as_view(&self) -> Self::View<'_>
    {
        self
    }
    fn to_owned(&self) -> Self::Owned
    where
        T: Clone
    {
        self.iter()
            .map(|&s| s.clone())
            .collect()
    }
    fn into_owned(self) -> Self::Owned
    where
        Self: Sized,
        T: Clone
    {
        self.iter()
            .map(|&s| s.clone())
            .collect()
    }
}
impl<T, const N: usize, const M: usize> MaybeContainer<T> for &[&[T; N]; M]
{
    type View<'a> = &'a [&'a [T; N]; M]
    where
        Self: 'a;
    type Owned = [[T; N]; M];

    fn as_view(&self) -> Self::View<'_>
    {
        *self
    }
    fn to_owned(&self) -> Self::Owned
    where
        T: Clone
    {
        self.map(|s| s.clone())
    }
    fn into_owned(self) -> Self::Owned
    where
        Self: Sized,
        T: Clone
    {
        self.map(|s| s.clone())
    }
}
impl<T, const N: usize> MaybeContainer<T> for &[&[T; N]]
{
    type View<'a> = &'a [&'a [T; N]]
    where
        Self: 'a;
    type Owned = Vec<[T; N]>;

    fn as_view(&self) -> Self::View<'_>
    {
        *self
    }
    fn to_owned(&self) -> Self::Owned
    where
        T: Clone
    {
        self.iter()
            .map(|&s| s.clone())
            .collect()
    }
    fn into_owned(self) -> Self::Owned
    where
        Self: Sized,
        T: Clone
    {
        self.iter()
            .map(|&s| s.clone())
            .collect()
    }
}

impl<T, D> MaybeContainer<T> for ArrayBase<OwnedRepr<T>, D>
where
    D: Dimension
{
    type View<'a> = ArrayView<'a, T, D>
    where
        T: 'a,
        Self: 'a;
    type Owned = ArrayBase<OwnedRepr<T>, D>;

    fn as_view(&self) -> Self::View<'_>
    {
        self.into()
    }
    fn to_owned(&self) -> Self::Owned
        where
            T: Clone
    {
        self.clone()
    }
    fn into_owned(self) -> Self::Owned
    where
        Self: Sized,
        T: Clone
    {
        self
    }
}

impl<'b, T, D> MaybeContainer<T> for ArrayView<'b, T, D>
where
    D: Dimension
{
    type View<'a> = ArrayView<'b, T, D>
    where
        T: 'a,
        Self: 'a;
    type Owned = ArrayBase<OwnedRepr<T>, D>;

    fn as_view(&self) -> Self::View<'_>
    {
        self.clone()
    }
    fn to_owned(&self) -> Self::Owned
    where
        T: Clone
    {
        self.to_owned()
    }
    fn into_owned(self) -> Self::Owned
    where
        Self: Sized,
        T: Clone
    {
        self.to_owned()
    }
}