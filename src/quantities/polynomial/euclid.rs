use core::ops::{AddAssign, Div, Mul, MulAssign, Rem, Sub};

use num::{traits::Euclid, Zero};

use array_math::SliceMath;

use crate::quantities::{MaybeList, Polynomial, Lists, NotPolynomial};

impl<T, C1, C2> Div<Polynomial<T, C2>> for Polynomial<T, C1>
where
    T: Zero + Sub<Output = T> + Mul<Output = T> + Div<Output = T> + AddAssign + MulAssign + Copy,
    C1: MaybeList<T>,
    C2: MaybeList<T>,
    Self: Into<Polynomial<T, Vec<T>>>,
    Polynomial<T, C2>: Into<Polynomial<T, Vec<T>>>
{
    type Output = Polynomial<T, Vec<T>>;

    fn div(self, rhs: Polynomial<T, C2>) -> Self::Output
    {
        self.into().div_euclid(&rhs.into())
    }
}

impl<T1, T2, T3, C> Div<T2> for Polynomial<T1, C>
where
    C: Lists<T1>,
    T2: NotPolynomial + Clone,
    T1: Div<T2, Output = T3> + Clone,
    C::Mapped<T3>: Lists<T3>
{
    type Output = Polynomial<T3, C::Mapped<T3>>;

    #[inline]
    fn div(self, rhs: T2) -> Self::Output
    {
        self.map_into_owned(|lhs| lhs/rhs.clone())
    }
}

impl<T, C1, C2> Rem<Polynomial<T, C2>> for Polynomial<T, C1>
where
    T: Zero + Sub<Output = T> + Mul<Output = T> + Div<Output = T> + AddAssign + MulAssign + Copy,
    C1: MaybeList<T>,
    C2: MaybeList<T>,
    Self: Into<Polynomial<T, Vec<T>>>,
    Polynomial<T, C2>: Into<Polynomial<T, Vec<T>>>
{
    type Output = Polynomial<T, Vec<T>>;

    fn rem(self, rhs: Polynomial<T, C2>) -> Self::Output
    {
        self.into().rem_euclid(&rhs.into())
    }
}

impl<T> Euclid for Polynomial<T, Vec<T>>
where
    T: Zero + Sub<Output = T> + Mul<Output = T> + Div<Output = T> + AddAssign + MulAssign + Copy
{
    fn div_euclid(&self, v: &Self) -> Self
    {
        self.div_rem_euclid(v).0
    }
    
    fn rem_euclid(&self, v: &Self) -> Self
    {
        self.div_rem_euclid(v).1
    }

    fn div_rem_euclid(&self, v: &Self) -> (Self, Self)
    {
        let mut q: Polynomial<T, Vec<T>> = Polynomial::zero();
        let mut r = self.clone();
        let d = v.trim_zeros_front().len() - 1;
        let c = *v.trim_zeros_front().first().unwrap();
        loop
        {
            let nr = r.trim_zeros_front().len();
            if nr <= d
            {
                while q.first().is_some_and(|x| x.is_zero())
                {
                    q.remove(0);
                }
                while r.first().is_some_and(|x| x.is_zero())
                {
                    r.remove(0);
                }
                return (q, r)
            }
            let mut s = Polynomial::new(vec![T::zero(); nr - d]);
            s[0] = *r.trim_zeros_front().first().unwrap()/c;
            q = q + s.as_view();
            r = r - s*v.as_view();
        }
    }
}