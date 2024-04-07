use core::ops::{BitAnd, Shr};

use num::{complex::ComplexFloat, pow::Pow, traits::Inv, Integer, One};
use option_trait::Maybe;

use crate::{MaybeList, Sos, Tf, ToSos};

impl<T, B, A, S, I> Pow<I> for Sos<T, B, A, S>
where
    T: ComplexFloat,
    B: Maybe<[T; 3]> + MaybeList<T>,
    A: Maybe<[T; 3]> + MaybeList<T>,
    S: MaybeList<Tf<T, B, A>>,
    I: Integer + BitAnd<I, Output = I> + Shr<usize, Output = I> + Copy,
    Self: ToSos<T, [T; 3], [T; 3], Vec<Tf<T, [T; 3], [T; 3]>>, (), ()> + Inv<Output: ToSos<T, [T; 3], [T; 3], Vec<Tf<T, [T; 3], [T; 3]>>, (), ()>> + One,
{
    type Output = Sos<T, [T; 3], [T; 3], Vec<Tf<T, [T; 3], [T; 3]>>>;

    fn pow(self, mut n: I) -> Self::Output
    {
        let mut x: Sos<T, [T; 3], [T; 3], Vec<Tf<T, [T; 3], [T; 3]>>> = if n < I::zero()
        {
            self.inv()
                .to_sos((), ())
        }
        else
        {
            self.to_sos((), ())
        };
        let mut r = if (n & I::one()) == I::one()
        {
            x.clone()
        }
        else
        {
            Sos::one()
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