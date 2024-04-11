use core::ops::{AddAssign, SubAssign};

use array_math::SliceOps;
use num::{complex::ComplexFloat, Zero};
use option_trait::Maybe;

use crate::{ComplexOp, List, Lists, MaybeList, MaybeLists, Rtf, RtfOrSystem, Sos, Tf, Container};

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

impl<'b, W, T, B, A, S, X, XX> FilterMut<X, XX> for Rtf<'b, W, Sos<T, B, A, S>>
where
    T: ComplexFloat + Into<W>,
    B: Maybe<[T; 3]> + MaybeList<T>,
    A: Maybe<[T; 3]> + MaybeList<T>,
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
                let mut a = sos.a.iter();
                let a0: T = a.next().copied()
                    .unwrap_or_else(Zero::zero);
                for (&a, &w) in a.zip(w.iter())
                {
                    w0 -= w*(a/a0).into()
                }
                let mut b = sos.b.iter();
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