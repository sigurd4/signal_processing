use core::ops::{BitAnd, Shr};

use num::{pow::Pow, Integer, Unsigned};

use crate::quantities::{MaybeList, ProductSequence};

impl<T, S, I> Pow<I> for ProductSequence<T, S>
where
    T: Clone,
    S: MaybeList<T>,
    I: Unsigned + Integer + BitAnd<I, Output = I> + Shr<usize, Output = I> + Copy,
    Self: Into<ProductSequence<T, Vec<T>>>
{
    type Output = ProductSequence<T, Vec<T>>;

    fn pow(self, mut n: I) -> Self::Output
    {
        let mut x = self.into();
        let mut r = if (n & I::one()) == I::one()
        {
            x.clone()
        }
        else
        {
            ProductSequence::one()
        };
    
        loop
        {
            n = n >> 1usize;
            if n == I::zero()
            {
                break;
            }
            x = x.as_view()*x.as_view();
            if (n & I::one()) == I::one()
            {
                r = r*x.as_view();
            }
        }
    
        r
    }
}