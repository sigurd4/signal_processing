use core::marker::PhantomData;

use crate::MaybeList;

moddef::moddef!(
    mod {
        borrow_mut,
        borrow,
        default,
        deref_mut,
        deref,
        from,
        mul,
        one,
        pow,
        product,
        try_from
    }
);

#[derive(Debug, Clone, Copy)]
pub struct ProductSequence<T, S>
where
    S: MaybeList<T>
{
    s: S,
    phantom: PhantomData<T>
}

impl<T, S> ProductSequence<T, S>
where
    S: MaybeList<T>
{
    pub fn new(s: S) -> Self
    {
        Self {
            s,
            phantom: PhantomData
        }
    }
    
    pub type View<'a> = ProductSequence<T, S::View<'a>>
    where
        S::View<'a>: MaybeList<T>,
        Self: 'a;
    pub type Owned = ProductSequence<T, S::Owned>
    where
        S::Owned: MaybeList<T>;

    pub fn as_view<'a>(&'a self) -> Self::View<'a>
    where
        S::View<'a>: MaybeList<T>
    {
        ProductSequence::new(self.s.as_view())
    }
    pub fn to_owned(&self) -> Self::Owned
    where
        S::Owned: MaybeList<T>,
        T: Clone
    {
        ProductSequence::new(self.s.to_owned())
    }
    pub fn into_inner(self) -> S
    {
        self.s
    }
}