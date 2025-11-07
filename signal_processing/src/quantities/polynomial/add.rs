use core::ops::Add;

use array_math::{max_len, ArrayOps};
use num::Zero;

use crate::quantities::Polynomial;

macro_rules! impl_add {
    (($(<$($a:lifetime),* $(,)? $($c:ident),* >)?) $lhs:ty, $rhs:ty [$n:tt, $m:tt] $(where $($w:tt)*)?) => {
        impl<$($($a,)* $(const $c: usize,)*)? T1, T2> Add<Polynomial<T2, $rhs>> for Polynomial<T1, $lhs>
        where
            Polynomial::<T1, [T1; $n]>: From<Polynomial<T1, $lhs>>,
            Polynomial::<T2, [T2; $m]>: From<Polynomial<T2, $rhs>>,
            Polynomial::<T1, [T1; $n]>: Add<Polynomial::<T2, [T2; $m]>>,
            $($($w)*)?
        {
            type Output = <Polynomial::<T1, [T1; $n]> as Add<Polynomial::<T2, [T2; $m]>>>::Output;

            fn add(self, rhs: Polynomial<T2, $rhs>) -> Self::Output
            {
                Polynomial::<T1, [T1; $n]>::from(self) + Polynomial::<T2, [T2; $m]>::from(rhs)
            }
        }
    };
    (($(<$($a:lifetime),* $(,)? $($c:ident),* >)?) $lhs:ty, $rhs:ty $(where $($w:tt)*)?) => {
        impl<$($($a,)* $(const $c: usize,)*)? T1, T2> Add<Polynomial<T2, $rhs>> for Polynomial<T1, $lhs>
        where
            Polynomial::<T1, Vec<T1>>: From<Polynomial<T1, $lhs>>,
            Polynomial::<T2, Vec<T2>>: From<Polynomial<T2, $rhs>>,
            Polynomial::<T1, Vec<T1>>: Add<Polynomial::<T2, Vec<T2>>>,
            $($($w)*)?
        {
            type Output = <Polynomial::<T1, Vec<T1>> as Add<Polynomial::<T2, Vec<T2>>>>::Output;

            fn add(self, rhs: Polynomial<T2, $rhs>) -> Self::Output
            {
                Polynomial::<T1, Vec<T1>>::from(self) + Polynomial::<T2, Vec<T2>>::from(rhs)
            }
        }
    };
}

impl_add!(() (), () [1, 1]);
impl_add!((<M>) (), [T2; M] [1, M]);
impl_add!((<'b, M>) (), &'b [T2; M] [1, M]);
impl_add!(() (), Vec<T2>);
impl_add!((<'b>) (), &'b [T2]);

impl_add!((<N>) [T1; N], () [N, 1]);
impl<T1, T2, const N: usize, const M: usize> Add<Polynomial<T2, [T2; M]>> for Polynomial<T1, [T1; N]>
where
    T1: Add<T2> + Zero,
    T2: Zero,
    [(); max_len(N, M)]:
{
    type Output = Polynomial<<T1 as Add<T2>>::Output, [<T1 as Add<T2>>::Output; max_len(N, M)]>;

    fn add(self, rhs: Polynomial<T2, [T2; M]>) -> Self::Output
    {
        Polynomial::new(
            self.c.rresize(|_| T1::zero())
                .add_each(rhs.c.rresize(|_| T2::zero()))
        )
    }
}
impl_add!((<'a, N, M>) [T1; N], &'a [T2; M] [N, M]);
impl_add!((<'a, N>) [T1; N], Vec<T2>);
impl_add!((<'a, N>) [T1; N], &'a [T2]);

impl_add!((<'a, N>) &'a [T1; N], () [N, 1]);
impl_add!((<'a, N, M>) &'a [T1; N], [T2; M] [N, M]);
impl_add!((<'a, 'b, N, M>) &'a [T1; N], &'b [T2; M] [N, M]);
impl_add!((<'a, N>) &'a [T1; N], Vec<T2>);
impl_add!((<'a, 'b, N>) &'a [T1; N], &'b [T2]);

impl_add!(() Vec<T1>, ());
impl_add!((<M>) Vec<T1>, [T2; M]);
impl_add!((<'b, M>) Vec<T1>, &'b [T2; M]);
impl<T1, T2> Add<Polynomial<T2, Vec<T2>>> for Polynomial<T1, Vec<T1>>
where
    T1: Zero + Clone + Add<T2>,
    T2: Zero + Clone
{
    type Output = Polynomial<<T1 as Add<T2>>::Output, Vec<<T1 as Add<T2>>::Output>>;

    fn add(mut self, mut rhs: Polynomial<T2, Vec<T2>>) -> Self::Output
    {
        let n = self.c.len();
        let m = rhs.c.len();
        let v = if m > n
        {
            let mut a = vec![T1::zero(); m - n];
            a.append(&mut self.c);
            a.into_iter()
                .zip(rhs.c)
                .map(|(a, b)| a + b)
                .collect()
        }
        else
        {
            let mut b = vec![T2::zero(); n - m];
            b.append(&mut rhs.c);
            self.c.into_iter()
                .zip(b)
                .map(|(a, b)| a + b)
                .collect()
        };
        Polynomial::new(v)
    }
}
impl_add!((<'b>) Vec<T1>, &'b [T2]);

impl_add!((<'a>) &'a [T1], ());
impl_add!((<'a, M>) &'a [T1], [T2; M]);
impl_add!((<'a, 'b, M>) &'a [T1], &'b [T2; M]);
impl_add!((<'a>) &'a [T1], Vec<T2>);
impl_add!((<'a, 'b>) &'a [T1], &'b [T2]);