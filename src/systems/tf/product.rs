use std::{iter::Product, ops::Mul};

use num::{complex::ComplexFloat, One};

use crate::{Lists, MaybeList, Tf};

impl<T1, B1, A1, T2, B2, A2> Product<Tf<T1, B1, A1>> for Tf<T2, B2, A2>
where
    T1: ComplexFloat,
    T2: ComplexFloat,
    B1: Lists<T1>,
    A1: MaybeList<T1>,
    B2: Lists<T2>,
    A2: MaybeList<T2>,
    Tf<T1, B1, A1>: Into<Tf<T2, B2, A2>>,
    Tf<T2, B2, A2>: Mul<Output = Tf<T2, B2, A2>> + One
{
    fn product<I: Iterator<Item = Tf<T1, B1, A1>>>(iter: I) -> Self
    {
        iter.map(|tf| tf.into())
            .reduce(|a, b| a*b)
            .unwrap_or_else(One::one)
    }
}