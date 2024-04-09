use core::ops::{AddAssign, DivAssign, MulAssign};

use ndarray::{Array1, Array2};
use num::{complex::ComplexFloat, traits::Euclid, Zero, Float};

use crate::{ListOrSingle, MaybeList, MaybeLists, OwnedLists, Polynomial, Ss, System, Tf};

pub trait Normalize: System
{
    type Output: System<Domain = Self::Domain>;

    fn normalize(self) -> Self::Output;
}

impl<T, B, A, B2> Normalize for Tf<T, B, A>
where
    T: ComplexFloat + AddAssign + MulAssign + DivAssign,
    B: MaybeLists<T, RowsMapped<Vec<T>> = B2>,
    A: MaybeList<T>,
    B2: OwnedLists<T, RowOwned = Vec<T>, RowsMapped<Vec<T>> = B2> + Clone,
    Polynomial<T, B>: Into<Polynomial<T, B2>>,
    Polynomial<T, A>: Into<Polynomial<T, Vec<T>>>,
    Polynomial<T, Vec<T>>: Euclid
{
    type Output = Tf<T, B::RowsMapped<Vec<T>>, Vec<T>>;

    fn normalize(self) -> Self::Output
    {
        let Tf::<T, B::RowsMapped<Vec<T>>, Vec<T>> {mut b, mut a} = Tf {
            b: self.b.into(),
            a: self.a.into()
        };

        let gcd: Vec<Polynomial<T, Vec<T>>> = b.clone()
            .gcd::<&[T]>(a.as_view())
            .to_vec();
        if let Some(gcd) = gcd.into_iter()
            .reduce(|a, b| a.gcd::<Vec<T>>(b))
        {
            b = Polynomial::new(b.into_inner()
                .map_rows_into_owned(|b| {
                    (Polynomial::new(b)/gcd.clone()).into_inner()
                }));
            a = a/gcd;
        }

        // Trim zeros
        b = Polynomial::new(b.into_inner()
            .map_rows_into_owned(|mut b| {
                while b.first().is_some_and(|x| x.abs() < T::Real::epsilon())
                {
                    b.remove(0);
                }
                b
            }));
        while a.first().is_some_and(|x| x.abs() < T::Real::epsilon())
        {
            a.remove(0);
        }
    
        if let Some(&norm) = a.first()
        {
            for b in b.as_mut_slices()
            {
                for b in b.iter_mut()
                {
                    *b /= norm
                }
            }
            for a in a.iter_mut()
            {
                *a /= norm
            }
        }

        Tf {
            b,
            a
        }
    }
}

impl<T> Normalize for Ss<T, Array2<T>, Array2<T>, Array2<T>, Array2<T>>
where
    T: ComplexFloat
{
    type Output = Ss<T, Array2<T>, Array2<T>, Array2<T>, Array2<T>>;

    fn normalize(mut self) -> Self::Output
    {
        let (ma, na) = self.a.dim();
        let (mb, nb) = self.b.dim();
        let (mc, nc) = self.c.dim();
        let (md, nd) = self.d.dim();

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

        resize(&mut self.a, (p, p));
        resize(&mut self.b, (p, q));
        resize(&mut self.c, (r, p));
        resize(&mut self.d, (r, q));

        self
    }
}