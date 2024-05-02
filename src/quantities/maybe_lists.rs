

use ndarray::{prelude::Array1, Array2, ArrayView1, ArrayView2};
use option_trait::StaticMaybe;

use crate::quantities::{ListOrSingle, Lists, MaybeContainer, MaybeList, ListsOrSingle};

pub trait MaybeLists<T>: MaybeContainer<T>
{
    type Height: StaticMaybe<usize>;
    type Width: StaticMaybe<usize>;
    const HEIGHT: usize;
    const WIDTH: usize;
    const IS_FLATTENED: bool;

    type RowsMapped<M>: ListOrSingle<M>;
    type RowView<'a>: MaybeList<T> + 'a
    where
        T: 'a,
        Self: 'a;
    type RowOwned: MaybeList<T>;

    fn index_view<'a>(&'a self, i: usize) -> Self::RowView<'a>
    where
        T: 'a,
        Self: 'a;
    fn as_views_option<'a>(&'a self) -> Option<Vec<Self::RowView<'a>>>
    where
        T: 'a,
        Self: 'a;
    fn as_view_slices_option<'a>(&'a self) -> Option<Vec<&'a [T]>>
    where
        T: 'a,
        Self: 'a;
    fn map_rows_to_owned<'a, F>(&'a self, map: F) -> Self::RowsMapped<F::Output>
    where
        T: 'a,
        Self: 'a,
        F: FnMut<(Self::RowView<'a>,)>;
    fn map_rows_into_owned<F>(self, map: F) -> Self::RowsMapped<F::Output>
    where
        T: Clone,
        Self: Sized,
        F: FnMut<(Self::RowOwned,)>;
    fn try_map_rows_to_owned<'a, F, O, E>(&'a self, map: F) -> Result<Self::RowsMapped<O>, E>
    where
        T: 'a,
        Self: 'a,
        F: FnMut(Self::RowView<'a>) -> Result<O, E>;
    fn try_map_rows_into_owned<F, O, E>(self, map: F) -> Result<Self::RowsMapped<O>, E>
    where
        T: Clone,
        Self: Sized,
        F: FnMut(Self::RowOwned) -> Result<O, E>;
    fn into_owned_rows(self) -> Vec<Self::RowOwned>
    where
        T: Clone,
        Self: Sized;
    fn to_vecs_option(&self) -> Option<Vec<Vec<T>>>
    where
        T: Clone;
    fn into_vecs_option(self) -> Option<Vec<Vec<T>>>
    where
        T: Clone,
        Self: Sized;
}

impl<T> MaybeLists<T> for ()
{
    type Height = usize;
    type Width = usize;
    const HEIGHT: usize = 1;
    const WIDTH: usize = 1;
    const IS_FLATTENED: bool = true;

    type RowsMapped<M> = M;
    type RowView<'a> = ()
    where
        T: 'a,
        Self: 'a;
    type RowOwned = ();

    fn index_view<'a>(&'a self, _i: usize) -> Self::RowView<'a>
    where
        T: 'a,
        Self: 'a
    {
        
    }
    fn as_views_option<'a>(&'a self) -> Option<Vec<Self::RowView<'a>>>
        where
            T: 'a,
            Self: 'a
    {
        None
    }
    fn as_view_slices_option<'a>(&'a self) -> Option<Vec<&'a [T]>>
    where
        T: 'a,
        Self: 'a
    {
        None
    }
    fn map_rows_to_owned<'a, F>(&'a self, mut map: F) -> Self::RowsMapped<F::Output>
    where
        T: 'a,
        Self: 'a,
        F: FnMut<(Self::RowView<'a>,)>
    {
        map(())
    }
    fn map_rows_into_owned<F>(self, mut map: F) -> Self::RowsMapped<F::Output>
    where
        T: Clone,
        Self: Sized,
        F: FnMut<(Self::RowOwned,)>
    {
        map(())
    }
    fn try_map_rows_to_owned<'a, F, O, E>(&'a self, mut map: F) -> Result<Self::RowsMapped<O>, E>
    where
        T: 'a,
        Self: 'a,
        F: FnMut(Self::RowView<'a>) -> Result<O, E>
    {
        map(())
    }
    fn try_map_rows_into_owned<F, O, E>(self, mut map: F) -> Result<Self::RowsMapped<O>, E>
    where
        T: Clone,
        Self: Sized,
        F: FnMut(Self::RowOwned) -> Result<O, E>
    {
        map(())
    }
    fn into_owned_rows(self) -> Vec<Self::RowOwned>
    where
        T: Clone
    {
        vec![()]
    }
    fn to_vecs_option(&self) -> Option<Vec<Vec<T>>>
    where
        T: Clone
    {
        None
    }
    fn into_vecs_option(self) -> Option<Vec<Vec<T>>>
    where
        T: Clone,
        Self: Sized
    {
        None
    }
}

impl<T> MaybeLists<T> for Vec<T>
{
    type Height = usize;
    type Width = ();
    const HEIGHT: usize = 1;
    const WIDTH: usize = usize::MAX;
    const IS_FLATTENED: bool = true;

    type RowsMapped<M> = M;
    type RowView<'a> = &'a [T]
    where
        T: 'a,
        Self: 'a;
    type RowOwned = Vec<T>;
        
    fn index_view<'a>(&'a self, i: usize) -> Self::RowView<'a>
    where
        T: 'a,
        Self: 'a
    {
        core::slice::from_ref(&self.as_slice())[i]
    }
    fn as_views_option<'a>(&'a self) -> Option<Vec<Self::RowView<'a>>>
    where
        T: 'a,
        Self: 'a
    {
        Some(Lists::<T>::as_views(self))
    }
    fn as_view_slices_option<'a>(&'a self) -> Option<Vec<&'a [T]>>
    where
        T: 'a,
        Self: 'a
    {
        Some(ListsOrSingle::<T>::as_view_slices(self))
    }
    fn map_rows_to_owned<'a, F>(&'a self, mut map: F) -> Self::RowsMapped<F::Output>
    where
        T: 'a,
        Self: 'a,
        F: FnMut<(Self::RowView<'a>,)>
    {
        map(self.as_view())
    }
    fn map_rows_into_owned<F>(self, mut map: F) -> Self::RowsMapped<F::Output>
    where
        T: Clone,
        Self: Sized,
        F: FnMut<(Self::RowOwned,)>
    {
        map(self)
    }
    fn try_map_rows_to_owned<'a, F, O, E>(&'a self, mut map: F) -> Result<Self::RowsMapped<O>, E>
    where
        T: 'a,
        Self: 'a,
        F: FnMut(Self::RowView<'a>) -> Result<O, E>
    {
        map(self.as_view())
    }
    fn try_map_rows_into_owned<F, O, E>(self, mut map: F) -> Result<Self::RowsMapped<O>, E>
    where
        T: Clone,
        Self: Sized,
        F: FnMut(Self::RowOwned) -> Result<O, E>
    {
        map(self)
    }
    fn into_owned_rows(self) -> Vec<Self::RowOwned>
    where
        T: Clone
    {
        vec![self]
    }
    fn to_vecs_option(&self) -> Option<Vec<Vec<T>>>
    where
        T: Clone
    {
        Some(self.to_vecs())
    }
    fn into_vecs_option(self) -> Option<Vec<Vec<T>>>
    where
        T: Clone,
        Self: Sized
    {
        Some(self.into_vecs())
    }
}
impl<T> MaybeLists<T> for [T]
{
    type Height = usize;
    type Width = ();
    const HEIGHT: usize = 1;
    const WIDTH: usize = usize::MAX;
    const IS_FLATTENED: bool = true;

    type RowsMapped<M> = M;
    type RowView<'a> = &'a [T]
    where
        T: 'a,
        Self: 'a;
    type RowOwned = Vec<T>;
        
    fn index_view<'a>(&'a self, i: usize) -> Self::RowView<'a>
    where
        T: 'a,
        Self: 'a
    {
        core::slice::from_ref(&self)[i]
    }
    fn as_views_option<'a>(&'a self) -> Option<Vec<Self::RowView<'a>>>
    where
        T: 'a,
        Self: 'a
    {
        Some(Lists::<T>::as_views(self))
    }
    fn as_view_slices_option<'a>(&'a self) -> Option<Vec<&'a [T]>>
    where
        T: 'a,
        Self: 'a
    {
        Some(ListsOrSingle::<T>::as_view_slices(self))
    }
    fn map_rows_to_owned<'a, F>(&'a self, mut map: F) -> Self::RowsMapped<F::Output>
    where
        T: 'a,
        Self: 'a,
        F: FnMut<(Self::RowView<'a>,)>
    {
        map(self.as_view())
    }
    fn map_rows_into_owned<F>(self, mut map: F) -> Self::RowsMapped<F::Output>
    where
        T: Clone,
        Self: Sized,
        F: FnMut<(Self::RowOwned,)>
    {
        map(self.to_vec())
    }
    fn try_map_rows_to_owned<'a, F, O, E>(&'a self, mut map: F) -> Result<Self::RowsMapped<O>, E>
    where
        T: 'a,
        Self: 'a,
        F: FnMut(Self::RowView<'a>) -> Result<O, E>
    {
        map(self.as_view())
    }
    fn try_map_rows_into_owned<F, O, E>(self, mut map: F) -> Result<Self::RowsMapped<O>, E>
    where
        T: Clone,
        Self: Sized,
        F: FnMut(Self::RowOwned) -> Result<O, E>
    {
        map(self.to_vec())
    }
    fn into_owned_rows(self) -> Vec<Self::RowOwned>
    where
        T: Clone,
        Self: Sized
    {
        vec![self.to_vec()]
    }
    fn to_vecs_option(&self) -> Option<Vec<Vec<T>>>
    where
        T: Clone
    {
        Some(self.to_vecs())
    }
    fn into_vecs_option(self) -> Option<Vec<Vec<T>>>
    where
        T: Clone,
        Self: Sized
    {
        Some(self.into_vecs())
    }
}
impl<T, const N: usize> MaybeLists<T> for [T; N]
{
    type Height = usize;
    type Width = usize;
    const HEIGHT: usize = 1;
    const WIDTH: usize = N;
    const IS_FLATTENED: bool = true;

    type RowsMapped<M> = M;
    type RowView<'a> = &'a [T; N]
    where
        T: 'a,
        Self: 'a;
    type RowOwned = [T; N];
        
    fn index_view<'a>(&'a self, i: usize) -> Self::RowView<'a>
    where
        T: 'a,
        Self: 'a
    {
        core::slice::from_ref(&self)[i]
    }
    fn as_views_option<'a>(&'a self) -> Option<Vec<Self::RowView<'a>>>
    where
        T: 'a,
        Self: 'a
    {
        Some(Lists::<T>::as_views(self))
    }
    fn as_view_slices_option<'a>(&'a self) -> Option<Vec<&'a [T]>>
    where
        T: 'a,
        Self: 'a
    {
        Some(ListsOrSingle::<T>::as_view_slices(self))
    }
    fn map_rows_to_owned<'a, F>(&'a self, mut map: F) -> Self::RowsMapped<F::Output>
    where
        T: 'a,
        Self: 'a,
        F: FnMut<(Self::RowView<'a>,)>
    {
        map(self.as_view())
    }
    fn map_rows_into_owned<F>(self, mut map: F) -> Self::RowsMapped<F::Output>
    where
        T: Clone,
        Self: Sized,
        F: FnMut<(Self::RowOwned,)>
    {
        map(self)
    }
    fn try_map_rows_to_owned<'a, F, O, E>(&'a self, mut map: F) -> Result<Self::RowsMapped<O>, E>
    where
        T: 'a,
        Self: 'a,
        F: FnMut(Self::RowView<'a>) -> Result<O, E>
    {
        map(self.as_view())
    }
    fn try_map_rows_into_owned<F, O, E>(self, mut map: F) -> Result<Self::RowsMapped<O>, E>
    where
        T: Clone,
        Self: Sized,
        F: FnMut(Self::RowOwned) -> Result<O, E>
    {
        map(self)
    }
    fn into_owned_rows(self) -> Vec<Self::RowOwned>
    where
        T: Clone
    {
        vec![self]
    }
    fn to_vecs_option(&self) -> Option<Vec<Vec<T>>>
    where
        T: Clone
    {
        Some(self.to_vecs())
    }
    fn into_vecs_option(self) -> Option<Vec<Vec<T>>>
    where
        T: Clone,
        Self: Sized
    {
        Some(self.into_vecs())
    }
}
impl<T> MaybeLists<T> for &[T]
{
    type Height = usize;
    type Width = ();
    const HEIGHT: usize = 1;
    const WIDTH: usize = usize::MAX;
    const IS_FLATTENED: bool = true;

    type RowsMapped<M> = M;
    type RowView<'a> = &'a [T]
    where
        T: 'a,
        Self: 'a;
    type RowOwned = Vec<T>;
        
    fn index_view<'a>(&'a self, i: usize) -> Self::RowView<'a>
    where
        T: 'a,
        Self: 'a
    {
        core::slice::from_ref(self)[i]
    }
    fn as_views_option<'a>(&'a self) -> Option<Vec<Self::RowView<'a>>>
    where
        T: 'a,
        Self: 'a
    {
        Some(Lists::<T>::as_views(self))
    }
    fn as_view_slices_option<'a>(&'a self) -> Option<Vec<&'a [T]>>
    where
        T: 'a,
        Self: 'a
    {
        Some(ListsOrSingle::<T>::as_view_slices(self))
    }
    fn map_rows_to_owned<'a, F>(&'a self, mut map: F) -> Self::RowsMapped<F::Output>
    where
        T: 'a,
        Self: 'a,
        F: FnMut<(Self::RowView<'a>,)>
    {
        map(self.as_view())
    }
    fn map_rows_into_owned<F>(self, mut map: F) -> Self::RowsMapped<F::Output>
    where
        T: Clone,
        Self: Sized,
        F: FnMut<(Self::RowOwned,)>
    {
        map(self.to_vec())
    }
    fn try_map_rows_to_owned<'a, F, O, E>(&'a self, mut map: F) -> Result<Self::RowsMapped<O>, E>
    where
        T: 'a,
        Self: 'a,
        F: FnMut(Self::RowView<'a>) -> Result<O, E>
    {
        map(self.as_view())
    }
    fn try_map_rows_into_owned<F, O, E>(self, mut map: F) -> Result<Self::RowsMapped<O>, E>
    where
        T: Clone,
        Self: Sized,
        F: FnMut(Self::RowOwned) -> Result<O, E>
    {
        map(self.to_vec())
    }
    fn into_owned_rows(self) -> Vec<Self::RowOwned>
    where
        T: Clone
    {
        vec![self.to_vec()]
    }
    fn to_vecs_option(&self) -> Option<Vec<Vec<T>>>
    where
        T: Clone
    {
        Some(self.to_vecs())
    }
    fn into_vecs_option(self) -> Option<Vec<Vec<T>>>
    where
        T: Clone,
        Self: Sized
    {
        Some(self.into_vecs())
    }
}
impl<T, const N: usize> MaybeLists<T> for &[T; N]
{
    type Height = usize;
    type Width = usize;
    const HEIGHT: usize = 1;
    const WIDTH: usize = N;
    const IS_FLATTENED: bool = true;

    type RowsMapped<M> = M;
    type RowView<'a> = &'a [T; N]
    where
        T: 'a,
        Self: 'a;
    type RowOwned = [T; N];
        
    fn index_view<'a>(&'a self, i: usize) -> Self::RowView<'a>
    where
        T: 'a,
        Self: 'a
    {
        core::slice::from_ref(self)[i]
    }
    fn as_views_option<'a>(&'a self) -> Option<Vec<Self::RowView<'a>>>
    where
        T: 'a,
        Self: 'a
    {
        Some(Lists::<T>::as_views(self))
    }
    fn as_view_slices_option<'a>(&'a self) -> Option<Vec<&'a [T]>>
    where
        T: 'a,
        Self: 'a
    {
        Some(ListsOrSingle::<T>::as_view_slices(self))
    }
    fn map_rows_to_owned<'a, F>(&'a self, mut map: F) -> Self::RowsMapped<F::Output>
    where
        T: 'a,
        Self: 'a,
        F: FnMut<(Self::RowView<'a>,)>
    {
        map(self.as_view())
    }
    fn map_rows_into_owned<F>(self, mut map: F) -> Self::RowsMapped<F::Output>
    where
        T: Clone,
        Self: Sized,
        F: FnMut<(Self::RowOwned,)>
    {
        map(self.clone())
    }
    fn try_map_rows_to_owned<'a, F, O, E>(&'a self, mut map: F) -> Result<Self::RowsMapped<O>, E>
    where
        T: 'a,
        Self: 'a,
        F: FnMut(Self::RowView<'a>) -> Result<O, E>
    {
        map(self.as_view())
    }
    fn try_map_rows_into_owned<F, O, E>(self, mut map: F) -> Result<Self::RowsMapped<O>, E>
    where
        T: Clone,
        Self: Sized,
        F: FnMut(Self::RowOwned) -> Result<O, E>
    {
        map(self.clone())
    }
    fn into_owned_rows(self) -> Vec<Self::RowOwned>
    where
        T: Clone
    {
        vec![self.clone()]
    }
    fn to_vecs_option(&self) -> Option<Vec<Vec<T>>>
    where
        T: Clone
    {
        Some(self.to_vecs())
    }
    fn into_vecs_option(self) -> Option<Vec<Vec<T>>>
    where
        T: Clone,
        Self: Sized
    {
        Some(self.into_vecs())
    }
}

impl<T> MaybeLists<T> for Vec<Vec<T>>
{
    type Height = ();
    type Width = ();
    const HEIGHT: usize = usize::MAX;
    const WIDTH: usize = usize::MAX;
    const IS_FLATTENED: bool = false;

    type RowsMapped<MM> = Vec<MM>;
    type RowView<'a> = &'a [T]
    where
        T: 'a,
        Self: 'a;
    type RowOwned = Vec<T>;

    fn index_view<'a>(&'a self, i: usize) -> Self::RowView<'a>
    where
        T: 'a,
        Self: 'a
    {
        self[i].as_view()
    }
    fn as_views_option<'a>(&'a self) -> Option<Vec<Self::RowView<'a>>>
    where
        T: 'a,
        Self: 'a
    {
        Some(Lists::<T>::as_views(self))
    }
    fn as_view_slices_option<'a>(&'a self) -> Option<Vec<&'a [T]>>
    where
        T: 'a,
        Self: 'a
    {
        Some(ListsOrSingle::<T>::as_view_slices(self))
    }
    fn map_rows_to_owned<'a, F>(&'a self, mut map: F) -> Self::RowsMapped<F::Output>
    where
        T: 'a,
        Self: 'a,
        F: FnMut<(Self::RowView<'a>,)>
    {
        self.iter()
            .map(|r| map(r.as_view()))
            .collect()
    }
    fn map_rows_into_owned<F>(self, mut map: F) -> Self::RowsMapped<F::Output>
    where
        T: Clone,
        Self: Sized,
        F: FnMut<(Self::RowOwned,)>
    {
        self.into_iter()
            .map(|r| map(r))
            .collect()
    }
    fn try_map_rows_to_owned<'a, F, O, E>(&'a self, mut map: F) -> Result<Self::RowsMapped<O>, E>
    where
        T: 'a,
        Self: 'a,
        F: FnMut(Self::RowView<'a>) -> Result<O, E>
    {
        self.iter()
            .map(|r| map(r.as_view()))
            .collect()
    }
    fn try_map_rows_into_owned<F, O, E>(self, mut map: F) -> Result<Self::RowsMapped<O>, E>
    where
        T: Clone,
        Self: Sized,
        F: FnMut(Self::RowOwned) -> Result<O, E>
    {
        self.into_iter()
            .map(|r| map(r))
            .collect()
    }
    fn into_owned_rows(self) -> Vec<Self::RowOwned>
    where
        T: Clone
    {
        self
    }
    fn to_vecs_option(&self) -> Option<Vec<Vec<T>>>
    where
        T: Clone
    {
        Some(self.to_vecs())
    }
    fn into_vecs_option(self) -> Option<Vec<Vec<T>>>
    where
        T: Clone,
        Self: Sized
    {
        Some(self.into_vecs())
    }
}
impl<T, const M: usize> MaybeLists<T> for [Vec<T>; M]
{
    type Height = usize;
    type Width = ();
    const HEIGHT: usize = M;
    const WIDTH: usize = usize::MAX;
    const IS_FLATTENED: bool = false;

    type RowsMapped<MM> = [MM; M];
    type RowView<'a> = &'a [T]
    where
        T: 'a,
        Self: 'a;
    type RowOwned = Vec<T>;
        
    fn index_view<'a>(&'a self, i: usize) -> Self::RowView<'a>
    where
        T: 'a,
        Self: 'a
    {
        self[i].as_view()
    }
    fn as_views_option<'a>(&'a self) -> Option<Vec<Self::RowView<'a>>>
    where
        T: 'a,
        Self: 'a
    {
        Some(Lists::<T>::as_views(self))
    }
    fn as_view_slices_option<'a>(&'a self) -> Option<Vec<&'a [T]>>
    where
        T: 'a,
        Self: 'a
    {
        Some(ListsOrSingle::<T>::as_view_slices(self))
    }
    fn map_rows_to_owned<'a, F>(&'a self, mut map: F) -> Self::RowsMapped<F::Output>
    where
        T: 'a,
        Self: 'a,
        F: FnMut<(Self::RowView<'a>,)>
    {
        self.each_ref()
            .map(|r| map(r.as_view()))
    }
    fn map_rows_into_owned<F>(self, mut map: F) -> Self::RowsMapped<F::Output>
    where
        T: Clone,
        Self: Sized,
        F: FnMut<(Self::RowOwned,)>
    {
        self.map(|r| map(r))
    }
    fn try_map_rows_to_owned<'a, F, O, E>(&'a self, mut map: F) -> Result<Self::RowsMapped<O>, E>
    where
        T: 'a,
        Self: 'a,
        F: FnMut(Self::RowView<'a>) -> Result<O, E>
    {
        self.each_ref()
            .try_map(|r| map(r.as_view()))
    }
    fn try_map_rows_into_owned<F, O, E>(self, mut map: F) -> Result<Self::RowsMapped<O>, E>
    where
        T: Clone,
        Self: Sized,
        F: FnMut(Self::RowOwned) -> Result<O, E>
    {
        self.try_map(|r| map(r))
    }
    fn into_owned_rows(self) -> Vec<Self::RowOwned>
    where
        T: Clone
    {
        self.into_iter()
            .collect()
    }
    fn to_vecs_option(&self) -> Option<Vec<Vec<T>>>
    where
        T: Clone
    {
        Some(self.to_vecs())
    }
    fn into_vecs_option(self) -> Option<Vec<Vec<T>>>
    where
        T: Clone,
        Self: Sized
    {
        Some(self.into_vecs())
    }
}
impl<T> MaybeLists<T> for [Vec<T>]
{
    type Height = ();
    type Width = ();
    const HEIGHT: usize = usize::MAX;
    const WIDTH: usize = usize::MAX;
    const IS_FLATTENED: bool = false;

    type RowsMapped<MM> = Vec<MM>;
    type RowView<'a> = &'a [T]
    where
        T: 'a,
        Self: 'a;
    type RowOwned = Vec<T>;

    fn index_view<'a>(&'a self, i: usize) -> Self::RowView<'a>
    where
        T: 'a,
        Self: 'a
    {
        self[i].as_view()
    }
    fn as_views_option<'a>(&'a self) -> Option<Vec<Self::RowView<'a>>>
    where
        T: 'a,
        Self: 'a
    {
        Some(Lists::<T>::as_views(self))
    }
    fn as_view_slices_option<'a>(&'a self) -> Option<Vec<&'a [T]>>
    where
        T: 'a,
        Self: 'a
    {
        Some(ListsOrSingle::<T>::as_view_slices(self))
    }
    fn map_rows_to_owned<'a, F>(&'a self, mut map: F) -> Self::RowsMapped<F::Output>
    where
        T: 'a,
        Self: 'a,
        F: FnMut<(Self::RowView<'a>,)>
    {
        self.iter()
            .map(|r| map(r.as_view()))
            .collect()
    }
    fn map_rows_into_owned<F>(self, mut map: F) -> Self::RowsMapped<F::Output>
    where
        T: Clone,
        Self: Sized,
        F: FnMut<(Self::RowOwned,)>
    {
        self.iter()
            .map(|r| map(r.clone()))
            .collect()
    }
    fn try_map_rows_to_owned<'a, F, O, E>(&'a self, mut map: F) -> Result<Self::RowsMapped<O>, E>
    where
        T: 'a,
        Self: 'a,
        F: FnMut(Self::RowView<'a>) -> Result<O, E>
    {
        self.iter()
            .map(|r| map(r.as_view()))
            .collect()
    }
    fn try_map_rows_into_owned<F, O, E>(self, mut map: F) -> Result<Self::RowsMapped<O>, E>
    where
        T: Clone,
        Self: Sized,
        F: FnMut(Self::RowOwned) -> Result<O, E>
    {
        self.iter()
            .map(|r| map(r.clone()))
            .collect()
    }
    fn into_owned_rows(self) -> Vec<Self::RowOwned>
    where
        T: Clone,
        Self: Sized
    {
        self.iter()
            .map(|r| r.clone())
            .collect()
    }
    fn to_vecs_option(&self) -> Option<Vec<Vec<T>>>
    where
        T: Clone
    {
        Some(self.to_vecs())
    }
    fn into_vecs_option(self) -> Option<Vec<Vec<T>>>
    where
        T: Clone,
        Self: Sized
    {
        Some(self.into_vecs())
    }
}
impl<T, const M: usize> MaybeLists<T> for &[Vec<T>; M]
{
    type Height = usize;
    type Width = ();
    const HEIGHT: usize = M;
    const WIDTH: usize = usize::MAX;
    const IS_FLATTENED: bool = false;

    type RowsMapped<MM> = [MM; M];
    type RowView<'a> = &'a [T]
    where
        T: 'a,
        Self: 'a;
    type RowOwned = Vec<T>;
        
    fn index_view<'a>(&'a self, i: usize) -> Self::RowView<'a>
    where
        T: 'a,
        Self: 'a
    {
        self[i].as_view()
    }
    fn as_views_option<'a>(&'a self) -> Option<Vec<Self::RowView<'a>>>
    where
        T: 'a,
        Self: 'a
    {
        Some(Lists::<T>::as_views(self))
    }
    fn as_view_slices_option<'a>(&'a self) -> Option<Vec<&'a [T]>>
    where
        T: 'a,
        Self: 'a
    {
        Some(ListsOrSingle::<T>::as_view_slices(self))
    }
    fn map_rows_to_owned<'a, F>(&'a self, mut map: F) -> Self::RowsMapped<F::Output>
    where
        T: 'a,
        Self: 'a,
        F: FnMut<(Self::RowView<'a>,)>
    {
        self.each_ref()
            .map(|r| map(r.as_view()))
    }
    fn map_rows_into_owned<F>(self, mut map: F) -> Self::RowsMapped<F::Output>
    where
        T: Clone,
        Self: Sized,
        F: FnMut<(Self::RowOwned,)>
    {
        self.each_ref()
            .map(|r| map(r.clone()))
    }
    fn try_map_rows_to_owned<'a, F, O, E>(&'a self, mut map: F) -> Result<Self::RowsMapped<O>, E>
    where
        T: 'a,
        Self: 'a,
        F: FnMut(Self::RowView<'a>) -> Result<O, E>
    {
        self.each_ref()
            .try_map(|r| map(r.as_view()))
    }
    fn try_map_rows_into_owned<F, O, E>(self, mut map: F) -> Result<Self::RowsMapped<O>, E>
    where
        T: Clone,
        Self: Sized,
        F: FnMut(Self::RowOwned) -> Result<O, E>
    {
        self.each_ref()
            .try_map(|r| map(r.clone()))
    }
    fn into_owned_rows(self) -> Vec<Self::RowOwned>
    where
        T: Clone
    {
        self.iter()
            .map(|r| r.clone())
            .collect()
    }
    fn to_vecs_option(&self) -> Option<Vec<Vec<T>>>
    where
        T: Clone
    {
        Some(self.to_vecs())
    }
    fn into_vecs_option(self) -> Option<Vec<Vec<T>>>
    where
        T: Clone,
        Self: Sized
    {
        Some(self.into_vecs())
    }
}
impl<T> MaybeLists<T> for &[Vec<T>]
{
    type Height = ();
    type Width = ();
    const HEIGHT: usize = usize::MAX;
    const WIDTH: usize = usize::MAX;
    const IS_FLATTENED: bool = false;

    type RowsMapped<MM> = Vec<MM>;
    type RowView<'a> = &'a [T]
    where
        T: 'a,
        Self: 'a;
    type RowOwned = Vec<T>;
        
    fn index_view<'a>(&'a self, i: usize) -> Self::RowView<'a>
    where
        T: 'a,
        Self: 'a
    {
        self[i].as_view()
    }
    fn as_views_option<'a>(&'a self) -> Option<Vec<Self::RowView<'a>>>
    where
        T: 'a,
        Self: 'a
    {
        Some(Lists::<T>::as_views(self))
    }
    fn as_view_slices_option<'a>(&'a self) -> Option<Vec<&'a [T]>>
    where
        T: 'a,
        Self: 'a
    {
        Some(ListsOrSingle::<T>::as_view_slices(self))
    }
    fn map_rows_to_owned<'a, F>(&'a self, mut map: F) -> Self::RowsMapped<F::Output>
    where
        T: 'a,
        Self: 'a,
        F: FnMut<(Self::RowView<'a>,)>
    {
        self.iter()
            .map(|r| map(r.as_view()))
            .collect()
    }
    fn map_rows_into_owned<F>(self, mut map: F) -> Self::RowsMapped<F::Output>
    where
        T: Clone,
        Self: Sized,
        F: FnMut<(Self::RowOwned,)>
    {
        self.iter()
            .map(|r| map(r.clone()))
            .collect()
    }
    fn try_map_rows_to_owned<'a, F, O, E>(&'a self, mut map: F) -> Result<Self::RowsMapped<O>, E>
    where
        T: 'a,
        Self: 'a,
        F: FnMut(Self::RowView<'a>) -> Result<O, E>
    {
        self.iter()
            .map(|r| map(r.as_view()))
            .collect()
    }
    fn try_map_rows_into_owned<F, O, E>(self, mut map: F) -> Result<Self::RowsMapped<O>, E>
    where
        T: Clone,
        Self: Sized,
        F: FnMut(Self::RowOwned) -> Result<O, E>
    {
        self.iter()
            .map(|r| map(r.clone()))
            .collect()
    }
    fn into_owned_rows(self) -> Vec<Self::RowOwned>
    where
        T: Clone
    {
        self.iter()
            .map(|r| r.clone())
            .collect()
    }
    fn to_vecs_option(&self) -> Option<Vec<Vec<T>>>
    where
        T: Clone
    {
        Some(self.to_vecs())
    }
    fn into_vecs_option(self) -> Option<Vec<Vec<T>>>
    where
        T: Clone,
        Self: Sized
    {
        Some(self.into_vecs())
    }
}

impl<T, const N: usize> MaybeLists<T> for Vec<[T; N]>
{
    type Height = ();
    type Width = usize;
    const HEIGHT: usize = usize::MAX;
    const WIDTH: usize = N;
    const IS_FLATTENED: bool = false;

    type RowsMapped<MM> = Vec<MM>;
    type RowView<'a> = &'a [T; N]
    where
        T: 'a,
        Self: 'a;
    type RowOwned = [T; N];
        
    fn index_view<'a>(&'a self, i: usize) -> Self::RowView<'a>
    where
        T: 'a,
        Self: 'a
    {
        self[i].as_view()
    }
    fn as_views_option<'a>(&'a self) -> Option<Vec<Self::RowView<'a>>>
    where
        T: 'a,
        Self: 'a
    {
        Some(Lists::<T>::as_views(self))
    }
    fn as_view_slices_option<'a>(&'a self) -> Option<Vec<&'a [T]>>
    where
        T: 'a,
        Self: 'a
    {
        Some(ListsOrSingle::<T>::as_view_slices(self))
    }
    fn map_rows_to_owned<'a, F>(&'a self, mut map: F) -> Self::RowsMapped<F::Output>
    where
        T: 'a,
        Self: 'a,
        F: FnMut<(Self::RowView<'a>,)>
    {
        self.iter()
            .map(|r| map(r.as_view()))
            .collect()
    }
    fn map_rows_into_owned<F>(self, mut map: F) -> Self::RowsMapped<F::Output>
    where
        T: Clone,
        Self: Sized,
        F: FnMut<(Self::RowOwned,)>
    {
        self.into_iter()
            .map(|r| map(r))
            .collect()
    }
    fn try_map_rows_to_owned<'a, F, O, E>(&'a self, mut map: F) -> Result<Self::RowsMapped<O>, E>
    where
        T: 'a,
        Self: 'a,
        F: FnMut(Self::RowView<'a>) -> Result<O, E>
    {
        self.iter()
            .map(|r| map(r.as_view()))
            .collect()
    }
    fn try_map_rows_into_owned<F, O, E>(self, mut map: F) -> Result<Self::RowsMapped<O>, E>
    where
        T: Clone,
        Self: Sized,
        F: FnMut(Self::RowOwned) -> Result<O, E>
    {
        self.into_iter()
            .map(|r| map(r))
            .collect()
    }
    fn into_owned_rows(self) -> Vec<Self::RowOwned>
    where
        T: Clone
    {
        self
    }
    fn to_vecs_option(&self) -> Option<Vec<Vec<T>>>
    where
        T: Clone
    {
        Some(self.to_vecs())
    }
    fn into_vecs_option(self) -> Option<Vec<Vec<T>>>
    where
        T: Clone,
        Self: Sized
    {
        Some(self.into_vecs())
    }
}
impl<T, const N: usize, const M: usize> MaybeLists<T> for [[T; N]; M]
{
    type Height = usize;
    type Width = usize;
    const HEIGHT: usize = M;
    const WIDTH: usize = N;
    const IS_FLATTENED: bool = false;

    type RowsMapped<MM> = [MM; M];
    type RowView<'a> = &'a [T; N]
    where
        T: 'a,
        Self: 'a;
    type RowOwned = [T; N];
        
    fn index_view<'a>(&'a self, i: usize) -> Self::RowView<'a>
    where
        T: 'a,
        Self: 'a
    {
        self[i].as_view()
    }
    fn as_views_option<'a>(&'a self) -> Option<Vec<Self::RowView<'a>>>
    where
        T: 'a,
        Self: 'a
    {
        Some(Lists::<T>::as_views(self))
    }
    fn as_view_slices_option<'a>(&'a self) -> Option<Vec<&'a [T]>>
    where
        T: 'a,
        Self: 'a
    {
        Some(ListsOrSingle::<T>::as_view_slices(self))
    }
    fn map_rows_to_owned<'a, F>(&'a self, mut map: F) -> Self::RowsMapped<F::Output>
    where
        T: 'a,
        Self: 'a,
        F: FnMut<(Self::RowView<'a>,)>
    {
        self.each_ref()
            .map(|r| map(r.as_view()))
    }
    fn map_rows_into_owned<F>(self, mut map: F) -> Self::RowsMapped<F::Output>
    where
        T: Clone,
        Self: Sized,
        F: FnMut<(Self::RowOwned,)>
    {
        self.map(|r| map(r))
    }
    fn try_map_rows_to_owned<'a, F, O, E>(&'a self, mut map: F) -> Result<Self::RowsMapped<O>, E>
    where
        T: 'a,
        Self: 'a,
        F: FnMut(Self::RowView<'a>) -> Result<O, E>
    {
        self.each_ref()
            .try_map(|r| map(r.as_view()))
    }
    fn try_map_rows_into_owned<F, O, E>(self, mut map: F) -> Result<Self::RowsMapped<O>, E>
    where
        T: Clone,
        Self: Sized,
        F: FnMut(Self::RowOwned) -> Result<O, E>
    {
        self.try_map(|r| map(r))
    }
    fn into_owned_rows(self) -> Vec<Self::RowOwned>
    where
        T: Clone
    {
        self.into_iter()
            .collect()
    }
    fn to_vecs_option(&self) -> Option<Vec<Vec<T>>>
    where
        T: Clone
    {
        Some(self.to_vecs())
    }
    fn into_vecs_option(self) -> Option<Vec<Vec<T>>>
    where
        T: Clone,
        Self: Sized
    {
        Some(self.into_vecs())
    }
}
impl<T, const N: usize> MaybeLists<T> for [[T; N]]
{
    type Height = ();
    type Width = usize;
    const HEIGHT: usize = usize::MAX;
    const WIDTH: usize = N;
    const IS_FLATTENED: bool = false;

    type RowsMapped<MM> = Vec<MM>;
    type RowView<'a> = &'a [T; N]
    where
        T: 'a,
        Self: 'a;
    type RowOwned = [T; N];
        
    fn index_view<'a>(&'a self, i: usize) -> Self::RowView<'a>
    where
        T: 'a,
        Self: 'a
    {
        self[i].as_view()
    }
    fn as_views_option<'a>(&'a self) -> Option<Vec<Self::RowView<'a>>>
    where
        T: 'a,
        Self: 'a
    {
        Some(Lists::<T>::as_views(self))
    }
    fn as_view_slices_option<'a>(&'a self) -> Option<Vec<&'a [T]>>
    where
        T: 'a,
        Self: 'a
    {
        Some(ListsOrSingle::<T>::as_view_slices(self))
    }
    fn map_rows_to_owned<'a, F>(&'a self, mut map: F) -> Self::RowsMapped<F::Output>
    where
        T: 'a,
        Self: 'a,
        F: FnMut<(Self::RowView<'a>,)>
    {
        self.iter()
            .map(|r| map(r.as_view()))
            .collect()
    }
    fn map_rows_into_owned<F>(self, mut map: F) -> Self::RowsMapped<F::Output>
    where
        T: Clone,
        Self: Sized,
        F: FnMut<(Self::RowOwned,)>
    {
        self.iter()
            .map(|r| map(r.clone()))
            .collect()
    }
    fn try_map_rows_to_owned<'a, F, O, E>(&'a self, mut map: F) -> Result<Self::RowsMapped<O>, E>
    where
        T: 'a,
        Self: 'a,
        F: FnMut(Self::RowView<'a>) -> Result<O, E>
    {
        self.iter()
            .map(|r| map(r.as_view()))
            .collect()
    }
    fn try_map_rows_into_owned<F, O, E>(self, mut map: F) -> Result<Self::RowsMapped<O>, E>
    where
        T: Clone,
        Self: Sized,
        F: FnMut(Self::RowOwned) -> Result<O, E>
    {
        self.iter()
            .map(|r| map(r.clone()))
            .collect()
    }
    fn into_owned_rows(self) -> Vec<Self::RowOwned>
    where
        T: Clone,
        Self: Sized
    {
        self.iter()
            .map(|r| r.clone())
            .collect()
    }
    fn to_vecs_option(&self) -> Option<Vec<Vec<T>>>
    where
        T: Clone
    {
        Some(self.to_vecs())
    }
    fn into_vecs_option(self) -> Option<Vec<Vec<T>>>
    where
        T: Clone,
        Self: Sized
    {
        Some(self.into_vecs())
    }
}
impl<T, const N: usize, const M: usize> MaybeLists<T> for &[[T; N]; M]
{
    type Height = usize;
    type Width = usize;
    const HEIGHT: usize = M;
    const WIDTH: usize = N;
    const IS_FLATTENED: bool = false;

    type RowsMapped<MM> = [MM; M];
    type RowView<'a> = &'a [T; N]
    where
        T: 'a,
        Self: 'a;
    type RowOwned = [T; N];
        
    fn index_view<'a>(&'a self, i: usize) -> Self::RowView<'a>
    where
        T: 'a,
        Self: 'a
    {
        self[i].as_view()
    }
    fn as_views_option<'a>(&'a self) -> Option<Vec<Self::RowView<'a>>>
    where
        T: 'a,
        Self: 'a
    {
        Some(Lists::<T>::as_views(self))
    }
    fn as_view_slices_option<'a>(&'a self) -> Option<Vec<&'a [T]>>
    where
        T: 'a,
        Self: 'a
    {
        Some(ListsOrSingle::<T>::as_view_slices(self))
    }
    fn map_rows_to_owned<'a, F>(&'a self, mut map: F) -> Self::RowsMapped<F::Output>
    where
        T: 'a,
        Self: 'a,
        F: FnMut<(Self::RowView<'a>,)>
    {
        self.each_ref()
            .map(|r| map(r.as_view()))
    }
    fn map_rows_into_owned<F>(self, mut map: F) -> Self::RowsMapped<F::Output>
    where
        T: Clone,
        Self: Sized,
        F: FnMut<(Self::RowOwned,)>
    {
        self.each_ref()
            .map(|r| map(r.clone()))
    }
    fn try_map_rows_to_owned<'a, F, O, E>(&'a self, mut map: F) -> Result<Self::RowsMapped<O>, E>
    where
        T: 'a,
        Self: 'a,
        F: FnMut(Self::RowView<'a>) -> Result<O, E>
    {
        self.each_ref()
            .try_map(|r| map(r.as_view()))
    }
    fn try_map_rows_into_owned<F, O, E>(self, mut map: F) -> Result<Self::RowsMapped<O>, E>
    where
        T: Clone,
        Self: Sized,
        F: FnMut(Self::RowOwned) -> Result<O, E>
    {
        self.each_ref()
            .try_map(|r| map(r.clone()))
    }
    fn into_owned_rows(self) -> Vec<Self::RowOwned>
    where
        T: Clone
    {
        self.iter()
            .map(|r| r.clone())
            .collect()
    }
    fn to_vecs_option(&self) -> Option<Vec<Vec<T>>>
    where
        T: Clone
    {
        Some(self.to_vecs())
    }
    fn into_vecs_option(self) -> Option<Vec<Vec<T>>>
    where
        T: Clone,
        Self: Sized
    {
        Some(self.into_vecs())
    }
}
impl<T, const N: usize> MaybeLists<T> for &[[T; N]]
{
    type Height = ();
    type Width = usize;
    const HEIGHT: usize = usize::MAX;
    const WIDTH: usize = N;
    const IS_FLATTENED: bool = false;

    type RowsMapped<MM> = Vec<MM>;
    type RowView<'a> = &'a [T; N]
    where
        T: 'a,
        Self: 'a;
    type RowOwned = [T; N];
        
    fn index_view<'a>(&'a self, i: usize) -> Self::RowView<'a>
    where
        T: 'a,
        Self: 'a
    {
        self[i].as_view()
    }
    fn as_views_option<'a>(&'a self) -> Option<Vec<Self::RowView<'a>>>
    where
        T: 'a,
        Self: 'a
    {
        Some(Lists::<T>::as_views(self))
    }
    fn as_view_slices_option<'a>(&'a self) -> Option<Vec<&'a [T]>>
    where
        T: 'a,
        Self: 'a
    {
        Some(ListsOrSingle::<T>::as_view_slices(self))
    }
    fn map_rows_to_owned<'a, F>(&'a self, mut map: F) -> Self::RowsMapped<F::Output>
    where
        T: 'a,
        Self: 'a,
        F: FnMut<(Self::RowView<'a>,)>
    {
        self.iter()
            .map(|r| map(r.as_view()))
            .collect()
    }
    fn map_rows_into_owned<F>(self, mut map: F) -> Self::RowsMapped<F::Output>
    where
        T: Clone,
        Self: Sized,
        F: FnMut<(Self::RowOwned,)>
    {
        self.iter()
            .map(|r| map(r.clone()))
            .collect()
    }
    fn try_map_rows_to_owned<'a, F, O, E>(&'a self, mut map: F) -> Result<Self::RowsMapped<O>, E>
    where
        T: 'a,
        Self: 'a,
        F: FnMut(Self::RowView<'a>) -> Result<O, E>
    {
        self.iter()
            .map(|r| map(r.as_view()))
            .collect()
    }
    fn try_map_rows_into_owned<F, O, E>(self, mut map: F) -> Result<Self::RowsMapped<O>, E>
    where
        T: Clone,
        Self: Sized,
        F: FnMut(Self::RowOwned) -> Result<O, E>
    {
        self.iter()
            .map(|r| map(r.clone()))
            .collect()
    }
    fn into_owned_rows(self) -> Vec<Self::RowOwned>
    where
        T: Clone
    {
        self.iter()
            .map(|r| r.clone())
            .collect()
    }
    fn to_vecs_option(&self) -> Option<Vec<Vec<T>>>
    where
        T: Clone
    {
        Some(self.to_vecs())
    }
    fn into_vecs_option(self) -> Option<Vec<Vec<T>>>
    where
        T: Clone,
        Self: Sized
    {
        Some(self.into_vecs())
    }
}

impl<T> MaybeLists<T> for Vec<&[T]>
{
    type Height = ();
    type Width = ();
    const HEIGHT: usize = usize::MAX;
    const WIDTH: usize = usize::MAX;
    const IS_FLATTENED: bool = false;

    type RowsMapped<MM> = Vec<MM>;
    type RowView<'a> = &'a [T]
    where
        T: 'a,
        Self: 'a;
    type RowOwned = Vec<T>;
        
    fn index_view<'a>(&'a self, i: usize) -> Self::RowView<'a>
    where
        T: 'a,
        Self: 'a
    {
        self[i].as_view()
    }
    fn as_views_option<'a>(&'a self) -> Option<Vec<Self::RowView<'a>>>
    where
        T: 'a,
        Self: 'a
    {
        Some(Lists::<T>::as_views(self))
    }
    fn as_view_slices_option<'a>(&'a self) -> Option<Vec<&'a [T]>>
    where
        T: 'a,
        Self: 'a
    {
        Some(ListsOrSingle::<T>::as_view_slices(self))
    }
    fn map_rows_to_owned<'a, F>(&'a self, mut map: F) -> Self::RowsMapped<F::Output>
    where
        T: 'a,
        Self: 'a,
        F: FnMut<(Self::RowView<'a>,)>
    {
        self.iter()
            .map(|r| map(r.as_view()))
            .collect()
    }
    fn map_rows_into_owned<F>(self, mut map: F) -> Self::RowsMapped<F::Output>
    where
        T: Clone,
        Self: Sized,
        F: FnMut<(Self::RowOwned,)>
    {
        self.into_iter()
            .map(|r| map(r.to_vec()))
            .collect()
    }
    fn try_map_rows_to_owned<'a, F, O, E>(&'a self, mut map: F) -> Result<Self::RowsMapped<O>, E>
    where
        T: 'a,
        Self: 'a,
        F: FnMut(Self::RowView<'a>) -> Result<O, E>
    {
        self.iter()
            .map(|r| map(r.as_view()))
            .collect()
    }
    fn try_map_rows_into_owned<F, O, E>(self, mut map: F) -> Result<Self::RowsMapped<O>, E>
    where
        T: Clone,
        Self: Sized,
        F: FnMut(Self::RowOwned) -> Result<O, E>
    {
        self.into_iter()
            .map(|r| map(r.to_vec()))
            .collect()
    }
    fn into_owned_rows(self) -> Vec<Self::RowOwned>
    where
        T: Clone
    {
        self.into_iter()
            .map(|r| r.to_vec())
            .collect()
    }
    fn to_vecs_option(&self) -> Option<Vec<Vec<T>>>
    where
        T: Clone
    {
        Some(self.to_vecs())
    }
    fn into_vecs_option(self) -> Option<Vec<Vec<T>>>
    where
        T: Clone,
        Self: Sized
    {
        Some(self.into_vecs())
    }
}
impl<T, const M: usize> MaybeLists<T> for [&[T]; M]
{
    type Height = usize;
    type Width = ();
    const HEIGHT: usize = M;
    const WIDTH: usize = usize::MAX;
    const IS_FLATTENED: bool = false;

    type RowsMapped<MM> = [MM; M];
    type RowView<'a> = &'a [T]
    where
        T: 'a,
        Self: 'a;
    type RowOwned = Vec<T>;
        
    fn index_view<'a>(&'a self, i: usize) -> Self::RowView<'a>
    where
        T: 'a,
        Self: 'a
    {
        self[i].as_view()
    }
    fn as_views_option<'a>(&'a self) -> Option<Vec<Self::RowView<'a>>>
    where
        T: 'a,
        Self: 'a
    {
        Some(Lists::<T>::as_views(self))
    }
    fn as_view_slices_option<'a>(&'a self) -> Option<Vec<&'a [T]>>
    where
        T: 'a,
        Self: 'a
    {
        Some(ListsOrSingle::<T>::as_view_slices(self))
    }
    fn map_rows_to_owned<'a, F>(&'a self, mut map: F) -> Self::RowsMapped<F::Output>
    where
        T: 'a,
        Self: 'a,
        F: FnMut<(Self::RowView<'a>,)>
    {
        self.each_ref()
            .map(|r| map(r.as_view()))
    }
    fn map_rows_into_owned<F>(self, mut map: F) -> Self::RowsMapped<F::Output>
    where
        T: Clone,
        Self: Sized,
        F: FnMut<(Self::RowOwned,)>
    {
        self.map(|r| map(r.to_vec()))
    }
    fn try_map_rows_to_owned<'a, F, O, E>(&'a self, mut map: F) -> Result<Self::RowsMapped<O>, E>
    where
        T: 'a,
        Self: 'a,
        F: FnMut(Self::RowView<'a>) -> Result<O, E>
    {
        self.each_ref()
            .try_map(|r| map(r.as_view()))
    }
    fn try_map_rows_into_owned<F, O, E>(self, mut map: F) -> Result<Self::RowsMapped<O>, E>
    where
        T: Clone,
        Self: Sized,
        F: FnMut(Self::RowOwned) -> Result<O, E>
    {
        self.try_map(|r| map(r.to_vec()))
    }
    fn into_owned_rows(self) -> Vec<Self::RowOwned>
    where
        T: Clone
    {
        self.into_iter()
            .map(|r| r.to_vec())
            .collect()
    }
    fn to_vecs_option(&self) -> Option<Vec<Vec<T>>>
    where
        T: Clone
    {
        Some(self.to_vecs())
    }
    fn into_vecs_option(self) -> Option<Vec<Vec<T>>>
    where
        T: Clone,
        Self: Sized
    {
        Some(self.into_vecs())
    }
}
impl<T> MaybeLists<T> for [&[T]]
{
    type Height = ();
    type Width = ();
    const HEIGHT: usize = usize::MAX;
    const WIDTH: usize = usize::MAX;
    const IS_FLATTENED: bool = false;

    type RowsMapped<MM> = Vec<MM>;
    type RowView<'a> = &'a [T]
    where
        T: 'a,
        Self: 'a;
    type RowOwned = Vec<T>;
        
    fn index_view<'a>(&'a self, i: usize) -> Self::RowView<'a>
    where
        T: 'a,
        Self: 'a
    {
        self[i].as_view()
    }
    fn as_views_option<'a>(&'a self) -> Option<Vec<Self::RowView<'a>>>
    where
        T: 'a,
        Self: 'a
    {
        Some(Lists::<T>::as_views(self))
    }
    fn as_view_slices_option<'a>(&'a self) -> Option<Vec<&'a [T]>>
    where
        T: 'a,
        Self: 'a
    {
        Some(ListsOrSingle::<T>::as_view_slices(self))
    }
    fn map_rows_to_owned<'a, F>(&'a self, mut map: F) -> Self::RowsMapped<F::Output>
    where
        T: 'a,
        Self: 'a,
        F: FnMut<(Self::RowView<'a>,)>
    {
        self.iter()
            .map(|&r| map(r.as_view()))
            .collect()
    }
    fn map_rows_into_owned<F>(self, mut map: F) -> Self::RowsMapped<F::Output>
    where
        T: Clone,
        Self: Sized,
        F: FnMut<(Self::RowOwned,)>
    {
        self.iter()
            .map(|&r| map(r.to_vec()))
            .collect()
    }
    fn try_map_rows_to_owned<'a, F, O, E>(&'a self, mut map: F) -> Result<Self::RowsMapped<O>, E>
    where
        T: 'a,
        Self: 'a,
        F: FnMut(Self::RowView<'a>) -> Result<O, E>
    {
        self.iter()
            .map(|&r| map(r.as_view()))
            .collect()
    }
    fn try_map_rows_into_owned<F, O, E>(self, mut map: F) -> Result<Self::RowsMapped<O>, E>
    where
        T: Clone,
        Self: Sized,
        F: FnMut(Self::RowOwned) -> Result<O, E>
    {
        self.iter()
            .map(|&r| map(r.to_vec()))
            .collect()
    }
    fn into_owned_rows(self) -> Vec<Self::RowOwned>
    where
        T: Clone,
        Self: Sized
    {
        self.iter()
            .map(|&r| r.to_vec())
            .collect()
    }
    fn to_vecs_option(&self) -> Option<Vec<Vec<T>>>
    where
        T: Clone
    {
        Some(self.to_vecs())
    }
    fn into_vecs_option(self) -> Option<Vec<Vec<T>>>
    where
        T: Clone,
        Self: Sized
    {
        Some(self.into_vecs())
    }
}
impl<T, const M: usize> MaybeLists<T> for &[&[T]; M]
{
    type Height = usize;
    type Width = ();
    const HEIGHT: usize = M;
    const WIDTH: usize = usize::MAX;
    const IS_FLATTENED: bool = false;

    type RowsMapped<MM> = [MM; M];
    type RowView<'a> = &'a [T]
    where
        T: 'a,
        Self: 'a;
    type RowOwned = Vec<T>;
        
    fn index_view<'a>(&'a self, i: usize) -> Self::RowView<'a>
    where
        T: 'a,
        Self: 'a
    {
        self[i].as_view()
    }
    fn as_views_option<'a>(&'a self) -> Option<Vec<Self::RowView<'a>>>
    where
        T: 'a,
        Self: 'a
    {
        Some(Lists::<T>::as_views(self))
    }
    fn as_view_slices_option<'a>(&'a self) -> Option<Vec<&'a [T]>>
    where
        T: 'a,
        Self: 'a
    {
        Some(ListsOrSingle::<T>::as_view_slices(self))
    }
    fn map_rows_to_owned<'a, F>(&'a self, mut map: F) -> Self::RowsMapped<F::Output>
    where
        T: 'a,
        Self: 'a,
        F: FnMut<(Self::RowView<'a>,)>
    {
        self.each_ref()
            .map(|r| map(r.as_view()))
    }
    fn map_rows_into_owned<F>(self, mut map: F) -> Self::RowsMapped<F::Output>
    where
        T: Clone,
        Self: Sized,
        F: FnMut<(Self::RowOwned,)>
    {
        self.map(|r| map(r.to_vec()))
    }
    fn try_map_rows_to_owned<'a, F, O, E>(&'a self, mut map: F) -> Result<Self::RowsMapped<O>, E>
    where
        T: 'a,
        Self: 'a,
        F: FnMut(Self::RowView<'a>) -> Result<O, E>
    {
        self.each_ref()
            .try_map(|r| map(r.as_view()))
    }
    fn try_map_rows_into_owned<F, O, E>(self, mut map: F) -> Result<Self::RowsMapped<O>, E>
    where
        T: Clone,
        Self: Sized,
        F: FnMut(Self::RowOwned) -> Result<O, E>
    {
        self.try_map(|r| map(r.to_vec()))
    }
    fn into_owned_rows(self) -> Vec<Self::RowOwned>
    where
        T: Clone
    {
        self.iter()
            .map(|r| r.to_vec())
            .collect()
    }
    fn to_vecs_option(&self) -> Option<Vec<Vec<T>>>
    where
        T: Clone
    {
        Some(self.to_vecs())
    }
    fn into_vecs_option(self) -> Option<Vec<Vec<T>>>
    where
        T: Clone,
        Self: Sized
    {
        Some(self.into_vecs())
    }
}
impl<T> MaybeLists<T> for &[&[T]]
{
    type Height = ();
    type Width = ();
    const HEIGHT: usize = usize::MAX;
    const WIDTH: usize = usize::MAX;
    const IS_FLATTENED: bool = false;

    type RowsMapped<MM> = Vec<MM>;
    type RowView<'a> = &'a [T]
    where
        T: 'a,
        Self: 'a;
    type RowOwned = Vec<T>;
        
    fn index_view<'a>(&'a self, i: usize) -> Self::RowView<'a>
    where
        T: 'a,
        Self: 'a
    {
        self[i].as_view()
    }
    fn as_views_option<'a>(&'a self) -> Option<Vec<Self::RowView<'a>>>
    where
        T: 'a,
        Self: 'a
    {
        Some(Lists::<T>::as_views(self))
    }
    fn as_view_slices_option<'a>(&'a self) -> Option<Vec<&'a [T]>>
    where
        T: 'a,
        Self: 'a
    {
        Some(ListsOrSingle::<T>::as_view_slices(self))
    }
    fn map_rows_to_owned<'a, F>(&'a self, mut map: F) -> Self::RowsMapped<F::Output>
    where
        T: 'a,
        Self: 'a,
        F: FnMut<(Self::RowView<'a>,)>
    {
        self.iter()
            .map(|r| map(r.as_view()))
            .collect()
    }
    fn map_rows_into_owned<F>(self, mut map: F) -> Self::RowsMapped<F::Output>
    where
        T: Clone,
        Self: Sized,
        F: FnMut<(Self::RowOwned,)>
    {
        self.iter()
            .map(|&r| map(r.to_vec()))
            .collect()
    }
    fn try_map_rows_to_owned<'a, F, O, E>(&'a self, mut map: F) -> Result<Self::RowsMapped<O>, E>
    where
        T: 'a,
        Self: 'a,
        F: FnMut(Self::RowView<'a>) -> Result<O, E>
    {
        self.iter()
            .map(|r| map(r.as_view()))
            .collect()
    }
    fn try_map_rows_into_owned<F, O, E>(self, mut map: F) -> Result<Self::RowsMapped<O>, E>
    where
        T: Clone,
        Self: Sized,
        F: FnMut(Self::RowOwned) -> Result<O, E>
    {
        self.iter()
            .map(|&r| map(r.to_vec()))
            .collect()
    }
    fn into_owned_rows(self) -> Vec<Self::RowOwned>
    where
        T: Clone
    {
        self.iter()
            .map(|&r| r.to_vec())
            .collect()
    }
    fn to_vecs_option(&self) -> Option<Vec<Vec<T>>>
    where
        T: Clone
    {
        Some(self.to_vecs())
    }
    fn into_vecs_option(self) -> Option<Vec<Vec<T>>>
    where
        T: Clone,
        Self: Sized
    {
        Some(self.into_vecs())
    }
}

impl<T, const N: usize> MaybeLists<T> for Vec<&[T; N]>
{
    type Height = ();
    type Width = usize;
    const HEIGHT: usize = usize::MAX;
    const WIDTH: usize = N;
    const IS_FLATTENED: bool = false;

    type RowsMapped<MM> = Vec<MM>;
    type RowView<'a> = &'a [T; N]
    where
        T: 'a,
        Self: 'a;
    type RowOwned = [T; N];
        
    fn index_view<'a>(&'a self, i: usize) -> Self::RowView<'a>
    where
        T: 'a,
        Self: 'a
    {
        self[i].as_view()
    }
    fn as_views_option<'a>(&'a self) -> Option<Vec<Self::RowView<'a>>>
    where
        T: 'a,
        Self: 'a
    {
        Some(Lists::<T>::as_views(self))
    }
    fn as_view_slices_option<'a>(&'a self) -> Option<Vec<&'a [T]>>
    where
        T: 'a,
        Self: 'a
    {
        Some(ListsOrSingle::<T>::as_view_slices(self))
    }
    fn map_rows_to_owned<'a, F>(&'a self, mut map: F) -> Self::RowsMapped<F::Output>
    where
        T: 'a,
        Self: 'a,
        F: FnMut<(Self::RowView<'a>,)>
    {
        self.iter()
            .map(|r| map(r.as_view()))
            .collect()
    }
    fn map_rows_into_owned<F>(self, mut map: F) -> Self::RowsMapped<F::Output>
    where
        T: Clone,
        Self: Sized,
        F: FnMut<(Self::RowOwned,)>
    {
        self.into_iter()
            .map(|r| map(r.clone()))
            .collect()
    }
    fn try_map_rows_to_owned<'a, F, O, E>(&'a self, mut map: F) -> Result<Self::RowsMapped<O>, E>
    where
        T: 'a,
        Self: 'a,
        F: FnMut(Self::RowView<'a>) -> Result<O, E>
    {
        self.iter()
            .map(|r| map(r.as_view()))
            .collect()
    }
    fn try_map_rows_into_owned<F, O, E>(self, mut map: F) -> Result<Self::RowsMapped<O>, E>
    where
        T: Clone,
        Self: Sized,
        F: FnMut(Self::RowOwned) -> Result<O, E>
    {
        self.into_iter()
            .map(|r| map(r.clone()))
            .collect()
    }
    fn into_owned_rows(self) -> Vec<Self::RowOwned>
    where
        T: Clone
    {
        self.into_iter()
            .map(|r| r.clone())
            .collect()
    }
    fn to_vecs_option(&self) -> Option<Vec<Vec<T>>>
    where
        T: Clone
    {
        Some(self.to_vecs())
    }
    fn into_vecs_option(self) -> Option<Vec<Vec<T>>>
    where
        T: Clone,
        Self: Sized
    {
        Some(self.into_vecs())
    }
}
impl<T, const N: usize, const M: usize> MaybeLists<T> for [&[T; N]; M]
{
    type Height = usize;
    type Width = usize;
    const HEIGHT: usize = M;
    const WIDTH: usize = N;
    const IS_FLATTENED: bool = false;

    type RowsMapped<MM> = [MM; M];
    type RowView<'a> = &'a [T; N]
    where
        T: 'a,
        Self: 'a;
    type RowOwned = [T; N];
        
    fn index_view<'a>(&'a self, i: usize) -> Self::RowView<'a>
    where
        T: 'a,
        Self: 'a
    {
        self[i].as_view()
    }
    fn as_views_option<'a>(&'a self) -> Option<Vec<Self::RowView<'a>>>
    where
        T: 'a,
        Self: 'a
    {
        Some(Lists::<T>::as_views(self))
    }
    fn as_view_slices_option<'a>(&'a self) -> Option<Vec<&'a [T]>>
    where
        T: 'a,
        Self: 'a
    {
        Some(ListsOrSingle::<T>::as_view_slices(self))
    }
    fn map_rows_to_owned<'a, F>(&'a self, mut map: F) -> Self::RowsMapped<F::Output>
    where
        T: 'a,
        Self: 'a,
        F: FnMut<(Self::RowView<'a>,)>
    {
        self.each_ref()
            .map(|r| map(r.as_view()))
    }
    fn map_rows_into_owned<F>(self, mut map: F) -> Self::RowsMapped<F::Output>
    where
        T: Clone,
        Self: Sized,
        F: FnMut<(Self::RowOwned,)>
    {
        self.map(|r| map(r.clone()))
    }
    fn try_map_rows_to_owned<'a, F, O, E>(&'a self, mut map: F) -> Result<Self::RowsMapped<O>, E>
    where
        T: 'a,
        Self: 'a,
        F: FnMut(Self::RowView<'a>) -> Result<O, E>
    {
        self.each_ref()
            .try_map(|r| map(r.as_view()))
    }
    fn try_map_rows_into_owned<F, O, E>(self, mut map: F) -> Result<Self::RowsMapped<O>, E>
    where
        T: Clone,
        Self: Sized,
        F: FnMut(Self::RowOwned) -> Result<O, E>
    {
        self.try_map(|r| map(r.clone()))
    }
    fn into_owned_rows(self) -> Vec<Self::RowOwned>
    where
        T: Clone
    {
        self.into_iter()
            .map(|r| r.clone())
            .collect()
    }
    fn to_vecs_option(&self) -> Option<Vec<Vec<T>>>
    where
        T: Clone
    {
        Some(self.to_vecs())
    }
    fn into_vecs_option(self) -> Option<Vec<Vec<T>>>
    where
        T: Clone,
        Self: Sized
    {
        Some(self.into_vecs())
    }
}
impl<T, const N: usize> MaybeLists<T> for [&[T; N]]
{
    type Height = ();
    type Width = usize;
    const HEIGHT: usize = usize::MAX;
    const WIDTH: usize = N;
    const IS_FLATTENED: bool = false;

    type RowsMapped<MM> = Vec<MM>;
    type RowView<'a> = &'a [T; N]
    where
        T: 'a,
        Self: 'a;
    type RowOwned = [T; N];
        
    fn index_view<'a>(&'a self, i: usize) -> Self::RowView<'a>
    where
        T: 'a,
        Self: 'a
    {
        self[i].as_view()
    }
    fn as_views_option<'a>(&'a self) -> Option<Vec<Self::RowView<'a>>>
    where
        T: 'a,
        Self: 'a
    {
        Some(Lists::<T>::as_views(self))
    }
    fn as_view_slices_option<'a>(&'a self) -> Option<Vec<&'a [T]>>
    where
        T: 'a,
        Self: 'a
    {
        Some(ListsOrSingle::<T>::as_view_slices(self))
    }
    fn map_rows_to_owned<'a, F>(&'a self, mut map: F) -> Self::RowsMapped<F::Output>
    where
        T: 'a,
        Self: 'a,
        F: FnMut<(Self::RowView<'a>,)>
    {
        self.iter()
            .map(|r| map(r.as_view()))
            .collect()
    }
    fn map_rows_into_owned<F>(self, mut map: F) -> Self::RowsMapped<F::Output>
    where
        T: Clone,
        Self: Sized,
        F: FnMut<(Self::RowOwned,)>
    {
        self.iter()
            .map(|&r| map(r.clone()))
            .collect()
    }
    fn try_map_rows_to_owned<'a, F, O, E>(&'a self, mut map: F) -> Result<Self::RowsMapped<O>, E>
    where
        T: 'a,
        Self: 'a,
        F: FnMut(Self::RowView<'a>) -> Result<O, E>
    {
        self.iter()
            .map(|r| map(r.as_view()))
            .collect()
    }
    fn try_map_rows_into_owned<F, O, E>(self, mut map: F) -> Result<Self::RowsMapped<O>, E>
    where
        T: Clone,
        Self: Sized,
        F: FnMut(Self::RowOwned) -> Result<O, E>
    {
        self.iter()
            .map(|&r| map(r.clone()))
            .collect()
    }
    fn into_owned_rows(self) -> Vec<Self::RowOwned>
    where
        T: Clone,
        Self: Sized
    {
        self.iter()
            .map(|&r| r.clone())
            .collect()
    }
    fn to_vecs_option(&self) -> Option<Vec<Vec<T>>>
    where
        T: Clone
    {
        Some(self.to_vecs())
    }
    fn into_vecs_option(self) -> Option<Vec<Vec<T>>>
    where
        T: Clone,
        Self: Sized
    {
        Some(self.into_vecs())
    }
}
impl<T, const N: usize, const M: usize> MaybeLists<T> for &[&[T; N]; M]
{
    type Height = usize;
    type Width = usize;
    const HEIGHT: usize = M;
    const WIDTH: usize = N;
    const IS_FLATTENED: bool = false;

    type RowsMapped<MM> = [MM; M];
    type RowView<'a> = &'a [T; N]
    where
        T: 'a,
        Self: 'a;
    type RowOwned = [T; N];
        
    fn index_view<'a>(&'a self, i: usize) -> Self::RowView<'a>
    where
        T: 'a,
        Self: 'a
    {
        self[i].as_view()
    }
    fn as_views_option<'a>(&'a self) -> Option<Vec<Self::RowView<'a>>>
    where
        T: 'a,
        Self: 'a
    {
        Some(Lists::<T>::as_views(self))
    }
    fn as_view_slices_option<'a>(&'a self) -> Option<Vec<&'a [T]>>
    where
        T: 'a,
        Self: 'a
    {
        Some(ListsOrSingle::<T>::as_view_slices(self))
    }
    fn map_rows_to_owned<'a, F>(&'a self, mut map: F) -> Self::RowsMapped<F::Output>
    where
        T: 'a,
        Self: 'a,
        F: FnMut<(Self::RowView<'a>,)>
    {
        self.each_ref()
            .map(|r| map(r.as_view()))
    }
    fn map_rows_into_owned<F>(self, mut map: F) -> Self::RowsMapped<F::Output>
    where
        T: Clone,
        Self: Sized,
        F: FnMut<(Self::RowOwned,)>
    {
        self.map(|r| map(r.clone()))
    }
    fn try_map_rows_to_owned<'a, F, O, E>(&'a self, mut map: F) -> Result<Self::RowsMapped<O>, E>
    where
        T: 'a,
        Self: 'a,
        F: FnMut(Self::RowView<'a>) -> Result<O, E>
    {
        self.each_ref()
            .try_map(|r| map(r.as_view()))
    }
    fn try_map_rows_into_owned<F, O, E>(self, mut map: F) -> Result<Self::RowsMapped<O>, E>
    where
        T: Clone,
        Self: Sized,
        F: FnMut(Self::RowOwned) -> Result<O, E>
    {
        self.try_map(|r| map(r.clone()))
    }
    fn into_owned_rows(self) -> Vec<Self::RowOwned>
    where
        T: Clone
    {
        (*self).into_iter()
            .map(|r| r.clone())
            .collect()
    }
    fn to_vecs_option(&self) -> Option<Vec<Vec<T>>>
    where
        T: Clone
    {
        Some(self.to_vecs())
    }
    fn into_vecs_option(self) -> Option<Vec<Vec<T>>>
    where
        T: Clone,
        Self: Sized
    {
        Some(self.into_vecs())
    }
}
impl<T, const N: usize> MaybeLists<T> for &[&[T; N]]
{
    type Height = ();
    type Width = usize;
    const HEIGHT: usize = usize::MAX;
    const WIDTH: usize = N;
    const IS_FLATTENED: bool = false;

    type RowsMapped<MM> = Vec<MM>;
    type RowView<'a> = &'a [T; N]
    where
        T: 'a,
        Self: 'a;
    type RowOwned = [T; N];

    fn index_view<'a>(&'a self, i: usize) -> Self::RowView<'a>
    where
        T: 'a,
        Self: 'a
    {
        self[i].as_view()
    }
    fn as_views_option<'a>(&'a self) -> Option<Vec<Self::RowView<'a>>>
    where
        T: 'a,
        Self: 'a
    {
        Some(Lists::<T>::as_views(self))
    }
    fn as_view_slices_option<'a>(&'a self) -> Option<Vec<&'a [T]>>
    where
        T: 'a,
        Self: 'a
    {
        Some(ListsOrSingle::<T>::as_view_slices(self))
    }
    fn map_rows_to_owned<'a, F>(&'a self, mut map: F) -> Self::RowsMapped<F::Output>
    where
        T: 'a,
        Self: 'a,
        F: FnMut<(Self::RowView<'a>,)>
    {
        self.iter()
            .map(|r| map(r.as_view()))
            .collect()
    }
    fn map_rows_into_owned<F>(self, mut map: F) -> Self::RowsMapped<F::Output>
    where
        T: Clone,
        Self: Sized,
        F: FnMut<(Self::RowOwned,)>
    {
        self.iter()
            .map(|&r| map(r.clone()))
            .collect()
    }
    fn try_map_rows_to_owned<'a, F, O, E>(&'a self, mut map: F) -> Result<Self::RowsMapped<O>, E>
    where
        T: 'a,
        Self: 'a,
        F: FnMut(Self::RowView<'a>) -> Result<O, E>
    {
        self.iter()
            .map(|r| map(r.as_view()))
            .collect()
    }
    fn try_map_rows_into_owned<F, O, E>(self, mut map: F) -> Result<Self::RowsMapped<O>, E>
    where
        T: Clone,
        Self: Sized,
        F: FnMut(Self::RowOwned) -> Result<O, E>
    {
        self.iter()
            .map(|&r| map(r.clone()))
            .collect()
    }
    fn into_owned_rows(self) -> Vec<Self::RowOwned>
    where
        T: Clone
    {
        self.iter()
            .map(|&r| r.clone())
            .collect()
    }
    fn to_vecs_option(&self) -> Option<Vec<Vec<T>>>
    where
        T: Clone
    {
        Some(self.to_vecs())
    }
    fn into_vecs_option(self) -> Option<Vec<Vec<T>>>
    where
        T: Clone,
        Self: Sized
    {
        Some(self.into_vecs())
    }
}

impl<T> MaybeLists<T> for Array1<T>
{
    type Height = usize;
    type Width = ();
    const HEIGHT: usize = 1;
    const WIDTH: usize = usize::MAX;
    const IS_FLATTENED: bool = true;

    type RowsMapped<M> = M;
    type RowView<'a> = ArrayView1<'a, T>
    where
        T: 'a,
        Self: 'a;
    type RowOwned = Array1<T>;

    fn index_view<'a>(&'a self, i: usize) -> Self::RowView<'a>
    where
        T: 'a,
        Self: 'a
    {
        (&core::array::from_ref(self)[i]).into()
    }
    fn as_views_option<'a>(&'a self) -> Option<Vec<Self::RowView<'a>>>
    where
        T: 'a,
        Self: 'a
    {
        Some(Lists::<T>::as_views(self))
    }
    fn as_view_slices_option<'a>(&'a self) -> Option<Vec<&'a [T]>>
    where
        T: 'a,
        Self: 'a
    {
        Some(ListsOrSingle::<T>::as_view_slices(self))
    }
    fn map_rows_to_owned<'a, F>(&'a self, mut map: F) -> Self::RowsMapped<F::Output>
    where
        T: 'a,
        Self: 'a,
        F: FnMut<(Self::RowView<'a>,)>
    {
        map(self.as_view())
    }
    fn map_rows_into_owned<F>(self, mut map: F) -> Self::RowsMapped<F::Output>
    where
        T: Clone,
        Self: Sized,
        F: FnMut<(Self::RowOwned,)>
    {
        map(self)
    }
    fn try_map_rows_to_owned<'a, F, O, E>(&'a self, mut map: F) -> Result<Self::RowsMapped<O>, E>
    where
        T: 'a,
        Self: 'a,
        F: FnMut(Self::RowView<'a>) -> Result<O, E>
    {
        map(self.as_view())
    }
    fn try_map_rows_into_owned<F, O, E>(self, mut map: F) -> Result<Self::RowsMapped<O>, E>
    where
        T: Clone,
        Self: Sized,
        F: FnMut(Self::RowOwned) -> Result<O, E>
    {
        map(self)
    }
    fn into_owned_rows(self) -> Vec<Self::RowOwned>
    where
        T: Clone
    {
        vec![self]
    }
    fn to_vecs_option(&self) -> Option<Vec<Vec<T>>>
    where
        T: Clone
    {
        Some(self.to_vecs())
    }
    fn into_vecs_option(self) -> Option<Vec<Vec<T>>>
    where
        T: Clone,
        Self: Sized
    {
        Some(self.into_vecs())
    }
}
impl<'c, T> MaybeLists<T> for ArrayView1<'c, T>
{
    type Height = usize;
    type Width = ();
    const HEIGHT: usize = 1;
    const WIDTH: usize = usize::MAX;
    const IS_FLATTENED: bool = true;

    type RowsMapped<M> = M;
    type RowView<'a> = ArrayView1<'a, T>
    where
        T: 'a,
        Self: 'a;
    type RowOwned = Array1<T>;

    fn index_view<'a>(&'a self, i: usize) -> Self::RowView<'a>
    where
        T: 'a,
        Self: 'a
    {
        core::array::from_ref(self)[i].reborrow()
    }
    fn as_views_option<'a>(&'a self) -> Option<Vec<Self::RowView<'a>>>
    where
        T: 'a,
        Self: 'a
    {
        Some(Lists::<T>::as_views(self))
    }
    fn as_view_slices_option<'a>(&'a self) -> Option<Vec<&'a [T]>>
    where
        T: 'a,
        Self: 'a
    {
        Some(ListsOrSingle::<T>::as_view_slices(self))
    }
    fn map_rows_to_owned<'a, F>(&'a self, mut map: F) -> Self::RowsMapped<F::Output>
    where
        T: 'a,
        Self: 'a,
        F: FnMut<(Self::RowView<'a>,)>
    {
        map(self.reborrow())
    }
    fn map_rows_into_owned<F>(self, mut map: F) -> Self::RowsMapped<F::Output>
    where
        T: Clone,
        Self: Sized,
        F: FnMut<(Self::RowOwned,)>
    {
        map(self.to_owned())
    }
    fn try_map_rows_to_owned<'a, F, O, E>(&'a self, mut map: F) -> Result<Self::RowsMapped<O>, E>
    where
        T: 'a,
        Self: 'a,
        F: FnMut(Self::RowView<'a>) -> Result<O, E>
    {
        map(self.reborrow())
    }
    fn try_map_rows_into_owned<F, O, E>(self, mut map: F) -> Result<Self::RowsMapped<O>, E>
    where
        T: Clone,
        Self: Sized,
        F: FnMut(Self::RowOwned) -> Result<O, E>
    {
        map(self.to_owned())
    }
    fn into_owned_rows(self) -> Vec<Self::RowOwned>
    where
        T: Clone
    {
        vec![self.to_owned()]
    }
    fn to_vecs_option(&self) -> Option<Vec<Vec<T>>>
    where
        T: Clone
    {
        Some(self.to_vecs())
    }
    fn into_vecs_option(self) -> Option<Vec<Vec<T>>>
    where
        T: Clone,
        Self: Sized
    {
        Some(self.into_vecs())
    }
}

impl<T> MaybeLists<T> for Array2<T>
{
    type Height = ();
    type Width = ();
    const HEIGHT: usize = usize::MAX;
    const WIDTH: usize = usize::MAX;
    const IS_FLATTENED: bool = false;

    type RowsMapped<M> = Vec<M>;
    type RowView<'a> = ArrayView1<'a, T>
    where
        T: 'a,
        Self: 'a;
    type RowOwned = Array1<T>;

    fn index_view<'a>(&'a self, i: usize) -> Self::RowView<'a>
    where
        T: 'a,
        Self: 'a
    {
        self.row(i)
    }
    fn as_views_option<'a>(&'a self) -> Option<Vec<Self::RowView<'a>>>
    where
        T: 'a,
        Self: 'a
    {
        Some(Lists::<T>::as_views(self))
    }
    fn as_view_slices_option<'a>(&'a self) -> Option<Vec<&'a [T]>>
    where
        T: 'a,
        Self: 'a
    {
        Some(ListsOrSingle::<T>::as_view_slices(self))
    }
    fn map_rows_to_owned<'a, F>(&'a self, mut map: F) -> Self::RowsMapped<F::Output>
    where
        T: 'a,
        Self: 'a,
        F: FnMut<(Self::RowView<'a>,)>
    {
        self.rows()
            .into_iter()
            .map(|r| map(r.as_view()))
            .collect()
    }
    fn map_rows_into_owned<F>(self, mut map: F) -> Self::RowsMapped<F::Output>
    where
        T: Clone,
        Self: Sized,
        F: FnMut<(Self::RowOwned,)>
    {
        self.rows()
            .into_iter()
            .map(|r| map(r.to_owned()))
            .collect()
    }
    fn try_map_rows_to_owned<'a, F, O, E>(&'a self, mut map: F) -> Result<Self::RowsMapped<O>, E>
    where
        T: 'a,
        Self: 'a,
        F: FnMut(Self::RowView<'a>) -> Result<O, E>
    {
        self.rows()
            .into_iter()
            .map(|r| map(r.as_view()))
            .collect()
    }
    fn try_map_rows_into_owned<F, O, E>(self, mut map: F) -> Result<Self::RowsMapped<O>, E>
    where
        T: Clone,
        Self: Sized,
        F: FnMut(Self::RowOwned) -> Result<O, E>
    {
        self.rows()
            .into_iter()
            .map(|r| map(r.to_owned()))
            .collect()
    }
    fn into_owned_rows(self) -> Vec<Self::RowOwned>
    where
        T: Clone
    {
        self.rows()
            .into_iter()
            .map(|r| r.to_owned())
            .collect()
    }
    fn to_vecs_option(&self) -> Option<Vec<Vec<T>>>
    where
        T: Clone
    {
        Some(self.to_vecs())
    }
    fn into_vecs_option(self) -> Option<Vec<Vec<T>>>
    where
        T: Clone,
        Self: Sized
    {
        Some(self.into_vecs())
    }
}
impl<'b, T> MaybeLists<T> for ArrayView2<'b, T>
{
    type Height = ();
    type Width = ();
    const HEIGHT: usize = usize::MAX;
    const WIDTH: usize = usize::MAX;
    const IS_FLATTENED: bool = false;
    
    type RowsMapped<M> = Vec<M>;
    type RowView<'a> = ArrayView1<'a, T>
    where
        T: 'a,
        Self: 'a;
    type RowOwned = Array1<T>;

    fn index_view<'a>(&'a self, i: usize) -> Self::RowView<'a>
        where
            T: 'a,
            Self: 'a
    {
        self.row(i)
    }
    fn as_views_option<'a>(&'a self) -> Option<Vec<Self::RowView<'a>>>
    where
        T: 'a,
        Self: 'a
    {
        Some(Lists::<T>::as_views(self))
    }
    fn as_view_slices_option<'a>(&'a self) -> Option<Vec<&'a [T]>>
    where
        T: 'a,
        Self: 'a
    {
        Some(ListsOrSingle::<T>::as_view_slices(self))
    }
    fn map_rows_to_owned<'a, F>(&'a self, mut map: F) -> Self::RowsMapped<F::Output>
    where
        T: 'a,
        Self: 'a,
        F: FnMut<(Self::RowView<'a>,)>
    {
        self.rows()
            .into_iter()
            .map(|r| map(r.as_view()))
            .collect()
    }
    fn map_rows_into_owned<F>(self, mut map: F) -> Self::RowsMapped<F::Output>
    where
        T: Clone,
        Self: Sized,
        F: FnMut<(Self::RowOwned,)>
    {
        self.rows()
            .into_iter()
            .map(|r| map(r.to_owned()))
            .collect()
    }
    fn try_map_rows_to_owned<'a, F, O, E>(&'a self, mut map: F) -> Result<Self::RowsMapped<O>, E>
    where
        T: 'a,
        Self: 'a,
        F: FnMut(Self::RowView<'a>) -> Result<O, E>
    {
        self.rows()
            .into_iter()
            .map(|r| map(r.as_view()))
            .collect()
    }
    fn try_map_rows_into_owned<F, O, E>(self, mut map: F) -> Result<Self::RowsMapped<O>, E>
    where
        T: Clone,
        Self: Sized,
        F: FnMut(Self::RowOwned) -> Result<O, E>
    {
        self.rows()
            .into_iter()
            .map(|r| map(r.to_owned()))
            .collect()
    }
    fn into_owned_rows(self) -> Vec<Self::RowOwned>
    where
        T: Clone
    {
        self.rows()
            .into_iter()
            .map(|r| r.to_owned())
            .collect()
    }
    fn to_vecs_option(&self) -> Option<Vec<Vec<T>>>
    where
        T: Clone
    {
        Some(self.to_vecs())
    }
    fn into_vecs_option(self) -> Option<Vec<Vec<T>>>
    where
        T: Clone,
        Self: Sized
    {
        Some(self.into_vecs())
    }
}