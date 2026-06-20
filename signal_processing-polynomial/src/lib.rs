#![feature(const_trait_impl)]
#![feature(const_precise_live_drops)]

use core::ops::Add;

use array_trait::{AsSlice};
use bulks::{Bulk, CollectNearest, IntoBulk, Merge};

pub struct Polynomial<I>(I)
where
    I: IntoBulk;

impl<I> Polynomial<I>
where
    I: IntoBulk
{
    pub const fn new(bulk: I) -> Self
    {
        Self(bulk)
    }

    pub const fn into_inner(self) -> I
    {
        let Self(bulk) = self;
        bulk
    }

    pub fn into_owned(self) -> Polynomial<<I::IntoBulk as CollectNearest>::Nearest>
    {
        let Self(bulk) = self;
        Polynomial(bulk.into_bulk().collect_nearest())
    }
}

impl<I> AsSlice for Polynomial<I>
where
    I: IntoBulk + AsSlice
{
    type Elem = I::Elem;

    fn as_slice(&self) -> &[Self::Elem]
    {
        self.0.as_slice()
    }

    fn as_mut_slice(&mut self) -> &mut [Self::Elem]
    {
        self.0.as_mut_slice()
    }
}

impl<I1, I2, T> Add<Polynomial<I2>> for Polynomial<I1>
where
    I1: IntoBulk<Item: Add<I2::Item, Output = T> + Into<T>>,
    I2: IntoBulk<Item: Into<T>>
{
    type Output = Polynomial<Merge<I1::IntoBulk, I2::IntoBulk, fn(I1::Item, I2::Item) -> T>>;

    fn add(self, rhs: Polynomial<I2>) -> Self::Output
    {
        let Self(lhs) = self;
        let Polynomial(rhs) = rhs;
        let lhs = lhs.into_bulk();
        let rhs = rhs.into_bulk();

        Polynomial(lhs.merge(rhs, Add::add))
    }
}