use array_trait::length::{self, LengthValue};
use bulks::{InplaceBulk, IntoBulk};

use crate::util;

pub const trait Permute: ~const IntoBulk<IntoBulk: ~const InplaceBulk>
{
    type Output: ~const IntoBulk<IntoBulk = Self::IntoBulk>;

    fn digit_rev_permute<R>(self, radix: R) -> Self::Output
    where
        R: LengthValue;

    fn bit_rev_permute(self) -> Self::Output
    {
        self.digit_rev_permute([(); 2])
    }
}

impl<T> const Permute for T
where
    T: ~const IntoBulk<IntoBulk: ~const InplaceBulk>
{
    type Output = Self::IntoBulk;

    fn digit_rev_permute<R>(self, radix: R) -> Self::IntoBulk
    where
        R: LengthValue
    {
        let bulk = self.into_bulk();
        let len = self.length();
        if length::value::le(len, radix)
        {
            return bulk;
        }
        assert!(util::is_power_of(len, radix), "Length must be a power of radix.");
        assert!(length::value::ne(radix, [(); 0]), "Radix must be nonzero!");

        let j0 = length::value::div(len, length::value::max(radix, [(); 1]));
        let rm1 = length::value::saturating_sub(radix, 1);
        let mut i = 1;
        let mut j = length::value::len(j0);
        while length::value::lt(i, length::value::saturating_sub(len, 1))
        {
            if i < j
            {
                self.swap_inplace(i, j);
            }
            let mut k = length::value::len(j0);
            let mut kk = length::value::len(length::value::mul(j0, rm1));
            while kk <= j
            {
                j -= kk;
                k /= length::value::len(radix);
                kk = k*length::value::len(rm1);
            }
            j += k;
            i += 1;
        }
        bulk
    }
}

#[cfg(test)]
mod test
{
    use bulks::{Bulk, IntoBulk};

    use super::*;

    #[test]
    fn it_works()
    {
        let a = [0, 1, 2, 3, 4, 5, 6, 7];

        let mut bulk = a.into_bulk();
        bulk.bit_rev_permute();
        for x in bulk.collect_array()
        {
            print!("{x:03b} ")
        }
    }
}