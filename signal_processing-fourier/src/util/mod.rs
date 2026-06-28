use core::borrow::BorrowMut;

use array_trait::length::{self, LengthValue};
use bulks::IntoBulk;
use moddef::moddef;

use crate::util;

moddef!(
    pub(crate) mod {
        fct_i,
        fct_ii,
        fct_iii,
        fct_iv,
        fst_i,
        fst_ii,
        fst_iii,
        fft
    },
    flat(pub(crate)) mod {
        assign,
        radix,
        is_prime,
        to_complex,
        indirect_reffable
    }
);

pub fn recurse_buffer<B, T>(buffer: &mut B) -> Option<&mut [T]>
where
    B: ?Sized,
    for<'a> &'a mut B: IntoBulk<Item: BorrowMut<T>>,
{
    trait Recurse<T>
    {
        fn buffer(&mut self) -> Option<&mut [T]>;
    }
    impl<B, T> Recurse<T> for B
    where
        B: ?Sized
    {
        default fn buffer(&mut self) -> Option<&mut [T]>
        {
            None
        }
    }
    impl<B, T> Recurse<T> for B
    where
        B: BorrowMut<[T]> + ?Sized
    {
        fn buffer(&mut self) -> Option<&mut [T]>
        {
            Some(self.borrow_mut())
        }
    }

    Recurse::buffer(buffer)
}

pub const fn closest_prime(x: usize) -> Option<usize>
{
    if x == 0
    {
        return None;
    }
    let mut n = x;
    let mut m = x - 1;
    while m != 0 || n != usize::MAX
    {
        if util::is_prime(n)
        {
            return Some(n)
        }
        if util::is_prime(m)
        {
            return Some(m)
        }
        n = n.saturating_add(1);
        m = m.saturating_sub(1);
    }
    None
}

#[inline]
pub const fn is_power_of<L, R>(n: L, r: R) -> bool
where
    L: LengthValue,
    R: LengthValue
{
    length::value::eq(length::value::len(r).pow(length::value::len(n).ilog(length::value::len(r))), n)
}