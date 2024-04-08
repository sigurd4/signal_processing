use core::marker::PhantomData;

use array_math::{ArrayOps, SliceMath};
use num::{complex::ComplexFloat, One, Zero};

use crate::{Lists, MaybeList, MaybeLists, TruncateIm};

moddef::moddef!(
    mod {
        add_assign,
        add,
        borrow,
        borrow_mut,
        deref,
        deref_mut,
        eq,
        r#fn,
        from,
        mul_assign,
        mul,
        neg,
        one,
        partial_eq,
        pow,
        product,
        sub_assign,
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

    pub fn map_into_owned<F>(self, map: F) -> Polynomial<F::Output, C::Mapped<F::Output>>
    where
        T: Clone,
        C: Lists<T>,
        F: FnMut<(T,)>,
        C::Mapped<F::Output>: MaybeLists<F::Output>
    {
        Polynomial::new(self.c.map_into_owned(map))
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
    pub fn is_zero(&self) -> bool
    where
        T: Zero,
        C: MaybeList<T>
    {
        if let Some(s) = self.as_view_slice_option()
        {
            return s.trim_zeros_front().len() == 0
        }
        false
    }
    pub fn is_one(&self) -> bool
    where
        T: One + Zero + PartialEq,
        C: MaybeList<T>
    {
        if let Some(s) = self.as_view_slice_option()
        {
            let s = s.trim_zeros_front();
            return s.len() == 1 && s[0].is_one()
        }
        true
    }
}