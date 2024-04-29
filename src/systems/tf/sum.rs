use core::{iter::Sum, ops::Mul};

use num::{complex::ComplexFloat, Zero};

use crate::{quantities::{MaybeList, MaybeLists}, systems::Tf};

impl<T1, B1, A1, T2, B2, A2> Sum<Tf<T1, B1, A1>> for Tf<T2, B2, A2>
where
    T1: ComplexFloat,
    T2: ComplexFloat,
    B1: MaybeLists<T1>,
    A1: MaybeList<T1>,
    B2: MaybeLists<T2>,
    A2: MaybeList<T2>,
    Tf<T1, B1, A1>: Into<Tf<T2, B2, A2>>,
    Tf<T2, B2, A2>: Mul<Output = Tf<T2, B2, A2>> + Zero
{
    fn sum<I: Iterator<Item = Tf<T1, B1, A1>>>(iter: I) -> Self
    {
        iter.map(|tf| tf.into())
            .reduce(|a, b| a + b)
            .unwrap_or_else(Zero::zero)
    }
}