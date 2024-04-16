use array_math::SliceOps;
use num::{complex::ComplexFloat, Float};
use option_trait::Maybe;

use crate::{ListOrSingle, MaybeList, MaybeLists, Sos, System, Tf, ToTf, Zpk};

pub trait IsLinPhase<'a>: System
{
    type Output: ListOrSingle<bool>;

    fn is_linphase<TOL>(&'a self, tol: TOL, generalized: bool) -> Self::Output
    where
        TOL: Maybe<<Self::Domain as ComplexFloat>::Real>;
}

impl<'a, T, B, A> IsLinPhase<'a> for Tf<T, B, A>
where
    T: ComplexFloat + 'a,
    B: MaybeLists<T> + 'a,
    A: MaybeList<T> + 'a,
    A::View<'a>: MaybeList<T> + Clone
{
    type Output = B::RowsMapped<bool>;

    fn is_linphase<TOL>(&'a self, tol: TOL, generalized: bool) -> Self::Output
    where
        TOL: Maybe<T::Real>
    {
        let tol = tol.into_option()
            .map(|tol| Float::abs(tol))
            .unwrap_or(T::Real::epsilon());

        let a = self.a.as_view_slice_option()
            .map(|a| a.trim_front(|a| a.abs() < tol));
        if let Some(a) = a.map(|a| a.trim_back(|a| a.abs() < tol))
        {
            if a.len() > 0
            {
                for (&a1, &a2) in a[..a.len().div_ceil(2).saturating_sub(1)].iter()
                    .zip(a[a.len()/2 + 1..].iter().rev())
                {
                    if !((a1 - a2).abs() <= tol) && !((a1 + a2).abs() <= tol)
                    {
                        return self.b.map_rows_to_owned(|_| false)
                    }
                }
            }
        }

        self.b.map_rows_to_owned(|b| {
            let b = b.as_view_slice_option()
                .map(|b| b.trim_front(|b| b.abs() < tol));
            if !generalized && b.map(|b| b.len()).unwrap_or(1) != a.map(|a| a.len()).unwrap_or(1)
            {
                false
            }
            else if let Some(b) = b.map(|b| b.trim_back(|b| b.abs() < tol))
            {
                let mut is_symmetric = true;
                if b.len() > 0
                {
                    for (&b1, &b2) in b[..b.len().div_ceil(2).saturating_sub(1)].iter()
                        .zip(b[b.len()/2 + 1..].iter().rev())
                    {
                        if !((b1 - b2).abs() <= tol) && !((b1 + b2).abs() <= tol)
                        {
                            is_symmetric = false;
                            break;
                        }
                    }
                }
                is_symmetric
            }
            else
            {
                true
            }
        })
    }
}

impl<'a, T, B, A, S> IsLinPhase<'a> for Sos<T, B, A, S>
where
    T: ComplexFloat,
    B: Maybe<[T; 3]> + MaybeList<T>,
    A: Maybe<[T; 3]> + MaybeList<T>,
    S: MaybeList<Tf<T, B, A>> + 'a,
    S::View<'a>: MaybeList<Tf<T, B, A>>,
    Sos<T, B, A, S::View<'a>>: ToTf<T, Vec<T>, Vec<T>, (), ()>
{
    type Output = bool;

    fn is_linphase<TOL>(&'a self, tol: TOL, generalized: bool) -> Self::Output
    where
        TOL: Maybe<T::Real>
    {
        if !generalized
        {
            return self.as_view()
                .to_tf((), ())
                .is_linphase(tol, generalized);
        }
        
        let tol = tol.into_option()
            .map(|tol| Float::abs(tol))
            .unwrap_or_else(T::Real::epsilon);

        if let Some(sos) = self.sos.as_view_slice_option()
        {
            for sos in sos.iter()
            {
                if let Some(a) = sos.a.as_view_slice_option()
                    .map(|a| a.trim(|a| a.abs() < tol))
                {
                    if a.len() > 0
                    {
                        for (&a1, &a2) in a[..a.len().div_ceil(2).saturating_sub(1)].iter()
                            .zip(a[a.len()/2 + 1..].iter().rev())
                        {
                            if !((a1 - a2).abs() <= tol) && !((a1 + a2).abs() <= tol)
                            {
                                return false
                            }
                        }
                    }
                }
                if let Some(b) = sos.b.as_view_slice_option()
                    .map(|b| b.trim(|b| b.abs() < tol))
                {
                    if b.len() > 0
                    {
                        for (&b1, &b2) in b[..b.len().div_ceil(2).saturating_sub(1)].iter()
                            .zip(b[b.len()/2 + 1..].iter().rev())
                        {
                            if !((b1 - b2).abs() <= tol) && !((b1 + b2).abs() <= tol)
                            {
                                return false
                            }
                        }
                    }
                }
            }
        }
        true
    }
}

impl<'a, T, Z, P, K> IsLinPhase<'a> for  Zpk<T, Z, P, K>
where
    T: ComplexFloat,
    Z: MaybeList<T>,
    P: MaybeList<T>,
    K: ComplexFloat<Real = T::Real>
{
    type Output = bool;

    fn is_linphase<TOL>(&'a self, tol: TOL, generalized: bool) -> Self::Output
    where
        TOL: Maybe<T::Real>
    {
        let tol = tol.into_option()
            .map(|tol| Float::abs(tol))
            .unwrap_or_else(T::Real::epsilon);

        // Check if all nonzero and noninfinite poles and zeros away from the unit circle come in reciprical pairs
        
        let mut z = self.z.to_vec_option()
            .unwrap_or_else(|| vec![]);
        let mut p = self.p.to_vec_option()
            .unwrap_or_else(|| vec![]);

        'lp:
        while let Some(zk) = z.pop()
        {
            let zk_recip = zk.recip();
            if (generalized && (zk.abs() <= tol || zk_recip.abs() <= tol)) || (zk - zk_recip).abs() <= tol
            {
                continue 'lp
            }
            let mut i = 0;
            while i < z.len()
            {
                if (z[i] - zk_recip).abs() <= tol
                {
                    z.remove(i);
                    continue 'lp
                }
                else
                {
                    i += 1
                }
            }
            return false
        }
        'lp:
        while let Some(pk) = p.pop()
        {
            let pk_recip = pk.recip();
            if (generalized && (pk.abs() <= tol || pk_recip.abs() <= tol)) || (pk - pk_recip).abs() <= tol
            {
                continue 'lp
            }
            let mut i = 0;
            while i < p.len()
            {
                if (p[i] - pk_recip).abs() <= tol
                {
                    p.remove(i);
                    continue 'lp
                }
                else
                {
                    i += 1
                }
            }
            return false
        }
        true
    }
}

#[cfg(test)]
mod test
{
    use crate::{IsLinPhase, Tf};

    #[test]
    fn test()
    {
        let h = Tf::new([1.0, 2.0, 1.0], [1.0]);

        let lg = h.is_linphase((), true);
        let l = h.is_linphase((), false);

        println!("{}, {}", lg, l)
    }
}