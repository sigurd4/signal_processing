use array_trait::{Array, length::{self, LengthValue}, same::Same};
use bulks::option::MaybeLength;
use moddef::moddef;

use crate::util;

moddef!(
    flat(pub(crate)) mod {
        radix,
        is_prime
    }
);

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