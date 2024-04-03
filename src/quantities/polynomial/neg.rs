use core::ops::Neg;

use array_math::ArrayOps;


use crate::Polynomial;

macro_rules! impl_neg {
    (($(<$($a:lifetime),* $(,)? $($c:ident),* >)?) $s:ty [$n:tt] $(where $($w:tt)*)?) => {
        impl<$($($a,)* $(const $c: usize,)*)? T> Neg for Polynomial<T, $s>
        where
            Polynomial::<T, [T; $n]>: From<Polynomial<T, $s>> + Neg,
            $($($w)*)?
        {
            type Output = <Polynomial::<T, [T; $n]> as Neg>::Output;

            fn neg(self) -> Self::Output
            {
                -Polynomial::<T, [T; $n]>::from(self)
            }
        }
    };
    (($(<$($a:lifetime),* $(,)? $($c:ident),* >)?) $s:ty $(where $($w:tt)*)?) => {
        impl<$($($a,)* $(const $c: usize,)*)? T> Neg for Polynomial<T, $s>
        where
            Polynomial::<T, Vec<T>>: From<Polynomial<T, $s>> + Neg,
            $($($w)*)?
        {
            type Output = <Polynomial::<T, Vec<T>> as Neg>::Output;

            fn neg(self) -> Self::Output
            {
                -Polynomial::<T, Vec<T>>::from(self)
            }
        }
    };
}

impl_neg!(() () [1]);
impl<const N: usize, T> Neg for Polynomial<T, [T; N]>
where
    T: Neg
{
    type Output = Polynomial<<T as Neg>::Output, [<T as Neg>::Output; N]>;

    fn neg(self) -> Self::Output
    {
        Polynomial::new(self.c.neg_all())
    }
}
impl_neg!((<'a, N>) &'a [T; N] [N]);
impl<T> Neg for Polynomial<T, Vec<T>>
where
    T: Neg
{
    type Output = Polynomial<<T as Neg>::Output, Vec<<T as Neg>::Output>>;

    fn neg(self) -> Self::Output
    {
        Polynomial::new(
            self.c.into_iter()
                .map(|c| -c)
                .collect()
        )
    }
}
impl_neg!((<'a>) &'a [T]);