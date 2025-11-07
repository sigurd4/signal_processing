use core::ops::{BitAnd, Mul, Shr};

use num::{pow::Pow, Integer, One, Unsigned};

use crate::quantities::{MaybeList, Polynomial};

impl<T, S, I> Pow<I> for Polynomial<T, S>
where
    T: Clone + One,
    S: MaybeList<T>,
    I: Unsigned + Integer + BitAnd<I, Output = I> + Shr<usize, Output = I> + Copy,
    Self: Into<Polynomial<T, Vec<T>>>,
    Polynomial<T, Vec<T>>: Mul<Polynomial<T, Vec<T>>, Output = Polynomial<T, Vec<T>>>
{
    type Output = Polynomial<T, Vec<T>>;

    fn pow(self, mut n: I) -> Self::Output
    {
        let mut x = self.into();
        let mut r = if (n & I::one()) == I::one()
        {
            x.clone()
        }
        else
        {
            Polynomial::one()
        };
    
        loop
        {
            n = n >> 1usize;
            if n == I::zero()
            {
                break;
            }
            x = x.clone()*x.clone();
            if (n & I::one()) == I::one()
            {
                r = r*x.clone();
            }
        }
    
        r
    }
}