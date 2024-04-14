use core::ops::DivAssign;

use num::complex::ComplexFloat;
use array_math::{SliceMath, SliceOps};

use crate::{ListOrSingle, MaybeList, MaybeLists, System, Tf};

pub trait IsAllPass: System
{
    type Output: ListOrSingle<bool>;

    fn isallpass(&self) -> Self::Output;
}

impl<T, B, A> IsAllPass for Tf<T, B, A>
where
    T: ComplexFloat + DivAssign,
    B: MaybeLists<T>,
    A: MaybeList<T>
{
    type Output = B::RowsMapped<bool>;

    fn isallpass(&self) -> Self::Output
    {
        let a = self.a.to_vec_option()
            .map(|mut a| {
                while a.first() == Some(&T::zero())
                {
                    a.remove(0);
                }
                if let Some(norm) = a.first().map(|&a| a)
                {
                    a.div_assign_all(norm)
                }
                a.conj_assign_all();
                a.reverse();
                a
            })
            .unwrap_or_else(|| vec![T::one()]);

        self.b.map_rows_to_owned(|b| {
            let b = b.to_vec_option()
                .map(|mut b| {
                    while b.first() == Some(&T::zero())
                    {
                        b.remove(0);
                    }
                    if let Some(norm) = b.last().map(|&b| b)
                    {
                        b.div_assign_all(norm)
                    }
                    b
                })
                .unwrap_or_else(|| vec![T::one()]);

            b == a || b.into_iter()
                .zip(a.iter())
                .all(|(b, a)| b == -*a)
        })
    }
}