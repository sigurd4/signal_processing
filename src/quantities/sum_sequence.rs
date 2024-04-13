use core::{iter::Sum, marker::PhantomData};

use num::Zero;
use option_trait::NotVoid;

use crate::MaybeList;

moddef::moddef!(
    mod {
        add,
        borrow_mut,
        borrow,
        default,
        deref_mut,
        deref,
        from,
        neg,
        sub,
        sum,
        try_from,
        zero
    }
);

#[derive(Debug, Clone, Copy)]
pub struct SumSequence<T, S>
where
    S: MaybeList<T>
{
    s: S,
    phantom: PhantomData<T>
}

impl<T, S> NotVoid for SumSequence<T, S>
where
    S: MaybeList<T>
{

}

impl<T, S> SumSequence<T, S>
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
    
    pub type View<'a> = SumSequence<T, S::View<'a>>
    where
        S::View<'a>: MaybeList<T>,
        Self: 'a;
    pub type Owned = SumSequence<T, S::Owned>
    where
        S::Owned: MaybeList<T>;

    pub fn as_view<'a>(&'a self) -> SumSequence<T, S::View<'a>>
    where
        S::View<'a>: MaybeList<T>
    {
        SumSequence::new(self.s.as_view())
    }
    pub fn to_owned(&self) -> SumSequence<T, S::Owned>
    where
        S::Owned: MaybeList<T>,
        T: Clone
    {
        SumSequence::new(self.s.to_owned())
    }
    pub fn into_owned(self) -> SumSequence<T, S::Owned>
    where
        S::Owned: MaybeList<T>,
        T: Clone
    {
        SumSequence::new(self.s.into_owned())
    }
    pub fn into_inner(self) -> S
    {
        self.s
    }
    pub fn zero() -> Self
    where
        SumSequence<T, ()>: Into<Self>
    {
        SumSequence::new(()).into()
    }
    pub fn is_zero(&self) -> bool
    where
        T: Zero + Clone + Sum
    {
        if let Some(s) = self.as_view_slice_option()
        {
            let y: T = s.iter()
                .map(|s| s.clone())
                .sum();
            return y.is_zero()
        }
        true
    }
}