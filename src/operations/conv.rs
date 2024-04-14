use core::{iter::Sum, ops::{AddAssign, Mul, MulAssign}};

use array_math::{ArrayMath, SliceMath};
use num::{complex::ComplexFloat, Complex};

use crate::{ListOrSingle, MaybeContainer};

pub trait Conv<T1, T2, Rhs>: MaybeContainer<T1>
where
    T1: Mul<T2>,
    Rhs: MaybeContainer<T2>
{
    type OutputT;
    type Output: MaybeContainer<Self::OutputT>;

    fn conv(self, rhs: Rhs) -> Self::Output;
}

macro_rules! impl_conv {
    (($(<$($a:lifetime),* $(,)? $($c:ident),* >)?) (), $rhs:ty $(where $($w:tt)*)?) => {
        impl<$($($a,)* $(const $c: usize,)*)? T1, T2> Conv<T1, T2, $rhs> for ()
        where
            T1: Mul<T2>,
            $($($w)*)?
        {
            type OutputT = T2;
            type Output = $rhs;

            #[inline]
            fn conv(self, rhs: $rhs) -> Self::Output
            {
                rhs
            }
        }
    };
    (($(<$($a:lifetime),* $(,)? $($c:ident),* >)?) $lhs:ty, () $(where $($w:tt)*)?) => {
        impl<$($($a,)* $(const $c: usize,)*)? T1, T2> Conv<T1, T2, ()> for $lhs
        where
            T1: Mul<T2>,
            $($($w)*)?
        {
            type OutputT = T1;
            type Output = $lhs;

            #[inline]
            fn conv(self, _: ()) -> Self::Output
            {
                self
            }
        }
    };
    (($(<$($a:lifetime),* $(,)? $($c:ident),* >)?) $lhs:ty, $rhs:ty [$n:tt, $m:tt] $(where $($w:tt)*)?) => {
        impl<$($($a,)* $(const $c: usize,)*)? T1, T2> Conv<T1, T2, $rhs> for $lhs
        where
            T1: ComplexFloat + Mul<T2, Output: ComplexFloat + From<<<T1 as Mul<T2>>::Output as ComplexFloat>::Real> + 'static>,
            T2: ComplexFloat,
            Complex<T1::Real>: From<T1> + AddAssign + MulAssign + Mul<Complex<T2::Real>, Output: ComplexFloat<Real = <<T1 as Mul<T2>>::Output as ComplexFloat>::Real> + MulAssign + AddAssign + From<Complex<<<T1 as Mul<T2>>::Output as ComplexFloat>::Real>> + Sum + 'static>,
            Complex<T2::Real>: From<T2> + AddAssign + MulAssign,
            [(); ($n + $m - 1).next_power_of_two() - $n]:,
            [(); ($n + $m - 1).next_power_of_two() - $m]:,
            [(); ($n + $m - 1).next_power_of_two() - ($n + $m - 1)]:
        {
            type OutputT = <T1 as Mul<T2>>::Output;
            type Output = [<T1 as Mul<T2>>::Output; $n + $m - 1];
        
            fn conv(self, rhs: $rhs) -> Self::Output
            {
                MaybeContainer::to_owned(&self).convolve_fft(MaybeContainer::to_owned(&rhs))
            }
        }
    };
    (($(<$($a:lifetime),* $(,)? $($c:ident),* >)?) $lhs:ty, $rhs:ty [] $(where $($w:tt)*)?) => {
        impl<$($($a,)* $(const $c: usize,)*)? T1, T2> Conv<T1, T2, $rhs> for $lhs
        where
            T1: ComplexFloat + Mul<T2, Output: ComplexFloat + From<<<T1 as Mul<T2>>::Output as ComplexFloat>::Real> + 'static>,
            T2: ComplexFloat,
            Complex<T1::Real>: From<T1> + AddAssign + MulAssign + Mul<Complex<T2::Real>, Output: ComplexFloat<Real = <<T1 as Mul<T2>>::Output as ComplexFloat>::Real> + MulAssign + AddAssign + From<Complex<<<T1 as Mul<T2>>::Output as ComplexFloat>::Real>> + Sum + 'static>,
            Complex<T2::Real>: From<T2> + AddAssign + MulAssign
        {
            type OutputT = <T1 as Mul<T2>>::Output;
            type Output = Vec<<T1 as Mul<T2>>::Output>;
        
            #[inline]
            fn conv(self, rhs: $rhs) -> Self::Output
            {
                ListOrSingle::<T1>::as_view_slice(&self).convolve_fft(ListOrSingle::<T2>::as_view_slice(&rhs))
            }
        }
    };
    (($(<$($a:lifetime),* $(,)? $($c:ident),* >)?) $lhs:ty, $rhs:ty => [$k:tt] [$n:tt, $m:tt] $(where $($w:tt)*)?) => {
        impl<$($($a,)* $(const $c: usize,)*)? T1, T2> Conv<T1, T2, $rhs> for $lhs
        where
            T1: ComplexFloat + Mul<T2, Output: ComplexFloat + From<<<T1 as Mul<T2>>::Output as ComplexFloat>::Real> + 'static>,
            T2: ComplexFloat,
            Complex<T1::Real>: From<T1> + AddAssign + MulAssign + Mul<Complex<T2::Real>, Output: ComplexFloat<Real = <<T1 as Mul<T2>>::Output as ComplexFloat>::Real> + MulAssign + AddAssign + From<Complex<<<T1 as Mul<T2>>::Output as ComplexFloat>::Real>> + Sum + 'static>,
            Complex<T2::Real>: From<T2> + AddAssign + MulAssign,
            [(); ($n + $m - 1).next_power_of_two() - $n]:,
            [(); ($n + $m - 1).next_power_of_two() - $m]:,
            [(); ($n + $m - 1).next_power_of_two() - ($n + $m - 1)]:
            $($($w)*)?
        {
            type OutputT = <T1 as Mul<T2>>::Output;
            type Output = [[<T1 as Mul<T2>>::Output; $n + $m - 1]; $k];
        
            #[inline]
            fn conv(self, rhs: $rhs) -> Self::Output
            {
                rhs.each_ref()
                    .map(|c| MaybeContainer::to_owned(&self).convolve_fft(MaybeContainer::to_owned(c)))
            }
        }
    };
    (($(<$($a:lifetime),* $(,)? $($c:ident),* >)?) $lhs:ty => [$k:tt], $rhs:ty [$n:tt, $m:tt] $(where $($w:tt)*)?) => {
        impl<$($($a,)* $(const $c: usize,)*)? T1, T2> Conv<T1, T2, $rhs> for $lhs
        where
            T1: ComplexFloat + Mul<T2, Output: ComplexFloat + From<<<T1 as Mul<T2>>::Output as ComplexFloat>::Real> + 'static>,
            T2: ComplexFloat,
            Complex<T1::Real>: From<T1> + AddAssign + MulAssign + Mul<Complex<T2::Real>, Output: ComplexFloat<Real = <<T1 as Mul<T2>>::Output as ComplexFloat>::Real> + MulAssign + AddAssign + From<Complex<<<T1 as Mul<T2>>::Output as ComplexFloat>::Real>> + Sum + 'static>,
            Complex<T2::Real>: From<T2> + AddAssign + MulAssign,
            [(); ($n + $m - 1).next_power_of_two() - $n]:,
            [(); ($n + $m - 1).next_power_of_two() - $m]:,
            [(); ($n + $m - 1).next_power_of_two() - ($n + $m - 1)]:
            $($($w)*)?
        {
            type OutputT = <T1 as Mul<T2>>::Output;
            type Output = [[<T1 as Mul<T2>>::Output; $n + $m - 1]; $k];
        
            #[inline]
            fn conv(self, rhs: $rhs) -> Self::Output
            {
                self.each_ref()
                    .map(|c| MaybeContainer::to_owned(c).convolve_fft(MaybeContainer::to_owned(&rhs)))
            }
        }
    };
    (($(<$($a:lifetime),* $(,)? $($c:ident),* >)?) $lhs:ty, $rhs:ty => [] [$n:tt, $m:tt] $(where $($w:tt)*)?) => {
        impl<$($($a,)* $(const $c: usize,)*)? T1, T2> Conv<T1, T2, $rhs> for $lhs
        where
            T1: ComplexFloat + Mul<T2, Output: ComplexFloat + From<<<T1 as Mul<T2>>::Output as ComplexFloat>::Real> + 'static>,
            T2: ComplexFloat,
            Complex<T1::Real>: From<T1> + AddAssign + MulAssign + Mul<Complex<T2::Real>, Output: ComplexFloat<Real = <<T1 as Mul<T2>>::Output as ComplexFloat>::Real> + MulAssign + AddAssign + From<Complex<<<T1 as Mul<T2>>::Output as ComplexFloat>::Real>> + Sum + 'static>,
            Complex<T2::Real>: From<T2> + AddAssign + MulAssign,
            [(); ($n + $m - 1).next_power_of_two() - $n]:,
            [(); ($n + $m - 1).next_power_of_two() - $m]:,
            [(); ($n + $m - 1).next_power_of_two() - ($n + $m - 1)]:
            $($($w)*)?
        {
            type OutputT = <T1 as Mul<T2>>::Output;
            type Output = Vec<[<T1 as Mul<T2>>::Output; $n + $m - 1]>;
        
            #[inline]
            fn conv(self, rhs: $rhs) -> Self::Output
            {
                rhs.iter()
                    .map(|c| MaybeContainer::to_owned(&self).convolve_fft(MaybeContainer::to_owned(c)))
                    .collect()
            }
        }
    };
    (($(<$($a:lifetime),* $(,)? $($c:ident),* >)?) $lhs:ty => [], $rhs:ty [$n:tt, $m:tt] $(where $($w:tt)*)?) => {
        impl<$($($a,)* $(const $c: usize,)*)? T1, T2> Conv<T1, T2, $rhs> for $lhs
        where
            T1: ComplexFloat + Mul<T2, Output: ComplexFloat + From<<<T1 as Mul<T2>>::Output as ComplexFloat>::Real> + 'static>,
            T2: ComplexFloat,
            Complex<T1::Real>: From<T1> + AddAssign + MulAssign + Mul<Complex<T2::Real>, Output: ComplexFloat<Real = <<T1 as Mul<T2>>::Output as ComplexFloat>::Real> + MulAssign + AddAssign + From<Complex<<<T1 as Mul<T2>>::Output as ComplexFloat>::Real>> + Sum + 'static>,
            Complex<T2::Real>: From<T2> + AddAssign + MulAssign,
            [(); ($n + $m - 1).next_power_of_two() - $n]:,
            [(); ($n + $m - 1).next_power_of_two() - $m]:,
            [(); ($n + $m - 1).next_power_of_two() - ($n + $m - 1)]:
            $($($w)*)?
        {
            type OutputT = <T1 as Mul<T2>>::Output;
            type Output = Vec<[<T1 as Mul<T2>>::Output; $n + $m - 1]>;
        
            #[inline]
            fn conv(self, rhs: $rhs) -> Self::Output
            {
                self.iter()
                    .map(|c| MaybeContainer::to_owned(c).convolve_fft(MaybeContainer::to_owned(&rhs)))
                    .collect()
            }
        }
    };
    (($(<$($a:lifetime),* $(,)? $($c:ident),* >)?) $lhs:ty, $rhs:ty => [$k:tt] [] $(where $($w:tt)*)?) => {
        impl<$($($a,)* $(const $c: usize,)*)? T1, T2> Conv<T1, T2, $rhs> for $lhs
        where
            T1: ComplexFloat + Mul<T2, Output: ComplexFloat + From<<<T1 as Mul<T2>>::Output as ComplexFloat>::Real> + 'static>,
            T2: ComplexFloat,
            Complex<T1::Real>: From<T1> + AddAssign + MulAssign + Mul<Complex<T2::Real>, Output: ComplexFloat<Real = <<T1 as Mul<T2>>::Output as ComplexFloat>::Real> + MulAssign + AddAssign + From<Complex<<<T1 as Mul<T2>>::Output as ComplexFloat>::Real>> + Sum + 'static>,
            Complex<T2::Real>: From<T2> + AddAssign + MulAssign
            $($($w)*)?
        {
            type OutputT = <T1 as Mul<T2>>::Output;
            type Output = [Vec<<T1 as Mul<T2>>::Output>; $k];
        
            #[inline]
            fn conv(self, rhs: $rhs) -> Self::Output
            {
                rhs.each_ref()
                    .map(|c| ListOrSingle::<T1>::as_view_slice(&self).convolve_fft(ListOrSingle::<T2>::as_view_slice(c)))
            }
        }
    };
    (($(<$($a:lifetime),* $(,)? $($c:ident),* >)?) $lhs:ty => [$k:tt], $rhs:ty [] $(where $($w:tt)*)?) => {
        impl<$($($a,)* $(const $c: usize,)*)? T1, T2> Conv<T1, T2, $rhs> for $lhs
        where
            T1: ComplexFloat + Mul<T2, Output: ComplexFloat + From<<<T1 as Mul<T2>>::Output as ComplexFloat>::Real> + 'static>,
            T2: ComplexFloat,
            Complex<T1::Real>: From<T1> + AddAssign + MulAssign + Mul<Complex<T2::Real>, Output: ComplexFloat<Real = <<T1 as Mul<T2>>::Output as ComplexFloat>::Real> + MulAssign + AddAssign + From<Complex<<<T1 as Mul<T2>>::Output as ComplexFloat>::Real>> + Sum + 'static>,
            Complex<T2::Real>: From<T2> + AddAssign + MulAssign
            $($($w)*)?
        {
            type OutputT = <T1 as Mul<T2>>::Output;
            type Output = [Vec<<T1 as Mul<T2>>::Output>; $k];
        
            #[inline]
            fn conv(self, rhs: $rhs) -> Self::Output
            {
                self.each_ref()
                    .map(|c| ListOrSingle::<T1>::as_view_slice(c).convolve_fft(ListOrSingle::<T2>::as_view_slice(&rhs)))
            }
        }
    };
    (($(<$($a:lifetime),* $(,)? $($c:ident),* >)?) $lhs:ty, $rhs:ty => [] [] $(where $($w:tt)*)?) => {
        impl<$($($a,)* $(const $c: usize,)*)? T1, T2> Conv<T1, T2, $rhs> for $lhs
        where
            T1: ComplexFloat + Mul<T2, Output: ComplexFloat + From<<<T1 as Mul<T2>>::Output as ComplexFloat>::Real> + 'static>,
            T2: ComplexFloat,
            Complex<T1::Real>: From<T1> + AddAssign + MulAssign + Mul<Complex<T2::Real>, Output: ComplexFloat<Real = <<T1 as Mul<T2>>::Output as ComplexFloat>::Real> + MulAssign + AddAssign + From<Complex<<<T1 as Mul<T2>>::Output as ComplexFloat>::Real>> + Sum + 'static>,
            Complex<T2::Real>: From<T2> + AddAssign + MulAssign
            $($($w)*)?
        {
            type OutputT = <T1 as Mul<T2>>::Output;
            type Output = Vec<Vec<<T1 as Mul<T2>>::Output>>;
        
            #[inline]
            fn conv(self, rhs: $rhs) -> Self::Output
            {
                rhs.iter()
                    .map(|c| ListOrSingle::<T1>::as_view_slice(&self).convolve_fft(ListOrSingle::<T2>::as_view_slice(c)))
                    .collect()
            }
        }
    };
    (($(<$($a:lifetime),* $(,)? $($c:ident),* >)?) $lhs:ty => [], $rhs:ty [] $(where $($w:tt)*)?) => {
        impl<$($($a,)* $(const $c: usize,)*)? T1, T2> Conv<T1, T2, $rhs> for $lhs
        where
            T1: ComplexFloat + Mul<T2, Output: ComplexFloat + From<<<T1 as Mul<T2>>::Output as ComplexFloat>::Real> + 'static>,
            T2: ComplexFloat,
            Complex<T1::Real>: From<T1> + AddAssign + MulAssign + Mul<Complex<T2::Real>, Output: ComplexFloat<Real = <<T1 as Mul<T2>>::Output as ComplexFloat>::Real> + MulAssign + AddAssign + From<Complex<<<T1 as Mul<T2>>::Output as ComplexFloat>::Real>> + Sum + 'static>,
            Complex<T2::Real>: From<T2> + AddAssign + MulAssign
            $($($w)*)?
        {
            type OutputT = <T1 as Mul<T2>>::Output;
            type Output = Vec<Vec<<T1 as Mul<T2>>::Output>>;

            #[inline]
            fn conv(self, rhs: $rhs) -> Self::Output
            {
                self.iter()
                    .map(|c| ListOrSingle::<T1>::as_view_slice(c).convolve_fft(ListOrSingle::<T2>::as_view_slice(&rhs)))
                    .collect()
            }
        }
    };
}

impl_conv!(() (), ());
impl_conv!((<M>) (), [T2; M]);
impl_conv!((<'b, M>) (), &'b [T2; M]);
impl_conv!(() (), Vec<T2>);
impl_conv!((<'b>) (), &'b [T2]);
impl_conv!((<K, M>) (), [[T2; M]; K]);
impl_conv!((<'b, K, M>) (), [&'b [T2; M]; K]);
impl_conv!((<K>) (), [Vec<T2>; K]);
impl_conv!((<'b, K>) (), [&'b [T2]; K]);
impl_conv!((<'c, K, M>) (), &'c [[T2; M]; K]);
impl_conv!((<'c, 'b, K, M>) (), &'c [&'b [T2; M]; K]);
impl_conv!((<'c, K>) (), &'c [Vec<T2>; K]);
impl_conv!((<'c, 'b, K>) (), &'c [&'b [T2]; K]);
impl_conv!((<M>) (), Vec<[T2; M]>);
impl_conv!((<'b, M>) (), Vec<&'b [T2; M]>);
impl_conv!(() (), Vec<Vec<T2>>);
impl_conv!((<'b>) (), Vec<&'b [T2]>);
impl_conv!((<'c, M>) (), &'c [[T2; M]]);
impl_conv!((<'c, 'b, M>) (), &'c [&'b [T2; M]]);
impl_conv!((<'c>) (), &'c [Vec<T2>]);
impl_conv!((<'c, 'b>) (), &'c [&'b [T2]]);

impl_conv!((<N>) [T1; N], ());
impl_conv!((<N, M>) [T1; N], [T2; M] [N, M]);
impl_conv!((<'b, N, M>) [T1; N], &'b [T2; M] [N, M]);
impl_conv!((<N>) [T1; N], Vec<T2> []);
impl_conv!((<'b, N>) [T1; N], &'b [T2] []);
impl_conv!((<K, N, M>) [T1; N], [[T2; M]; K] => [K] [N, M]);
impl_conv!((<'b, K, N, M>) [T1; N], [&'b [T2; M]; K] => [K] [N, M]);
impl_conv!((<K, N>) [T1; N], [Vec<T2>; K] => [K] []);
impl_conv!((<'b, K, N>) [T1; N], [&'b [T2]; K] => [K] []);
impl_conv!((<'c, K, N, M>) [T1; N], &'c [[T2; M]; K] => [K] [N, M]);
impl_conv!((<'c, 'b, K, N, M>) [T1; N], &'c [&'b [T2; M]; K] => [K] [N, M]);
impl_conv!((<'c, K, N>) [T1; N], &'c [Vec<T2>; K] => [K] []);
impl_conv!((<'c, 'b, K, N>) [T1; N], &'c [&'b [T2]; K] => [K] []);
impl_conv!((<N, M>) [T1; N], Vec<[T2; M]> => [] [N, M]);
impl_conv!((<'b, N, M>) [T1; N], Vec<&'b [T2; M]> => [] [N, M]);
impl_conv!((<N>) [T1; N], Vec<Vec<T2>> => [] []);
impl_conv!((<'b, N>) [T1; N], Vec<&'b [T2]> => [] []);
impl_conv!((<'c, N, M>) [T1; N], &'c [[T2; M]] => [] [N, M]);
impl_conv!((<'c, 'b, N, M>) [T1; N], &'c [&'b [T2; M]] => [] [N, M]);
impl_conv!((<'c, N>) [T1; N], &'c [Vec<T2>] => [] []);
impl_conv!((<'c, 'b, N>) [T1; N], &'c [&'b [T2]] => [] []);

impl_conv!((<'a, N>) &'a [T1; N], ());
impl_conv!((<'a, N, M>) &'a [T1; N], [T2; M] [N, M]);
impl_conv!((<'a, 'b N, M>) &'a [T1; N], &'b [T2; M] [N, M]);
impl_conv!((<'a, N>) &'a [T1; N], Vec<T2> []);
impl_conv!((<'a, 'b, N>) &'a [T1; N], &'b [T2] []);
impl_conv!((<K, N, M>) &[T1; N], [[T2; M]; K] => [K] [N, M]);
impl_conv!((<'b, K, N, M>) &[T1; N], [&'b [T2; M]; K] => [K] [N, M]);
impl_conv!((<K, N>) &[T1; N], [Vec<T2>; K] => [K] []);
impl_conv!((<'b, K, N>) &[T1; N], [&'b [T2]; K] => [K] []);
impl_conv!((<'c, K, N, M>) &[T1; N], &'c [[T2; M]; K] => [K] [N, M]);
impl_conv!((<'c, 'b, K, N, M>) &[T1; N], &'c [&'b [T2; M]; K] => [K] [N, M]);
impl_conv!((<'c, K, N>) &[T1; N], &'c [Vec<T2>; K] => [K] []);
impl_conv!((<'c, 'b, K, N>) &[T1; N], &'c [&'b [T2]; K] => [K] []);
impl_conv!((<N, M>) &[T1; N], Vec<[T2; M]> => [] [N, M]);
impl_conv!((<'b, N, M>) &[T1; N], Vec<&'b [T2; M]> => [] [N, M]);
impl_conv!((<N>) &[T1; N], Vec<Vec<T2>> => [] []);
impl_conv!((<'b, N>) &[T1; N], Vec<&'b [T2]> => [] []);
impl_conv!((<'c, N, M>) &[T1; N], &'c [[T2; M]] => [] [N, M]);
impl_conv!((<'c, 'b, N, M>) &[T1; N], &'c [&'b [T2; M]] => [] [N, M]);
impl_conv!((<'c, N>) &[T1; N], &'c [Vec<T2>] => [] []);
impl_conv!((<'c, 'b, N>) &[T1; N], &'c [&'b [T2]] => [] []);

impl_conv!(() Vec<T1>, ());
impl_conv!((<M>) Vec<T1>, [T2; M] []);
impl_conv!((<'b, M>) Vec<T1>, &'b [T2; M] []);
impl_conv!(() Vec<T1>, Vec<T2> []);
impl_conv!((<'b>) Vec<T1>, &'b [T2] []);
impl_conv!((<K, M>) Vec<T1>, [[T2; M]; K] => [K] []);
impl_conv!((<'b, K, M>) Vec<T1>, [&'b [T2; M]; K] => [K] []);
impl_conv!((<K>) Vec<T1>, [Vec<T2>; K] => [K] []);
impl_conv!((<'b, K>) Vec<T1>, [&'b [T2]; K] => [K] []);
impl_conv!((<'c, K, M>) Vec<T1>, &'c [[T2; M]; K] => [K] []);
impl_conv!((<'c, 'b, K, M>) Vec<T1>, &'c [&'b [T2; M]; K] => [K] []);
impl_conv!((<'c, K>) Vec<T1>, &'c [Vec<T2>; K] => [K] []);
impl_conv!((<'c, 'b, K>) Vec<T1>, &'c [&'b [T2]; K] => [K] []);
impl_conv!((<M>) Vec<T1>, Vec<[T2; M]> => [] []);
impl_conv!((<'b, M>) Vec<T1>, Vec<&'b [T2; M]> => [] []);
impl_conv!(() Vec<T1>, Vec<Vec<T2>> => [] []);
impl_conv!((<'b>) Vec<T1>, Vec<&'b [T2]> => [] []);
impl_conv!((<'c, M>) Vec<T1>, &'c [[T2; M]] => [] []);
impl_conv!((<'c, 'b, M>) Vec<T1>, &'c [&'b [T2; M]] => [] []);
impl_conv!((<'c>) Vec<T1>, &'c [Vec<T2>] => [] []);
impl_conv!((<'c, 'b>) Vec<T1>, &'c [&'b [T2]] => [] []);

impl_conv!((<'a>) &'a [T1], ());
impl_conv!((<'a, M>) &'a [T1], [T2; M] []);
impl_conv!((<'a, 'b, M>) &'a [T1], &'b [T2; M] []);
impl_conv!((<'a>) &'a [T1], Vec<T2> []);
impl_conv!((<'a, 'b>) &'a [T1], &'b [T2] []);
impl_conv!((<K, M>) &[T1], [[T2; M]; K] => [K] []);
impl_conv!((<'b, K, M>) &[T1], [&'b [T2; M]; K] => [K] []);
impl_conv!((<K>) &[T1], [Vec<T2>; K] => [K] []);
impl_conv!((<'b, K>) &[T1], [&'b [T2]; K] => [K] []);
impl_conv!((<'c, K, M>) &[T1], &'c [[T2; M]; K] => [K] []);
impl_conv!((<'c, 'b, K, M>) &[T1], &'c [&'b [T2; M]; K] => [K] []);
impl_conv!((<'c, K>) &[T1], &'c [Vec<T2>; K] => [K] []);
impl_conv!((<'c, 'b, K>) &[T1], &'c [&'b [T2]; K] => [K] []);
impl_conv!((<M>) &[T1], Vec<[T2; M]> => [] []);
impl_conv!((<'b, M>) &[T1], Vec<&'b [T2; M]> => [] []);
impl_conv!(() &[T1], Vec<Vec<T2>> => [] []);
impl_conv!((<'b>) &[T1], Vec<&'b [T2]> => [] []);
impl_conv!((<'c, M>) &[T1], &'c [[T2; M]] => [] []);
impl_conv!((<'c, 'b, M>) &[T1], &'c [&'b [T2; M]] => [] []);
impl_conv!((<'c>) &[T1], &'c [Vec<T2>] => [] []);
impl_conv!((<'c, 'b>) &[T1], &'c [&'b [T2]] => [] []);

impl_conv!((<K, N>) [[T1; N]; K], ());
impl_conv!((<K, N, M>) [[T1; N]; K] => [K], [T2; M] [N, M]);
impl_conv!((<'b, K, N, M>) [[T1; N]; K] => [K], &'b [T2; M] [N, M]);
impl_conv!((<K, N>) [[T1; N]; K] => [K], Vec<T2> []);
impl_conv!((<'b, K, N>) [[T1; N]; K] => [K], &'b [T2] []);
impl_conv!((<'c, K, N>) &'c [[T1; N]; K], ());
impl_conv!((<K, N, M>) &[[T1; N]; K] => [K], [T2; M] [N, M]);
impl_conv!((<'b, K, N, M>) &[[T1; N]; K] => [K], &'b [T2; M] [N, M]);
impl_conv!((<K, N>) &[[T1; N]; K] => [K], Vec<T2> []);
impl_conv!((<'b, K, N>) &[[T1; N]; K] => [K], &'b [T2] []);
impl_conv!((<N>) Vec<[T1; N]>, ());
impl_conv!((<N, M>) Vec<[T1; N]> => [], [T2; M] [N, M]);
impl_conv!((<'b, N, M>) Vec<[T1; N]> => [], &'b [T2; M] [N, M]);
impl_conv!((<N>) Vec<[T1; N]> => [], Vec<T2> []);
impl_conv!((<'b, N>) Vec<[T1; N]> => [], &'b [T2] []);
impl_conv!((<'c, N>) &'c [[T1; N]], ());
impl_conv!((<N, M>) &[[T1; N]] => [], [T2; M] [N, M]);
impl_conv!((<'b, N, M>) &[[T1; N]] => [], &'b [T2; M] [N, M]);
impl_conv!((<N>) &[[T1; N]] => [], Vec<T2> []);
impl_conv!((<'b, N>) &[[T1; N]] => [], &'b [T2] []);

impl_conv!((<'a, K, N>) [&'a [T1; N]; K], ());
impl_conv!((<K, N, M>) [&[T1; N]; K] => [K], [T2; M] [N, M]);
impl_conv!((<'b, K, N, M>) [&[T1; N]; K] => [K], &'b [T2; M] [N, M]);
impl_conv!((<K, N>) [&[T1; N]; K] => [K], Vec<T2> []);
impl_conv!((<'b, K, N>) [&[T1; N]; K] => [K], &'b [T2] []);
impl_conv!((<'a, 'c, K, N>) &'c [&'a [T1; N]; K], ());
impl_conv!((<K, N, M>) &[&[T1; N]; K] => [K], [T2; M] [N, M]);
impl_conv!((<'b, K, N, M>) &[&[T1; N]; K] => [K], &'b [T2; M] [N, M]);
impl_conv!((<K, N>) &[&[T1; N]; K] => [K], Vec<T2> []);
impl_conv!((<'b, K, N>) &[&[T1; N]; K] => [K], &'b [T2] []);
impl_conv!((<'a, N>) Vec<&'a [T1; N]>, ());
impl_conv!((<N, M>) Vec<&[T1; N]> => [], [T2; M] [N, M]);
impl_conv!((<'b, N, M>) Vec<&[T1; N]> => [], &'b [T2; M] [N, M]);
impl_conv!((<N>) Vec<&[T1; N]> => [], Vec<T2> []);
impl_conv!((<'b, N>) Vec<&[T1; N]> => [], &'b [T2] []);
impl_conv!((<'a, 'c, N>) &'c [&'a [T1; N]], ());
impl_conv!((<N, M>) &[&[T1; N]] => [], [T2; M] [N, M]);
impl_conv!((<'b, N, M>) &[&[T1; N]] => [], &'b [T2; M] [N, M]);
impl_conv!((<N>) &[&[T1; N]] => [], Vec<T2> []);
impl_conv!((<'b, N>) &[&[T1; N]] => [], &'b [T2] []);

impl_conv!((<K>) [Vec<T1>; K], ());
impl_conv!((<K, M>) [Vec<T1>; K] => [K], [T2; M] []);
impl_conv!((<'b, K, M>) [Vec<T1>; K] => [K], &'b [T2; M] []);
impl_conv!((<K>) [Vec<T1>; K] => [K], Vec<T2> []);
impl_conv!((<'b, K>) [Vec<T1>; K] => [K], &'b [T2] []);
impl_conv!((<'c, K>) &'c [Vec<T1>; K], ());
impl_conv!((<K, M>) &[Vec<T1>; K] => [K], [T2; M] []);
impl_conv!((<'b, K, M>) &[Vec<T1>; K] => [K], &'b [T2; M] []);
impl_conv!((<K>) &[Vec<T1>; K] => [K], Vec<T2> []);
impl_conv!((<'b, K>) &[Vec<T1>; K] => [K], &'b [T2] []);
impl_conv!(() Vec<Vec<T1>>, ());
impl_conv!((<M>) Vec<Vec<T1>> => [], [T2; M] []);
impl_conv!((<'b, M>) Vec<Vec<T1>> => [], &'b [T2; M] []);
impl_conv!(() Vec<Vec<T1>> => [], Vec<T2> []);
impl_conv!((<'b>) Vec<Vec<T1>> => [], &'b [T2] []);
impl_conv!((<'c>) &'c [Vec<T1>], ());
impl_conv!((<M>) &[Vec<T1>] => [], [T2; M] []);
impl_conv!((<'b, M>) &[Vec<T1>] => [], &'b [T2; M] []);
impl_conv!(() &[Vec<T1>] => [], Vec<T2> []);
impl_conv!((<'b>) &[Vec<T1>] => [], &'b [T2] []);

impl_conv!((<'a, K>) [&'a [T1]; K], ());
impl_conv!((<K, M>) [&[T1]; K] => [K], [T2; M] []);
impl_conv!((<'b, K, M>) [&[T1]; K] => [K], &'b [T2; M] []);
impl_conv!((<K>) [&[T1]; K] => [K], Vec<T2> []);
impl_conv!((<'b, K>) [&[T1]; K] => [K], &'b [T2] []);
impl_conv!((<'a, 'c, K>) &'c [&'a [T1]; K], ());
impl_conv!((<K, M>) &[&[T1]; K] => [K], [T2; M] []);
impl_conv!((<'b, K, M>) &[&[T1]; K] => [K], &'b [T2; M] []);
impl_conv!((<K>) &[&[T1]; K] => [K], Vec<T2> []);
impl_conv!((<'b, K>) &[&[T1]; K] => [K], &'b [T2] []);
impl_conv!((<'a>) Vec<&'a [T1]>, ());
impl_conv!((<M>) Vec<&[T1]> => [], [T2; M] []);
impl_conv!((<'b, M>) Vec<&[T1]> => [], &'b [T2; M] []);
impl_conv!(() Vec<&[T1]> => [], Vec<T2> []);
impl_conv!((<'b>) Vec<&[T1]> => [], &'b [T2] []);
impl_conv!((<'a, 'c>) &'c [&'a [T1]], ());
impl_conv!((<M>) &[&[T1]] => [], [T2; M] []);
impl_conv!((<'b, M>) &[&[T1]] => [], &'b [T2; M] []);
impl_conv!(() &[&[T1]] => [], Vec<T2> []);
impl_conv!((<'b>) &[&[T1]] => [], &'b [T2] []);

#[cfg(test)]
mod test
{
    use crate::Conv;

    #[test]
    fn test()
    {
        let a: [f32; _] = [1.0, 2.0, 3.0];
        let b: [f32; _] = [2.0, 2.0];

        let c = a.conv(b);

        println!("{:?}", c)
    }
}