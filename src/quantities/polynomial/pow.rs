use core::ops::{BitAnd, Mul, Shr};

use num::{pow::Pow, Integer, One, Unsigned};

use crate::{MaybeList, Polynomial};

impl<T, S, I> Pow<I> for Polynomial<T, S>
where
    T: Clone,
    S: MaybeList<T>,
    I: Unsigned + Integer + BitAnd<I, Output = I> + Shr<usize, Output = I> + Copy,
    Self: Into<Polynomial<T, Vec<T>>>,
    Polynomial<T, Vec<T>>: One + for<'b> Mul<Polynomial<T, &'b [T]>, Output = Polynomial<T, Vec<T>>>,
    for<'a, 'b> Polynomial<T, &'a [T]>: Mul<Polynomial<T, &'b [T]>, Output = Polynomial<T, Vec<T>>>
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
            One::one()
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