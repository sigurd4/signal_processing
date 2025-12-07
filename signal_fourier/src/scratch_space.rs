use core::{borrow::BorrowMut, marker::PhantomData, mem::MaybeUninit};

use array_trait::length::{self, Value};
use bulks::{Bulk, CollectionAdapter, CollectionStrategy, FromBulk, Map, RepeatNWith, StaticBulk};


pub const trait ScratchSpace<A = length::Mapped<<Self as Bulk>::Length, <Self as IntoIterator>::Item>>: ~const Bulk
where
    A: CollectionAdapter + ?Sized
{
    type ScratchSpace: ~const BorrowMut<[MaybeUninit<A::Elem>]> + Sized + FromBulk<length::Mapped<A, MaybeUninit<A::Elem>>>;

    fn scratch_space(&self) -> Self::ScratchSpace;
}
impl<I, T> ScratchSpace<[T]> for I
where
    I: Bulk
{
    type ScratchSpace = Vec<MaybeUninit<T>>;

    fn scratch_space(&self) -> Self::ScratchSpace
    {
        let n = length::value::or_len::<Value<I::Length>>(self.len());
        bulks::repeat_n_with(Uniniter(PhantomData), n).collect_nearest()
    }
}
impl<I, T, const N: usize> const ScratchSpace<[T; N]> for I
where
    I: ~const Bulk + StaticBulk<Array<T> = [T; N]>
{
    type ScratchSpace = [MaybeUninit<T>; N];

    fn scratch_space(&self) -> Self::ScratchSpace
    {
        let n = length::value::or_len::<Value<I::Length>>(self.len());
        bulks::repeat_n_with(Uniniter(PhantomData), n).collect_nearest()
    }
}

struct Uniniter<T>(PhantomData<T>);

impl<T> const FnOnce<()> for Uniniter<T>
{
    type Output = MaybeUninit<T>;

    extern "rust-call" fn call_once(self, (): ()) -> Self::Output
    {
        MaybeUninit::uninit()
    }
}
impl<T> const FnMut<()> for Uniniter<T>
{
    extern "rust-call" fn call_mut(&mut self, (): ()) -> Self::Output
    {
        MaybeUninit::uninit()
    }
}
impl<T> const Fn<()> for Uniniter<T>
{
    extern "rust-call" fn call(&self, (): ()) -> Self::Output
    {
        MaybeUninit::uninit()
    }
}