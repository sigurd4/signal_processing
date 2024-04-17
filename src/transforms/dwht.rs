use core::ops::MulAssign;

use array_math::{SliceMath, SliceOps};
use num::complex::ComplexFloat;
use option_trait::Maybe;

use crate::{Lists, Matrix, OwnedList};

pub enum WhtOrdering
{
    Sequency,
    Hadamard,
    Dyadic
}

pub trait Dwht<T, O, OO, const OOO: bool>: Lists<T>
where
    T: ComplexFloat,
    O: OwnedList<T>,
    OO: Maybe<O>
{
    fn dwht(self, ordering: WhtOrdering) -> Self::RowsMapped<OO>;
}

impl<T, L> Dwht<T, L::RowOwned, Option<L::RowOwned>, false> for L
where
    T: ComplexFloat + MulAssign<T::Real>,
    L: Lists<T>,
    L::RowOwned: OwnedList<T>,
{
    fn dwht(self, ordering: WhtOrdering) -> Self::RowsMapped<Option<L::RowOwned>>
    {
        self.map_rows_into_owned(|mut h| {
            let h_mut = h.as_mut_slice();
            if !h_mut.len().is_power_of_two()
            {
                None
            }
            else
            {
                h_mut.fwht();
                match ordering
                {
                    WhtOrdering::Sequency => {
                        h_mut.bit_rev_permutation();
                        h_mut.grey_code_permutation();
                    },
                    WhtOrdering::Hadamard => (),
                    WhtOrdering::Dyadic => {
                        h_mut.bit_rev_permutation()
                    },
                }
                Some(h)
            }
        })
    }
}

impl<T, L> Dwht<T, L::RowOwned, L::RowOwned, true> for L
where
    T: ComplexFloat + MulAssign<T::Real>,
    L: Lists<T> + Matrix<T, Width = usize>,
    L::RowOwned: OwnedList<T>,
    [(); L::WIDTH.is_power_of_two() as usize - 1]:
{
    fn dwht(self, ordering: WhtOrdering) -> Self::RowsMapped<L::RowOwned>
    {
        self.map_rows_into_owned(|mut h| {
            let h_mut = h.as_mut_slice();
            h_mut.fwht();
            match ordering
            {
                WhtOrdering::Sequency => {
                    h_mut.bit_rev_permutation();
                    h_mut.grey_code_permutation();
                },
                WhtOrdering::Hadamard => (),
                WhtOrdering::Dyadic => {
                    h_mut.bit_rev_permutation()
                },
            }
            h
        })
    }
}

#[cfg(test)]
mod test
{
    use crate::{WhtOrdering, Dwht};

    #[test]
    fn test()
    {
        let x = [19, -1, 11, -9, -7, 13, -15, 5].map(|x| x as f32);

        let xf: [_; _] = x.dwht(WhtOrdering::Sequency);
        
        println!("{:?}", xf);
    }
}