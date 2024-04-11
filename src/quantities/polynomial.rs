use core::{marker::PhantomData, ops::DivAssign};

use array_math::{ArrayOps, SliceMath, SliceOps};
use num::{complex::ComplexFloat, traits::Euclid, One, Zero, Float};
use option_trait::NotVoid;

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
        euclid,
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

impl<T, C> NotVoid for Polynomial<T, C>
where
    C: MaybeLists<T>
{

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

    pub fn as_view<'a>(&'a self) -> Polynomial<T, C::View<'a>>
    where
        C::View<'a>: MaybeLists<T>
    {
        Polynomial::new(self.c.as_view())
    }
    pub fn to_owned(&self) -> Polynomial<T, C::Owned>
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
        C: MaybeLists<T>
    {
        if let Some(s) = self.as_view_slices_option()
        {
            return s.iter()
                .all(|s| s.trim_zeros_front().len() == 0)
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

    pub fn gcd<C2>(self, rhs: Polynomial<T, C2>) -> C::RowsMapped<Polynomial<T, Vec<T>>>
    where
        T: ComplexFloat + DivAssign,
        C::RowOwned: MaybeList<T>,
        C2: MaybeList<T> + Clone,
        Polynomial<T, C2>: Into<Polynomial<T, Vec<T>>>,
        Polynomial<T, C::RowOwned>: Into<Polynomial<T, Vec<T>>>,
        Polynomial<T, Vec<T>>: Euclid
    {
        self.into_inner()
            .map_rows_into_owned(|lhs| {
                let mut a: Polynomial<T, Vec<T>> = Polynomial::new(lhs).into();
                let mut b: Polynomial<T, Vec<T>> = rhs.clone().into();

                while !b.iter()
                    .all(|b| b.abs() < T::Real::epsilon())
                {
                    let r = a.rem_euclid(&b);
                    a = b;
                    b = r;
                }
                if a.is_zero()
                {
                    Polynomial::one()
                }
                else
                {
                    let norm = a[0];
                    a.div_assign_all(norm);
                    a
                }
            })
    }
}