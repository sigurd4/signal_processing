use core::{borrow::BorrowMut, marker::PhantomData, mem::MaybeUninit};

use array_trait::length::{self, Nearest, Value};
use bulks::{Bulk, CollectionAdapter, CollectionStrategy, FromBulk, RepeatNWith};


pub const trait ScratchSpace<T>: [const] Bulk<Length: Nearest<Nearest<MaybeUninit<T>>: [const] BorrowMut<[MaybeUninit<T>]> + Sized>>
{
    fn scratch_space(&self) -> <Self::Length as Nearest>::Nearest<MaybeUninit<T>>;
}
impl<B, T, C, S> const ScratchSpace<T> for B
where
    B: [const] Bulk<Length: Nearest<Nearest<MaybeUninit<T>> = C>, Strategy<MaybeUninit<T>, C> = S>,
    C: [const] FromBulk<S> + [const] BorrowMut<[MaybeUninit<T>]>,
    S: CollectionAdapter<Elem = MaybeUninit<T>> + [const] CollectionStrategy<RepeatNWith<Uniniter<T>, B::Length>, C>
{
    default fn scratch_space(&self) -> <Self::Length as Nearest>::Nearest<MaybeUninit<T>>
    {
        ScratchSpaceConstSpec::scratch_space(self)
    }
}

const trait ScratchSpaceConstSpec<T>: [const] Bulk<Length: Nearest<Nearest<MaybeUninit<T>>: [const] BorrowMut<[MaybeUninit<T>]> + Sized>>
{
    fn scratch_space(&self) -> <Self::Length as Nearest>::Nearest<MaybeUninit<T>>;
}
impl<B, T, C, S> ScratchSpaceConstSpec<T> for B
where
    B: Bulk<Length: Nearest<Nearest<MaybeUninit<T>> = C>, Strategy<MaybeUninit<T>, C> = S>,
    C: FromBulk<S> + BorrowMut<[MaybeUninit<T>]>,
    S: CollectionAdapter<Elem = MaybeUninit<T>> + CollectionStrategy<RepeatNWith<Uniniter<T>, B::Length>, C>
{
    default fn scratch_space(&self) -> <Self::Length as Nearest>::Nearest<MaybeUninit<T>>
    {
        ScratchSpaceSpec::scratch_space(self)
    }
}
impl<B, T, C, S> const ScratchSpaceConstSpec<T> for B
where
    B: [const] Bulk<Length: Nearest<Nearest<MaybeUninit<T>> = C>, Strategy<MaybeUninit<T>, C> = S>,
    C: [const] FromBulk<S> + [const] BorrowMut<[MaybeUninit<T>]>,
    S: CollectionAdapter<Elem = MaybeUninit<T>> + [const] CollectionStrategy<RepeatNWith<Uniniter<T>, B::Length>, C>
{
    fn scratch_space(&self) -> <Self::Length as Nearest>::Nearest<MaybeUninit<T>>
    {
        let n = length::value::or_len::<Value<B::Length>>(self.len());
        bulks::repeat_n_with(Uniniter(PhantomData), n).collect()
    }
}

trait ScratchSpaceSpec<T>: Bulk<Length: Nearest<Nearest<MaybeUninit<T>>: BorrowMut<[MaybeUninit<T>]> + Sized>>
{
    fn scratch_space(&self) -> <Self::Length as Nearest>::Nearest<MaybeUninit<T>>;
}
impl<B, T, C, S> ScratchSpaceSpec<T> for B
where
    B: Bulk<Length: Nearest<Nearest<MaybeUninit<T>> = C>, Strategy<MaybeUninit<T>, C> = S>,
    C: FromBulk<S> + BorrowMut<[MaybeUninit<T>]>,
    S: CollectionAdapter<Elem = MaybeUninit<T>> + CollectionStrategy<RepeatNWith<Uniniter<T>, B::Length>, C>
{
    default fn scratch_space(&self) -> <Self::Length as Nearest>::Nearest<MaybeUninit<T>>
    {
        let n = length::value::or_len::<Value<Self::Length>>(self.len());
        bulks::repeat_n_with(Uniniter(PhantomData), n).collect()
    }
}
macro_rules! spec {
    (<$t:ident $(, $a:ident)?> $ty:ty |$this:ident| $expr:expr) => {
        impl<$($a: Allocator,)? B, $t, S> ScratchSpaceSpec<T> for B
        where
            B: Bulk<Length: Nearest<Nearest<MaybeUninit<$t>> = $ty>, Strategy<MaybeUninit<$t>, $ty> = S>,
            $ty: FromBulk<S>,
            S: CollectionAdapter<Elem = MaybeUninit<$t>> + CollectionStrategy<RepeatNWith<Uniniter<$t>, B::Length>, $ty>,
            $t: Copy
        {
            fn scratch_space(&$this) -> <Self::Length as Nearest>::Nearest<MaybeUninit<$t>>
            {
                $expr
            }
        }
    };
}

#[cfg(feature = "alloc")]
spec!(<T> Vec<MaybeUninit<T>> |self| vec![MaybeUninit::uninit(); self.len()]);

impl<B, T, S, const N: usize> const ScratchSpace<T> for B
where
    B: [const] Bulk<Length: Nearest<Nearest<MaybeUninit<T>> = [MaybeUninit<T>; N]>, Strategy<MaybeUninit<T>, [MaybeUninit<T>; N]> = S>,
    [MaybeUninit<T>; N]: [const] FromBulk<S> + [const] BorrowMut<[MaybeUninit<T>]>,
    S: CollectionAdapter<Elem = MaybeUninit<T>> + [const] CollectionStrategy<RepeatNWith<Uniniter<T>, B::Length>, [MaybeUninit<T>; N]>,
    T: Copy
{
    fn scratch_space(&self) -> <Self::Length as Nearest>::Nearest<MaybeUninit<T>>
    {
        [MaybeUninit::uninit(); N]
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