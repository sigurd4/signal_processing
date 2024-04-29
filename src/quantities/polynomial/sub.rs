use core::ops::Sub;

use array_math::{max_len, ArrayOps};
use num::{Zero};

use crate::quantities::Polynomial;

macro_rules! impl_sub {
    (($(<$($a:lifetime),* $(,)? $($c:ident),* >)?) $lhs:ty, $rhs:ty [$n:tt, $m:tt] $(where $($w:tt)*)?) => {
        impl<$($($a,)* $(const $c: usize,)*)? T1, T2> Sub<Polynomial<T2, $rhs>> for Polynomial<T1, $lhs>
        where
            Polynomial::<T1, [T1; $n]>: From<Polynomial<T1, $lhs>>,
            Polynomial::<T2, [T2; $m]>: From<Polynomial<T2, $rhs>>,
            Polynomial::<T1, [T1; $n]>: Sub<Polynomial::<T2, [T2; $m]>>,
            $($($w)*)?
        {
            type Output = <Polynomial::<T1, [T1; $n]> as Sub<Polynomial::<T2, [T2; $m]>>>::Output;

            fn sub(self, rhs: Polynomial<T2, $rhs>) -> Self::Output
            {
                Polynomial::<T1, [T1; $n]>::from(self) - Polynomial::<T2, [T2; $m]>::from(rhs)
            }
        }
    };
    (($(<$($a:lifetime),* $(,)? $($c:ident),* >)?) $lhs:ty, $rhs:ty $(where $($w:tt)*)?) => {
        impl<$($($a,)* $(const $c: usize,)*)? T1, T2> Sub<Polynomial<T2, $rhs>> for Polynomial<T1, $lhs>
        where
            Polynomial::<T1, Vec<T1>>: From<Polynomial<T1, $lhs>>,
            Polynomial::<T2, Vec<T2>>: From<Polynomial<T2, $rhs>>,
            Polynomial::<T1, Vec<T1>>: Sub<Polynomial::<T2, Vec<T2>>>,
            $($($w)*)?
        {
            type Output = <Polynomial::<T1, Vec<T1>> as Sub<Polynomial::<T2, Vec<T2>>>>::Output;

            fn sub(self, rhs: Polynomial<T2, $rhs>) -> Self::Output
            {
                Polynomial::<T1, Vec<T1>>::from(self) - Polynomial::<T2, Vec<T2>>::from(rhs)
            }
        }
    };
}

impl_sub!(() (), () [1, 1]);
impl_sub!((<M>) (), [T2; M] [1, M]);
impl_sub!((<'b, M>) (), &'b [T2; M] [1, M]);
impl_sub!(() (), Vec<T2>);
impl_sub!((<'b>) (), &'b [T2]);

impl_sub!((<N>) [T1; N], () [N, 1]);
impl<T1, T2, const N: usize, const M: usize> Sub<Polynomial<T2, [T2; M]>> for Polynomial<T1, [T1; N]>
where
    T1: Sub<T2> + Zero,
    T2: Zero,
    [(); max_len(N, M)]:
{
    type Output = Polynomial<<T1 as Sub<T2>>::Output, [<T1 as Sub<T2>>::Output; max_len(N, M)]>;

    fn sub(self, rhs: Polynomial<T2, [T2; M]>) -> Self::Output
    {
        Polynomial::new(
            self.c.rresize(|_| T1::zero())
                .sub_each(rhs.c.rresize(|_| T2::zero()))
        )
    }
}
impl_sub!((<'a, N, M>) [T1; N], &'a [T2; M] [N, M]);
impl_sub!((<'a, N>) [T1; N], Vec<T2>);
impl_sub!((<'a, N>) [T1; N], &'a [T2]);

impl_sub!((<'a, N>) &'a [T1; N], () [N, 1]);
impl_sub!((<'a, N, M>) &'a [T1; N], [T2; M] [N, M]);
impl_sub!((<'a, 'b, N, M>) &'a [T1; N], &'b [T2; M] [N, M]);
impl_sub!((<'a, N>) &'a [T1; N], Vec<T2>);
impl_sub!((<'a, 'b, N>) &'a [T1; N], &'b [T2]);

impl_sub!(() Vec<T1>, ());
impl_sub!((<M>) Vec<T1>, [T2; M]);
impl_sub!((<'b, M>) Vec<T1>, &'b [T2; M]);
impl<T1, T2> Sub<Polynomial<T2, Vec<T2>>> for Polynomial<T1, Vec<T1>>
where
    T1: Zero + Clone + Sub<T2>,
    T2: Zero + Clone
{
    type Output = Polynomial<<T1 as Sub<T2>>::Output, Vec<<T1 as Sub<T2>>::Output>>;

    fn sub(mut self, mut rhs: Polynomial<T2, Vec<T2>>) -> Self::Output
    {
        let n = self.c.len();
        let m = rhs.c.len();
        let v = if m > n
        {
            let mut a = vec![T1::zero(); m - n];
            a.append(&mut self.c);
            a.into_iter()
                .zip(rhs.c)
                .map(|(a, b)| a - b)
                .collect()
        }
        else
        {
            let mut b = vec![T2::zero(); n - m];
            b.append(&mut rhs.c);
            self.c.into_iter()
                .zip(b)
                .map(|(a, b)| a - b)
                .collect()
        };
        Polynomial::new(v)
    }
}
impl_sub!((<'b>) Vec<T1>, &'b [T2]);

impl_sub!((<'a>) &'a [T1], ());
impl_sub!((<'a, M>) &'a [T1], [T2; M]);
impl_sub!((<'a, 'b, M>) &'a [T1], &'b [T2; M]);
impl_sub!((<'a>) &'a [T1], Vec<T2>);
impl_sub!((<'a, 'b>) &'a [T1], &'b [T2]);