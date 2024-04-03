use core::ops::DivAssign;

use num::{complex::ComplexFloat, One, Zero};

use crate::{ComplexOp, List, ListOrSingle, Lists, MaybeList, MaybeLists, System, Tf};

pub trait FiltIC<'a, X, XX, Y>: System
where
    Self::Domain: ComplexOp<X>,
    X: ComplexFloat + Into<<Self::Domain as ComplexOp<X>>::Output>,
    XX: List<X>,
    XX::Mapped<<Self::Domain as ComplexOp<X>>::Output>: List<<Self::Domain as ComplexOp<X>>::Output>,
    Y: ListOrSingle<XX::Mapped<<Self::Domain as ComplexOp<X>>::Output>>
{
    fn filtic(&'a self, y: Y, x: XX) -> Vec<<Self::Domain as ComplexOp<X>>::Output>;
}

impl<'a, T, B, A, X, XX, Y> FiltIC<'a, X, XX, B::RowsMapped<XX::Mapped<Y>>> for Tf<T, B, A>
where
    T: ComplexOp<X, Output = Y>,
    B: MaybeLists<T>,
    A: MaybeList<T>,
    X: ComplexFloat + Into<Y>,
    Y: ComplexFloat + DivAssign,
    XX: List<X>,
    XX::Mapped<Y>: List<Y>,
    B::RowsMapped<XX::Mapped<Y>>: Lists<Y>,
    Self: 'a,
    &'a Self: Into<Tf<T, Vec<Vec<T>>, Vec<T>>>
{
    fn filtic(&'a self, y: B::RowsMapped<XX::Mapped<Y>>, x: XX) -> Vec<Y>
    {
        let Tf {b, mut a}: Tf<_, Vec<Vec<_>>, Vec<_>> = self.into();

        let na = a.len();

        let mut w = vec![];
        let mut b = b.into_inner()
            .into_iter();
        y.map_rows_to_owned(|y| {
            let mut b = b.next().unwrap();
            let mut x: Vec<X> = x.to_vec();
            let mut y: Vec<Y> = y.as_view_slice_option()
                .map(|y| y.to_vec())
                .unwrap_or_else(|| vec![One::one()]);

            let nb = b.len();
            let n = nb.max(na);
            let nz = n.saturating_sub(1);
            let mut zf = vec![Zero::zero(); nz];
            b.resize(n, Zero::zero());
            a.resize(n, Zero::zero());
            x.resize(x.len().max(nz), Zero::zero());
            y.resize(y.len().max(nz), Zero::zero());

            for i in (0..nz).rev()
            {
                for j in i..nz - 1
                {
                    zf[j] = Into::<Y>::into(b[j + 1])*x[i].into() - Into::<Y>::into(a[j + 1])*y[i] + zf[j + 1]
                }
                zf[nz - 1] = Into::<Y>::into(b[nz])*x[i].into() - Into::<Y>::into(a[nz])*y[i]
            }
            let a0 = a.first().copied()
                .unwrap_or_else(Zero::zero);
            for zf in zf.iter_mut()
            {
                *zf /= a0.into()
            }
            w.append(&mut zf);
        });
        w
    }
}

#[cfg(test)]
mod test
{
    use crate::{Butter, FiltIC, Filter, FilterGenPlane, FilterGenType, Tf};

    #[test]
    fn test()
    {
        let h = Tf::butter(2, [100.0], FilterGenType::LowPass, FilterGenPlane::Z { sampling_frequency: Some(1000.0) })
            .unwrap();

        let w1 = vec![10.0, -5.0];

        const N: usize = 16;
        let x = [0.0; N];

        let y = h.filter(x, w1.clone());
        let w2 = h.filtic(y, x);

        println!("w1 = {:?}", w1);
        println!("w2 = {:?}", w2);
    }
}