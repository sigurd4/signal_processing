use core::ops::{BitAnd, Shr};

use num::{complex::ComplexFloat, pow::Pow, traits::Inv, Integer, One};

use crate::{MaybeList, ToZpk, Zpk};


impl<T, Z, P, K, I> Pow<I> for Zpk<T, Z, P, K>
where
    T: ComplexFloat,
    Z: MaybeList<T>,
    P: MaybeList<T>,
    K: ComplexFloat<Real = T::Real>,
    I: Integer + BitAnd<I, Output = I> + Shr<usize, Output = I> + Copy,
    Self: ToZpk<T, Vec<T>, Vec<T>, K, (), ()> + Inv<Output: ToZpk<T, Vec<T>, Vec<T>, K, (), ()>>
{
    type Output = Zpk<T, Vec<T>, Vec<T>, K>;

    fn pow(self, mut n: I) -> Self::Output
    {
        let mut x = if n < I::zero()
        {
            self.inv()
                .to_zpk((), ())
        }
        else
        {
            self.to_zpk((), ())
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