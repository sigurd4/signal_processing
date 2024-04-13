use array_math::{ArrayMath, SliceMath};
use num::Zero;
use option_trait::NotVoid;

use core::ops::{AddAssign, Mul};

use crate::{Polynomial, ListOrSingle, MaybeContainer, Lists, NotPolynomial};

macro_rules! impl_mul {
    (($(<$($a:lifetime),* $(,)? $($c:ident),* >)?) (), $rhs:ty $(where $($w:tt)*)?) => {
        impl<$($($a,)* $(const $c: usize,)*)? T1, T2> Mul<Polynomial<T2, $rhs>> for Polynomial<T1, ()>
        where
            $($($w)*)?
        {
            type Output = Polynomial<T2, $rhs>;

            #[inline]
            fn mul(self, rhs: Polynomial<T2, $rhs>) -> Self::Output
            {
                rhs
            }
        }
    };
    (($(<$($a:lifetime),* $(,)? $($c:ident),* >)?) $lhs:ty, () $(where $($w:tt)*)?) => {
        impl<$($($a,)* $(const $c: usize,)*)? T1, T2> Mul<Polynomial<T2, ()>> for Polynomial<T1, $lhs>
        where
            $($($w)*)?
        {
            type Output = Polynomial<T1, $lhs>;

            #[inline]
            fn mul(self, _: Polynomial<T2, ()>) -> Self::Output
            {
                self
            }
        }
    };
    (($(<$($a:lifetime),* $(,)? $($c:ident),* >)?) $lhs:ty, $rhs:ty [$n:tt, $m:tt] $(where $($w:tt)*)?) => {
        impl<$($($a,)* $(const $c: usize,)*)? T1, T2> Mul<Polynomial<T2, $rhs>> for Polynomial<T1, $lhs>
        where
            T1: Mul<T2, Output: AddAssign + Zero> + Copy,
            T2: Copy,
            [(); $n + $m - 1]:,
            $($($w)*)?
        {
            type Output = Polynomial<<T1 as Mul<T2>>::Output, [<T1 as Mul<T2>>::Output; $n + $m - 1]>;

            #[inline]
            fn mul(self, rhs: Polynomial<T2, $rhs>) -> Self::Output
            {
                Polynomial::new(
                    self.c.as_view().convolve_direct(rhs.c.as_view())
                )
            }
        }
    };
    (($(<$($a:lifetime),* $(,)? $($c:ident),* >)?) $lhs:ty, $rhs:ty [] $(where $($w:tt)*)?) => {
        impl<$($($a,)* $(const $c: usize,)*)? T1, T2> Mul<Polynomial<T2, $rhs>> for Polynomial<T1, $lhs>
        where
            T1: Mul<T2, Output: AddAssign + Zero> + Copy,
            T2: Copy,
            $($($w)*)?
        {
            type Output = Polynomial<<T1 as Mul<T2>>::Output, Vec<<T1 as Mul<T2>>::Output>>;

            #[inline]
            fn mul(self, rhs: Polynomial<T2, $rhs>) -> Self::Output
            {
                Polynomial::new(
                    ListOrSingle::<T1>::as_view_slice(&self.c).convolve_direct(ListOrSingle::<T2>::as_view_slice(&rhs.c))
                )
            }
        }
    };
    (($(<$($a:lifetime),* $(,)? $($c:ident),* >)?) $lhs:ty, $rhs:ty => [$k:tt] [$n:tt, $m:tt] $(where $($w:tt)*)?) => {
        impl<$($($a,)* $(const $c: usize,)*)? T1, T2> Mul<Polynomial<T2, $rhs>> for Polynomial<T1, $lhs>
        where
            T1: Mul<T2, Output: AddAssign + Zero> + Copy,
            T2: Copy,
            [(); $n + $m - 1]:,
            $($($w)*)?
        {
            type Output = Polynomial<<T1 as Mul<T2>>::Output, [[<T1 as Mul<T2>>::Output; $n + $m - 1]; $k]>;
        
            #[inline]
            fn mul(self, rhs: Polynomial<T2, $rhs>) -> Self::Output
            {
                Polynomial::new(
                    rhs.c.each_ref()
                        .map(|c| self.c.as_view().convolve_direct(c.as_view()))
                )
            }
        }
    };
    (($(<$($a:lifetime),* $(,)? $($c:ident),* >)?) $lhs:ty => [$k:tt], $rhs:ty [$n:tt, $m:tt] $(where $($w:tt)*)?) => {
        impl<$($($a,)* $(const $c: usize,)*)? T1, T2> Mul<Polynomial<T2, $rhs>> for Polynomial<T1, $lhs>
        where
            T1: Mul<T2, Output: AddAssign + Zero> + Copy,
            T2: Copy,
            [(); $n + $m - 1]:,
            $($($w)*)?
        {
            type Output = Polynomial<<T1 as Mul<T2>>::Output, [[<T1 as Mul<T2>>::Output; $n + $m - 1]; $k]>;
        
            #[inline]
            fn mul(self, rhs: Polynomial<T2, $rhs>) -> Self::Output
            {
                Polynomial::new(
                    self.c.each_ref()
                        .map(|c| c.as_view().convolve_direct(rhs.c.as_view()))
                )
            }
        }
    };
    (($(<$($a:lifetime),* $(,)? $($c:ident),* >)?) $lhs:ty, $rhs:ty => [] [$n:tt, $m:tt] $(where $($w:tt)*)?) => {
        impl<$($($a,)* $(const $c: usize,)*)? T1, T2> Mul<Polynomial<T2, $rhs>> for Polynomial<T1, $lhs>
        where
            T1: Mul<T2, Output: AddAssign + Zero> + Copy,
            T2: Copy,
            [(); $n + $m - 1]:,
            $($($w)*)?
        {
            type Output = Polynomial<<T1 as Mul<T2>>::Output, Vec<[<T1 as Mul<T2>>::Output; $n + $m - 1]>>;
        
            #[inline]
            fn mul(self, rhs: Polynomial<T2, $rhs>) -> Self::Output
            {
                Polynomial::new(
                    rhs.c.iter()
                        .map(|c| self.c.as_view().convolve_direct(c.as_view()))
                        .collect()
                )
            }
        }
    };
    (($(<$($a:lifetime),* $(,)? $($c:ident),* >)?) $lhs:ty => [], $rhs:ty [$n:tt, $m:tt] $(where $($w:tt)*)?) => {
        impl<$($($a,)* $(const $c: usize,)*)? T1, T2> Mul<Polynomial<T2, $rhs>> for Polynomial<T1, $lhs>
        where
            T1: Mul<T2, Output: AddAssign + Zero> + Copy,
            T2: Copy,
            [(); $n + $m - 1]:,
            $($($w)*)?
        {
            type Output = Polynomial<<T1 as Mul<T2>>::Output, Vec<[<T1 as Mul<T2>>::Output; $n + $m - 1]>>;
        
            #[inline]
            fn mul(self, rhs: Polynomial<T2, $rhs>) -> Self::Output
            {
                Polynomial::new(
                    self.c.iter()
                        .map(|c| c.as_view().convolve_direct(rhs.c.as_view()))
                        .collect()
                )
            }
        }
    };
    (($(<$($a:lifetime),* $(,)? $($c:ident),* >)?) $lhs:ty, $rhs:ty => [$k:tt] [] $(where $($w:tt)*)?) => {
        impl<$($($a,)* $(const $c: usize,)*)? T1, T2> Mul<Polynomial<T2, $rhs>> for Polynomial<T1, $lhs>
        where
            T1: Mul<T2, Output: AddAssign + Zero> + Copy,
            T2: Copy,
            $($($w)*)?
        {
            type Output = Polynomial<<T1 as Mul<T2>>::Output, [Vec<<T1 as Mul<T2>>::Output>; $k]>;
        
            #[inline]
            fn mul(self, rhs: Polynomial<T2, $rhs>) -> Self::Output
            {
                Polynomial::new(
                    rhs.c.each_ref()
                        .map(|c| ListOrSingle::<T1>::as_view_slice(&self.c).convolve_direct(ListOrSingle::<T2>::as_view_slice(c)))
                )
            }
        }
    };
    (($(<$($a:lifetime),* $(,)? $($c:ident),* >)?) $lhs:ty => [$k:tt], $rhs:ty [] $(where $($w:tt)*)?) => {
        impl<$($($a,)* $(const $c: usize,)*)? T1, T2> Mul<Polynomial<T2, $rhs>> for Polynomial<T1, $lhs>
        where
            T1: Mul<T2, Output: AddAssign + Zero> + Copy,
            T2: Copy,
            $($($w)*)?
        {
            type Output = Polynomial<<T1 as Mul<T2>>::Output, [Vec<<T1 as Mul<T2>>::Output>; $k]>;
        
            #[inline]
            fn mul(self, rhs: Polynomial<T2, $rhs>) -> Self::Output
            {
                Polynomial::new(
                    self.c.each_ref()
                        .map(|c| ListOrSingle::<T1>::as_view_slice(c).convolve_direct(ListOrSingle::<T2>::as_view_slice(&rhs.c)))
                )
            }
        }
    };
    (($(<$($a:lifetime),* $(,)? $($c:ident),* >)?) $lhs:ty, $rhs:ty => [] [] $(where $($w:tt)*)?) => {
        impl<$($($a,)* $(const $c: usize,)*)? T1, T2> Mul<Polynomial<T2, $rhs>> for Polynomial<T1, $lhs>
        where
            T1: Mul<T2, Output: AddAssign + Zero> + Copy,
            T2: Copy,
            $($($w)*)?
        {
            type Output = Polynomial<<T1 as Mul<T2>>::Output, Vec<Vec<<T1 as Mul<T2>>::Output>>>;
        
            #[inline]
            fn mul(self, rhs: Polynomial<T2, $rhs>) -> Self::Output
            {
                Polynomial::new(
                    rhs.c.iter()
                        .map(|c| ListOrSingle::<T1>::as_view_slice(&self.c).convolve_direct(ListOrSingle::<T2>::as_view_slice(c)))
                        .collect()
                )
            }
        }
    };
    (($(<$($a:lifetime),* $(,)? $($c:ident),* >)?) $lhs:ty => [], $rhs:ty [] $(where $($w:tt)*)?) => {
        impl<$($($a,)* $(const $c: usize,)*)? T1, T2> Mul<Polynomial<T2, $rhs>> for Polynomial<T1, $lhs>
        where
            T1: Mul<T2, Output: AddAssign + Zero> + Copy,
            T2: Copy,
            $($($w)*)?
        {
            type Output = Polynomial<<T1 as Mul<T2>>::Output, Vec<Vec<<T1 as Mul<T2>>::Output>>>;

            #[inline]
            fn mul(self, rhs: Polynomial<T2, $rhs>) -> Self::Output
            {
                Polynomial::new(
                    self.c.iter()
                        .map(|c| ListOrSingle::<T1>::as_view_slice(c).convolve_direct(ListOrSingle::<T2>::as_view_slice(&rhs.c)))
                        .collect()
                )
            }
        }
    };
}

impl_mul!(() (), ());
impl_mul!((<M>) (), [T2; M]);
impl_mul!((<'b, M>) (), &'b [T2; M]);
impl_mul!(() (), Vec<T2>);
impl_mul!((<'b>) (), &'b [T2]);
impl_mul!((<K, M>) (), [[T2; M]; K]);
impl_mul!((<'b, K, M>) (), [&'b [T2; M]; K]);
impl_mul!((<K>) (), [Vec<T2>; K]);
impl_mul!((<'b, K>) (), [&'b [T2]; K]);
impl_mul!((<'c, K, M>) (), &'c [[T2; M]; K]);
impl_mul!((<'c, 'b, K, M>) (), &'c [&'b [T2; M]; K]);
impl_mul!((<'c, K>) (), &'c [Vec<T2>; K]);
impl_mul!((<'c, 'b, K>) (), &'c [&'b [T2]; K]);
impl_mul!((<M>) (), Vec<[T2; M]>);
impl_mul!((<'b, M>) (), Vec<&'b [T2; M]>);
impl_mul!(() (), Vec<Vec<T2>>);
impl_mul!((<'b>) (), Vec<&'b [T2]>);
impl_mul!((<'c, M>) (), &'c [[T2; M]]);
impl_mul!((<'c, 'b, M>) (), &'c [&'b [T2; M]]);
impl_mul!((<'c>) (), &'c [Vec<T2>]);
impl_mul!((<'c, 'b>) (), &'c [&'b [T2]]);

impl_mul!((<N>) [T1; N], ());
impl_mul!((<N, M>) [T1; N], [T2; M] [N, M]);
impl_mul!((<'b, N, M>) [T1; N], &'b [T2; M] [N, M]);
impl_mul!((<N>) [T1; N], Vec<T2> []);
impl_mul!((<'b, N>) [T1; N], &'b [T2] []);
impl_mul!((<K, N, M>) [T1; N], [[T2; M]; K] => [K] [N, M]);
impl_mul!((<'b, K, N, M>) [T1; N], [&'b [T2; M]; K] => [K] [N, M]);
impl_mul!((<K, N>) [T1; N], [Vec<T2>; K] => [K] []);
impl_mul!((<'b, K, N>) [T1; N], [&'b [T2]; K] => [K] []);
impl_mul!((<'c, K, N, M>) [T1; N], &'c [[T2; M]; K] => [K] [N, M]);
impl_mul!((<'c, 'b, K, N, M>) [T1; N], &'c [&'b [T2; M]; K] => [K] [N, M]);
impl_mul!((<'c, K, N>) [T1; N], &'c [Vec<T2>; K] => [K] []);
impl_mul!((<'c, 'b, K, N>) [T1; N], &'c [&'b [T2]; K] => [K] []);
impl_mul!((<N, M>) [T1; N], Vec<[T2; M]> => [] [N, M]);
impl_mul!((<'b, N, M>) [T1; N], Vec<&'b [T2; M]> => [] [N, M]);
impl_mul!((<N>) [T1; N], Vec<Vec<T2>> => [] []);
impl_mul!((<'b, N>) [T1; N], Vec<&'b [T2]> => [] []);
impl_mul!((<'c, N, M>) [T1; N], &'c [[T2; M]] => [] [N, M]);
impl_mul!((<'c, 'b, N, M>) [T1; N], &'c [&'b [T2; M]] => [] [N, M]);
impl_mul!((<'c, N>) [T1; N], &'c [Vec<T2>] => [] []);
impl_mul!((<'c, 'b, N>) [T1; N], &'c [&'b [T2]] => [] []);

impl_mul!((<'a, N>) &'a [T1; N], ());
impl_mul!((<'a, N, M>) &'a [T1; N], [T2; M] [N, M]);
impl_mul!((<'a, 'b N, M>) &'a [T1; N], &'b [T2; M] [N, M]);
impl_mul!((<'a, N>) &'a [T1; N], Vec<T2> []);
impl_mul!((<'a, 'b, N>) &'a [T1; N], &'b [T2] []);
impl_mul!((<K, N, M>) &[T1; N], [[T2; M]; K] => [K] [N, M]);
impl_mul!((<'b, K, N, M>) &[T1; N], [&'b [T2; M]; K] => [K] [N, M]);
impl_mul!((<K, N>) &[T1; N], [Vec<T2>; K] => [K] []);
impl_mul!((<'b, K, N>) &[T1; N], [&'b [T2]; K] => [K] []);
impl_mul!((<'c, K, N, M>) &[T1; N], &'c [[T2; M]; K] => [K] [N, M]);
impl_mul!((<'c, 'b, K, N, M>) &[T1; N], &'c [&'b [T2; M]; K] => [K] [N, M]);
impl_mul!((<'c, K, N>) &[T1; N], &'c [Vec<T2>; K] => [K] []);
impl_mul!((<'c, 'b, K, N>) &[T1; N], &'c [&'b [T2]; K] => [K] []);
impl_mul!((<N, M>) &[T1; N], Vec<[T2; M]> => [] [N, M]);
impl_mul!((<'b, N, M>) &[T1; N], Vec<&'b [T2; M]> => [] [N, M]);
impl_mul!((<N>) &[T1; N], Vec<Vec<T2>> => [] []);
impl_mul!((<'b, N>) &[T1; N], Vec<&'b [T2]> => [] []);
impl_mul!((<'c, N, M>) &[T1; N], &'c [[T2; M]] => [] [N, M]);
impl_mul!((<'c, 'b, N, M>) &[T1; N], &'c [&'b [T2; M]] => [] [N, M]);
impl_mul!((<'c, N>) &[T1; N], &'c [Vec<T2>] => [] []);
impl_mul!((<'c, 'b, N>) &[T1; N], &'c [&'b [T2]] => [] []);

impl_mul!(() Vec<T1>, ());
impl_mul!((<M>) Vec<T1>, [T2; M] []);
impl_mul!((<'b, M>) Vec<T1>, &'b [T2; M] []);
impl_mul!(() Vec<T1>, Vec<T2> []);
impl_mul!((<'b>) Vec<T1>, &'b [T2] []);
impl_mul!((<K, M>) Vec<T1>, [[T2; M]; K] => [K] []);
impl_mul!((<'b, K, M>) Vec<T1>, [&'b [T2; M]; K] => [K] []);
impl_mul!((<K>) Vec<T1>, [Vec<T2>; K] => [K] []);
impl_mul!((<'b, K>) Vec<T1>, [&'b [T2]; K] => [K] []);
impl_mul!((<'c, K, M>) Vec<T1>, &'c [[T2; M]; K] => [K] []);
impl_mul!((<'c, 'b, K, M>) Vec<T1>, &'c [&'b [T2; M]; K] => [K] []);
impl_mul!((<'c, K>) Vec<T1>, &'c [Vec<T2>; K] => [K] []);
impl_mul!((<'c, 'b, K>) Vec<T1>, &'c [&'b [T2]; K] => [K] []);
impl_mul!((<M>) Vec<T1>, Vec<[T2; M]> => [] []);
impl_mul!((<'b, M>) Vec<T1>, Vec<&'b [T2; M]> => [] []);
impl_mul!(() Vec<T1>, Vec<Vec<T2>> => [] []);
impl_mul!((<'b>) Vec<T1>, Vec<&'b [T2]> => [] []);
impl_mul!((<'c, M>) Vec<T1>, &'c [[T2; M]] => [] []);
impl_mul!((<'c, 'b, M>) Vec<T1>, &'c [&'b [T2; M]] => [] []);
impl_mul!((<'c>) Vec<T1>, &'c [Vec<T2>] => [] []);
impl_mul!((<'c, 'b>) Vec<T1>, &'c [&'b [T2]] => [] []);

impl_mul!((<'a>) &'a [T1], ());
impl_mul!((<'a, M>) &'a [T1], [T2; M] []);
impl_mul!((<'a, 'b, M>) &'a [T1], &'b [T2; M] []);
impl_mul!((<'a>) &'a [T1], Vec<T2> []);
impl_mul!((<'a, 'b>) &'a [T1], &'b [T2] []);
impl_mul!((<K, M>) &[T1], [[T2; M]; K] => [K] []);
impl_mul!((<'b, K, M>) &[T1], [&'b [T2; M]; K] => [K] []);
impl_mul!((<K>) &[T1], [Vec<T2>; K] => [K] []);
impl_mul!((<'b, K>) &[T1], [&'b [T2]; K] => [K] []);
impl_mul!((<'c, K, M>) &[T1], &'c [[T2; M]; K] => [K] []);
impl_mul!((<'c, 'b, K, M>) &[T1], &'c [&'b [T2; M]; K] => [K] []);
impl_mul!((<'c, K>) &[T1], &'c [Vec<T2>; K] => [K] []);
impl_mul!((<'c, 'b, K>) &[T1], &'c [&'b [T2]; K] => [K] []);
impl_mul!((<M>) &[T1], Vec<[T2; M]> => [] []);
impl_mul!((<'b, M>) &[T1], Vec<&'b [T2; M]> => [] []);
impl_mul!(() &[T1], Vec<Vec<T2>> => [] []);
impl_mul!((<'b>) &[T1], Vec<&'b [T2]> => [] []);
impl_mul!((<'c, M>) &[T1], &'c [[T2; M]] => [] []);
impl_mul!((<'c, 'b, M>) &[T1], &'c [&'b [T2; M]] => [] []);
impl_mul!((<'c>) &[T1], &'c [Vec<T2>] => [] []);
impl_mul!((<'c, 'b>) &[T1], &'c [&'b [T2]] => [] []);

impl_mul!((<K, N>) [[T1; N]; K], ());
impl_mul!((<K, N, M>) [[T1; N]; K] => [K], [T2; M] [N, M]);
impl_mul!((<'b, K, N, M>) [[T1; N]; K] => [K], &'b [T2; M] [N, M]);
impl_mul!((<K, N>) [[T1; N]; K] => [K], Vec<T2> []);
impl_mul!((<'b, K, N>) [[T1; N]; K] => [K], &'b [T2] []);
impl_mul!((<'c, K, N>) &'c [[T1; N]; K], ());
impl_mul!((<K, N, M>) &[[T1; N]; K] => [K], [T2; M] [N, M]);
impl_mul!((<'b, K, N, M>) &[[T1; N]; K] => [K], &'b [T2; M] [N, M]);
impl_mul!((<K, N>) &[[T1; N]; K] => [K], Vec<T2> []);
impl_mul!((<'b, K, N>) &[[T1; N]; K] => [K], &'b [T2] []);
impl_mul!((<N>) Vec<[T1; N]>, ());
impl_mul!((<N, M>) Vec<[T1; N]> => [], [T2; M] [N, M]);
impl_mul!((<'b, N, M>) Vec<[T1; N]> => [], &'b [T2; M] [N, M]);
impl_mul!((<N>) Vec<[T1; N]> => [], Vec<T2> []);
impl_mul!((<'b, N>) Vec<[T1; N]> => [], &'b [T2] []);
impl_mul!((<'c, N>) &'c [[T1; N]], ());
impl_mul!((<N, M>) &[[T1; N]] => [], [T2; M] [N, M]);
impl_mul!((<'b, N, M>) &[[T1; N]] => [], &'b [T2; M] [N, M]);
impl_mul!((<N>) &[[T1; N]] => [], Vec<T2> []);
impl_mul!((<'b, N>) &[[T1; N]] => [], &'b [T2] []);

impl_mul!((<'a, K, N>) [&'a [T1; N]; K], ());
impl_mul!((<K, N, M>) [&[T1; N]; K] => [K], [T2; M] [N, M]);
impl_mul!((<'b, K, N, M>) [&[T1; N]; K] => [K], &'b [T2; M] [N, M]);
impl_mul!((<K, N>) [&[T1; N]; K] => [K], Vec<T2> []);
impl_mul!((<'b, K, N>) [&[T1; N]; K] => [K], &'b [T2] []);
impl_mul!((<'a, 'c, K, N>) &'c [&'a [T1; N]; K], ());
impl_mul!((<K, N, M>) &[&[T1; N]; K] => [K], [T2; M] [N, M]);
impl_mul!((<'b, K, N, M>) &[&[T1; N]; K] => [K], &'b [T2; M] [N, M]);
impl_mul!((<K, N>) &[&[T1; N]; K] => [K], Vec<T2> []);
impl_mul!((<'b, K, N>) &[&[T1; N]; K] => [K], &'b [T2] []);
impl_mul!((<'a, N>) Vec<&'a [T1; N]>, ());
impl_mul!((<N, M>) Vec<&[T1; N]> => [], [T2; M] [N, M]);
impl_mul!((<'b, N, M>) Vec<&[T1; N]> => [], &'b [T2; M] [N, M]);
impl_mul!((<N>) Vec<&[T1; N]> => [], Vec<T2> []);
impl_mul!((<'b, N>) Vec<&[T1; N]> => [], &'b [T2] []);
impl_mul!((<'a, 'c, N>) &'c [&'a [T1; N]], ());
impl_mul!((<N, M>) &[&[T1; N]] => [], [T2; M] [N, M]);
impl_mul!((<'b, N, M>) &[&[T1; N]] => [], &'b [T2; M] [N, M]);
impl_mul!((<N>) &[&[T1; N]] => [], Vec<T2> []);
impl_mul!((<'b, N>) &[&[T1; N]] => [], &'b [T2] []);

impl_mul!((<K>) [Vec<T1>; K], ());
impl_mul!((<K, M>) [Vec<T1>; K] => [K], [T2; M] []);
impl_mul!((<'b, K, M>) [Vec<T1>; K] => [K], &'b [T2; M] []);
impl_mul!((<K>) [Vec<T1>; K] => [K], Vec<T2> []);
impl_mul!((<'b, K>) [Vec<T1>; K] => [K], &'b [T2] []);
impl_mul!((<'c, K>) &'c [Vec<T1>; K], ());
impl_mul!((<K, M>) &[Vec<T1>; K] => [K], [T2; M] []);
impl_mul!((<'b, K, M>) &[Vec<T1>; K] => [K], &'b [T2; M] []);
impl_mul!((<K>) &[Vec<T1>; K] => [K], Vec<T2> []);
impl_mul!((<'b, K>) &[Vec<T1>; K] => [K], &'b [T2] []);
impl_mul!(() Vec<Vec<T1>>, ());
impl_mul!((<M>) Vec<Vec<T1>> => [], [T2; M] []);
impl_mul!((<'b, M>) Vec<Vec<T1>> => [], &'b [T2; M] []);
impl_mul!(() Vec<Vec<T1>> => [], Vec<T2> []);
impl_mul!((<'b>) Vec<Vec<T1>> => [], &'b [T2] []);
impl_mul!((<'c>) &'c [Vec<T1>], ());
impl_mul!((<M>) &[Vec<T1>] => [], [T2; M] []);
impl_mul!((<'b, M>) &[Vec<T1>] => [], &'b [T2; M] []);
impl_mul!(() &[Vec<T1>] => [], Vec<T2> []);
impl_mul!((<'b>) &[Vec<T1>] => [], &'b [T2] []);

impl_mul!((<'a, K>) [&'a [T1]; K], ());
impl_mul!((<K, M>) [&[T1]; K] => [K], [T2; M] []);
impl_mul!((<'b, K, M>) [&[T1]; K] => [K], &'b [T2; M] []);
impl_mul!((<K>) [&[T1]; K] => [K], Vec<T2> []);
impl_mul!((<'b, K>) [&[T1]; K] => [K], &'b [T2] []);
impl_mul!((<'a, 'c, K>) &'c [&'a [T1]; K], ());
impl_mul!((<K, M>) &[&[T1]; K] => [K], [T2; M] []);
impl_mul!((<'b, K, M>) &[&[T1]; K] => [K], &'b [T2; M] []);
impl_mul!((<K>) &[&[T1]; K] => [K], Vec<T2> []);
impl_mul!((<'b, K>) &[&[T1]; K] => [K], &'b [T2] []);
impl_mul!((<'a>) Vec<&'a [T1]>, ());
impl_mul!((<M>) Vec<&[T1]> => [], [T2; M] []);
impl_mul!((<'b, M>) Vec<&[T1]> => [], &'b [T2; M] []);
impl_mul!(() Vec<&[T1]> => [], Vec<T2> []);
impl_mul!((<'b>) Vec<&[T1]> => [], &'b [T2] []);
impl_mul!((<'a, 'c>) &'c [&'a [T1]], ());
impl_mul!((<M>) &[&[T1]] => [], [T2; M] []);
impl_mul!((<'b, M>) &[&[T1]] => [], &'b [T2; M] []);
impl_mul!(() &[&[T1]] => [], Vec<T2> []);
impl_mul!((<'b>) &[&[T1]] => [], &'b [T2] []);

impl<T1, T2, T3, C> Mul<T2> for Polynomial<T1, C>
where
    C: Lists<T1> + NotVoid,
    T2: NotPolynomial + Clone,
    T1: Mul<T2, Output = T3> + Clone,
    C::Mapped<T3>: Lists<T3>
{
    type Output = Polynomial<T3, C::Mapped<T3>>;

    #[inline]
    fn mul(self, rhs: T2) -> Self::Output
    {
        self.map_into_owned(|lhs| lhs*rhs.clone())
    }
}