use core::ops::DivAssign;

use num::{Float, complex::ComplexFloat};
use array_math::{SliceMath, SliceOps};
use option_trait::Maybe;

use crate::{ListOrSingle, MaybeOwnedList, MaybeList, MaybeLists, Sos, System, Tf, ToTf, Zpk};

pub trait IsAllPass<'a>: System
{
    type Output: ListOrSingle<bool>;

    fn is_allpass<TOL>(&'a self, tol: TOL) -> Self::Output
    where
        TOL: Maybe<<Self::Domain as ComplexFloat>::Real>;
}

impl<'a, T, B, A> IsAllPass<'a> for Tf<T, B, A>
where
    T: ComplexFloat + DivAssign,
    B: MaybeLists<T>,
    A: MaybeList<T>
{
    type Output = B::RowsMapped<bool>;

    fn is_allpass<TOL>(&'a self, tol: TOL) -> Self::Output
    where
        TOL: Maybe<T::Real>
    {
        let tol = tol.into_option()
            .map(|tol| Float::abs(tol))
            .unwrap_or_else(T::Real::epsilon);

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

            b.len() == 0 || b.len() == a.len() && (
                b.iter()
                    .zip(a.iter())
                    .all(|(b, a)| (*b - *a).abs() <= tol)
                || b.into_iter()
                    .zip(a.iter())
                    .all(|(b, a)| (b + *a).abs() <= tol)
            )
        })
    }
}

impl<'a, T, B, A, S> IsAllPass<'a> for Sos<T, B, A, S>
where
    T: ComplexFloat,
    B: Maybe<[T; 3]> + MaybeOwnedList<T>,
    A: Maybe<[T; 3]> + MaybeOwnedList<T>,
    S: MaybeList<Tf<T, B, A>> + 'a,
    S::View<'a>: MaybeList<Tf<T, B, A>>,
    Sos<T, B, A, S::View<'a>>: ToTf<T, Vec<T>, Vec<T>, (), ()>,
    Tf<T, Vec<T>, Vec<T>>: for<'b> IsAllPass<'b, Output = bool> + System<Domain = T>
{
    type Output = bool;

    fn is_allpass<TOL>(&'a self, tol: TOL) -> Self::Output
    where
        TOL: Maybe<T::Real>
    {
        self.as_view()
            .to_tf((), ())
            .is_allpass(tol)
    }
}

impl<'a, T, Z, P, K> IsAllPass<'a> for  Zpk<T, Z, P, K>
where
    T: ComplexFloat,
    Z: MaybeList<T>,
    P: MaybeList<T>,
    K: ComplexFloat<Real = T::Real>
{
    type Output = bool;

    fn is_allpass<TOL>(&'a self, tol: TOL) -> Self::Output
    where
        TOL: Maybe<T::Real>
    {
        let tol = tol.into_option()
            .map(|tol| Float::abs(tol))
            .unwrap_or_else(T::Real::epsilon);

        let z = self.z.to_vec_option()
            .unwrap_or_else(|| vec![]);
        let mut p = self.p.to_vec_option()
            .unwrap_or_else(|| vec![]);

        'lp:
        for z in z.into_iter()
        {
            let mut i = 0;
            while i < p.len()
            {
                if (p[i].conj().recip() - z).abs() <= tol
                {
                    p.remove(i);
                    continue 'lp;
                }
                else
                {
                    i += 1
                }
            }
            return false
        }
        p.len() == 0
    }
}