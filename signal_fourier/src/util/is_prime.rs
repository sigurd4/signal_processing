use array_trait::{Array, length::{self, LengthValue}};

pub const fn is_prime<L>(n: L) -> bool
where
    L: LengthValue
{
    LengthValueExt::is_prime(n)
}

const fn _is_prime<L>(n: L) -> bool
where
    L: LengthValue
{
    if length::value::eq(n, [(); 0])
    {
        return false
    }
    let n = length::value::len(n);
    let n_sqrt = 1 << ((n.ilog2() + 1) / 2);
    let mut m = 2;

    if n <= 1
    {
        return false;
    }

    while m <= n_sqrt
    {
        if n % m == 0
        {
            return false
        }
        m += 1
    }

    true
}

const trait LengthValueExt: LengthValue
{
    type AlwaysPrime: Array<Elem = ()>;

    fn is_prime(n: Self) -> bool;
}
impl<I> const LengthValueExt for I
where
    I: LengthValue
{
    default type AlwaysPrime = [(); 0];

    default fn is_prime(n: Self) -> bool
    {
        _is_prime(n)
    }
}
impl<const N: usize> const LengthValueExt for [(); N]
{
    default type AlwaysPrime = [(); 0];

    default fn is_prime(_: Self) -> bool
    {
        const { _is_prime(N) }
    }
}
impl<const N: usize> const LengthValueExt for [(); N]
where
    [(); _is_prime(N) as usize]:
{
    type AlwaysPrime = [(); _is_prime(N) as usize];

    fn is_prime(_: Self) -> bool
    {
        const { _is_prime(N) }
    }
}

pub(super) trait LengthAlwaysPrime: LengthValue
{

}
impl<L> LengthAlwaysPrime for L
where
    L: LengthValueExt<AlwaysPrime = [(); 1]>
{

}