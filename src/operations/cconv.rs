use core::ops::Mul;

use crate::MaybeList;

pub trait CConv<T1, T2, Rhs>: MaybeList<T1>
where
    T1: Mul<T2>,
    Rhs: MaybeList<T2>
{
    type Output: MaybeList<<T1 as Mul<T2>>::Output>;

    fn cconv(self, rhs: Rhs) -> Self::Output;
}