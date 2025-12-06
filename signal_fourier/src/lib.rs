#![feature(unboxed_closures)]
#![feature(fn_traits)]
#![feature(const_trait_impl)]
#![feature(const_convert)]
#![feature(trusted_random_access)]
#![feature(core_intrinsics)]
#![feature(specialization)]

#[cfg(not(any(feature = "std", feature = "libm")))]
compile_error!("Either the \"std\" or the \"libm\" feature must be enabled to compile");

moddef::moddef!(
    pub mod {
        //fft,
        permute
    },
    flat(pub) mod {
        fft_output,
        inplace_bulk,
        scratch_space
    }
);

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