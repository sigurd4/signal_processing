use num::{complex::ComplexFloat, Complex, Float, One};
use option_trait::Maybe;

use crate::{quantities::{ListOrSingle, MaybeOwnedList, MaybeList, MaybeLists, Polynomial}, System, systems::{Sos, Tf, Zpk}, transforms::system::ToZpk};

pub trait IsMaxPhase<'a>: System
{
    type Output: ListOrSingle<bool>;

    fn is_maxphase<TOL>(&'a self, tol: TOL) -> Self::Output
    where
        TOL: Maybe<<Self::Domain as ComplexFloat>::Real>;
}

impl<'a, T, B, A> IsMaxPhase<'a> for Tf<T, B, A>
where
    T: ComplexFloat + 'a,
    B: MaybeLists<T> + 'a,
    A: MaybeList<T> + 'a,
    A::View<'a>: MaybeList<T> + Clone,
    Tf<T, B::RowView<'a>, A::View<'a>>: ToZpk<Complex<T::Real>, Vec<Complex<T::Real>>, Vec<Complex<T::Real>>, T, (), ()>,
    Zpk<Complex<T::Real>, Vec<Complex<T::Real>>, Vec<Complex<T::Real>>, T>: for<'b> IsMaxPhase<'b, Output = bool> + System<Domain = T>
{
    type Output = B::RowsMapped<bool>;

    fn is_maxphase<TOL>(&'a self, tol: TOL) -> Self::Output
    where
        TOL: Maybe<T::Real>
    {
        let tol = tol.into_option();

        let a = self.a.as_view();

        self.b.map_rows_to_owned(|b| {
            let tf = Tf {
                b: Polynomial::new(b),
                a: a.clone()
            };

            tf.to_zpk((), ())
                .is_maxphase(tol)
        })
    }
}

impl<'a, T, B, A, S> IsMaxPhase<'a> for Sos<T, B, A, S>
where
    T: ComplexFloat,
    B: Maybe<[T; 3]> + MaybeOwnedList<T>,
    A: Maybe<[T; 3]> + MaybeOwnedList<T>,
    S: MaybeList<Tf<T, B, A>> + 'a,
    S::View<'a>: MaybeList<Tf<T, B, A>>,
    Sos<T, B, A, S::View<'a>>: ToZpk<Complex<T::Real>, Vec<Complex<T::Real>>, Vec<Complex<T::Real>>, T, (), ()>,
    Zpk<Complex<T::Real>, Vec<Complex<T::Real>>, Vec<Complex<T::Real>>, T>: for<'b> IsMaxPhase<'b, Output = bool> + System<Domain = T>
{
    type Output = bool;

    fn is_maxphase<TOL>(&'a self, tol: TOL) -> Self::Output
    where
        TOL: Maybe<T::Real>
    {
        self.as_view()
            .to_zpk((), ())
            .is_maxphase(tol)
    }
}

impl<'a, T, Z, P, K> IsMaxPhase<'a> for  Zpk<T, Z, P, K>
where
    T: ComplexFloat,
    Z: MaybeList<T>,
    P: MaybeList<T>,
    K: ComplexFloat<Real = T::Real>
{
    type Output = bool;

    fn is_maxphase<TOL>(&'a self, tol: TOL) -> Self::Output
    where
        TOL: Maybe<T::Real>
    {
        let tol = tol.into_option()
            .map(|tol| Float::abs(tol))
            .unwrap_or_else(T::Real::epsilon);

        let z = self.z.to_vec_option()
            .unwrap_or_else(|| vec![]);
        let p = self.p.to_vec_option()
            .unwrap_or_else(|| vec![]);

        let one = T::Real::one();

        (
            z.len() == 0 || z.into_iter()
                .all(|z| z.abs() > one + tol)
        ) && (
            p.len() == 0 || p.into_iter()
                .all(|p| p.abs() < one - tol)
        )
    }
}