use core::{iter::Product, ops::Mul};

use ndarray::{Array1, Array2};
use ndarray_linalg::EigVals;
use num::{complex::ComplexFloat, Complex, One, Zero};

use option_trait::{Maybe, MaybeOr, NotVoid, StaticMaybe};

use crate::{quantities::{List, MaybeContainer, MaybeList, MaybeLists, MaybeOwnedList, Polynomial}, operations::Simplify, decompositions::SplitNumerDenom, systems::{Sos, Ss, SsAMatrix, SsBMatrix, SsCMatrix, SsDMatrix, Tf, Zpk}, transforms::system::{ToSos, ToSs, ToZpk}, System};

pub trait ToTf<T, B, A, I, O>: System
where
    T: ComplexFloat,
    B: MaybeLists<T>,
    A: MaybeList<T>,
    I: Maybe<usize>,
    O: Maybe<usize>
{
    fn to_tf(self, input: I, output: O) -> Tf<T, B, A>;
}

impl<'a, T1, B1, A1, T2, B2, A2> ToTf<T2, B2, A2, (), ()> for Tf<T1, B1, A1>
where
    T1: ComplexFloat,
    T2: ComplexFloat,
    B1: MaybeLists<T1>,
    A1: MaybeList<T1>,
    B2: MaybeLists<T2>,
    A2: MaybeList<T2>,
    Polynomial<T1, B1>: Into<Polynomial<T2, B2>>,
    Polynomial<T1, A1>: Into<Polynomial<T2, A2>>
{
    fn to_tf(self, (): (), (): ()) -> Tf<T2, B2, A2>
    {
        Tf {
            b: self.b.into(),
            a: self.a.into()
        }
    }
}
impl<T1, B1, A1, T2, B2, A2> ToTf<T2, B2, A2, (), usize> for Tf<T1, B1, A1>
where
    T1: ComplexFloat + 'static,
    T2: ComplexFloat,
    B1: MaybeLists<T1>,
    A1: MaybeList<T1>,
    B2: MaybeLists<T2>,
    A2: MaybeList<T2>,
    for<'a> Polynomial<T1, B1::RowView<'a>>: Into<Polynomial<T2, B2>>,
    Polynomial<T1, A1>: Into<Polynomial<T2, A2>>,
{
    fn to_tf(self, (): (), output: usize) -> Tf<T2, B2, A2>
    {
        Tf {
            b: Polynomial::new(self.b.index_view(output)).into(),
            a: self.a.into()
        }
    }
}

impl<'a, T1, T2, Z, Z2, P, P2, K, BB, B, A> ToTf<T2, B, A, (), ()> for Zpk<T1, Z, P, K>
where
    T1: ComplexFloat,
    T2: ComplexFloat<Real = T1::Real> + 'static,
    Polynomial<Complex<T2::Real>, Vec<Complex<T2::Real>>>: One,
    Polynomial<T2, Vec<T2>>: Mul<Polynomial<T2, [T2; 1]>, Output = Polynomial<T2, Vec<T2>>>,
    T1::Real: Into<T2> + 'static,
    K: ComplexFloat<Real = T1::Real>,
    Z: MaybeList<T1, MaybeSome: StaticMaybe<Z::Some, Maybe<Vec<Complex<T1::Real>>> = Z2>>,
    P: MaybeList<T1, MaybeSome: StaticMaybe<P::Some, Maybe<Vec<Complex<T1::Real>>> = P2>>,
    B: MaybeList<T2>,
    A: MaybeList<T2> + StaticMaybe<Vec<T2>>,
    Z2: MaybeList<Complex<T1::Real>, MaybeSome: StaticMaybe<Z2::Some, Maybe<Vec<T2>> = BB>> + Maybe<Vec<Complex<T2::Real>>>,
    P2: MaybeList<Complex<T1::Real>, MaybeSome: StaticMaybe<P2::Some, Maybe<Vec<T2>>: MaybeOr<Vec<T2>, A, Output = A>>> + Maybe<Vec<Complex<T2::Real>>>,
    BB: MaybeList<T2, Some: List<T2> + Sized, MaybeSome: Sized> + StaticMaybe<Vec<T2>>,
    Polynomial<T2, <BB as MaybeContainer<T2>>::Some>: Into<Polynomial<T2, B>>,
    Polynomial<T2, [T2; 1]>: Into<Polynomial<T2, B>>,
    Self: ToZpk<Complex<T2::Real>, Z2, P2, T2, (), ()>
{
    fn to_tf(self, (): (), (): ()) -> Tf<T2, B, A>
    {
        let Zpk::<Complex<T2::Real>, Z2, P2, T2> {z, p, k} = self.to_zpk((), ());

        let z_op: Option<Vec<_>> = z.into_inner().into_option();
        let b = if let Some(z) = z_op
        {
            Some(z.into_iter()
                .map(|z| Polynomial::new([One::one(), -z]))
                .product::<Polynomial<Complex<T2::Real>, Vec<Complex<T2::Real>>>>()
                .truncate_im::<T2>()
                *Polynomial::new([k]))
        }
        else
        {
            None
        };
        let p_op: Option<Vec<_>> = p.into_inner().into_option();
        let a = if let Some(p) = p_op
        {
            Some(p.into_iter()
                .map(|p| Polynomial::new([One::one(), -p]))
                .product::<Polynomial<Complex<T2::Real>, Vec<Complex<T2::Real>>>>()
                .truncate_im::<T2>())
        }
        else
        {
            None
        };
        Tf {
            b: b.map(|b| {
                let b = BB::maybe_from_fn(|| b.into_inner());
                b.into_maybe_some()
                    .into_option()
                    .map(|b| Polynomial::new(b).into())
                    .unwrap_or_else(|| Polynomial::new([k]).into())
            }).unwrap_or_else(|| Polynomial::new([k]).into()),
            a: Polynomial::new(StaticMaybe::maybe_from_fn(|| a.unwrap().into_inner()))
        }
    }
}

impl<T1, T2, A, B, C, D> ToTf<T2, Vec<Vec<T2>>, Vec<T2>, usize, ()> for Ss<T1, A, B, C, D>
where
    T1: ComplexFloat + Into<T2> + 'static,
    T2: ComplexFloat + 'static,
    Array2<T1>: EigVals<EigVal = Array1<Complex<T1::Real>>>,
    Polynomial<Complex<T1::Real>, Vec<Complex<T1::Real>>>: One,
    Polynomial<T1, Vec<T1>>: One,
    T1::Real: Into<T1>,
    A: SsAMatrix<T1, B, C, D>,
    B: SsBMatrix<T1, A, C, D>,
    C: SsCMatrix<T1, A, B, D>,
    D: SsDMatrix<T1, A, B, C>,
    Self: Simplify<Output: ToSs<T1, Array2<T1>, Array2<T1>, Array2<T1>, Array2<T1>>>,
    Array2<T1>: SsAMatrix<T1, Array2<T1>, Array2<T1>, Array2<T1>> + SsBMatrix<T1, Array2<T1>, Array2<T1>, Array2<T1>> + SsCMatrix<T1, Array2<T1>, Array2<T1>, Array2<T1>>+ SsDMatrix<T1, Array2<T1>, Array2<T1>, Array2<T1>>
{
    fn to_tf(self, input: usize, (): ()) -> Tf<T2, Vec<Vec<T2>>, Vec<T2>>
    {
        let ss = self.simplify().to_ss().unwrap();

        let (nout, _nin) = ss.d.dim();

        let b = ss.b.column(input);
        let d = ss.d.column(input);

        fn poly<T>(m: &Array2<T>) -> Polynomial<T, Vec<T>>
        where
            T: ComplexFloat + 'static,
            Array2<T>: EigVals<EigVal = Array1<Complex<T::Real>>>,
            Polynomial<Complex<T::Real>, Vec<Complex<T::Real>>>: One,
            Polynomial<T, Vec<T>>: One,
            T::Real: Into<T>
        {
            let z = m.eigvals();
            let z = if let Ok(z) = z
            {
                z.to_vec()
            }
            else
            {
                return Polynomial::one()
            };
            if z.is_empty()
            {
                return Polynomial::one()
            }
            
            z.into_iter()
                .map(|z| Polynomial::new([One::one(), -z]))
                .product::<Polynomial<Complex<T::Real>, Vec<Complex<T::Real>>>>()
                .truncate_im()
        }

        let mut den = poly(&ss.a).map_into_owned(|den| den.into());

        if b.shape().iter().any(Zero::is_zero) && ss.c.shape().iter().any(Zero::is_zero)
        {
            let num = d.to_vec();
            if d.shape().iter().any(Zero::is_zero) && ss.a.shape().iter().any(Zero::is_zero)
            {
                den = Polynomial::new(vec![])
            }
            return Tf {
                b: Polynomial::new(vec![num.into_iter().map(Into::into).collect()]),
                a: den
            }
        }

        let num = (0..nout)
            .map(|k| {
                let ck = ss.c.row(k);
                let bck = b.dot(&ck);
                let mut p = poly(&Array2::from_shape_fn(
                    ss.a.dim(),
                    |i| ss.a[i] - if i.0 == i.1 {bck} else {T1::one()}
                ));
                let dkm1 = d[k] - T1::one();
                let mut q = den.iter()
                    .map(|&den| den*dkm1.into())
                    .collect::<Vec<_>>();
                let n = p.len().max(q.len());
                p.resize(n, T1::zero());
                q.resize(n, T2::zero());
                p.into_inner()
                    .into_iter()
                    .zip(q)
                    .map(|(p, q)| p.into() + q)
                    .collect()
            }).collect();
        
        Tf {
            b: Polynomial::new(num),
            a: den
        }
    }
}

impl<T1, T2, B, A, S, BS, AS> ToTf<T2, B::Maybe<Vec<T2>>, A::Maybe<Vec<T2>>, (), ()> for Sos<T1, B, A, S>
where
    T1: ComplexFloat + Into<T2>,
    T2: ComplexFloat + 'static,
    B: StaticMaybe<[T1; 3]> + MaybeOwnedList<T1>,
    A: StaticMaybe<[T1; 3]> + MaybeOwnedList<T1>,
    B::MaybeMapped<T2>: StaticMaybe<[T2; 3]> + MaybeOwnedList<T2>,
    A::MaybeMapped<T2>: StaticMaybe<[T2; 3]> + MaybeOwnedList<T2>,
    Self: ToSos<T2, B::MaybeMapped<T2>, A::MaybeMapped<T2>, Vec<Tf<T2, B::MaybeMapped<T2>, A::MaybeMapped<T2>>>, (), ()>,
    Polynomial<T2, B::Maybe<Vec<T2>>>: Product<Polynomial<T2, B::MaybeMapped<T2>>> + One,
    Polynomial<T2, A::Maybe<Vec<T2>>>: Product<Polynomial<T2, A::MaybeMapped<T2>>> + One,
    S: MaybeList<Tf<T1, B, A>>,
    B::Maybe<Vec<T2>>: MaybeOwnedList<T2>,
    A::Maybe<Vec<T2>>: MaybeOwnedList<T2>,
    Vec<T1>: NotVoid,
    Vec<T2>: NotVoid,
    Tf<T2, B::Maybe<Vec<T2>>, A::Maybe<Vec<T2>>>: Simplify<Output = Tf<T2, B::Maybe<Vec<T2>>, A::Maybe<Vec<T2>>>>,
    Sos<T2, B::MaybeMapped<T2>, A::MaybeMapped<T2>, Vec<Tf<T2, B::MaybeMapped<T2>, A::MaybeMapped<T2>>>>: SplitNumerDenom<OutputNum = Sos<T2, B::MaybeMapped<T2>, (), BS>, OutputDen = Sos<T2, (), A::MaybeMapped<T2>, AS>>,
    BS: Maybe<Vec<Tf<T2, B::MaybeMapped<T2>, ()>>> + MaybeList<Tf<T2, B::MaybeMapped<T2>, ()>>,
    AS: Maybe<Vec<Tf<T2, (), A::MaybeMapped<T2>>>> + MaybeList<Tf<T2, (), A::MaybeMapped<T2>>>
{
    fn to_tf(self, (): (), (): ()) -> Tf<T2, B::Maybe<Vec<T2>>, A::Maybe<Vec<T2>>>
    {
        let sos: Sos<T2, B::MaybeMapped<T2>, A::MaybeMapped<T2>, Vec<_>> = self.to_sos((), ());
        
        let (b, a) = sos.split_numer_denom();

        let b_op: Option<Vec<Tf<T2, B::MaybeMapped<T2>, ()>>> = b.sos.into_inner()
            .into_option();
        let b = if let Some(b) = b_op
        {
            b.into_iter()
                .map(|sos| sos.b)
                .product::<Polynomial<T2, B::Maybe<Vec<T2>>>>()
        }
        else
        {
            One::one()
        };
        let a_op: Option<Vec<Tf<T2, (), A::MaybeMapped<T2>>>> = a.sos.into_inner()
            .into_option();
        let a = if let Some(a) = a_op
        {
            a.into_iter()
                .map(|sos| sos.a)
                .product::<Polynomial<T2, A::Maybe<Vec<T2>>>>()
        }
        else
        {
            One::one()
        };
        Tf {
            b,
            a
        }.simplify()
    }
}