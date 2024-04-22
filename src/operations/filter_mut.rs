use core::ops::{AddAssign, SubAssign};

use array_math::{SliceOps, SliceMath};
use ndarray::{Array1, Array2};
use num::{complex::ComplexFloat, Zero};
use option_trait::{Maybe, StaticMaybe};

use crate::{ComplexOp, ContainerOrSingle, List, Lists, Matrix, MaybeLenEq, MaybeList, MaybeLists, MaybeMatrix, MaybeOwnedList, Overlay, OwnedList, Rtf, RtfOrSystem, Sos, Ss, SsAMatrix, SsBMatrix, SsCMatrix, SsDMatrix, Tf};

pub trait FilterMut<X, XX>: RtfOrSystem
where
    Self::Domain: ComplexOp<X>,
    X: Into<<Self::Domain as ComplexOp<X>>::Output>,
    X: ComplexFloat<Real = <Self::Domain as ComplexFloat>::Real>,
    XX: Lists<X>
{
    type Output: Lists<<Self::Domain as ComplexOp<X>>::Output>;

    fn filter_mut(&mut self, x: XX) -> Self::Output;
}

impl<'b, W, T, B, A, X, XX> FilterMut<X, XX> for Rtf<'b, W, Tf<T, B, A>>
where
    W: ComplexFloat<Real = T::Real>,
    T: ComplexFloat + ComplexOp<X>,
    B: MaybeLists<T>,
    A: MaybeList<T>,
    X: ComplexFloat<Real = T::Real>,
    XX: List<X>,
    &'b Tf<T, B, A>: Into<Tf<T, Vec<Vec<T>>, Vec<T>>>,
    T: ComplexOp<X, Output = W>,
    W: ComplexOp<X, Output = W> + AddAssign + SubAssign,
    X: Into<W>,
    B::RowsMapped<XX::Mapped<W>>: Lists<W>
{
    type Output = B::RowsMapped<XX::Mapped<W>>;

    fn filter_mut(&mut self, x: XX) -> Self::Output
    {
        let Tf {b, a}: Tf<_, Vec<Vec<_>>, Vec<_>> = self.sys.into();

        let na = a.len();
        let nb: Vec<_> = b.iter()
            .map(|b| b.len())
            .collect();
        let nw = nb.iter()
            .map(|&nb| nb.max(na).saturating_sub(1))
            .sum();
        self.w.resize(nw, W::zero());
        let mut i = 0;
        let mut b = b.into_inner()
            .into_iter();
        let y = self.sys.b.map_rows_to_owned(|_| {
                let b = b.next().unwrap();
                let nw = a.len().max(b.len()).saturating_sub(1);
                let w = &mut self.w[i..(i + nw)];

                let y = x.map_to_owned(|&x: &X| {
                    let mut w0: W = x.into();
                    let mut a = a.trim_zeros_front()
                        .iter();
                    let a0: T = a.next()
                        .copied()
                        .unwrap_or_else(Zero::zero);
                    for (&a, &w) in a.zip(w.iter())
                    {
                        w0 -= w*(a/a0).into()
                    }
                    let mut b = b.trim_zeros_front()
                        .iter();
                    let b0 = b.next()
                        .copied()
                        .unwrap_or_else(Zero::zero);
                    let mut y: W = w0*b0.into();
                    for (&b, &w) in b.zip(w.iter())
                    {
                        y += w*b.into()
                    }
                    w.shift_right(&mut w0);

                    y
                });

                i += nw;

                y
            });
        y
    }
}

impl<'b, W, T, B, A, S, X, XX> FilterMut<X, XX> for Rtf<'b, W, Sos<T, B, A, S>>
where
    T: ComplexFloat + Into<W>,
    B: Maybe<[T; 3]> + MaybeOwnedList<T>,
    A: Maybe<[T; 3]> + MaybeOwnedList<T>,
    S: MaybeList<Tf<T, B, A>>,
    W: ComplexFloat<Real = T::Real> + ComplexOp<X, Output = W> + SubAssign + AddAssign,
    X: ComplexFloat<Real = T::Real> + Into<W>,
    XX: List<X>,
    XX::Mapped<W>: List<W, Mapped<W> = XX::Mapped<W>>,
    &'b Sos<T, B, A, S>: Into<Sos<T, [T; 3], [T; 3], Vec<Tf<T, [T; 3], [T; 3]>>>>
{
    type Output = XX::Mapped<W>;

    fn filter_mut(&mut self, x: XX) -> Self::Output
    {
        let Sos {sos}: Sos<T, [T; 3], [T; 3], Vec<Tf<T, [T; 3], [T; 3]>>>  = self.sys.into();

        let nw = 2*sos.len();
        self.w.resize(nw, W::zero());

        let mut y = x.map_into_owned(|x| {
            x.into()
        });

        let mut i = 0;
        for sos in sos.into_inner()
        {
            let w = &mut self.w[i..(i + 2)];

            y = y.map_into_owned(|x| {
                let mut w0: W = x.into();
                let mut a = sos.a.trim_zeros_front()
                    .iter();
                let a0: T = a.next().copied()
                    .unwrap_or_else(Zero::zero);
                for (&a, &w) in a.zip(w.iter())
                {
                    w0 -= w*(a/a0).into()
                }
                let mut b = sos.b.trim_zeros_front()
                    .iter();
                let b0 = b.next().copied()
                    .unwrap_or_else(Zero::zero);
                let mut y: W = w0*b0.into();
                for (&b, &w) in b.zip(w.iter())
                {
                    y += w*b.into()
                }
                w.shift_right(&mut w0);

                y
            });

            i += 2;
        }

        y
    }
}

impl<'b, W, T, A, B, C, D, DD, X, XX, XW> FilterMut<X, XX> for Rtf<'b, W, Ss<T, A, B, C, D>>
where
    T: ComplexFloat + Into<W>,
    A: SsAMatrix<T, B, C, D, Mapped<W>: Matrix<W>>,
    B: SsBMatrix<T, A, C, D, Mapped<W>: Matrix<W>>,
    C: SsCMatrix<T, A, B, D, Mapped<W>: Matrix<W>> + Matrix<T, Mapped<()>: Matrix<()>>,
    D: SsDMatrix<T, A, B, C, Mapped<W>: Matrix<W>> + Matrix<T, Mapped<()>: Matrix<(), Transpose: Matrix<(), RowOwned: Overlay<(), <<C::Mapped<()> as MaybeMatrix<()>>::Transpose as MaybeLists<()>>::RowOwned, Output = DD>>>>,
    W: ComplexFloat<Real = T::Real> + ComplexOp<X, Output = W> + SubAssign + AddAssign + 'static,
    X: ComplexFloat<Real = T::Real> + Into<W>,
    XX: Matrix<X, Mapped<W>: Matrix<W>> + Matrix<X, Mapped<()>: Matrix<(), Owned: Matrix<(), RowOwned: List<(), Mapped<W> = XW>>, Width: StaticMaybe<usize, Opposite: Sized>, Height: StaticMaybe<usize, Opposite: Sized>>>,
    DD: OwnedList<(), Owned = DD, Width: StaticMaybe<usize, Opposite: Sized>, Height: StaticMaybe<usize, Opposite: Sized>>,
    XW: List<W>,
    DD::Mapped<XW>: Lists<W>,
    <XX::Transpose as MaybeLists<X>>::RowOwned: MaybeLenEq<D::RowOwned, true>,
    Array2<W>: SsAMatrix<W, Array2<W>, Array2<W>, Array2<W>> + SsBMatrix<W, Array2<W>, Array2<W>, Array2<W>> + SsCMatrix<W, Array2<W>, Array2<W>, Array2<W>>+ SsDMatrix<W, Array2<W>, Array2<W>, Array2<W>>
{
    type Output = DD::Mapped<XW>;

    fn filter_mut(&mut self, x: XX) -> Self::Output
    {
        let Ss {mut a, mut b, mut c, mut d, ..} = Ss::new(
            self.sys.a.to_array2().map(|&a| a.into()),
            self.sys.b.to_array2().map(|&b| b.into()),
            self.sys.c.to_array2().map(|&c| c.into()),
            self.sys.d.to_array2().map(|&d| d.into())
        );

        let (mu, nu) = x.matrix_dim();
        let (ma, na) = a.dim();
        let (mb, nb) = b.dim();
        let (mc, nc) = c.dim();
        let (md, nd) = d.dim();

        let n = ma.max(na).max(mb).max(nc);
        let p = nb.max(nd).max(mu);
        let q = mc.max(md);

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

        resize(&mut a, (n, n));
        resize(&mut b, (n, p));
        resize(&mut c, (q, n));
        resize(&mut d, (q, p));
        
        self.w.resize(n, W::zero());

        let mut y_t: Vec<std::vec::IntoIter<W>> = x.to_array2()
            .columns()
            .into_iter()
            .map(|u| {
                let mut x = Array2::from_shape_fn((n, 1), |(i, _)| self.w[i]);
                let u = Array2::from_shape_fn((p, 1), |(i, _)| u.get(i).map(|&u| u).unwrap_or_else(Zero::zero).into());

                let y = c.dot(&x) + d.dot(&u);
                x = a.dot(&x) + b.dot(&u);

                for (&x, w) in x.column(0)
                    .into_iter()
                    .zip(self.w.iter_mut())
                {
                    *w = x;
                }

                y.column(0)
                    .to_vec()
                    .into_iter()
            }).collect();
        let mut y = vec![];
        'lp:
        loop
        {
            let mut first = true;

            for y_t in y_t.iter_mut()
            {
                if let Some(y_t) = y_t.next()
                {
                    if first
                    {
                        y.push(vec![]);
                        first = false;
                    }
                    let y = y.last_mut().unwrap();
                    y.push(y_t)
                }
                else
                {
                    break 'lp
                }
            }
        }
        let mut y = y.into_iter();

        let d_empty = self.sys.d.map_to_owned(|_| ());
        let c_empty = self.sys.c.map_to_owned(|_| ());
        let d_empty_t = d_empty.matrix_transpose();
        let c_empty_t = c_empty.matrix_transpose();
        let d_empty_c = d_empty_t.into_owned_rows()
            .into_iter()
            .next()
            .unwrap();
        let c_empty_c = c_empty_t.into_owned_rows()
            .into_iter()
            .next()
            .unwrap();
        let dr_empty_c = d_empty_c.overlay(c_empty_c)
            .resize_to_owned((StaticMaybe::maybe_from_fn(|| 1), StaticMaybe::maybe_from_fn(|| y.len())), || ());
        let mut x_r = x.map_to_owned(|_| ())
            .resize_to_owned((StaticMaybe::maybe_from_fn(|| p), StaticMaybe::maybe_from_fn(|| nu)), || ())
            .into_owned_rows()
            .into_iter();
        dr_empty_c.map_into_owned(|()| {
            let mut y = y.next()
                .unwrap()
                .into_iter();
            x_r.next()
                .unwrap()
                .map_to_owned(|_| y.next().unwrap())
        })
    }
}