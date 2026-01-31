#![feature(unboxed_closures)]
#![feature(fn_traits)]
#![feature(const_trait_impl)]
#![feature(const_convert)]
#![feature(try_trait_v2)]
#![feature(const_option_ops)]
#![feature(const_try)]
#![feature(trusted_random_access)]
#![feature(maybe_uninit_uninit_array_transpose)]
#![feature(const_destruct)]
#![feature(const_result_trait_fn)]
#![feature(macro_metavar_expr_concat)]
#![feature(const_ops)]
#![feature(str_as_str)]
#![feature(generic_const_exprs)]
#![feature(core_intrinsics)]
#![feature(specialization)]

#[cfg(not(any(feature = "std", feature = "libm")))]
compile_error!("Either the \"std\" or the \"libm\" feature must be enabled to compile");

moddef::moddef!(
    pub mod {
        fft,
        permute
    },
    flat(pub) mod {
        scratch_space,
        fourier_inplace
    },
    mod {
        util
    }
);

macro_rules! temp {
    ($temp:ident for $len:expr) => {
        temp!($temp for $len => $temp)
    };
    ($temp:ident for $len:expr => $x:ident) => {
        let mut ${concat($temp, _owned)} = None;
        let $x = match $temp.take()
        {
            Some($temp) => $temp,
            None => ${concat($temp, _owned)}.insert(crate::ScratchLength::scratch_space($len, num_traits::Zero::zero())).borrow_mut()
        };
    };
}
use temp as temp;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works()
    {
        
    }
}