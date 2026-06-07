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
#![feature(impl_trait_in_assoc_type)]
#![feature(generic_const_exprs)]
#![feature(core_intrinsics)]
#![feature(specialization)]

moddef::moddef!(
    pub mod {
        permute
    },
    flat(pub) mod {
        dft_inplace,
        dft,
        idft
    },
    mod {
        util
    },
    flat mod {
        scratch_space
    }
);

pub mod conf
{
    pub struct TwoSided;
    pub struct OneSided;
    pub struct TwoSidedCentered;
}

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
mod tests
{
    use num_complex::ComplexFloat;

    use super::*;

    pub(crate) fn approx_eq<T>(lhs: &[T], rhs: &[T], diff: T::Real) -> bool
    where
        T: ComplexFloat
    {
        if lhs.len() != rhs.len()
        {
            return false
        }
        lhs.iter()
            .zip(rhs)
            .all(|(a, b)| (*a - *b).abs() <= diff)
    }

    #[test]
    fn it_works()
    {
        
    }
}