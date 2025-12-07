use bulks::InplaceBulk;

pub(crate) const fn digit_rev_permute<I, T>(mut bulk: I, radix: usize) -> I
where
    I: for<'a> ~const InplaceBulk<'a, ItemMut = &'a mut T>
{
    let len = bulk.len();
    if len <= radix
    {
        return bulk;
    }

    let mut i = 1;
    let mut j = len/radix;
    while i < len - 1
    {
        if i < j
        {
            bulk.swap_inplace(i, j);
        }
        let mut k = len/radix;
        while k*(radix - 1) <= j
        {
            j -= k*(radix - 1);
            k /= radix;
        }
        j += k;
        i += 1;
    }
    bulk
}

#[cfg(test)]
mod test
{
    use bulks::{Bulk, IntoBulk};

    use crate::FourierBulk;

    #[test]
    fn it_works()
    {
        let a = [0, 1, 2, 3, 4, 5, 6, 7];

        for x in a.into_bulk()
            .bit_rev_permute()
            .collect_array()
        {
            print!("{x:03b} ")
        }
    }
}