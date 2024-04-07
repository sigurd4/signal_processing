


use ndarray::{ArrayBase, ArrayView, Dimension, NdIndex, OwnedRepr};
use option_trait::Maybe;

use crate::Container;

pub trait MaybeContainer<T>
{
    type View<'a>: MaybeContainer<T> + 'a
    where
        Self: 'a;
    type Owned: MaybeContainer<T> + Sized;
    type Some: Container<T> + ?Sized;
    type MaybeSome: Maybe<Self::Some> + ?Sized
    where
        (): Maybe<Self::Some>;
    type MaybeMapped<M>: MaybeContainer<M> + Sized;

    fn as_view(&self) -> Self::View<'_>;
    fn to_owned(&self) -> Self::Owned
    where
        T: Clone;
    fn into_owned(self) -> Self::Owned
    where
        Self: Sized,
        T: Clone;
    fn into_maybe_some(self) -> Self::MaybeSome
    where
        Self: Sized,
        (): Maybe<Self::Some>;
    fn maybe_map_to_owned<'a, F>(&'a self, map: F) -> Self::MaybeMapped<F::Output>
    where
        T: 'a,
        F: FnMut<(&'a T,)>;
    fn maybe_map_into_owned<F>(self, map: F) -> Self::MaybeMapped<F::Output>
    where
        T: Clone,
        Self: Sized,
        F: FnMut<(T,)>;
}

impl<T> MaybeContainer<T> for ()
{
    type View<'a> = ();
    type Owned = ();
    type Some = [T; 1];
    type MaybeSome = ()
    where
        (): Maybe<Self::Some>;
    type MaybeMapped<M> = ();

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
    fn into_maybe_some(self) -> Self::MaybeSome
    where
        Self: Sized,
        (): Maybe<Self::Some>
    {

    }
    fn maybe_map_to_owned<'a, F>(&'a self, _map: F) -> Self::MaybeMapped<F::Output>
    where
        T: 'a,
        F: FnMut<(&'a T,)>
    {

    }
    fn maybe_map_into_owned<F>(self, _map: F) -> Self::MaybeMapped<F::Output>
    where
        T: Clone,
        Self: Sized,
        F: FnMut<(T,)>
    {

    }
}

impl<T> MaybeContainer<T> for Vec<T>
{
    type View<'a> = &'a [T]
    where
        Self: 'a;
    type Owned = Vec<T>;
    type Some = Self;
    type MaybeSome = Self
    where
        (): Maybe<Self>;
    type MaybeMapped<MM> = <Self as Container<T>>::Mapped<MM>;

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
    fn into_maybe_some(self) -> Self::MaybeSome
    where
        Self: Sized,
        (): Maybe<Self::Some>
    {
        self
    }
    fn maybe_map_to_owned<'a, F>(&'a self, map: F) -> Self::MaybeMapped<F::Output>
    where
        T: 'a,
        F: FnMut<(&'a T,)>
    {
        self.map_to_owned(map)
    }
    fn maybe_map_into_owned<F>(self, map: F) -> Self::MaybeMapped<F::Output>
    where
        T: Clone,
        Self: Sized,
        F: FnMut<(T,)>
    {
        self.map_into_owned(map)
    }
}
impl<T> MaybeContainer<T> for [T]
{
    type View<'a> = &'a [T]
    where
        Self: 'a;
    type Owned = Vec<T>;
    type Some = Self;
    type MaybeSome = Self
    where
        (): Maybe<Self>;
    type MaybeMapped<MM> = <Self as Container<T>>::Mapped<MM>;

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
    fn into_maybe_some(self) -> Self::MaybeSome
    where
        Self: Sized,
        (): Maybe<Self::Some>
    {
        self
    }
    fn maybe_map_to_owned<'a, F>(&'a self, map: F) -> Self::MaybeMapped<F::Output>
    where
        T: 'a,
        F: FnMut<(&'a T,)>
    {
        self.map_to_owned(map)
    }
    fn maybe_map_into_owned<F>(self, map: F) -> Self::MaybeMapped<F::Output>
    where
        T: Clone,
        Self: Sized,
        F: FnMut<(T,)>
    {
        self.map_into_owned(map)
    }
}
impl<T, const N: usize> MaybeContainer<T> for [T; N]
{
    type View<'a> = &'a [T; N]
    where
        Self: 'a;
    type Owned = [T; N];
    type Some = Self;
    type MaybeSome = Self
    where
        (): Maybe<Self>;
    type MaybeMapped<MM> = <Self as Container<T>>::Mapped<MM>;

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
    fn into_maybe_some(self) -> Self::MaybeSome
    where
        Self: Sized,
        (): Maybe<Self::Some>
    {
        self
    }
    fn maybe_map_to_owned<'a, F>(&'a self, map: F) -> Self::MaybeMapped<F::Output>
    where
        T: 'a,
        F: FnMut<(&'a T,)>
    {
        self.map_to_owned(map)
    }
    fn maybe_map_into_owned<F>(self, map: F) -> Self::MaybeMapped<F::Output>
    where
        T: Clone,
        Self: Sized,
        F: FnMut<(T,)>
    {
        self.map_into_owned(map)
    }
}
impl<T> MaybeContainer<T> for &[T]
{
    type View<'a> = &'a [T]
    where
        Self: 'a;
    type Owned = Vec<T>;
    type Some = Self;
    type MaybeSome = Self
    where
        (): Maybe<Self>;
    type MaybeMapped<MM> = <Self as Container<T>>::Mapped<MM>;

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
    fn into_maybe_some(self) -> Self::MaybeSome
    where
        Self: Sized,
        (): Maybe<Self::Some>
    {
        self
    }
    fn maybe_map_to_owned<'a, F>(&'a self, map: F) -> Self::MaybeMapped<F::Output>
    where
        T: 'a,
        F: FnMut<(&'a T,)>
    {
        self.map_to_owned(map)
    }
    fn maybe_map_into_owned<F>(self, map: F) -> Self::MaybeMapped<F::Output>
    where
        T: Clone,
        Self: Sized,
        F: FnMut<(T,)>
    {
        self.map_into_owned(map)
    }
}
impl<T, const N: usize> MaybeContainer<T> for &[T; N]
{
    type View<'a> = &'a [T; N]
    where
        Self: 'a;
    type Owned = [T; N];
    type Some = Self;
    type MaybeSome = Self
    where
        (): Maybe<Self>;
    type MaybeMapped<MM> = <Self as Container<T>>::Mapped<MM>;

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
    fn into_maybe_some(self) -> Self::MaybeSome
    where
        Self: Sized,
        (): Maybe<Self::Some>
    {
        self
    }
    fn maybe_map_to_owned<'a, F>(&'a self, map: F) -> Self::MaybeMapped<F::Output>
    where
        T: 'a,
        F: FnMut<(&'a T,)>
    {
        self.map_to_owned(map)
    }
    fn maybe_map_into_owned<F>(self, map: F) -> Self::MaybeMapped<F::Output>
    where
        T: Clone,
        Self: Sized,
        F: FnMut<(T,)>
    {
        self.map_into_owned(map)
    }
}

impl<T> MaybeContainer<T> for Vec<Vec<T>>
{
    type View<'a> = Vec<&'a [T]>
    where
        Self: 'a;
    type Owned = Vec<Vec<T>>;
    type Some = Self;
    type MaybeSome = Self
    where
        (): Maybe<Self>;
    type MaybeMapped<MM> = <Self as Container<T>>::Mapped<MM>;

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
    fn into_maybe_some(self) -> Self::MaybeSome
    where
        Self: Sized,
        (): Maybe<Self::Some>
    {
        self
    }
    fn maybe_map_to_owned<'a, F>(&'a self, map: F) -> Self::MaybeMapped<F::Output>
    where
        T: 'a,
        F: FnMut<(&'a T,)>
    {
        self.map_to_owned(map)
    }
    fn maybe_map_into_owned<F>(self, map: F) -> Self::MaybeMapped<F::Output>
    where
        T: Clone,
        Self: Sized,
        F: FnMut<(T,)>
    {
        self.map_into_owned(map)
    }
}
impl<T, const M: usize> MaybeContainer<T> for [Vec<T>; M]
{
    type View<'a> = [&'a [T]; M]
    where
        Self: 'a;
    type Owned = [Vec<T>; M];
    type Some = Self;
    type MaybeSome = Self
    where
        (): Maybe<Self>;
    type MaybeMapped<MM> = <Self as Container<T>>::Mapped<MM>;

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
    fn into_maybe_some(self) -> Self::MaybeSome
    where
        Self: Sized,
        (): Maybe<Self::Some>
    {
        self
    }
    fn maybe_map_to_owned<'a, F>(&'a self, map: F) -> Self::MaybeMapped<F::Output>
    where
        T: 'a,
        F: FnMut<(&'a T,)>
    {
        self.map_to_owned(map)
    }
    fn maybe_map_into_owned<F>(self, map: F) -> Self::MaybeMapped<F::Output>
    where
        T: Clone,
        Self: Sized,
        F: FnMut<(T,)>
    {
        self.map_into_owned(map)
    }
}
impl<T> MaybeContainer<T> for [Vec<T>]
{
    type View<'a> = Vec<&'a [T]>
    where
        Self: 'a;
    type Owned = Vec<Vec<T>>;
    type Some = Self;
    type MaybeSome = Self
    where
        (): Maybe<Self>;
    type MaybeMapped<MM> = <Self as Container<T>>::Mapped<MM>;

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
    fn into_maybe_some(self) -> Self::MaybeSome
    where
        Self: Sized,
        (): Maybe<Self::Some>
    {
        self
    }
    fn maybe_map_to_owned<'a, F>(&'a self, map: F) -> Self::MaybeMapped<F::Output>
    where
        T: 'a,
        F: FnMut<(&'a T,)>
    {
        self.map_to_owned(map)
    }
    fn maybe_map_into_owned<F>(self, map: F) -> Self::MaybeMapped<F::Output>
    where
        T: Clone,
        Self: Sized,
        F: FnMut<(T,)>
    {
        self.map_into_owned(map)
    }
}
impl<T, const M: usize> MaybeContainer<T> for &[Vec<T>; M]
{
    type View<'a> = [&'a [T]; M]
    where
        Self: 'a;
    type Owned = [Vec<T>; M];
    type Some = Self;
    type MaybeSome = Self
    where
        (): Maybe<Self>;
    type MaybeMapped<MM> = <Self as Container<T>>::Mapped<MM>;

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
    fn into_maybe_some(self) -> Self::MaybeSome
    where
        Self: Sized,
        (): Maybe<Self::Some>
    {
        self
    }
    fn maybe_map_to_owned<'a, F>(&'a self, map: F) -> Self::MaybeMapped<F::Output>
    where
        T: 'a,
        F: FnMut<(&'a T,)>
    {
        self.map_to_owned(map)
    }
    fn maybe_map_into_owned<F>(self, map: F) -> Self::MaybeMapped<F::Output>
    where
        T: Clone,
        Self: Sized,
        F: FnMut<(T,)>
    {
        self.map_into_owned(map)
    }
}
impl<T> MaybeContainer<T> for &[Vec<T>]
{
    type View<'a> = Vec<&'a [T]>
    where
        Self: 'a;
    type Owned = Vec<Vec<T>>;
    type Some = Self;
    type MaybeSome = Self
    where
        (): Maybe<Self>;
    type MaybeMapped<MM> = <Self as Container<T>>::Mapped<MM>;

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
    fn into_maybe_some(self) -> Self::MaybeSome
    where
        Self: Sized,
        (): Maybe<Self::Some>
    {
        self
    }
    fn maybe_map_to_owned<'a, F>(&'a self, map: F) -> Self::MaybeMapped<F::Output>
    where
        T: 'a,
        F: FnMut<(&'a T,)>
    {
        self.map_to_owned(map)
    }
    fn maybe_map_into_owned<F>(self, map: F) -> Self::MaybeMapped<F::Output>
    where
        T: Clone,
        Self: Sized,
        F: FnMut<(T,)>
    {
        self.map_into_owned(map)
    }
}

impl<T, const N: usize> MaybeContainer<T> for Vec<[T; N]>
{
    type View<'a> = Vec<&'a [T; N]>
    where
        Self: 'a;
    type Owned = Vec<[T; N]>;
    type Some = Self;
    type MaybeSome = Self
    where
        (): Maybe<Self>;
    type MaybeMapped<MM> = <Self as Container<T>>::Mapped<MM>;

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
    fn into_maybe_some(self) -> Self::MaybeSome
    where
        Self: Sized,
        (): Maybe<Self::Some>
    {
        self
    }
    fn maybe_map_to_owned<'a, F>(&'a self, map: F) -> Self::MaybeMapped<F::Output>
    where
        T: 'a,
        F: FnMut<(&'a T,)>
    {
        self.map_to_owned(map)
    }
    fn maybe_map_into_owned<F>(self, map: F) -> Self::MaybeMapped<F::Output>
    where
        T: Clone,
        Self: Sized,
        F: FnMut<(T,)>
    {
        self.map_into_owned(map)
    }
}
impl<T, const N: usize, const M: usize> MaybeContainer<T> for [[T; N]; M]
{
    type View<'a> = &'a [[T; N]; M]
    where
        Self: 'a;
    type Owned = [[T; N]; M];
    type Some = Self;
    type MaybeSome = Self
    where
        (): Maybe<Self>;
    type MaybeMapped<MM> = <Self as Container<T>>::Mapped<MM>;

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
    fn into_maybe_some(self) -> Self::MaybeSome
    where
        Self: Sized,
        (): Maybe<Self::Some>
    {
        self
    }
    fn maybe_map_to_owned<'a, F>(&'a self, map: F) -> Self::MaybeMapped<F::Output>
    where
        T: 'a,
        F: FnMut<(&'a T,)>
    {
        self.map_to_owned(map)
    }
    fn maybe_map_into_owned<F>(self, map: F) -> Self::MaybeMapped<F::Output>
    where
        T: Clone,
        Self: Sized,
        F: FnMut<(T,)>
    {
        self.map_into_owned(map)
    }
}
impl<T, const N: usize> MaybeContainer<T> for [[T; N]]
{
    type View<'a> = Vec<&'a [T; N]>
    where
        Self: 'a;
    type Owned = Vec<[T; N]>;
    type Some = Self;
    type MaybeSome = Self
    where
        (): Maybe<Self>;
    type MaybeMapped<MM> = <Self as Container<T>>::Mapped<MM>;

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
    fn into_maybe_some(self) -> Self::MaybeSome
    where
        Self: Sized,
        (): Maybe<Self::Some>
    {
        self
    }
    fn maybe_map_to_owned<'a, F>(&'a self, map: F) -> Self::MaybeMapped<F::Output>
    where
        T: 'a,
        F: FnMut<(&'a T,)>
    {
        self.map_to_owned(map)
    }
    fn maybe_map_into_owned<F>(self, map: F) -> Self::MaybeMapped<F::Output>
    where
        T: Clone,
        Self: Sized,
        F: FnMut<(T,)>
    {
        self.map_into_owned(map)
    }
}
impl<T, const N: usize, const M: usize> MaybeContainer<T> for &[[T; N]; M]
{
    type View<'a> = &'a [[T; N]; M]
    where
        Self: 'a;
    type Owned = [[T; N]; M];
    type Some = Self;
    type MaybeSome = Self
    where
        (): Maybe<Self>;
    type MaybeMapped<MM> = <Self as Container<T>>::Mapped<MM>;

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
    fn into_maybe_some(self) -> Self::MaybeSome
    where
        Self: Sized,
        (): Maybe<Self::Some>
    {
        self
    }
    fn maybe_map_to_owned<'a, F>(&'a self, map: F) -> Self::MaybeMapped<F::Output>
    where
        T: 'a,
        F: FnMut<(&'a T,)>
    {
        self.map_to_owned(map)
    }
    fn maybe_map_into_owned<F>(self, map: F) -> Self::MaybeMapped<F::Output>
    where
        T: Clone,
        Self: Sized,
        F: FnMut<(T,)>
    {
        self.map_into_owned(map)
    }
}
impl<T, const N: usize> MaybeContainer<T> for &[[T; N]]
{
    type View<'a> = &'a [[T; N]]
    where
        Self: 'a;
    type Owned = Vec<[T; N]>;
    type Some = Self;
    type MaybeSome = Self
    where
        (): Maybe<Self>;
    type MaybeMapped<MM> = <Self as Container<T>>::Mapped<MM>;

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
    fn into_maybe_some(self) -> Self::MaybeSome
    where
        Self: Sized,
        (): Maybe<Self::Some>
    {
        self
    }
    fn maybe_map_to_owned<'a, F>(&'a self, map: F) -> Self::MaybeMapped<F::Output>
    where
        T: 'a,
        F: FnMut<(&'a T,)>
    {
        self.map_to_owned(map)
    }
    fn maybe_map_into_owned<F>(self, map: F) -> Self::MaybeMapped<F::Output>
    where
        T: Clone,
        Self: Sized,
        F: FnMut<(T,)>
    {
        self.map_into_owned(map)
    }
}

impl<T> MaybeContainer<T> for Vec<&[T]>
{
    type View<'a> = &'a [&'a [T]]
    where
        Self: 'a;
    type Owned = Vec<Vec<T>>;
    type Some = Self;
    type MaybeSome = Self
    where
        (): Maybe<Self>;
    type MaybeMapped<MM> = <Self as Container<T>>::Mapped<MM>;

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
    fn into_maybe_some(self) -> Self::MaybeSome
    where
        Self: Sized,
        (): Maybe<Self::Some>
    {
        self
    }
    fn maybe_map_to_owned<'a, F>(&'a self, map: F) -> Self::MaybeMapped<F::Output>
    where
        T: 'a,
        F: FnMut<(&'a T,)>
    {
        self.map_to_owned(map)
    }
    fn maybe_map_into_owned<F>(self, map: F) -> Self::MaybeMapped<F::Output>
    where
        T: Clone,
        Self: Sized,
        F: FnMut<(T,)>
    {
        self.map_into_owned(map)
    }
}
impl<T, const M: usize> MaybeContainer<T> for [&[T]; M]
{
    type View<'a> = &'a [&'a [T]; M]
    where
        Self: 'a;
    type Owned = [Vec<T>; M];
    type Some = Self;
    type MaybeSome = Self
    where
        (): Maybe<Self>;
    type MaybeMapped<MM> = <Self as Container<T>>::Mapped<MM>;

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
    fn into_maybe_some(self) -> Self::MaybeSome
    where
        Self: Sized,
        (): Maybe<Self::Some>
    {
        self
    }
    fn maybe_map_to_owned<'a, F>(&'a self, map: F) -> Self::MaybeMapped<F::Output>
    where
        T: 'a,
        F: FnMut<(&'a T,)>
    {
        self.map_to_owned(map)
    }
    fn maybe_map_into_owned<F>(self, map: F) -> Self::MaybeMapped<F::Output>
    where
        T: Clone,
        Self: Sized,
        F: FnMut<(T,)>
    {
        self.map_into_owned(map)
    }
}
impl<T> MaybeContainer<T> for [&[T]]
{
    type View<'a> = &'a [&'a [T]]
    where
        Self: 'a;
    type Owned = Vec<Vec<T>>;
    type Some = Self;
    type MaybeSome = Self
    where
        (): Maybe<Self>;
    type MaybeMapped<MM> = <Self as Container<T>>::Mapped<MM>;

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
    fn into_maybe_some(self) -> Self::MaybeSome
    where
        Self: Sized,
        (): Maybe<Self::Some>
    {
        self
    }
    fn maybe_map_to_owned<'a, F>(&'a self, map: F) -> Self::MaybeMapped<F::Output>
    where
        T: 'a,
        F: FnMut<(&'a T,)>
    {
        self.map_to_owned(map)
    }
    fn maybe_map_into_owned<F>(self, map: F) -> Self::MaybeMapped<F::Output>
    where
        T: Clone,
        Self: Sized,
        F: FnMut<(T,)>
    {
        self.map_into_owned(map)
    }
}
impl<T, const M: usize> MaybeContainer<T> for &[&[T]; M]
{
    type View<'a> = &'a [&'a [T]; M]
    where
        Self: 'a;
    type Owned = [Vec<T>; M];
    type Some = Self;
    type MaybeSome = Self
    where
        (): Maybe<Self>;
    type MaybeMapped<MM> = <Self as Container<T>>::Mapped<MM>;

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
    fn into_maybe_some(self) -> Self::MaybeSome
    where
        Self: Sized,
        (): Maybe<Self::Some>
    {
        self
    }
    fn maybe_map_to_owned<'a, F>(&'a self, map: F) -> Self::MaybeMapped<F::Output>
    where
        T: 'a,
        F: FnMut<(&'a T,)>
    {
        self.map_to_owned(map)
    }
    fn maybe_map_into_owned<F>(self, map: F) -> Self::MaybeMapped<F::Output>
    where
        T: Clone,
        Self: Sized,
        F: FnMut<(T,)>
    {
        self.map_into_owned(map)
    }
}
impl<T> MaybeContainer<T> for &[&[T]]
{
    type View<'a> = &'a [&'a [T]]
    where
        Self: 'a;
    type Owned = Vec<Vec<T>>;
    type Some = Self;
    type MaybeSome = Self
    where
        (): Maybe<Self>;
    type MaybeMapped<MM> = <Self as Container<T>>::Mapped<MM>;

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
    fn into_maybe_some(self) -> Self::MaybeSome
    where
        Self: Sized,
        (): Maybe<Self::Some>
    {
        self
    }
    fn maybe_map_to_owned<'a, F>(&'a self, map: F) -> Self::MaybeMapped<F::Output>
    where
        T: 'a,
        F: FnMut<(&'a T,)>
    {
        self.map_to_owned(map)
    }
    fn maybe_map_into_owned<F>(self, map: F) -> Self::MaybeMapped<F::Output>
    where
        T: Clone,
        Self: Sized,
        F: FnMut<(T,)>
    {
        self.map_into_owned(map)
    }
}

impl<T, const N: usize> MaybeContainer<T> for Vec<&[T; N]>
{
    type View<'a> = &'a [&'a [T; N]]
    where
        Self: 'a;
    type Owned = Vec<[T; N]>;
    type Some = Self;
    type MaybeSome = Self
    where
        (): Maybe<Self>;
    type MaybeMapped<MM> = <Self as Container<T>>::Mapped<MM>;

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
    fn into_maybe_some(self) -> Self::MaybeSome
    where
        Self: Sized,
        (): Maybe<Self::Some>
    {
        self
    }
    fn maybe_map_to_owned<'a, F>(&'a self, map: F) -> Self::MaybeMapped<F::Output>
    where
        T: 'a,
        F: FnMut<(&'a T,)>
    {
        self.map_to_owned(map)
    }
    fn maybe_map_into_owned<F>(self, map: F) -> Self::MaybeMapped<F::Output>
    where
        T: Clone,
        Self: Sized,
        F: FnMut<(T,)>
    {
        self.map_into_owned(map)
    }
}
impl<T, const N: usize, const M: usize> MaybeContainer<T> for [&[T; N]; M]
{
    type View<'a> = &'a [&'a [T; N]; M]
    where
        Self: 'a;
    type Owned = [[T; N]; M];
    type Some = Self;
    type MaybeSome = Self
    where
        (): Maybe<Self>;
    type MaybeMapped<MM> = <Self as Container<T>>::Mapped<MM>;

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
    fn into_maybe_some(self) -> Self::MaybeSome
    where
        Self: Sized,
        (): Maybe<Self::Some>
    {
        self
    }
    fn maybe_map_to_owned<'a, F>(&'a self, map: F) -> Self::MaybeMapped<F::Output>
    where
        T: 'a,
        F: FnMut<(&'a T,)>
    {
        self.map_to_owned(map)
    }
    fn maybe_map_into_owned<F>(self, map: F) -> Self::MaybeMapped<F::Output>
    where
        T: Clone,
        Self: Sized,
        F: FnMut<(T,)>
    {
        self.map_into_owned(map)
    }
}
impl<T, const N: usize> MaybeContainer<T> for [&[T; N]]
{
    type View<'a> = &'a [&'a [T; N]]
    where
        Self: 'a;
    type Owned = Vec<[T; N]>;
    type Some = Self;
    type MaybeSome = Self
    where
        (): Maybe<Self>;
    type MaybeMapped<MM> = <Self as Container<T>>::Mapped<MM>;

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
    fn into_maybe_some(self) -> Self::MaybeSome
    where
        Self: Sized,
        (): Maybe<Self::Some>
    {
        self
    }
    fn maybe_map_to_owned<'a, F>(&'a self, map: F) -> Self::MaybeMapped<F::Output>
    where
        T: 'a,
        F: FnMut<(&'a T,)>
    {
        self.map_to_owned(map)
    }
    fn maybe_map_into_owned<F>(self, map: F) -> Self::MaybeMapped<F::Output>
    where
        T: Clone,
        Self: Sized,
        F: FnMut<(T,)>
    {
        self.map_into_owned(map)
    }
}
impl<T, const N: usize, const M: usize> MaybeContainer<T> for &[&[T; N]; M]
{
    type View<'a> = &'a [&'a [T; N]; M]
    where
        Self: 'a;
    type Owned = [[T; N]; M];
    type Some = Self;
    type MaybeSome = Self
    where
        (): Maybe<Self>;
    type MaybeMapped<MM> = <Self as Container<T>>::Mapped<MM>;

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
    fn into_maybe_some(self) -> Self::MaybeSome
    where
        Self: Sized,
        (): Maybe<Self::Some>
    {
        self
    }
    fn maybe_map_to_owned<'a, F>(&'a self, map: F) -> Self::MaybeMapped<F::Output>
    where
        T: 'a,
        F: FnMut<(&'a T,)>
    {
        self.map_to_owned(map)
    }
    fn maybe_map_into_owned<F>(self, map: F) -> Self::MaybeMapped<F::Output>
    where
        T: Clone,
        Self: Sized,
        F: FnMut<(T,)>
    {
        self.map_into_owned(map)
    }
}
impl<T, const N: usize> MaybeContainer<T> for &[&[T; N]]
{
    type View<'a> = &'a [&'a [T; N]]
    where
        Self: 'a;
    type Owned = Vec<[T; N]>;
    type Some = Self;
    type MaybeSome = Self
    where
        (): Maybe<Self>;
    type MaybeMapped<MM> = <Self as Container<T>>::Mapped<MM>;

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
    fn into_maybe_some(self) -> Self::MaybeSome
    where
        Self: Sized,
        (): Maybe<Self::Some>
    {
        self
    }
    fn maybe_map_to_owned<'a, F>(&'a self, map: F) -> Self::MaybeMapped<F::Output>
    where
        T: 'a,
        F: FnMut<(&'a T,)>
    {
        self.map_to_owned(map)
    }
    fn maybe_map_into_owned<F>(self, map: F) -> Self::MaybeMapped<F::Output>
    where
        T: Clone,
        Self: Sized,
        F: FnMut<(T,)>
    {
        self.map_into_owned(map)
    }
}

impl<T, D> MaybeContainer<T> for ArrayBase<OwnedRepr<T>, D>
where
    D: Dimension,
    <D as Dimension>::Pattern: NdIndex<D>
{
    type View<'a> = ArrayView<'a, T, D>
    where
        T: 'a,
        Self: 'a;
    type Owned = ArrayBase<OwnedRepr<T>, D>;
    type Some = Self;
    type MaybeSome = Self
    where
        (): Maybe<Self>;
    type MaybeMapped<MM> = <Self as Container<T>>::Mapped<MM>;

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
    fn into_maybe_some(self) -> Self::MaybeSome
    where
        Self: Sized,
        (): Maybe<Self::Some>
    {
        self
    }
    fn maybe_map_to_owned<'a, F>(&'a self, map: F) -> Self::MaybeMapped<F::Output>
    where
        T: 'a,
        F: FnMut<(&'a T,)>
    {
        self.map_to_owned(map)
    }
    fn maybe_map_into_owned<F>(self, map: F) -> Self::MaybeMapped<F::Output>
    where
        T: Clone,
        Self: Sized,
        F: FnMut<(T,)>
    {
        self.map_into_owned(map)
    }
}

impl<'b, T, D> MaybeContainer<T> for ArrayView<'b, T, D>
where
    D: Dimension,
    <D as Dimension>::Pattern: NdIndex<D>
{
    type View<'a> = ArrayView<'b, T, D>
    where
        T: 'a,
        Self: 'a;
    type Owned = ArrayBase<OwnedRepr<T>, D>;
    type Some = Self;
    type MaybeSome = Self
    where
        (): Maybe<Self>;
    type MaybeMapped<MM> = <Self as Container<T>>::Mapped<MM>;

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
    fn into_maybe_some(self) -> Self::MaybeSome
    where
        Self: Sized,
        (): Maybe<Self::Some>
    {
        self
    }
    fn maybe_map_to_owned<'a, F>(&'a self, map: F) -> Self::MaybeMapped<F::Output>
    where
        T: 'a,
        F: FnMut<(&'a T,)>
    {
        self.map_to_owned(map)
    }
    fn maybe_map_into_owned<F>(self, map: F) -> Self::MaybeMapped<F::Output>
    where
        T: Clone,
        Self: Sized,
        F: FnMut<(T,)>
    {
        self.map_into_owned(map)
    }
}