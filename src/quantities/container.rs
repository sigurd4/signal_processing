

use ndarray::{prelude::ArrayView, ArrayBase, Dimension, OwnedRepr};

use crate::MaybeContainer;

pub trait Container<T>: MaybeContainer<T>
{
    type Mapped<M>: Container<M> + Sized;

    fn map_to_owned<'a, F>(&'a self, map: F) -> Self::Mapped<F::Output>
    where
        T: 'a,
        F: FnMut<(&'a T,)>;
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
}

impl<T, D> Container<T> for ArrayBase<OwnedRepr<T>, D>
where
    D: Dimension
{
    type Mapped<M> = ArrayBase<OwnedRepr<M>, D>;

    fn map_to_owned<'a, F>(&'a self, map: F) -> Self::Mapped<F::Output>
    where
        T: 'a,
        F: FnMut<(&'a T,)>
    {
        self.map(map)
    }
}

impl<'c, T, D> Container<T> for ArrayView<'c, T, D>
where
    D: Dimension
{
    type Mapped<M> = ArrayBase<OwnedRepr<M>, D>;
    
    fn map_to_owned<'a, F>(&'a self, map: F) -> Self::Mapped<F::Output>
    where
        T: 'a,
        F: FnMut<(&'a T,)>
    {
        self.map(map)
    }
}