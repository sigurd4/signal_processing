use core::{borrow::BorrowMut, marker::PhantomData, mem::MaybeUninit};

use array_trait::{length, same::Same};
use bulks::{AsBulk, Bulk, IntoBulk, StaticBulk};

pub type ScratchSpace<I, A = <I as IntoIterator>::Item> = <I as ScratchBulk<A>>::ScratchSpace;

pub const trait ScratchBulk<A = <Self as IntoIterator>::Item>: ~const Bulk
{
    type ScratchSpace: ~const BorrowMut<[MaybeUninit<A>]> + Sized + IntoBulk<Item = MaybeUninit<A>> + AsBulk;

    fn scratch_space(&self) -> Self::ScratchSpace;
}
impl<I, T> ScratchBulk<T> for I
where
    I: Bulk
{
    default type ScratchSpace = Vec<MaybeUninit<T>>;

    default fn scratch_space(&self) -> Self::ScratchSpace
    {
        Box::<[T]>::new_uninit_slice(self.len()).into_vec().same().unwrap()
    }
}
impl<I, T, const N: usize> const ScratchBulk<T> for I
where
    I: ~const Bulk + StaticBulk<Array<()> = [(); N]>
{
    type ScratchSpace = [MaybeUninit<T>; N];

    fn scratch_space(&self) -> Self::ScratchSpace
    {
        MaybeUninit::uninit().transpose()
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