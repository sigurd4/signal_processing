use core::ops::{AddAssign, SubAssign};

use array_math::SliceOps;
use num::{complex::ComplexFloat, Zero};

use crate::{ComplexOp, ListOrSingle, Lists, MaybeList, MaybeLists, Rtf, RtfOrSystem, Tf};

pub trait FilterMut<X, XX>: RtfOrSystem
where
    Self::Domain: ComplexOp<X>,
    X: Into<<Self::Domain as ComplexOp<X>>::Output>,
    X: ComplexFloat<Real = <Self::Domain as ComplexFloat>::Real>,
    XX: Lists<X>
{
    type Y: Lists<<Self::Domain as ComplexOp<X>>::Output>;
    type Output: ListOrSingle<Self::Y>;

    fn filter_mut(&mut self, x: XX) -> Self::Output;
}

impl<'b, W, T, B, A, X, XX> FilterMut<X, XX> for Rtf<'b, W, Tf<T, B, A>>
where
    W: ComplexFloat<Real = T::Real>,
    T: ComplexFloat + ComplexOp<X>,
    B: MaybeLists<T>,
    A: MaybeList<T>,
    X: ComplexFloat<Real = T::Real>,
    XX: Lists<X>,
    &'b Tf<T, B, A>: Into<Tf<T, Vec<Vec<T>>, Vec<T>>>,
    T: ComplexOp<X, Output = W>,
    W: ComplexOp<X, Output = W> + AddAssign + SubAssign,
    X: Into<W>,
    XX::Mapped<W>: Lists<W>
{
    type Y = XX::Mapped<W>;
    type Output = B::RowsMapped<Self::Y>;

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
                    let mut a = a.iter();
                    let a0: T = a.next().copied()
                        .unwrap_or_else(Zero::zero);
                    for (&a, &w) in a.zip(w.iter())
                    {
                        w0 -= w*(a/a0).into()
                    }
                    let mut b = b.iter();
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

                i += nw;

                y
            });
        y
    }
}