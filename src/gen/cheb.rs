use core::ops::{Add, AddAssign, Mul, Neg, Sub};

use array_math::{ArrayMath};
use num::{Num, One, Zero};
use option_trait::Maybe;

use crate::{List, Polynomial};

pub trait Cheb<T, N>: List<T> + Sized
where
    T: Num,
    N: Maybe<usize>
{
    type Output: Maybe<Self>;

    fn cheb(kind: usize, order: N) -> Self::Output;
}

impl<T> Cheb<T, usize> for Vec<T>
where
    T: Num + Copy + Add<Output = T> + Sub<Output = T> + Neg<Output = T> + AddAssign + Mul<Output = T> + One + Zero
{
    type Output = Self;

    fn cheb(kind: usize, order: usize) -> Self
    {
        let zero = T::zero();
        let one = T::one();
        let two = one + one;
        let mut t_prev = Polynomial::new(vec![one]);
        if order == 0
        {
            return t_prev.into_inner()
        }
        
        let mut kind_c = zero;
        let mut k = 0;
        while k < kind
        {
            kind_c += one;
            k += 1;
        }
    
        let mut t = Polynomial::new(vec![kind_c, zero]);
    
        let mut k = 1;
        while k < order
        {
            let mut t2 = t.as_view()*Polynomial::new([two]);
            t2.push(zero);
            let t_next = t2 - t_prev;
    
            t_prev = t;
            t = t_next;
            k += 1;
        }
    
        t.into_inner()
    }
}

impl<T, const N: usize> Cheb<T, ()> for [T; N]
where
    T: Num + Copy + Add<Output = T> + Sub<Output = T> + Neg<Output = T> + AddAssign + Mul<Output = T> + One + Zero
{
    type Output = Self;

    fn cheb(kind: usize, (): ()) -> Self
    {
        Self::cheb(kind, N.saturating_sub(1))
            .unwrap()
    }
}

impl<T, const N: usize> Cheb<T, usize> for [T; N]
where
    T: Num + Copy + Add<Output = T> + Sub<Output = T> + Neg<Output = T> + AddAssign + Mul<Output = T> + One + Zero
{
    type Output = Option<Self>;

    fn cheb(kind: usize, order: usize) -> Option<Self>
    {
        Self::chebyshev_polynomial(kind, order)
            .map(|mut c| {
                c.reverse();
                c
            })
    }
}

#[cfg(test)]
mod test
{
    use crate::Cheb;

    #[test]
    fn test()
    {
        let c1 = Vec::<f32>::cheb(1, 3);
        let c2 = <[f32; 4]>::cheb(1, ());

        println!("{:?}", c1);
        println!("{:?}", c2);
    }
}