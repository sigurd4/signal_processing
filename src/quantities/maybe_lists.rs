

use ndarray::{prelude::Array1, Array2, ArrayView1, ArrayView2};


use crate::{ListOrSingle, Lists, MaybeContainer, MaybeList};

pub trait MaybeLists<T>: MaybeContainer<T>
{
    type RowsMapped<M>: ListOrSingle<M>;
    type IndexView<'a>: MaybeList<T> + 'a
    where
        T: 'a,
        Self: 'a;

    fn index_view<'a>(&'a self, i: usize) -> Self::IndexView<'a>
    where
        T: 'a,
        Self: 'a;
    fn as_views_option<'a>(&'a self) -> Option<Vec<Self::IndexView<'a>>>
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
        F: FnMut<(Self::IndexView<'a>,)>;
}

impl<T> MaybeLists<T> for ()
{
    type RowsMapped<M> = M;
    type IndexView<'a> = ()
    where
        T: 'a,
        Self: 'a;

    fn index_view<'a>(&'a self, _i: usize) -> Self::IndexView<'a>
    where
        T: 'a,
        Self: 'a
    {
        
    }
    fn as_views_option<'a>(&'a self) -> Option<Vec<Self::IndexView<'a>>>
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
        F: FnMut<(Self::IndexView<'a>,)>
    {
        map(())
    }
}

impl<T> MaybeLists<T> for Vec<T>
{
    type RowsMapped<M> = M;
    type IndexView<'a> = &'a [T]
    where
        T: 'a,
        Self: 'a;
        
    fn index_view<'a>(&'a self, i: usize) -> Self::IndexView<'a>
    where
        T: 'a,
        Self: 'a
    {
        core::slice::from_ref(&self.as_slice())[i]
    }
    fn as_views_option<'a>(&'a self) -> Option<Vec<Self::IndexView<'a>>>
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
        F: FnMut<(Self::IndexView<'a>,)>
    {
        map(self.as_view())
    }
}
impl<T> MaybeLists<T> for [T]
{
    type RowsMapped<M> = M;
    type IndexView<'a> = &'a [T]
    where
        T: 'a,
        Self: 'a;
        
    fn index_view<'a>(&'a self, i: usize) -> Self::IndexView<'a>
    where
        T: 'a,
        Self: 'a
    {
        core::slice::from_ref(&self)[i]
    }
    fn as_views_option<'a>(&'a self) -> Option<Vec<Self::IndexView<'a>>>
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
        F: FnMut<(Self::IndexView<'a>,)>
    {
        map(self.as_view())
    }
}
impl<T, const N: usize> MaybeLists<T> for [T; N]
{
    type RowsMapped<M> = M;
    type IndexView<'a> = &'a [T; N]
    where
        T: 'a,
        Self: 'a;
        
    fn index_view<'a>(&'a self, i: usize) -> Self::IndexView<'a>
    where
        T: 'a,
        Self: 'a
    {
        core::slice::from_ref(&self)[i]
    }
    fn as_views_option<'a>(&'a self) -> Option<Vec<Self::IndexView<'a>>>
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
        F: FnMut<(Self::IndexView<'a>,)>
    {
        map(self.as_view())
    }
}
impl<T> MaybeLists<T> for &[T]
{
    type RowsMapped<M> = M;
    type IndexView<'a> = &'a [T]
    where
        T: 'a,
        Self: 'a;
        
    fn index_view<'a>(&'a self, i: usize) -> Self::IndexView<'a>
    where
        T: 'a,
        Self: 'a
    {
        core::slice::from_ref(self)[i]
    }
    fn as_views_option<'a>(&'a self) -> Option<Vec<Self::IndexView<'a>>>
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
        F: FnMut<(Self::IndexView<'a>,)>
    {
        map(self.as_view())
    }
}
impl<T, const N: usize> MaybeLists<T> for &[T; N]
{
    type RowsMapped<M> = M;
    type IndexView<'a> = &'a [T; N]
    where
        T: 'a,
        Self: 'a;
        
    fn index_view<'a>(&'a self, i: usize) -> Self::IndexView<'a>
    where
        T: 'a,
        Self: 'a
    {
        core::slice::from_ref(self)[i]
    }
    fn as_views_option<'a>(&'a self) -> Option<Vec<Self::IndexView<'a>>>
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
        F: FnMut<(Self::IndexView<'a>,)>
    {
        map(self.as_view())
    }
}

impl<T> MaybeLists<T> for Vec<Vec<T>>
{
    type RowsMapped<MM> = Vec<MM>;
    type IndexView<'a> = &'a [T]
    where
        T: 'a,
        Self: 'a;

    fn index_view<'a>(&'a self, i: usize) -> Self::IndexView<'a>
    where
        T: 'a,
        Self: 'a
    {
        self[i].as_view()
    }
    fn as_views_option<'a>(&'a self) -> Option<Vec<Self::IndexView<'a>>>
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
        F: FnMut<(Self::IndexView<'a>,)>
    {
        self.iter()
            .map(|r| map(r.as_view()))
            .collect()
    }
}
impl<T, const M: usize> MaybeLists<T> for [Vec<T>; M]
{
    type RowsMapped<MM> = [MM; M];
    type IndexView<'a> = &'a [T]
    where
        T: 'a,
        Self: 'a;
        
    fn index_view<'a>(&'a self, i: usize) -> Self::IndexView<'a>
    where
        T: 'a,
        Self: 'a
    {
        self[i].as_view()
    }
    fn as_views_option<'a>(&'a self) -> Option<Vec<Self::IndexView<'a>>>
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
        F: FnMut<(Self::IndexView<'a>,)>
    {
        self.each_ref()
            .map(|r| map(r.as_view()))
    }
}
impl<T> MaybeLists<T> for [Vec<T>]
{
    type RowsMapped<MM> = Vec<MM>;
    type IndexView<'a> = &'a [T]
    where
        T: 'a,
        Self: 'a;

    fn index_view<'a>(&'a self, i: usize) -> Self::IndexView<'a>
    where
        T: 'a,
        Self: 'a
    {
        self[i].as_view()
    }
    fn as_views_option<'a>(&'a self) -> Option<Vec<Self::IndexView<'a>>>
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
        F: FnMut<(Self::IndexView<'a>,)>
    {
        self.iter()
            .map(|r| map(r.as_view()))
            .collect()
    }
}
impl<T, const M: usize> MaybeLists<T> for &[Vec<T>; M]
{
    type RowsMapped<MM> = [MM; M];
    type IndexView<'a> = &'a [T]
    where
        T: 'a,
        Self: 'a;
        
    fn index_view<'a>(&'a self, i: usize) -> Self::IndexView<'a>
    where
        T: 'a,
        Self: 'a
    {
        self[i].as_view()
    }
    fn as_views_option<'a>(&'a self) -> Option<Vec<Self::IndexView<'a>>>
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
        F: FnMut<(Self::IndexView<'a>,)>
    {
        self.each_ref()
            .map(|r| map(r.as_view()))
    }
}
impl<T> MaybeLists<T> for &[Vec<T>]
{
    type RowsMapped<MM> = Vec<MM>;
    type IndexView<'a> = &'a [T]
    where
        T: 'a,
        Self: 'a;
        
    fn index_view<'a>(&'a self, i: usize) -> Self::IndexView<'a>
    where
        T: 'a,
        Self: 'a
    {
        self[i].as_view()
    }
    fn as_views_option<'a>(&'a self) -> Option<Vec<Self::IndexView<'a>>>
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
        F: FnMut<(Self::IndexView<'a>,)>
    {
        self.iter()
            .map(|r| map(r.as_view()))
            .collect()
    }
}

impl<T, const N: usize> MaybeLists<T> for Vec<[T; N]>
{
    type RowsMapped<MM> = Vec<MM>;
    type IndexView<'a> = &'a [T; N]
    where
        T: 'a,
        Self: 'a;
        
    fn index_view<'a>(&'a self, i: usize) -> Self::IndexView<'a>
    where
        T: 'a,
        Self: 'a
    {
        self[i].as_view()
    }
    fn as_views_option<'a>(&'a self) -> Option<Vec<Self::IndexView<'a>>>
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
        F: FnMut<(Self::IndexView<'a>,)>
    {
        self.iter()
            .map(|r| map(r.as_view()))
            .collect()
    }
}
impl<T, const N: usize, const M: usize> MaybeLists<T> for [[T; N]; M]
{
    type RowsMapped<MM> = [MM; M];
    type IndexView<'a> = &'a [T; N]
    where
        T: 'a,
        Self: 'a;
        
    fn index_view<'a>(&'a self, i: usize) -> Self::IndexView<'a>
    where
        T: 'a,
        Self: 'a
    {
        self[i].as_view()
    }
    fn as_views_option<'a>(&'a self) -> Option<Vec<Self::IndexView<'a>>>
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
        F: FnMut<(Self::IndexView<'a>,)>
    {
        self.each_ref()
            .map(|r| map(r.as_view()))
    }
}
impl<T, const N: usize> MaybeLists<T> for [[T; N]]
{
    type RowsMapped<MM> = Vec<MM>;
    type IndexView<'a> = &'a [T; N]
    where
        T: 'a,
        Self: 'a;
        
    fn index_view<'a>(&'a self, i: usize) -> Self::IndexView<'a>
    where
        T: 'a,
        Self: 'a
    {
        self[i].as_view()
    }
    fn as_views_option<'a>(&'a self) -> Option<Vec<Self::IndexView<'a>>>
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
        F: FnMut<(Self::IndexView<'a>,)>
    {
        self.iter()
            .map(|r| map(r.as_view()))
            .collect()
    }
}
impl<T, const N: usize, const M: usize> MaybeLists<T> for &[[T; N]; M]
{
    type RowsMapped<MM> = [MM; M];
    type IndexView<'a> = &'a [T; N]
    where
        T: 'a,
        Self: 'a;
        
    fn index_view<'a>(&'a self, i: usize) -> Self::IndexView<'a>
    where
        T: 'a,
        Self: 'a
    {
        self[i].as_view()
    }
    fn as_views_option<'a>(&'a self) -> Option<Vec<Self::IndexView<'a>>>
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
        F: FnMut<(Self::IndexView<'a>,)>
    {
        self.each_ref()
            .map(|r| map(r.as_view()))
    }
}
impl<T, const N: usize> MaybeLists<T> for &[[T; N]]
{
    type RowsMapped<MM> = Vec<MM>;
    type IndexView<'a> = &'a [T; N]
    where
        T: 'a,
        Self: 'a;
        
    fn index_view<'a>(&'a self, i: usize) -> Self::IndexView<'a>
    where
        T: 'a,
        Self: 'a
    {
        self[i].as_view()
    }
    fn as_views_option<'a>(&'a self) -> Option<Vec<Self::IndexView<'a>>>
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
        F: FnMut<(Self::IndexView<'a>,)>
    {
        self.iter()
            .map(|r| map(r.as_view()))
            .collect()
    }
}

impl<T> MaybeLists<T> for Vec<&[T]>
{
    type RowsMapped<MM> = Vec<MM>;
    type IndexView<'a> = &'a [T]
    where
        T: 'a,
        Self: 'a;
        
    fn index_view<'a>(&'a self, i: usize) -> Self::IndexView<'a>
    where
        T: 'a,
        Self: 'a
    {
        self[i].as_view()
    }
    fn as_views_option<'a>(&'a self) -> Option<Vec<Self::IndexView<'a>>>
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
        F: FnMut<(Self::IndexView<'a>,)>
    {
        self.iter()
            .map(|r| map(r.as_view()))
            .collect()
    }
}
impl<T, const M: usize> MaybeLists<T> for [&[T]; M]
{
    type RowsMapped<MM> = [MM; M];
    type IndexView<'a> = &'a [T]
    where
        T: 'a,
        Self: 'a;
        
    fn index_view<'a>(&'a self, i: usize) -> Self::IndexView<'a>
    where
        T: 'a,
        Self: 'a
    {
        self[i].as_view()
    }
    fn as_views_option<'a>(&'a self) -> Option<Vec<Self::IndexView<'a>>>
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
        F: FnMut<(Self::IndexView<'a>,)>
    {
        self.each_ref()
            .map(|r| map(r.as_view()))
    }
}
impl<T> MaybeLists<T> for [&[T]]
{
    type RowsMapped<MM> = Vec<MM>;
    type IndexView<'a> = &'a [T]
    where
        T: 'a,
        Self: 'a;
        
    fn index_view<'a>(&'a self, i: usize) -> Self::IndexView<'a>
    where
        T: 'a,
        Self: 'a
    {
        self[i].as_view()
    }
    fn as_views_option<'a>(&'a self) -> Option<Vec<Self::IndexView<'a>>>
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
        F: FnMut<(Self::IndexView<'a>,)>
    {
        self.iter()
            .map(|r| map(r.as_view()))
            .collect()
    }
}
impl<T, const M: usize> MaybeLists<T> for &[&[T]; M]
{
    type RowsMapped<MM> = [MM; M];
    type IndexView<'a> = &'a [T]
    where
        T: 'a,
        Self: 'a;
        
    fn index_view<'a>(&'a self, i: usize) -> Self::IndexView<'a>
    where
        T: 'a,
        Self: 'a
    {
        self[i].as_view()
    }
    fn as_views_option<'a>(&'a self) -> Option<Vec<Self::IndexView<'a>>>
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
        F: FnMut<(Self::IndexView<'a>,)>
    {
        self.each_ref()
            .map(|r| map(r.as_view()))
    }
}
impl<T> MaybeLists<T> for &[&[T]]
{
    type RowsMapped<MM> = Vec<MM>;
    type IndexView<'a> = &'a [T]
    where
        T: 'a,
        Self: 'a;
        
    fn index_view<'a>(&'a self, i: usize) -> Self::IndexView<'a>
    where
        T: 'a,
        Self: 'a
    {
        self[i].as_view()
    }
    fn as_views_option<'a>(&'a self) -> Option<Vec<Self::IndexView<'a>>>
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
        F: FnMut<(Self::IndexView<'a>,)>
    {
        self.iter()
            .map(|r| map(r.as_view()))
            .collect()
    }
}

impl<T, const N: usize> MaybeLists<T> for Vec<&[T; N]>
{
    type RowsMapped<MM> = Vec<MM>;
    type IndexView<'a> = &'a [T; N]
    where
        T: 'a,
        Self: 'a;
        
    fn index_view<'a>(&'a self, i: usize) -> Self::IndexView<'a>
    where
        T: 'a,
        Self: 'a
    {
        self[i].as_view()
    }
    fn as_views_option<'a>(&'a self) -> Option<Vec<Self::IndexView<'a>>>
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
        F: FnMut<(Self::IndexView<'a>,)>
    {
        self.iter()
            .map(|r| map(r.as_view()))
            .collect()
    }
}
impl<T, const N: usize, const M: usize> MaybeLists<T> for [&[T; N]; M]
{
    type RowsMapped<MM> = [MM; M];
    type IndexView<'a> = &'a [T; N]
    where
        T: 'a,
        Self: 'a;
        
    fn index_view<'a>(&'a self, i: usize) -> Self::IndexView<'a>
    where
        T: 'a,
        Self: 'a
    {
        self[i].as_view()
    }
    fn as_views_option<'a>(&'a self) -> Option<Vec<Self::IndexView<'a>>>
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
        F: FnMut<(Self::IndexView<'a>,)>
    {
        self.each_ref()
            .map(|r| map(r.as_view()))
    }
}
impl<T, const N: usize> MaybeLists<T> for [&[T; N]]
{
    type RowsMapped<MM> = Vec<MM>;
    type IndexView<'a> = &'a [T; N]
    where
        T: 'a,
        Self: 'a;
        
    fn index_view<'a>(&'a self, i: usize) -> Self::IndexView<'a>
    where
        T: 'a,
        Self: 'a
    {
        self[i].as_view()
    }
    fn as_views_option<'a>(&'a self) -> Option<Vec<Self::IndexView<'a>>>
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
        F: FnMut<(Self::IndexView<'a>,)>
    {
        self.iter()
            .map(|r| map(r.as_view()))
            .collect()
    }
}
impl<T, const N: usize, const M: usize> MaybeLists<T> for &[&[T; N]; M]
{
    type RowsMapped<MM> = [MM; M];
    type IndexView<'a> = &'a [T; N]
    where
        T: 'a,
        Self: 'a;
        
    fn index_view<'a>(&'a self, i: usize) -> Self::IndexView<'a>
    where
        T: 'a,
        Self: 'a
    {
        self[i].as_view()
    }
    fn as_views_option<'a>(&'a self) -> Option<Vec<Self::IndexView<'a>>>
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
        F: FnMut<(Self::IndexView<'a>,)>
    {
        self.each_ref()
            .map(|r| map(r.as_view()))
    }
}
impl<T, const N: usize> MaybeLists<T> for &[&[T; N]]
{
    type RowsMapped<MM> = Vec<MM>;
    type IndexView<'a> = &'a [T; N]
    where
        T: 'a,
        Self: 'a;

    fn index_view<'a>(&'a self, i: usize) -> Self::IndexView<'a>
    where
        T: 'a,
        Self: 'a
    {
        self[i].as_view()
    }
    fn as_views_option<'a>(&'a self) -> Option<Vec<Self::IndexView<'a>>>
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
        F: FnMut<(Self::IndexView<'a>,)>
    {
        self.iter()
            .map(|r| map(r.as_view()))
            .collect()
    }
}

impl<T> MaybeLists<T> for Array1<T>
{
    type RowsMapped<M> = M;
    type IndexView<'a> = ArrayView1<'a, T>
    where
        T: 'a,
        Self: 'a;

    fn index_view<'a>(&'a self, i: usize) -> Self::IndexView<'a>
    where
        T: 'a,
        Self: 'a
    {
        (&core::array::from_ref(self)[i]).into()
    }
    fn as_views_option<'a>(&'a self) -> Option<Vec<Self::IndexView<'a>>>
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
        F: FnMut<(Self::IndexView<'a>,)>
    {
        map(self.as_view())
    }
}
impl<'c, T> MaybeLists<T> for ArrayView1<'c, T>
{
    type RowsMapped<M> = M;
    type IndexView<'a> = ArrayView1<'a, T>
    where
        T: 'a,
        Self: 'a;

    fn index_view<'a>(&'a self, i: usize) -> Self::IndexView<'a>
    where
        T: 'a,
        Self: 'a
    {
        core::array::from_ref(self)[i].reborrow()
    }
    fn as_views_option<'a>(&'a self) -> Option<Vec<Self::IndexView<'a>>>
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
        F: FnMut<(Self::IndexView<'a>,)>
    {
        map(self.reborrow())
    }
}

impl<T> MaybeLists<T> for Array2<T>
{
    type RowsMapped<M> = Vec<M>;
    type IndexView<'a> = ArrayView1<'a, T>
    where
        T: 'a,
        Self: 'a;

    fn index_view<'a>(&'a self, i: usize) -> Self::IndexView<'a>
    where
        T: 'a,
        Self: 'a
    {
        self.row(i)
    }
    fn as_views_option<'a>(&'a self) -> Option<Vec<Self::IndexView<'a>>>
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
        F: FnMut<(Self::IndexView<'a>,)>
    {
        self.rows()
            .into_iter()
            .map(|r| map(r.as_view()))
            .collect()
    }
}
impl<'b, T> MaybeLists<T> for ArrayView2<'b, T>
{
    type RowsMapped<M> = Vec<M>;
    type IndexView<'a> = ArrayView1<'a, T>
    where
        T: 'a,
        Self: 'a;

    fn index_view<'a>(&'a self, i: usize) -> Self::IndexView<'a>
        where
            T: 'a,
            Self: 'a
    {
        self.row(i)
    }
    fn as_views_option<'a>(&'a self) -> Option<Vec<Self::IndexView<'a>>>
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
        F: FnMut<(Self::IndexView<'a>,)>
    {
        self.rows()
            .into_iter()
            .map(|r| map(r.as_view()))
            .collect()
    }
}