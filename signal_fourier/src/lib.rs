#![feature(unboxed_closures)]
#![feature(fn_traits)]
#![feature(const_trait_impl)]
#![feature(const_convert)]
#![feature(try_trait_v2)]
#![feature(const_try)]
#![feature(trusted_random_access)]
#![feature(const_destruct)]
#![feature(core_intrinsics)]
#![feature(specialization)]

use bulks::{Bulk, InplaceBulk};

#[cfg(not(any(feature = "std", feature = "libm")))]
compile_error!("Either the \"std\" or the \"libm\" feature must be enabled to compile");

moddef::moddef!(
    pub mod {
        //fft,
        permute
    },
    flat(pub) mod {
        //scratch_space
    }
);

pub const trait FourierBulk: ~const Bulk
{
    fn digit_rev_permute(self, radix: usize) -> Self
    where
        Self: for<'a> ~const InplaceBulk<'a, ItemMut = &'a mut <Self as IntoIterator>::Item> + Sized
    {
        permute::digit_rev_permute(self, radix)
    }

    fn bit_rev_permute(self) -> Self
    where
        Self: for<'a> ~const InplaceBulk<'a, ItemMut = &'a mut <Self as IntoIterator>::Item> + Sized
    {
        self.digit_rev_permute(2)
    }
}
impl<I> const FourierBulk for I
where
    I: ~const Bulk
{

}

#[inline]
pub const fn is_power_of(n: usize, r: usize) -> bool
{
    r.pow(n.ilog(r)) == n
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works()
    {
        
    }
}