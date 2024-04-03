use core::{marker::PhantomData};

use array_math::ArrayOps;
use num::{complex::ComplexFloat, Zero};

use crate::{Lists, MaybeLists, TruncateIm};

moddef::moddef!(
    mod {
        add,
        borrow,
        borrow_mut,
        deref,
        deref_mut,
        r#fn,
        from,
        mul,
        neg,
        one,
        pow,
        product,
        sub
    }
);

#[derive(Debug, Clone, Copy)]
pub struct Polynomial<T, C>
where
    C: MaybeLists<T>
{
    c: C,
    phantom: PhantomData<T>
}

impl<T, C> Polynomial<T, C>
where
    C: MaybeLists<T>
{
    pub fn new(c: C) -> Self
    {
        Polynomial {
            c,
            phantom: PhantomData
        }
    }

    pub type View<'a> = Polynomial<T, C::View<'a>>
    where
        C::View<'a>: MaybeLists<T>,
        Self: 'a;
    pub type Owned = Polynomial<T, C::Owned>
    where
        C::Owned: MaybeLists<T>;

    pub fn as_view<'a>(&'a self) -> Self::View<'a>
    where
        C::View<'a>: MaybeLists<T>
    {
        Polynomial::new(self.c.as_view())
    }
    pub fn to_owned(&self) -> Self::Owned
    where
        C::Owned: MaybeLists<T>,
        T: Clone
    {
        Polynomial::new(self.c.to_owned())
    }
    pub fn into_inner(self) -> C
    {
        self.c
    }

    pub fn map_to_owned<'a, F>(&'a self, map: F) -> Polynomial<F::Output, C::Mapped<F::Output>>
    where
        C: Lists<T>,
        T: 'a,
        F: FnMut<(&'a T,)>,
        C::Mapped<F::Output>: MaybeLists<F::Output>
    {
        Polynomial::new(self.c.map_to_owned(map))
    }

    pub fn re(&self) -> Polynomial<T::Real, C::Mapped<T::Real>>
    where
        C: Lists<T>,
        T: ComplexFloat,
        C::Mapped<T::Real>: MaybeLists<T::Real>
    {
        self.map_to_owned(|c| c.re())
    }
    
    pub fn truncate_im<U>(&self) -> Polynomial<U, C::Mapped<U>>
    where
        C: Lists<T>,
        T: TruncateIm,
        T::Real: Into<U>,
        U: ComplexFloat<Real = T::Real> + 'static,
        C::Mapped<U>: MaybeLists<U>
    {
        self.map_to_owned(|c| c.truncate_im())
    }

    pub fn truncate<const N: usize>(self) -> Polynomial<T, [T; N]>
    where
        T: Zero,
        Self: Into<Polynomial<T, Vec<T>>>
    {
        let mut p: Polynomial<T, Vec<T>> = self.into();

        p.c.reverse();
        p.c.resize_with(N, T::zero);
        p.c.reverse();

        let mut c = p.c.into_iter();
        Polynomial::new(
            ArrayOps::fill(|_| c.next().unwrap())
        )
    }

    pub fn one() -> Self
    where
        Polynomial<T, ()>: Into<Self>
    {
        Polynomial::new(()).into()
    }
    
    pub fn zero() -> Self
    where
        Polynomial<T, [T; 0]>: Into<Self>
    {
        Polynomial::new([]).into()
    }
}