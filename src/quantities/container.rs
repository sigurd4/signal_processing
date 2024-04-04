

use ndarray::{prelude::ArrayView, ArrayBase, Dimension, IndexLonger, NdIndex, OwnedRepr};

use crate::MaybeContainer;

pub trait Container<T>: MaybeContainer<T>
{
    type Mapped<M>: Container<M> + Sized;

    fn map_to_owned<'a, F>(&'a self, map: F) -> Self::Mapped<F::Output>
    where
        T: 'a,
        F: FnMut<(&'a T,)>;
        
    fn map_into_owned<F>(self, map: F) -> Self::Mapped<F::Output>
    where
        T: Clone,
        Self: Sized,
        F: FnMut<(T,)>;
}

impl<T> Container<T> for Vec<T>
{
    type Mapped<M> = Vec<M>;

    fn map_to_owned<'a, F>(&'a self, map: F) -> Self::Mapped<F::Output>
    where
        T: 'a,
        F: FnMut<(&'a T,)>
    {
        self.iter()
            .map(map)
            .collect()
    }

    fn map_into_owned<F>(self, map: F) -> Self::Mapped<F::Output>
    where
        T: Clone,
        Self: Sized,
        F: FnMut<(T,)>
    {
        self.into_iter()
            .map(map)
            .collect()
    }
}
impl<T> Container<T> for [T]
{
    type Mapped<M> = <Self::Owned as Container<T>>::Mapped<M>;

    fn map_to_owned<'a, F>(&'a self, map: F) -> Self::Mapped<F::Output>
    where
        T: 'a,
        F: FnMut<(&'a T,)>
    {
        self.iter()
            .map(map)
            .collect()
    }

    fn map_into_owned<F>(self, mut map: F) -> Self::Mapped<F::Output>
    where
        T: Clone,
        Self: Sized,
        F: FnMut<(T,)>
    {
        self.iter()
            .map(|x| map(x.clone()))
            .collect()
    }
}
impl<T, const N: usize> Container<T> for [T; N]
{
    type Mapped<M> = [M; N];

    fn map_to_owned<'a, F>(&'a self, map: F) -> Self::Mapped<F::Output>
    where
        T: 'a,
        F: FnMut<(&'a T,)>
    {
        self.each_ref()
            .map(map)
    }

    fn map_into_owned<F>(self, map: F) -> Self::Mapped<F::Output>
    where
        T: Clone,
        Self: Sized,
        F: FnMut<(T,)>
    {
        self.map(map)
    }
}
impl<'c, T> Container<T> for &'c [T]
{
    type Mapped<M> = <Self::Owned as Container<T>>::Mapped<M>;

    fn map_to_owned<'a, F>(&'a self, map: F) -> Self::Mapped<F::Output>
    where
        T: 'a,
        F: FnMut<(&'a T,)>
    {
        self.iter()
            .map(map)
            .collect()
    }

    fn map_into_owned<F>(self, mut map: F) -> Self::Mapped<F::Output>
    where
        T: Clone,
        Self: Sized,
        F: FnMut<(T,)>
    {
        self.iter()
            .map(|x| map(x.clone()))
            .collect()
    }
}
impl<'b, T, const N: usize> Container<T> for &'b [T; N]
{
    type Mapped<M> = <Self::Owned as Container<T>>::Mapped<M>;

    fn map_to_owned<'a, F>(&'a self, map: F) -> Self::Mapped<F::Output>
    where
        T: 'a,
        F: FnMut<(&'a T,)>
    {
        self.each_ref()
            .map(map)
    }

    fn map_into_owned<F>(self, mut map: F) -> Self::Mapped<F::Output>
    where
        T: Clone,
        Self: Sized,
        F: FnMut<(T,)>
    {
        self.each_ref()
            .map(|x| map(x.clone()))
    }
}

impl<T> Container<T> for Vec<Vec<T>>
{
    type Mapped<MM> = Vec<Vec<MM>>;

    fn map_to_owned<'a, F>(&'a self, mut map: F) -> Self::Mapped<F::Output>
    where
        T: 'a,
        F: FnMut<(&'a T,)>
    {
        self.iter()
            .map(|s| s.iter()
                .map(&mut map)
                .collect()
            ).collect()
    }

    fn map_into_owned<F>(self, mut map: F) -> Self::Mapped<F::Output>
    where
        T: Clone,
        Self: Sized,
        F: FnMut<(T,)>
    {
        self.into_iter()
            .map(|x| x.into_iter()
                .map(&mut map)
                .collect()
            ).collect()
    }
}
impl<T, const M: usize> Container<T> for [Vec<T>; M]
{
    type Mapped<MM> = [Vec<MM>; M];

    fn map_to_owned<'a, F>(&'a self, mut map: F) -> Self::Mapped<F::Output>
    where
        T: 'a,
        F: FnMut<(&'a T,)>
    {
        self.each_ref()
            .map(|s| s.iter()
                .map(&mut map)
                .collect()
            )
    }

    fn map_into_owned<F>(self, mut map: F) -> Self::Mapped<F::Output>
    where
        T: Clone,
        Self: Sized,
        F: FnMut<(T,)>
    {
        self.map(|x| x.into_iter()
                .map(&mut map)
                .collect()
            )
    }
}
impl<T> Container<T> for [Vec<T>]
{
    type Mapped<MM> = <Self::Owned as Container<T>>::Mapped<MM>;

    fn map_to_owned<'a, F>(&'a self, mut map: F) -> Self::Mapped<F::Output>
    where
        T: 'a,
        F: FnMut<(&'a T,)>
    {
        self.iter()
            .map(|s| s.iter()
                .map(&mut map)
                .collect()
            ).collect()
    }

    fn map_into_owned<F>(self, mut map: F) -> Self::Mapped<F::Output>
    where
        T: Clone,
        Self: Sized,
        F: FnMut<(T,)>
    {
        self.iter()
            .map(|x| x.iter()
                .map(|x| map(x.clone()))
                .collect()
            ).collect()
    }
}
impl<'b, T, const M: usize> Container<T> for &'b [Vec<T>; M]
{
    type Mapped<MM> = [Vec<MM>; M];

    fn map_to_owned<'a, F>(&'a self, mut map: F) -> Self::Mapped<F::Output>
    where
        T: 'a,
        F: FnMut<(&'a T,)>
    {
        self.each_ref()
            .map(|s| s.iter()
                .map(&mut map)
                .collect()
            )
    }

    fn map_into_owned<F>(self, mut map: F) -> Self::Mapped<F::Output>
    where
        T: Clone,
        Self: Sized,
        F: FnMut<(T,)>
    {
        self.each_ref()
            .map(|x| x.iter()
                .map(|x| map(x.clone()))
                .collect()
            )
    }
}
impl<'b, T> Container<T> for &'b [Vec<T>]
{
    type Mapped<MM> = <Self::Owned as Container<T>>::Mapped<MM>;

    fn map_to_owned<'a, F>(&'a self, mut map: F) -> Self::Mapped<F::Output>
    where
        T: 'a,
        F: FnMut<(&'a T,)>
    {
        self.iter()
            .map(|s| s.iter()
                .map(&mut map)
                .collect()
            ).collect()
    }

    fn map_into_owned<F>(self, mut map: F) -> Self::Mapped<F::Output>
    where
        T: Clone,
        Self: Sized,
        F: FnMut<(T,)>
    {
        self.iter()
            .map(|x| x.iter()
                .map(|x| map(x.clone()))
                .collect()
            ).collect()
    }
}

impl<T, const N: usize> Container<T> for Vec<[T; N]>
{
    type Mapped<MM> = Vec<[MM; N]>;

    fn map_to_owned<'a, F>(&'a self, mut map: F) -> Self::Mapped<F::Output>
    where
        T: 'a,
        F: FnMut<(&'a T,)>
    {
        self.iter()
            .map(|s| s.each_ref()
                .map(&mut map)
            ).collect()
    }

    fn map_into_owned<F>(self, mut map: F) -> Self::Mapped<F::Output>
    where
        T: Clone,
        Self: Sized,
        F: FnMut<(T,)>
    {
        self.iter()
            .map(|x| x.each_ref()
                .map(|x| map(x.clone()))
            ).collect()
    }
}
impl<T, const N: usize, const M: usize> Container<T> for [[T; N]; M]
{
    type Mapped<MM> = [[MM; N]; M];

    fn map_to_owned<'a, F>(&'a self, mut map: F) -> Self::Mapped<F::Output>
    where
        T: 'a,
        F: FnMut<(&'a T,)>
    {
        self.each_ref()
            .map(|s| s.each_ref()
                .map(&mut map)
            )
    }

    fn map_into_owned<F>(self, mut map: F) -> Self::Mapped<F::Output>
    where
        T: Clone,
        Self: Sized,
        F: FnMut<(T,)>
    {
        self.map(|x| x.map(&mut map))
    }
}
impl<T, const N: usize> Container<T> for [[T; N]]
{
    type Mapped<MM> = <Self::Owned as Container<T>>::Mapped<MM>;

    fn map_to_owned<'a, F>(&'a self, mut map: F) -> Self::Mapped<F::Output>
    where
        T: 'a,
        F: FnMut<(&'a T,)>
    {
        self.iter()
            .map(|s| s.each_ref()
                .map(&mut map)
            ).collect()
    }

    fn map_into_owned<F>(self, mut map: F) -> Self::Mapped<F::Output>
    where
        T: Clone,
        Self: Sized,
        F: FnMut<(T,)>
    {
        self.iter()
            .map(|x| x.each_ref()
                .map(|x| map(x.clone()))
            ).collect()
    }
}
impl<'b, T, const N: usize, const M: usize> Container<T> for &'b [[T; N]; M]
{
    type Mapped<MM> = <Self::Owned as Container<T>>::Mapped<MM>;

    fn map_to_owned<'a, F>(&'a self, mut map: F) -> Self::Mapped<F::Output>
    where
        T: 'a,
        F: FnMut<(&'a T,)>
    {
        self.each_ref()
            .map(|s| s.each_ref()
                .map(&mut map)
            )
    }

    fn map_into_owned<F>(self, mut map: F) -> Self::Mapped<F::Output>
    where
        T: Clone,
        Self: Sized,
        F: FnMut<(T,)>
    {
        self.each_ref()
            .map(|x| x.each_ref()
                .map(|x| map(x.clone()))
            )
    }
}
impl<'b, T, const N: usize> Container<T> for &'b [[T; N]]
{
    type Mapped<MM> = <Self::Owned as Container<T>>::Mapped<MM>;

    fn map_to_owned<'a, F>(&'a self, mut map: F) -> Self::Mapped<F::Output>
    where
        T: 'a,
        F: FnMut<(&'a T,)>
    {
        self.iter()
            .map(|s| s.each_ref()
                .map(&mut map)
            ).collect()
    }

    fn map_into_owned<F>(self, mut map: F) -> Self::Mapped<F::Output>
    where
        T: Clone,
        Self: Sized,
        F: FnMut<(T,)>
    {
        self.iter()
            .map(|x| x.each_ref()
                .map(|x| map(x.clone()))
            ).collect()
    }
}

impl<'b, T> Container<T> for Vec<&'b [T]>
{
    type Mapped<MM> = <Self::Owned as Container<T>>::Mapped<MM>;

    fn map_to_owned<'a, F>(&'a self, mut map: F) -> Self::Mapped<F::Output>
    where
        T: 'a,
        F: FnMut<(&'a T,)>
    {
        self.iter()
            .map(|s| s.iter()
                .map(&mut map)
                .collect()
            ).collect()
    }

    fn map_into_owned<F>(self, mut map: F) -> Self::Mapped<F::Output>
    where
        T: Clone,
        Self: Sized,
        F: FnMut<(T,)>
    {
        self.into_iter()
            .map(|x| x.iter()
                .map(|x| map(x.clone()))
                .collect()
            ).collect()
    }
}
impl<'b, T, const M: usize> Container<T> for [&'b [T]; M]
{
    type Mapped<MM> = <Self::Owned as Container<T>>::Mapped<MM>;

    fn map_to_owned<'a, F>(&'a self, mut map: F) -> Self::Mapped<F::Output>
    where
        T: 'a,
        F: FnMut<(&'a T,)>
    {
        self.each_ref()
            .map(|s| s.iter()
                .map(&mut map)
                .collect()
            )
    }

    fn map_into_owned<F>(self, mut map: F) -> Self::Mapped<F::Output>
    where
        T: Clone,
        Self: Sized,
        F: FnMut<(T,)>
    {
        self.map(|x| x.iter()
                .map(|x| map(x.clone()))
                .collect()
            )
    }
}
impl<'b, T> Container<T> for [&'b [T]]
{
    type Mapped<MM> = <Self::Owned as Container<T>>::Mapped<MM>;

    fn map_to_owned<'a, F>(&'a self, mut map: F) -> Self::Mapped<F::Output>
    where
        T: 'a,
        F: FnMut<(&'a T,)>
    {
        self.iter()
            .map(|s| s.iter()
                .map(&mut map)
                .collect()
            ).collect()
    }

    fn map_into_owned<F>(self, mut map: F) -> Self::Mapped<F::Output>
    where
        T: Clone,
        Self: Sized,
        F: FnMut<(T,)>
    {
        self.iter()
            .map(|x| x.iter()
                .map(|x| map(x.clone()))
                .collect()
            ).collect()
    }
}
impl<'b, 'c, T, const M: usize> Container<T> for &'b [&'c [T]; M]
{
    type Mapped<MM> = <Self::Owned as Container<T>>::Mapped<MM>;

    fn map_to_owned<'a, F>(&'a self, mut map: F) -> Self::Mapped<F::Output>
    where
        T: 'a,
        F: FnMut<(&'a T,)>
    {
        self.each_ref()
            .map(|s| s.iter()
                .map(&mut map)
                .collect()
            )
    }

    fn map_into_owned<F>(self, mut map: F) -> Self::Mapped<F::Output>
    where
        T: Clone,
        Self: Sized,
        F: FnMut<(T,)>
    {
        self.each_ref()
            .map(|x| x.iter()
                .map(|x| map(x.clone()))
                .collect()
            )
    }
}
impl<'b, 'c, T> Container<T> for &'b [&'c [T]]
{
    type Mapped<MM> = <Self::Owned as Container<T>>::Mapped<MM>;

    fn map_to_owned<'a, F>(&'a self, mut map: F) -> Self::Mapped<F::Output>
    where
        T: 'a,
        F: FnMut<(&'a T,)>
    {
        self.iter()
            .map(|s| s.iter()
                .map(&mut map)
                .collect()
            ).collect()
    }

    fn map_into_owned<F>(self, mut map: F) -> Self::Mapped<F::Output>
    where
        T: Clone,
        Self: Sized,
        F: FnMut<(T,)>
    {
        self.iter()
            .map(|x| x.iter()
                .map(|x| map(x.clone()))
                .collect()
            ).collect()
    }
}

impl<'b, T, const N: usize> Container<T> for Vec<&'b [T; N]>
{
    type Mapped<MM> = <Self::Owned as Container<T>>::Mapped<MM>;

    fn map_to_owned<'a, F>(&'a self, mut map: F) -> Self::Mapped<F::Output>
    where
        T: 'a,
        F: FnMut<(&'a T,)>
    {
        self.iter()
            .map(|s| s.each_ref()
                .map(&mut map)
            ).collect()
    }

    fn map_into_owned<F>(self, mut map: F) -> Self::Mapped<F::Output>
    where
        T: Clone,
        Self: Sized,
        F: FnMut<(T,)>
    {
        self.into_iter()
            .map(|x| x.each_ref()
                .map(|x| map(x.clone()))
            ).collect()
    }
}
impl<'b, T, const N: usize, const M: usize> Container<T> for [&'b [T; N]; M]
{
    type Mapped<MM> = <Self::Owned as Container<T>>::Mapped<MM>;

    fn map_to_owned<'a, F>(&'a self, mut map: F) -> Self::Mapped<F::Output>
    where
        T: 'a,
        F: FnMut<(&'a T,)>
    {
        self.each_ref()
            .map(|s| s.each_ref()
                .map(&mut map)
            )
    }

    fn map_into_owned<F>(self, mut map: F) -> Self::Mapped<F::Output>
    where
        T: Clone,
        Self: Sized,
        F: FnMut<(T,)>
    {
        self.map(|x| x.each_ref()
                .map(|x| map(x.clone()))
            )
    }
}
impl<'b, T, const N: usize> Container<T> for [&'b [T; N]]
{
    type Mapped<MM> = <Self::Owned as Container<T>>::Mapped<MM>;

    fn map_to_owned<'a, F>(&'a self, mut map: F) -> Self::Mapped<F::Output>
    where
        T: 'a,
        F: FnMut<(&'a T,)>
    {
        self.iter()
            .map(|s| s.each_ref()
                .map(&mut map)
            ).collect()
    }

    fn map_into_owned<F>(self, mut map: F) -> Self::Mapped<F::Output>
    where
        T: Clone,
        Self: Sized,
        F: FnMut<(T,)>
    {
        self.iter()
            .map(|x| x.each_ref()
                .map(|x| map(x.clone()))
            ).collect()
    }
}
impl<'b, 'c, T, const N: usize, const M: usize> Container<T> for &'b [&'c [T; N]; M]
{
    type Mapped<MM> = <Self::Owned as Container<T>>::Mapped<MM>;

    fn map_to_owned<'a, F>(&'a self, mut map: F) -> Self::Mapped<F::Output>
    where
        T: 'a,
        F: FnMut<(&'a T,)>
    {
        self.each_ref()
            .map(|s| s.each_ref()
                .map(&mut map)
            )
    }

    fn map_into_owned<F>(self, mut map: F) -> Self::Mapped<F::Output>
    where
        T: Clone,
        Self: Sized,
        F: FnMut<(T,)>
    {
        self.map(|x| x.each_ref()
                .map(|x| map(x.clone()))
            )
    }
}
impl<'b, 'c, T, const N: usize> Container<T> for &'b [&'c [T; N]]
{
    type Mapped<MM> = <Self::Owned as Container<T>>::Mapped<MM>;

    fn map_to_owned<'a, F>(&'a self, mut map: F) -> Self::Mapped<F::Output>
    where
        T: 'a,
        F: FnMut<(&'a T,)>
    {
        self.iter()
            .map(|s| s.each_ref()
                .map(&mut map)
            ).collect()
    }

    fn map_into_owned<F>(self, mut map: F) -> Self::Mapped<F::Output>
    where
        T: Clone,
        Self: Sized,
        F: FnMut<(T,)>
    {
        self.iter()
            .map(|x| x.each_ref()
                .map(|x| map(x.clone()))
            ).collect()
    }
}

impl<T, D> Container<T> for ArrayBase<OwnedRepr<T>, D>
where
    D: Dimension,
    <D as Dimension>::Pattern: NdIndex<D>
{
    type Mapped<M> = ArrayBase<OwnedRepr<M>, D>;

    fn map_to_owned<'a, F>(&'a self, mut map: F) -> Self::Mapped<F::Output>
    where
        T: 'a,
        F: FnMut<(&'a T,)>
    {
        ArrayBase::from_shape_fn(self.dim(), |i| map(&self[i]))
    }

    fn map_into_owned<F>(self, mut map: F) -> Self::Mapped<F::Output>
    where
        T: Clone,
        Self: Sized,
        F: FnMut<(T,)>
    {
        ArrayBase::from_shape_fn(self.dim(), |i| map(self[i].clone()))
    }
}

impl<'c, T, D> Container<T> for ArrayView<'c, T, D>
where
    D: Dimension,
    <D as Dimension>::Pattern: NdIndex<D>
{
    type Mapped<M> = ArrayBase<OwnedRepr<M>, D>;
    
    fn map_to_owned<'a, F>(&'a self, mut map: F) -> Self::Mapped<F::Output>
    where
        T: 'a,
        F: FnMut<(&'a T,)>
    {
        ArrayBase::from_shape_fn(self.dim(), |i| map(&self[i]))
    }

    fn map_into_owned<F>(self, mut map: F) -> Self::Mapped<F::Output>
    where
        T: Clone,
        Self: Sized,
        F: FnMut<(T,)>
    {
        ArrayBase::from_shape_fn(self.dim(), |i| map(self[i].clone()))
    }
}