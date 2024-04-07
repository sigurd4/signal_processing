use core::ops::MulAssign;

use array_math::{SliceMath, SliceOps};
use num::complex::ComplexFloat;
use option_trait::Maybe;

use crate::{List, Lists, Matrix, MaybeContainer, OwnedList};

pub enum WHTOrdering
{
    Sequency,
    Hadamard,
    Dyadic
}

pub trait WHT<'a, T, O, OO, const OOO: bool>: Lists<T>
where
    T: ComplexFloat,
    O: OwnedList<T>,
    OO: Maybe<O>
{
    fn wht(&'a self, ordering: WHTOrdering) -> Self::RowsMapped<OO>;
}

impl<'a, T, L> WHT<'a, T, <L::IndexView<'a> as MaybeContainer<T>>::Owned, Option<<L::IndexView<'a> as MaybeContainer<T>>::Owned>, false> for L
where
    T: ComplexFloat + MulAssign<T::Real>,
    L: Lists<T>,
    L::IndexView<'a>: List<T, Owned: OwnedList<T>>,
{
    fn wht(&'a self, ordering: WHTOrdering) -> Self::RowsMapped<Option<<L::IndexView<'a> as MaybeContainer<T>>::Owned>>
    {
        self.map_rows_to_owned(|h| {
            let mut h = h.to_owned();
            if !h.as_mut_slice().len().is_power_of_two()
            {
                None
            }
            else
            {
                h.as_mut_slice().fwht();
                match ordering
                {
                    WHTOrdering::Sequency => {
                        h.as_mut_slice().bit_rev_permutation();
                        h.as_mut_slice().grey_code_permutation();
                    },
                    WHTOrdering::Hadamard => (),
                    WHTOrdering::Dyadic => {
                        h.as_mut_slice().bit_rev_permutation()
                    },
                }
                Some(h)
            }
        })
    }
}

impl<'a, T, L> WHT<'a, T, <L::IndexView<'a> as MaybeContainer<T>>::Owned, <L::IndexView<'a> as MaybeContainer<T>>::Owned, true> for L
where
    T: ComplexFloat + MulAssign<T::Real>,
    L: Lists<T> + Matrix<T, Width = usize>,
    L::IndexView<'a>: List<T, Owned: OwnedList<T>>,
    [(); L::WIDTH.is_power_of_two() as usize - 1]:
{
    fn wht(&'a self, ordering: WHTOrdering) -> Self::RowsMapped<<L::IndexView<'a> as MaybeContainer<T>>::Owned>
    {
        self.map_rows_to_owned(|h| {
            let mut h = h.to_owned();
            h.as_mut_slice().fwht();
            match ordering
            {
                WHTOrdering::Sequency => {
                    h.as_mut_slice().bit_rev_permutation();
                    h.as_mut_slice().grey_code_permutation();
                },
                WHTOrdering::Hadamard => (),
                WHTOrdering::Dyadic => {
                    h.as_mut_slice().bit_rev_permutation()
                },
            }
            h
        })
    }
}

#[cfg(test)]
mod test
{
    use crate::{WHTOrdering, WHT};

    #[test]
    fn test()
    {
        let x = [19, -1, 11, -9, -7, 13, -15, 5].map(|x| x as f32);

        let xf: [_; _] = x.wht(WHTOrdering::Sequency);
        
        println!("{:?}", xf);
    }
}