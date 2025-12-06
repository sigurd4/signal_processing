use core::iter::TrustedRandomAccess;

use bulks::Bulk;

pub trait DigitRevPermutation: MutRandomAccessBulk
{
    fn digit_rev_permutation(&mut self, radix: usize) -> &mut Self
    {
        let len = self.len();
        if len <= radix
        {
            return self;
        }
        assert!(crate::is_power_of(len, radix), "Length must be a power of radix.");

        let mut i = 1;
        let mut j = len/radix + 1;
        while i < len - 1
        {
            if i < j - 1
            {
                core::mem::swap(self.get_mut(i), self.get_mut(j - 1));
            }
            let mut k = len/radix;
            while k*(radix - 1) < j
            {
                j -= k*(radix - 1);
                k /= radix;
            }
            j += k;
            i += 1;
        }
        self
    }
}

fn digit_rev_permutation<T>(slice: &mut [T], radix: usize)
{
    let len = slice.len();
    if len <= radix
    {
        return;
    }
    assert!(crate::is_power_of(len, radix), "Length must be a power of radix.");

    let mut i = 1;
    let mut j = len/radix + 1;
    while i < len - 1
    {
        if i < j - 1
        {
            unsafe {
                core::ptr::swap_nonoverlapping(slice.as_mut_ptr().add(i), slice.as_mut_ptr().add(j - 1), 1);
            }
        }
        let mut k = len/radix;
        while k*(radix - 1) < j
        {
            j -= k*(radix - 1);
            k /= radix;
        }
        j += k;
        i += 1;
    }
}