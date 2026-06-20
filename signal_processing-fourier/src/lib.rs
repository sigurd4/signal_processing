#![feature(unboxed_closures)]
#![feature(const_trait_impl)]
#![feature(const_convert)]
#![feature(const_option_ops)]
#![feature(macro_metavar_expr_concat)]
#![feature(const_ops)]
#![feature(generic_const_exprs)]
#![feature(specialization)]

moddef::moddef!(
    flat(pub) mod {
        permute
    },
    flat(pub) mod {
        //czt,
        //dct_2d,
        //dct,
        dft,
        //dst_2d,
        //dst,
        dtft
    },
    mod {
        util
    },
    flat mod {
        scratch_space
    }
);

macro_rules! transform_2d_inplace {
    ($bulks:ident.$transform:ident()) => {
        {
            use bulks::*;
            use array_trait::length;

            let bulks = $bulks;
            let mut len = length::value::or_len(0);
            for bulk in bulks.bulk_mut()
            {
                bulk.$transform();
                len = length::value::max(bulk.bulk_mut().length(), len);
            }
            for i in 0..length::value::len(len)
            {
                let mut v = bulks.bulk_mut()
                    .map(|bulk| bulk.bulk_mut().get_mut(i).ok_or(T::zero()))
                    .collect::<Vec<_>>();

                crate::util::IndirectReffable(&mut v).$transform()
            }
        }
    };
}

use transform_2d_inplace as transform_2d_inplace;

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