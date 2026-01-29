use array_trait::length::{self, LengthValue};

use crate::util::{self, LengthAlwaysPrime};

pub(crate) type Radix<L> = <L as LengthValueExt>::Radix;

pub(crate) const fn radix<L>(n: L) -> Option<Radix<L>>
where
    L: LengthValue
{
    LengthValueExt::radix(n)
}

pub(crate) const fn _radix<L, Y>(n: L) -> Option<Y>
where
    L: LengthValue,
    Y: LengthValue
{
    if length::value::ne(n, [(); 0]) && let Some(r) = closest_mod0_of(1 << ((length::value::len(n).ilog2() + 1) / 2), n) && length::value::ne(r, [(); 0])
    {
        Some(r)
    }
    else
    {
        None
    }
}

pub(crate) const trait LengthValueExt: LengthValue
{
    type Radix: LengthValue;

    fn radix(n: Self) -> Option<Self::Radix>;
}
impl<I> const LengthValueExt for I
where
    I: LengthValue
{
    default type Radix = usize;

    default fn radix(n: Self) -> Option<Self::Radix>
    {
        _radix(n)
    }
}
impl<const N: usize> const LengthValueExt for [(); N]
where
    [(); _radix(N).unwrap_or(0)]:
{
    default type Radix = [(); _radix(N).unwrap_or(0)];

    default fn radix(_: Self) -> Option<Self::Radix>
    {
        const { _radix(N) }
    }
}
impl<const N: usize> const LengthValueExt for [(); N]
where
    Self: LengthAlwaysPrime,
    [(); _radix(N).unwrap_or(0)]:
{
    type Radix = [(); N];

    fn radix(n: Self) -> Option<Self::Radix>
    {
        Some(n)
    }
}

const fn closest_mod0_of<L, R, Y>(x: L, y: R) -> Option<Y>
where
    L: LengthValue,
    R: LengthValue,
    Y: LengthValue
{
    if length::value::eq(y, [(); 0])
    {
        return None
    }
    if util::is_prime(y)
    {
        return Some(length::value::or_len(length::value::len(y)));
    }
    let mut m = length::value::len(length::value::saturating_sub(x, [(); 1]));
    let mut n = length::value::len(length::value::add(m, [(); 1]));
    while n != 0 && (m != 0 || n != usize::MAX)
    {
        if length::value::eq(length::value::rem(y, n), [(); 0])
        {
            return Some(length::value::or_len(n))
        }
        if m != 0 && length::value::eq(length::value::rem(y, m), [(); 0])
        {
            return Some(length::value::or_len(m))
        }
        n = n.saturating_add(1);
        m = m.saturating_sub(1);
    }
    None
}