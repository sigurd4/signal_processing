#![feature(unboxed_closures)]
#![feature(const_trait_impl)]
#![feature(const_convert)]
#![feature(const_option_ops)]
#![feature(macro_metavar_expr_concat)]
#![feature(const_ops)]
#![feature(const_destruct)]
#![feature(generic_const_exprs)]
#![feature(specialization)]

moddef::moddef!(
    flat(pub) mod {
        permute
    },
    flat(pub) mod {
        czt,
        dct_2d for cfg(feature = "ndarray"),
        dct,
        dft_2d for cfg(feature = "ndarray"),
        dft,
        dst_2d for cfg(feature = "ndarray"),
        dst,
        dtft
    },
    mod {
        util
    },
    flat mod {
        scratch_space
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