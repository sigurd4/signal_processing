use core::ops::{AddAssign, BitAnd, Shr};

use num::{complex::ComplexFloat, pow::Pow, traits::Inv, Integer, One};

use crate::{MaybeList, Tf};

impl<T, B, A, I> Pow<I> for Tf<T, B, A>
where
    T: ComplexFloat + AddAssign,
    B: MaybeList<T>,
    A: MaybeList<T>,
    I: Integer + BitAnd<I, Output = I> + Shr<usize, Output = I> + Copy,
    Self: Into<Tf<T, Vec<T>, Vec<T>>> + Inv<Output: Into<Tf<T, Vec<T>, Vec<T>>>>
{
    type Output = Tf<T, Vec<T>, Vec<T>>;

    fn pow(self, mut n: I) -> Self::Output
    {
        let mut x = if n < I::zero()
        {
            self.inv().into()
        }
        else
        {
            self.into()
        };
        let mut r = if (n & I::one()) == I::one()
        {
            x.clone()
        }
        else
        {
            One::one()
        };
    
        let two = I::one() + I::one();
        loop
        {
            n = n/two;
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