use array_trait::{length::{self, LengthValue}};
use bulks::{AsBulk, Bulk, IntoBulk};

use crate::util;

pub const trait Permute
{
    fn digit_rev_permute<R>(&mut self, radix: R)
    where
        R: LengthValue;

    fn bit_rev_permute(&mut self)
    {
        self.digit_rev_permute([(); 2])
    }
}

const impl<B, T> Permute for B
where
    for<'a> &'a mut B: ~const IntoBulk<Item = &'a mut T>,
    B: ?Sized
{
    fn digit_rev_permute<R>(&mut self, radix: R)
    where
        R: LengthValue
    {
        let bulk = self.bulk_mut();
        let len = bulk.length();
        bulk.for_each(core::mem::drop);
        if length::value::le(len, radix)
        {
            return;
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
                self.bulk_mut().swap::<T, _, _>(i, j);
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
    }
}

#[cfg(test)]
mod test
{
    use super::Permute;

    #[test]
    fn it_works()
    {
        let mut a = [0u8, 1, 2, 3, 4, 5, 6, 7];

        a.bit_rev_permute();
        for x in a
        {
            print!("{x:03b} ")
        }
    }
}