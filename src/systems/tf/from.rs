
use num::complex::ComplexFloat;

use crate::{MaybeList, MaybeLists, Polynomial, Tf};

impl<'a, T1, B1, A1, T2, B2, A2> From<&'a Tf<T1, B1, A1>> for Tf<T2, B2, A2>
where
    T1: ComplexFloat,
    T2: ComplexFloat,
    B1: MaybeLists<T1>,
    A1: MaybeList<T1>,
    B2: MaybeLists<T2>,
    A2: MaybeList<T2>,
    B1::View<'a>: MaybeLists<T1>,
    A1::View<'a>: MaybeLists<T1>,
    Polynomial<T1, B1::View<'a>>: Into<Polynomial<T2, B2>>,
    Polynomial<T1, A1::View<'a>>: Into<Polynomial<T2, A2>>,
{
    fn from(tf: &'a Tf<T1, B1, A1>) -> Self
    {
        Tf {
            b: tf.b.as_view().into(),
            a: tf.a.as_view().into()
        }
    }
}