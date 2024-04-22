use ndarray::{ArrayBase, ArrayView, Dimension, NdIndex, OwnedRepr};
use option_trait::NotVoid;

use crate::MaybeContainer;

pub trait ContainerOrSingle<T>
{
    type Index;
    type Mapped<M>: ContainerOrSingle<M> + Sized;

    fn map_to_owned<'a, F>(&'a self, map: F) -> Self::Mapped<F::Output>
    where
        T: 'a,
        F: FnMut<(&'a T,)>;
        
    fn map_into_owned<F>(self, map: F) -> Self::Mapped<F::Output>
    where
        T: Clone,
        Self: Sized,
        F: FnMut<(T,)>;
    fn index_get(&self, i: Self::Index) -> Option<&T>;
}

impl<T> ContainerOrSingle<T> for T
{
    type Index = ();
    type Mapped<M> = M;

    fn map_to_owned<'a, F>(&'a self, mut map: F) -> Self::Mapped<F::Output>
    where
        T: 'a,
        F: FnMut<(&'a T,)>
    {
        map(self)
    }
    fn map_into_owned<F>(self, mut map: F) -> Self::Mapped<F::Output>
    where
        T: Clone,
        Self: Sized,
        F: FnMut<(T,)>
    {
        map(self)
    }
    fn index_get(&self, (): Self::Index) -> Option<&T>
    {
        Some(self)
    }
}

impl<T> ContainerOrSingle<T> for Vec<T>
{
    type Mapped<M> = Vec<M>;
    type Index = usize;

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
    fn index_get(&self, i: Self::Index) -> Option<&T>
    {
        self.get(i)
    }
}
impl<T> ContainerOrSingle<T> for [T]
{
    type Mapped<M> = <<Self as MaybeContainer<T>>::Owned as ContainerOrSingle<T>>::Mapped<M>;
    type Index = usize;

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
    fn index_get(&self, i: Self::Index) -> Option<&T>
    {
        self.get(i)
    }
}
impl<T, const N: usize> ContainerOrSingle<T> for [T; N]
{
    type Mapped<M> = [M; N];
    type Index = usize;

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
    fn index_get(&self, i: Self::Index) -> Option<&T>
    {
        self.get(i)
    }
}
impl<'c, T> ContainerOrSingle<T> for &'c [T]
{
    type Mapped<M> = <<Self as MaybeContainer<T>>::Owned as ContainerOrSingle<T>>::Mapped<M>;
    type Index = usize;

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
    fn index_get(&self, i: Self::Index) -> Option<&T>
    {
        self.get(i)
    }
}
impl<'b, T, const N: usize> ContainerOrSingle<T> for &'b [T; N]
{
    type Mapped<M> = <<Self as MaybeContainer<T>>::Owned as ContainerOrSingle<T>>::Mapped<M>;
    type Index = usize;

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
    fn index_get(&self, i: Self::Index) -> Option<&T>
    {
        self.get(i)
    }
}

impl<T> ContainerOrSingle<T> for Vec<Vec<T>>
{
    type Mapped<MM> = Vec<Vec<MM>>;
    type Index = (usize, usize);

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
    fn index_get(&self, i: Self::Index) -> Option<&T>
    {
        self.get(i.0)
            .and_then(|r| r.get(i.1))
    }
}
impl<T, const M: usize> ContainerOrSingle<T> for [Vec<T>; M]
{
    type Mapped<MM> = [Vec<MM>; M];
    type Index = (usize, usize);

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
    fn index_get(&self, i: Self::Index) -> Option<&T>
    {
        self.get(i.0)
            .and_then(|r| r.get(i.1))
    }
}
impl<T> ContainerOrSingle<T> for [Vec<T>]
{
    type Mapped<MM> = <<Self as MaybeContainer<T>>::Owned as ContainerOrSingle<T>>::Mapped<MM>;
    type Index = (usize, usize);

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
    fn index_get(&self, i: Self::Index) -> Option<&T>
    {
        self.get(i.0)
            .and_then(|r| r.get(i.1))
    }
}
impl<'b, T, const M: usize> ContainerOrSingle<T> for &'b [Vec<T>; M]
{
    type Mapped<MM> = [Vec<MM>; M];
    type Index = (usize, usize);

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
    fn index_get(&self, i: Self::Index) -> Option<&T>
    {
        self.get(i.0)
            .and_then(|r| r.get(i.1))
    }
}
impl<'b, T> ContainerOrSingle<T> for &'b [Vec<T>]
{
    type Mapped<MM> = <<Self as MaybeContainer<T>>::Owned as ContainerOrSingle<T>>::Mapped<MM>;
    type Index = (usize, usize);

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
    fn index_get(&self, i: Self::Index) -> Option<&T>
    {
        self.get(i.0)
            .and_then(|r| r.get(i.1))
    }
}

impl<T, const N: usize> ContainerOrSingle<T> for Vec<[T; N]>
{
    type Mapped<MM> = Vec<[MM; N]>;
    type Index = (usize, usize);

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
    fn index_get(&self, i: Self::Index) -> Option<&T>
    {
        self.get(i.0)
            .and_then(|r| r.get(i.1))
    }
}
impl<T, const N: usize, const M: usize> ContainerOrSingle<T> for [[T; N]; M]
{
    type Mapped<MM> = [[MM; N]; M];
    type Index = (usize, usize);

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
    fn index_get(&self, i: Self::Index) -> Option<&T>
    {
        self.get(i.0)
            .and_then(|r| r.get(i.1))
    }
}
impl<T, const N: usize> ContainerOrSingle<T> for [[T; N]]
{
    type Mapped<MM> = <<Self as MaybeContainer<T>>::Owned as ContainerOrSingle<T>>::Mapped<MM>;
    type Index = (usize, usize);

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
    fn index_get(&self, i: Self::Index) -> Option<&T>
    {
        self.get(i.0)
            .and_then(|r| r.get(i.1))
    }
}
impl<'b, T, const N: usize, const M: usize> ContainerOrSingle<T> for &'b [[T; N]; M]
{
    type Mapped<MM> = <<Self as MaybeContainer<T>>::Owned as ContainerOrSingle<T>>::Mapped<MM>;
    type Index = (usize, usize);

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
    fn index_get(&self, i: Self::Index) -> Option<&T>
    {
        self.get(i.0)
            .and_then(|r| r.get(i.1))
    }
}
impl<'b, T, const N: usize> ContainerOrSingle<T> for &'b [[T; N]]
{
    type Mapped<MM> = <<Self as MaybeContainer<T>>::Owned as ContainerOrSingle<T>>::Mapped<MM>;
    type Index = (usize, usize);

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
    fn index_get(&self, i: Self::Index) -> Option<&T>
    {
        self.get(i.0)
            .and_then(|r| r.get(i.1))
    }
}

impl<'b, T> ContainerOrSingle<T> for Vec<&'b [T]>
{
    type Mapped<MM> = <<Self as MaybeContainer<T>>::Owned as ContainerOrSingle<T>>::Mapped<MM>;
    type Index = (usize, usize);

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
    fn index_get(&self, i: Self::Index) -> Option<&T>
    {
        self.get(i.0)
            .and_then(|r| r.get(i.1))
    }
}
impl<'b, T, const M: usize> ContainerOrSingle<T> for [&'b [T]; M]
{
    type Mapped<MM> = <<Self as MaybeContainer<T>>::Owned as ContainerOrSingle<T>>::Mapped<MM>;
    type Index = (usize, usize);

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
    fn index_get(&self, i: Self::Index) -> Option<&T>
    {
        self.get(i.0)
            .and_then(|r| r.get(i.1))
    }
}
impl<'b, T> ContainerOrSingle<T> for [&'b [T]]
{
    type Mapped<MM> = <<Self as MaybeContainer<T>>::Owned as ContainerOrSingle<T>>::Mapped<MM>;
    type Index = (usize, usize);

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
    fn index_get(&self, i: Self::Index) -> Option<&T>
    {
        self.get(i.0)
            .and_then(|r| r.get(i.1))
    }
}
impl<'b, 'c, T, const M: usize> ContainerOrSingle<T> for &'b [&'c [T]; M]
{
    type Mapped<MM> = <<Self as MaybeContainer<T>>::Owned as ContainerOrSingle<T>>::Mapped<MM>;
    type Index = (usize, usize);

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
    fn index_get(&self, i: Self::Index) -> Option<&T>
    {
        self.get(i.0)
            .and_then(|r| r.get(i.1))
    }
}
impl<'b, 'c, T> ContainerOrSingle<T> for &'b [&'c [T]]
{
    type Mapped<MM> = <<Self as MaybeContainer<T>>::Owned as ContainerOrSingle<T>>::Mapped<MM>;
    type Index = (usize, usize);

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
    fn index_get(&self, i: Self::Index) -> Option<&T>
    {
        self.get(i.0)
            .and_then(|r| r.get(i.1))
    }
}

impl<'b, T, const N: usize> ContainerOrSingle<T> for Vec<&'b [T; N]>
{
    type Mapped<MM> = <<Self as MaybeContainer<T>>::Owned as ContainerOrSingle<T>>::Mapped<MM>;
    type Index = (usize, usize);

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
    fn index_get(&self, i: Self::Index) -> Option<&T>
    {
        self.get(i.0)
            .and_then(|r| r.get(i.1))
    }
}
impl<'b, T, const N: usize, const M: usize> ContainerOrSingle<T> for [&'b [T; N]; M]
{
    type Mapped<MM> = <<Self as MaybeContainer<T>>::Owned as ContainerOrSingle<T>>::Mapped<MM>;
    type Index = (usize, usize);

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
    fn index_get(&self, i: Self::Index) -> Option<&T>
    {
        self.get(i.0)
            .and_then(|r| r.get(i.1))
    }
}
impl<'b, T, const N: usize> ContainerOrSingle<T> for [&'b [T; N]]
{
    type Mapped<MM> = <<Self as MaybeContainer<T>>::Owned as ContainerOrSingle<T>>::Mapped<MM>;
    type Index = (usize, usize);

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
    fn index_get(&self, i: Self::Index) -> Option<&T>
    {
        self.get(i.0)
            .and_then(|r| r.get(i.1))
    }
}
impl<'b, 'c, T, const N: usize, const M: usize> ContainerOrSingle<T> for &'b [&'c [T; N]; M]
{
    type Mapped<MM> = <<Self as MaybeContainer<T>>::Owned as ContainerOrSingle<T>>::Mapped<MM>;
    type Index = (usize, usize);

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
    fn index_get(&self, i: Self::Index) -> Option<&T>
    {
        self.get(i.0)
            .and_then(|r| r.get(i.1))
    }
}
impl<'b, 'c, T, const N: usize> ContainerOrSingle<T> for &'b [&'c [T; N]]
{
    type Mapped<MM> = <<Self as MaybeContainer<T>>::Owned as ContainerOrSingle<T>>::Mapped<MM>;
    type Index = (usize, usize);

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
    fn index_get(&self, i: Self::Index) -> Option<&T>
    {
        self.get(i.0)
            .and_then(|r| r.get(i.1))
    }
}

impl<T, D> ContainerOrSingle<T> for ArrayBase<OwnedRepr<T>, D>
where
    D: Dimension + NotVoid,
    <D as Dimension>::Pattern: NdIndex<D>
{
    type Mapped<M> = ArrayBase<OwnedRepr<M>, D>;
    type Index = <D as Dimension>::Pattern;

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
    fn index_get(&self, i: Self::Index) -> Option<&T>
    {
        self.get(i)
    }
}

impl<'c, T, D> ContainerOrSingle<T> for ArrayView<'c, T, D>
where
    D: Dimension + NotVoid,
    <D as Dimension>::Pattern: NdIndex<D>
{
    type Mapped<M> = ArrayBase<OwnedRepr<M>, D>;
    type Index = <D as Dimension>::Pattern;
    
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
    fn index_get(&self, i: Self::Index) -> Option<&T>
    {
        self.get(i)
    }
}