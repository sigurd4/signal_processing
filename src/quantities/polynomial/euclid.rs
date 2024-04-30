use core::ops::{AddAssign, Deref, Div, Rem, Sub};

use num::{traits::Euclid, One, Zero};

use array_math::SliceMath;

use crate::quantities::{Lists, MaybeList, NotPolynomial, Polynomial};

impl<T, C1, C2> Div<Polynomial<T, C2>> for Polynomial<T, C1>
where
    T: Zero + One + Sub<Output = T> + Div<Output = T> + AddAssign + Copy,
    C1: MaybeList<T>,
    C2: MaybeList<T>,
{
    type Output = Polynomial<T, Vec<T>>;

    fn div(self, rhs: Polynomial<T, C2>) -> Self::Output
    {
        let one = T::one();
        let (mut q, _): (Vec<_>, Vec<_>) = self.deref()
            .as_view_slice_option()
            .unwrap_or_else(|| core::slice::from_ref(&one))
            .deconvolve_direct(rhs.deref()
                .as_view_slice_option()
                .unwrap_or_else(|| core::slice::from_ref(&one))
            ).unwrap();
        while let Some(q0) = q.first() && q0.is_zero()
        {
            q.remove(0);
        }
        Polynomial::new(q)
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
    T: Zero + One + Sub<Output = T> + Div<Output = T> + AddAssign + Copy,
    C1: MaybeList<T>,
    C2: MaybeList<T>,
{
    type Output = Polynomial<T, Vec<T>>;

    #[inline]
    fn rem(self, rhs: Polynomial<T, C2>) -> Self::Output
    {
        let one = T::one();
        let (_, mut r): (Vec<_>, Vec<_>) = self.deref()
            .as_view_slice_option()
            .unwrap_or_else(|| core::slice::from_ref(&one))
            .deconvolve_direct(rhs.deref()
                .as_view_slice_option()
                .unwrap_or_else(|| core::slice::from_ref(&one))
            ).unwrap();
        while let Some(r0) = r.first() && r0.is_zero()
        {
            r.remove(0);
        }
        Polynomial::new(r)
    }
}

impl<T> Euclid for Polynomial<T, Vec<T>>
where
    T: Zero + One + Sub<Output = T> + Div<Output = T> + AddAssign + Copy,
{
    #[inline]
    fn div_euclid(&self, v: &Self) -> Self
    {
        let (mut q, _): (Vec<_>, Vec<_>) = self.deconvolve_direct(v.deref())
            .unwrap();
        while let Some(q0) = q.first() && q0.is_zero()
        {
            q.remove(0);
        }
        Polynomial::new(q)
    }
    
    #[inline]
    fn rem_euclid(&self, v: &Self) -> Self
    {
        let (_, mut r): (Vec<_>, Vec<_>) = self.deconvolve_direct(v.deref())
            .unwrap();
        while let Some(r0) = r.first() && r0.is_zero()
        {
            r.remove(0);
        }
        Polynomial::new(r)
    }

    #[inline]
    fn div_rem_euclid(&self, v: &Self) -> (Self, Self)
    {
        let (mut q, mut r): (Vec<_>, Vec<_>) = self.deconvolve_direct(v.deref())
            .unwrap();
        while let Some(q0) = q.first() && q0.is_zero()
        {
            q.remove(0);
        }
        while let Some(r0) = r.first() && r0.is_zero()
        {
            r.remove(0);
        }
        (Polynomial::new(q), Polynomial::new(r))
    }
}