

use ndarray::{prelude::Array1, Array2, ArrayView1, ArrayView2};


use crate::{ListOrSingle, Lists, MaybeContainer, MaybeList};

pub trait MaybeLists<T>: MaybeContainer<T>
{
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
    fn into_owned_rows(self) -> Vec<Self::RowOwned>
    where
        T: Clone,
        Self: Sized;
}

impl<T> MaybeLists<T> for ()
{
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
    fn into_owned_rows(self) -> Vec<Self::RowOwned>
    where
        T: Clone
    {
        vec![()]
    }
}

impl<T> MaybeLists<T> for Vec<T>
{
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
        Some(Lists::<T>::as_view_slices(self))
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
    fn into_owned_rows(self) -> Vec<Self::RowOwned>
    where
        T: Clone
    {
        vec![self]
    }
}
impl<T> MaybeLists<T> for [T]
{
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
        Some(Lists::<T>::as_view_slices(self))
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
    fn into_owned_rows(self) -> Vec<Self::RowOwned>
    where
        T: Clone,
        Self: Sized
    {
        vec![self.to_vec()]
    }
}
impl<T, const N: usize> MaybeLists<T> for [T; N]
{
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
        Some(Lists::<T>::as_view_slices(self))
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
    fn into_owned_rows(self) -> Vec<Self::RowOwned>
    where
        T: Clone
    {
        vec![self]
    }
}
impl<T> MaybeLists<T> for &[T]
{
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
        Some(Lists::<T>::as_view_slices(self))
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
    fn into_owned_rows(self) -> Vec<Self::RowOwned>
    where
        T: Clone
    {
        vec![self.to_vec()]
    }
}
impl<T, const N: usize> MaybeLists<T> for &[T; N]
{
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
        Some(Lists::<T>::as_view_slices(self))
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
    fn into_owned_rows(self) -> Vec<Self::RowOwned>
    where
        T: Clone
    {
        vec![self.clone()]
    }
}

impl<T> MaybeLists<T> for Vec<Vec<T>>
{
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
        Some(Lists::<T>::as_view_slices(self))
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
    fn into_owned_rows(self) -> Vec<Self::RowOwned>
    where
        T: Clone
    {
        self
    }
}
impl<T, const M: usize> MaybeLists<T> for [Vec<T>; M]
{
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
        Some(Lists::<T>::as_view_slices(self))
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
    fn into_owned_rows(self) -> Vec<Self::RowOwned>
    where
        T: Clone
    {
        self.into_iter()
            .collect()
    }
}
impl<T> MaybeLists<T> for [Vec<T>]
{
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
        Some(Lists::<T>::as_view_slices(self))
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
    fn into_owned_rows(self) -> Vec<Self::RowOwned>
    where
        T: Clone,
        Self: Sized
    {
        self.iter()
            .map(|r| r.clone())
            .collect()
    }
}
impl<T, const M: usize> MaybeLists<T> for &[Vec<T>; M]
{
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
        Some(Lists::<T>::as_view_slices(self))
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
    fn into_owned_rows(self) -> Vec<Self::RowOwned>
    where
        T: Clone
    {
        self.iter()
            .map(|r| r.clone())
            .collect()
    }
}
impl<T> MaybeLists<T> for &[Vec<T>]
{
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
        Some(Lists::<T>::as_view_slices(self))
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
    fn into_owned_rows(self) -> Vec<Self::RowOwned>
    where
        T: Clone
    {
        self.iter()
            .map(|r| r.clone())
            .collect()
    }
}

impl<T, const N: usize> MaybeLists<T> for Vec<[T; N]>
{
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
        Some(Lists::<T>::as_view_slices(self))
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
    fn into_owned_rows(self) -> Vec<Self::RowOwned>
    where
        T: Clone
    {
        self
    }
}
impl<T, const N: usize, const M: usize> MaybeLists<T> for [[T; N]; M]
{
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
        Some(Lists::<T>::as_view_slices(self))
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
    fn into_owned_rows(self) -> Vec<Self::RowOwned>
    where
        T: Clone
    {
        self.into_iter()
            .collect()
    }
}
impl<T, const N: usize> MaybeLists<T> for [[T; N]]
{
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
        Some(Lists::<T>::as_view_slices(self))
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
    fn into_owned_rows(self) -> Vec<Self::RowOwned>
    where
        T: Clone,
        Self: Sized
    {
        self.iter()
            .map(|r| r.clone())
            .collect()
    }
}
impl<T, const N: usize, const M: usize> MaybeLists<T> for &[[T; N]; M]
{
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
        Some(Lists::<T>::as_view_slices(self))
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
    fn into_owned_rows(self) -> Vec<Self::RowOwned>
    where
        T: Clone
    {
        self.iter()
            .map(|r| r.clone())
            .collect()
    }
}
impl<T, const N: usize> MaybeLists<T> for &[[T; N]]
{
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
        Some(Lists::<T>::as_view_slices(self))
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
    fn into_owned_rows(self) -> Vec<Self::RowOwned>
    where
        T: Clone
    {
        self.iter()
            .map(|r| r.clone())
            .collect()
    }
}

impl<T> MaybeLists<T> for Vec<&[T]>
{
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
        Some(Lists::<T>::as_view_slices(self))
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
    fn into_owned_rows(self) -> Vec<Self::RowOwned>
    where
        T: Clone
    {
        self.into_iter()
            .map(|r| r.to_vec())
            .collect()
    }
}
impl<T, const M: usize> MaybeLists<T> for [&[T]; M]
{
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
        Some(Lists::<T>::as_view_slices(self))
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
    fn into_owned_rows(self) -> Vec<Self::RowOwned>
    where
        T: Clone
    {
        self.into_iter()
            .map(|r| r.to_vec())
            .collect()
    }
}
impl<T> MaybeLists<T> for [&[T]]
{
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
        Some(Lists::<T>::as_view_slices(self))
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
    fn into_owned_rows(self) -> Vec<Self::RowOwned>
    where
        T: Clone,
        Self: Sized
    {
        self.iter()
            .map(|&r| r.to_vec())
            .collect()
    }
}
impl<T, const M: usize> MaybeLists<T> for &[&[T]; M]
{
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
        Some(Lists::<T>::as_view_slices(self))
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
    fn into_owned_rows(self) -> Vec<Self::RowOwned>
    where
        T: Clone
    {
        self.iter()
            .map(|r| r.to_vec())
            .collect()
    }
}
impl<T> MaybeLists<T> for &[&[T]]
{
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
        Some(Lists::<T>::as_view_slices(self))
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
    fn into_owned_rows(self) -> Vec<Self::RowOwned>
    where
        T: Clone
    {
        self.iter()
            .map(|&r| r.to_vec())
            .collect()
    }
}

impl<T, const N: usize> MaybeLists<T> for Vec<&[T; N]>
{
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
        Some(Lists::<T>::as_view_slices(self))
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
    fn into_owned_rows(self) -> Vec<Self::RowOwned>
    where
        T: Clone
    {
        self.into_iter()
            .map(|r| r.clone())
            .collect()
    }
}
impl<T, const N: usize, const M: usize> MaybeLists<T> for [&[T; N]; M]
{
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
        Some(Lists::<T>::as_view_slices(self))
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
    fn into_owned_rows(self) -> Vec<Self::RowOwned>
    where
        T: Clone
    {
        self.into_iter()
            .map(|r| r.clone())
            .collect()
    }
}
impl<T, const N: usize> MaybeLists<T> for [&[T; N]]
{
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
        Some(Lists::<T>::as_view_slices(self))
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
    fn into_owned_rows(self) -> Vec<Self::RowOwned>
    where
        T: Clone,
        Self: Sized
    {
        self.iter()
            .map(|&r| r.clone())
            .collect()
    }
}
impl<T, const N: usize, const M: usize> MaybeLists<T> for &[&[T; N]; M]
{
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
        Some(Lists::<T>::as_view_slices(self))
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
    fn into_owned_rows(self) -> Vec<Self::RowOwned>
    where
        T: Clone
    {
        (*self).into_iter()
            .map(|r| r.clone())
            .collect()
    }
}
impl<T, const N: usize> MaybeLists<T> for &[&[T; N]]
{
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
        Some(Lists::<T>::as_view_slices(self))
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
    fn into_owned_rows(self) -> Vec<Self::RowOwned>
    where
        T: Clone
    {
        self.iter()
            .map(|&r| r.clone())
            .collect()
    }
}

impl<T> MaybeLists<T> for Array1<T>
{
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
        Some(Lists::<T>::as_view_slices(self))
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
    fn into_owned_rows(self) -> Vec<Self::RowOwned>
    where
        T: Clone
    {
        vec![self]
    }
}
impl<'c, T> MaybeLists<T> for ArrayView1<'c, T>
{
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
        Some(Lists::<T>::as_view_slices(self))
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
    fn into_owned_rows(self) -> Vec<Self::RowOwned>
    where
        T: Clone
    {
        vec![self.to_owned()]
    }
}

impl<T> MaybeLists<T> for Array2<T>
{
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
        Some(Lists::<T>::as_view_slices(self))
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
    fn into_owned_rows(self) -> Vec<Self::RowOwned>
    where
        T: Clone
    {
        self.rows()
            .into_iter()
            .map(|r| r.to_owned())
            .collect()
    }
}
impl<'b, T> MaybeLists<T> for ArrayView2<'b, T>
{
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
        Some(Lists::<T>::as_view_slices(self))
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
    fn into_owned_rows(self) -> Vec<Self::RowOwned>
    where
        T: Clone
    {
        self.rows()
            .into_iter()
            .map(|r| r.to_owned())
            .collect()
    }
}