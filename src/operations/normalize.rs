use core::ops::{AddAssign, DerefMut, DivAssign, MulAssign};

use ndarray::{Array1, Array2};
use num::{complex::ComplexFloat, traits::Euclid, Zero, Float};
use option_trait::{Maybe, MaybeOr, NotVoid, StaticMaybe};

use crate::{ListOrSingle, MaybeList, MaybeLists, OwnedLists, Polynomial, ProductSequence, Ss, SsAMatrix, SsBMatrix, SsCMatrix, SsDMatrix, System, Tf, Zpk};

pub trait Normalize: System
{
    type Output: System<Domain = Self::Domain>;

    fn normalize(self) -> Self::Output;
}

impl<T, B, A, B2, BB2, BB, AA> Normalize for Tf<T, B, A>
where
    Vec<T>: NotVoid,
    T: ComplexFloat + AddAssign + MulAssign + DivAssign,
    B: MaybeLists<T, RowsMapped<Vec<T>> = B2>,
    B::MaybeSome: StaticMaybe<B::Some, Maybe<Vec<T>> = BB>,
    A::MaybeSome: StaticMaybe<A::Some, Maybe<Vec<T>> = AA>,
    A: MaybeList<T>,
    B2: OwnedLists<T, RowOwned = Vec<T>, RowsMapped<Vec<T>> = B2> + Clone,
    B2::MaybeSome: StaticMaybe<B2::Some, Maybe<B2>: MaybeOr<B2, B::RowsMapped<BB>, Output = BB2>>,
    Polynomial<T, B>: Into<Polynomial<T, BB2>>,
    Polynomial<T, A>: Into<Polynomial<T, AA>>,
    Polynomial<T, Vec<T>>: Euclid,
    BB: MaybeList<T> + Maybe<Vec<T>>,
    AA: MaybeList<T> + Maybe<Vec<T>> + Clone,
    B::RowsMapped<BB>: MaybeLists<T> + StaticMaybe<B2>,
    BB2: MaybeLists<T> + Maybe<B2> + Clone,
    Polynomial<T, AA>: Into<Polynomial<T, Vec<T>>>,
    Polynomial<T, BB2::RowOwned>: Into<Polynomial<T, Vec<T>>>
{
    type Output = Tf<T, BB2, AA>;

    fn normalize(self) -> Self::Output
    {
        let Tf::<T, BB2, AA> {mut b, mut a} = Tf {
            b: self.b.into(),
            a: self.a.into()
        };

        let mut b_op: Option<&mut B2> = b.deref_mut().as_option_mut();
        let mut a_op: Option<&mut Vec<T>> = a.deref_mut().as_option_mut();

        if let Some(b) = &mut b_op && let Some(a) = &mut a_op
        {
            let gcd: Vec<Polynomial<T, Vec<T>>> = Polynomial::new(b.clone())
                .gcd(Polynomial::new(a.clone()))
                .to_vec();
            if let Some(gcd) = gcd.into_iter()
                .reduce(|a, b| a.gcd::<Vec<T>>(b))
            {
                **b = b.clone().map_rows_into_owned(|b| {
                        (Polynomial::new(b)/gcd.clone()).into_inner()
                    });
                **a = (Polynomial::new(a.clone())/gcd).into_inner();
            }
        }

        // Trim zeros
        if let Some(b) = &mut b_op
        {
            **b = b.clone()
                .map_rows_into_owned(|mut b| {
                    while b.first().is_some_and(|x| x.abs() < T::Real::epsilon())
                    {
                        b.remove(0);
                    }
                    b
                });
        }
        if let Some(a) = &mut a_op
        {
            while a.first().is_some_and(|x| x.abs() < T::Real::epsilon())
            {
                a.remove(0);
            }
        }
    
        if let Some(b) = &mut b_op && let Some(a) = &mut a_op
        {
            if let Some(&norm) = a.first()
            {
                for b in b.as_mut_slices()
                {
                    for b in b.iter_mut()
                    {
                        *b /= norm
                    }
                }
                for a in a.as_mut_slice()
                    .iter_mut()
                {
                    *a /= norm
                }
            }
        }

        Tf {
            b,
            a
        }
    }
}

impl<T, Z, P, Z2, P2, K> Normalize for Zpk<T, Z, P, K>
where
    T: ComplexFloat,
    Z: MaybeList<T, MaybeSome: StaticMaybe<Z::Some, Maybe<Vec<T>> = Z2>>,
    P: MaybeList<T, MaybeSome: StaticMaybe<Z::Some, Maybe<Vec<T>> = P2>>,
    K: ComplexFloat<Real = T::Real>,
    ProductSequence<T, Z>: Into<ProductSequence<T, Z2>>,
    ProductSequence<T, P>: Into<ProductSequence<T, P2>>,
    Z2: MaybeList<T> + Maybe<Vec<T>>,
    P2: MaybeList<T> + Maybe<Vec<T>>,
{
    type Output = Zpk<T, Z2, P2, K>;

    fn normalize(self) -> Self::Output
    {
        let Zpk::<T, Z2, P2, K> {mut z, mut p, k} = Zpk {
            z: self.z.into(),
            p: self.p.into(),
            k: self.k
        };

        let z_op: Option<&mut Vec<_>> = z.deref_mut().as_option_mut();
        let p_op: Option<&mut Vec<_>> = p.deref_mut().as_option_mut();

        if let Some(z) = z_op && let Some(p) = p_op
        {
            let mut i = 0;
            'lp:
            while i < z.len()
            {
                let mut j = 0;
                while j < p.len()
                {
                    if (z[i] - p[j]).abs() < T::Real::epsilon()
                    {
                        z.remove(i);
                        p.remove(j);
                        continue 'lp;
                    }
                    else
                    {
                        j += 1;
                    }
                }
                i += 1;
            }
        }

        Zpk {
            z,
            p,
            k
        }
    }
}


impl<T, A, B, C, D> Normalize for Ss<T, A, B, C, D>
where
    T: ComplexFloat,
    A: SsAMatrix<T, B, C, D>,
    B: SsBMatrix<T, A, C, D>,
    C: SsCMatrix<T, A, B, D>,
    D: SsDMatrix<T, A, B, C>,
    Array2<T>: SsAMatrix<T, Array2<T>, Array2<T>, Array2<T>> + SsBMatrix<T, Array2<T>, Array2<T>, Array2<T>> + SsCMatrix<T, Array2<T>, Array2<T>, Array2<T>>+ SsDMatrix<T, Array2<T>, Array2<T>, Array2<T>>
{
    type Output = Ss<T, Array2<T>, Array2<T>, Array2<T>, Array2<T>>;

    fn normalize(self) -> Self::Output
    {
        let Ss {mut a, mut b, mut c, mut d, ..} = Ss::new(
            self.a.to_array2(),
            self.b.to_array2(),
            self.c.to_array2(),
            self.d.to_array2()
        );

        let (ma, na) = a.dim();
        let (mb, nb) = b.dim();
        let (mc, nc) = c.dim();
        let (md, nd) = d.dim();

        let p = ma.max(na).max(mb).max(nc);
        let q = nb.max(nd);
        let r = mc.max(md);

        fn resize<T>(m: &mut Array2<T>, dim: (usize, usize))
        where
            T: Zero + Clone
        {
            let row = Array1::from_elem(m.dim().1, T::zero());
            while m.dim().0 < dim.0
            {
                m.push_row(row.view()).unwrap();
            }
            let col = Array1::from_elem(m.dim().0, T::zero());
            while m.dim().1 < dim.1
            {
                m.push_column(col.view()).unwrap();
            }
        }

        resize(&mut a, (p, p));
        resize(&mut b, (p, q));
        resize(&mut c, (r, p));
        resize(&mut d, (r, q));

        Ss::new(a, b, c, d)
    }
}