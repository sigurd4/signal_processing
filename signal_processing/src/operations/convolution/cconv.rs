use core::{ops::{Mul, MulAssign, AddAssign}, iter::Sum};

use num::{complex::ComplexFloat, Complex};
use array_math::{max_len, SliceMath, ArrayMath};

use crate::quantities::{MaybeLists, ListOrSingle, MaybeContainer};

pub trait CConv<T1, T2, Rhs>: MaybeLists<T1>
where
    T1: Mul<T2>,
    Rhs: MaybeLists<T2>
{
    type OutputT;
    type Output: MaybeLists<Self::OutputT>;

    fn cconv(self, rhs: Rhs) -> Self::Output;
}

macro_rules! impl_cconv {
    (($(<$($a:lifetime),* $(,)? $($c:ident),* >)?) (), $rhs:ty $(where $($w:tt)*)?) => {
        impl<$($($a,)* $(const $c: usize,)*)? T1, T2> CConv<T1, T2, $rhs> for ()
        where
            T1: Mul<T2>,
            $($($w)*)?
        {
            type OutputT = T2;
            type Output = $rhs;

            #[inline]
            fn cconv(self, rhs: $rhs) -> Self::Output
            {
                rhs
            }
        }
    };
    (($(<$($a:lifetime),* $(,)? $($c:ident),* >)?) $lhs:ty, () $(where $($w:tt)*)?) => {
        impl<$($($a,)* $(const $c: usize,)*)? T1, T2> CConv<T1, T2, ()> for $lhs
        where
            T1: Mul<T2>,
            $($($w)*)?
        {
            type OutputT = T1;
            type Output = $lhs;

            #[inline]
            fn cconv(self, _: ()) -> Self::Output
            {
                self
            }
        }
    };
    (($(<$($a:lifetime),* $(,)? $($c:ident),* >)?) $lhs:ty, $rhs:ty [$n:tt, $m:tt] $(where $($w:tt)*)?) => {
        impl<$($($a,)* $(const $c: usize,)*)? T1, T2> CConv<T1, T2, $rhs> for $lhs
        where
            T1: ComplexFloat + Mul<T2, Output: ComplexFloat + From<<<T1 as Mul<T2>>::Output as ComplexFloat>::Real> + 'static>,
            T2: ComplexFloat,
            Complex<T1::Real>: From<T1> + AddAssign + MulAssign + Mul<Complex<T2::Real>, Output: ComplexFloat<Real = <<T1 as Mul<T2>>::Output as ComplexFloat>::Real> + MulAssign + AddAssign + MulAssign<<<T1 as Mul<T2>>::Output as ComplexFloat>::Real> + From<Complex<<<T1 as Mul<T2>>::Output as ComplexFloat>::Real>> + Sum + 'static>,
            Complex<T2::Real>: From<T2> + AddAssign + MulAssign,
            [(); max_len($n, $m)]:,
            $($($w)*)?
        {
            type OutputT = <T1 as Mul<T2>>::Output;
            type Output = [<T1 as Mul<T2>>::Output; max_len($n, $m)];
        
            fn cconv(self, rhs: $rhs) -> Self::Output
            {
                MaybeContainer::to_owned(&self).cconvolve_fft(MaybeContainer::to_owned(&rhs))
            }
        }
    };
    (($(<$($a:lifetime),* $(,)? $($c:ident),* >)?) $lhs:ty, $rhs:ty [] $(where $($w:tt)*)?) => {
        impl<$($($a,)* $(const $c: usize,)*)? T1, T2> CConv<T1, T2, $rhs> for $lhs
        where
            T1: ComplexFloat + Mul<T2, Output: ComplexFloat + From<<<T1 as Mul<T2>>::Output as ComplexFloat>::Real> + 'static>,
            T2: ComplexFloat,
            Complex<T1::Real>: From<T1> + AddAssign + MulAssign + Mul<Complex<T2::Real>, Output: ComplexFloat<Real = <<T1 as Mul<T2>>::Output as ComplexFloat>::Real> + MulAssign + AddAssign + MulAssign<<<T1 as Mul<T2>>::Output as ComplexFloat>::Real> + From<Complex<<<T1 as Mul<T2>>::Output as ComplexFloat>::Real>> + Sum + 'static>,
            Complex<T2::Real>: From<T2> + AddAssign + MulAssign,
            $($($w)*)?
        {
            type OutputT = <T1 as Mul<T2>>::Output;
            type Output = Vec<<T1 as Mul<T2>>::Output>;
        
            #[inline]
            fn cconv(self, rhs: $rhs) -> Self::Output
            {
                ListOrSingle::<T1>::as_view_slice(&self).cconvolve_fft(ListOrSingle::<T2>::as_view_slice(&rhs))
            }
        }
    };
    (($(<$($a:lifetime),* $(,)? $($c:ident),* >)?) $lhs:ty, $rhs:ty => [$k:tt] [$n:tt, $m:tt] $(where $($w:tt)*)?) => {
        impl<$($($a,)* $(const $c: usize,)*)? T1, T2> CConv<T1, T2, $rhs> for $lhs
        where
            T1: ComplexFloat + Mul<T2, Output: ComplexFloat + From<<<T1 as Mul<T2>>::Output as ComplexFloat>::Real> + 'static>,
            T2: ComplexFloat,
            Complex<T1::Real>: From<T1> + AddAssign + MulAssign + Mul<Complex<T2::Real>, Output: ComplexFloat<Real = <<T1 as Mul<T2>>::Output as ComplexFloat>::Real> + MulAssign + AddAssign + MulAssign<<<T1 as Mul<T2>>::Output as ComplexFloat>::Real> + From<Complex<<<T1 as Mul<T2>>::Output as ComplexFloat>::Real>> + Sum + 'static>,
            Complex<T2::Real>: From<T2> + AddAssign + MulAssign,
            [(); max_len($n, $m)]:,
            $($($w)*)?
        {
            type OutputT = <T1 as Mul<T2>>::Output;
            type Output = [[<T1 as Mul<T2>>::Output; max_len($n, $m)]; $k];
        
            #[inline]
            fn cconv(self, rhs: $rhs) -> Self::Output
            {
                rhs.each_ref()
                    .map(|c| MaybeContainer::to_owned(&self).cconvolve_fft(MaybeContainer::to_owned(c)))
            }
        }
    };
    (($(<$($a:lifetime),* $(,)? $($c:ident),* >)?) $lhs:ty => [$k:tt], $rhs:ty [$n:tt, $m:tt] $(where $($w:tt)*)?) => {
        impl<$($($a,)* $(const $c: usize,)*)? T1, T2> CConv<T1, T2, $rhs> for $lhs
        where
            T1: ComplexFloat + Mul<T2, Output: ComplexFloat + From<<<T1 as Mul<T2>>::Output as ComplexFloat>::Real> + 'static>,
            T2: ComplexFloat,
            Complex<T1::Real>: From<T1> + AddAssign + MulAssign + Mul<Complex<T2::Real>, Output: ComplexFloat<Real = <<T1 as Mul<T2>>::Output as ComplexFloat>::Real> + MulAssign + AddAssign + MulAssign<<<T1 as Mul<T2>>::Output as ComplexFloat>::Real> + From<Complex<<<T1 as Mul<T2>>::Output as ComplexFloat>::Real>> + Sum + 'static>,
            Complex<T2::Real>: From<T2> + AddAssign + MulAssign,
            [(); max_len($n, $m)]:,
            $($($w)*)?
        {
            type OutputT = <T1 as Mul<T2>>::Output;
            type Output = [[<T1 as Mul<T2>>::Output; max_len($n, $m)]; $k];
        
            #[inline]
            fn cconv(self, rhs: $rhs) -> Self::Output
            {
                self.each_ref()
                    .map(|c| MaybeContainer::to_owned(c).cconvolve_fft(MaybeContainer::to_owned(&rhs)))
            }
        }
    };
    (($(<$($a:lifetime),* $(,)? $($c:ident),* >)?) $lhs:ty, $rhs:ty => [] [$n:tt, $m:tt] $(where $($w:tt)*)?) => {
        impl<$($($a,)* $(const $c: usize,)*)? T1, T2> CConv<T1, T2, $rhs> for $lhs
        where
            T1: ComplexFloat + Mul<T2, Output: ComplexFloat + From<<<T1 as Mul<T2>>::Output as ComplexFloat>::Real> + 'static>,
            T2: ComplexFloat,
            Complex<T1::Real>: From<T1> + AddAssign + MulAssign + Mul<Complex<T2::Real>, Output: ComplexFloat<Real = <<T1 as Mul<T2>>::Output as ComplexFloat>::Real> + MulAssign + AddAssign + MulAssign<<<T1 as Mul<T2>>::Output as ComplexFloat>::Real> + From<Complex<<<T1 as Mul<T2>>::Output as ComplexFloat>::Real>> + Sum + 'static>,
            Complex<T2::Real>: From<T2> + AddAssign + MulAssign,
            [(); max_len($n, $m)]:,
            $($($w)*)?
        {
            type OutputT = <T1 as Mul<T2>>::Output;
            type Output = Vec<[<T1 as Mul<T2>>::Output; max_len($n, $m)]>;
        
            #[inline]
            fn cconv(self, rhs: $rhs) -> Self::Output
            {
                rhs.iter()
                    .map(|c| MaybeContainer::to_owned(&self).cconvolve_fft(MaybeContainer::to_owned(c)))
                    .collect()
            }
        }
    };
    (($(<$($a:lifetime),* $(,)? $($c:ident),* >)?) $lhs:ty => [], $rhs:ty [$n:tt, $m:tt] $(where $($w:tt)*)?) => {
        impl<$($($a,)* $(const $c: usize,)*)? T1, T2> CConv<T1, T2, $rhs> for $lhs
        where
            T1: ComplexFloat + Mul<T2, Output: ComplexFloat + From<<<T1 as Mul<T2>>::Output as ComplexFloat>::Real> + 'static>,
            T2: ComplexFloat,
            Complex<T1::Real>: From<T1> + AddAssign + MulAssign + Mul<Complex<T2::Real>, Output: ComplexFloat<Real = <<T1 as Mul<T2>>::Output as ComplexFloat>::Real> + MulAssign + AddAssign + MulAssign<<<T1 as Mul<T2>>::Output as ComplexFloat>::Real> + From<Complex<<<T1 as Mul<T2>>::Output as ComplexFloat>::Real>> + Sum + 'static>,
            Complex<T2::Real>: From<T2> + AddAssign + MulAssign,
            [(); max_len($n, $m)]:,
            $($($w)*)?
        {
            type OutputT = <T1 as Mul<T2>>::Output;
            type Output = Vec<[<T1 as Mul<T2>>::Output; max_len($n, $m)]>;
        
            #[inline]
            fn cconv(self, rhs: $rhs) -> Self::Output
            {
                self.iter()
                    .map(|c| MaybeContainer::to_owned(c).cconvolve_fft(MaybeContainer::to_owned(&rhs)))
                    .collect()
            }
        }
    };
    (($(<$($a:lifetime),* $(,)? $($c:ident),* >)?) $lhs:ty, $rhs:ty => [$k:tt] [] $(where $($w:tt)*)?) => {
        impl<$($($a,)* $(const $c: usize,)*)? T1, T2> CConv<T1, T2, $rhs> for $lhs
        where
            T1: ComplexFloat + Mul<T2, Output: ComplexFloat + From<<<T1 as Mul<T2>>::Output as ComplexFloat>::Real> + 'static>,
            T2: ComplexFloat,
            Complex<T1::Real>: From<T1> + AddAssign + MulAssign + Mul<Complex<T2::Real>, Output: ComplexFloat<Real = <<T1 as Mul<T2>>::Output as ComplexFloat>::Real> + MulAssign + AddAssign + MulAssign<<<T1 as Mul<T2>>::Output as ComplexFloat>::Real> + From<Complex<<<T1 as Mul<T2>>::Output as ComplexFloat>::Real>> + Sum + 'static>,
            Complex<T2::Real>: From<T2> + AddAssign + MulAssign,
            $($($w)*)?
        {
            type OutputT = <T1 as Mul<T2>>::Output;
            type Output = [Vec<<T1 as Mul<T2>>::Output>; $k];
        
            #[inline]
            fn cconv(self, rhs: $rhs) -> Self::Output
            {
                rhs.each_ref()
                    .map(|c| ListOrSingle::<T1>::as_view_slice(&self).cconvolve_fft(ListOrSingle::<T2>::as_view_slice(c)))
            }
        }
    };
    (($(<$($a:lifetime),* $(,)? $($c:ident),* >)?) $lhs:ty => [$k:tt], $rhs:ty [] $(where $($w:tt)*)?) => {
        impl<$($($a,)* $(const $c: usize,)*)? T1, T2> CConv<T1, T2, $rhs> for $lhs
        where
            T1: ComplexFloat + Mul<T2, Output: ComplexFloat + From<<<T1 as Mul<T2>>::Output as ComplexFloat>::Real> + 'static>,
            T2: ComplexFloat,
            Complex<T1::Real>: From<T1> + AddAssign + MulAssign + Mul<Complex<T2::Real>, Output: ComplexFloat<Real = <<T1 as Mul<T2>>::Output as ComplexFloat>::Real> + MulAssign + AddAssign + MulAssign<<<T1 as Mul<T2>>::Output as ComplexFloat>::Real> + From<Complex<<<T1 as Mul<T2>>::Output as ComplexFloat>::Real>> + Sum + 'static>,
            Complex<T2::Real>: From<T2> + AddAssign + MulAssign,
            $($($w)*)?
        {
            type OutputT = <T1 as Mul<T2>>::Output;
            type Output = [Vec<<T1 as Mul<T2>>::Output>; $k];
        
            #[inline]
            fn cconv(self, rhs: $rhs) -> Self::Output
            {
                self.each_ref()
                    .map(|c| ListOrSingle::<T1>::as_view_slice(c).cconvolve_fft(ListOrSingle::<T2>::as_view_slice(&rhs)))
            }
        }
    };
    (($(<$($a:lifetime),* $(,)? $($c:ident),* >)?) $lhs:ty, $rhs:ty => [] [] $(where $($w:tt)*)?) => {
        impl<$($($a,)* $(const $c: usize,)*)? T1, T2> CConv<T1, T2, $rhs> for $lhs
        where
            T1: ComplexFloat + Mul<T2, Output: ComplexFloat + From<<<T1 as Mul<T2>>::Output as ComplexFloat>::Real> + 'static>,
            T2: ComplexFloat,
            Complex<T1::Real>: From<T1> + AddAssign + MulAssign + Mul<Complex<T2::Real>, Output: ComplexFloat<Real = <<T1 as Mul<T2>>::Output as ComplexFloat>::Real> + MulAssign + AddAssign + MulAssign<<<T1 as Mul<T2>>::Output as ComplexFloat>::Real> + From<Complex<<<T1 as Mul<T2>>::Output as ComplexFloat>::Real>> + Sum + 'static>,
            Complex<T2::Real>: From<T2> + AddAssign + MulAssign,
            $($($w)*)?
        {
            type OutputT = <T1 as Mul<T2>>::Output;
            type Output = Vec<Vec<<T1 as Mul<T2>>::Output>>;
        
            #[inline]
            fn cconv(self, rhs: $rhs) -> Self::Output
            {
                rhs.iter()
                    .map(|c| ListOrSingle::<T1>::as_view_slice(&self).cconvolve_fft(ListOrSingle::<T2>::as_view_slice(c)))
                    .collect()
            }
        }
    };
    (($(<$($a:lifetime),* $(,)? $($c:ident),* >)?) $lhs:ty => [], $rhs:ty [] $(where $($w:tt)*)?) => {
        impl<$($($a,)* $(const $c: usize,)*)? T1, T2> CConv<T1, T2, $rhs> for $lhs
        where
            T1: ComplexFloat + Mul<T2, Output: ComplexFloat + From<<<T1 as Mul<T2>>::Output as ComplexFloat>::Real> + 'static>,
            T2: ComplexFloat,
            Complex<T1::Real>: From<T1> + AddAssign + MulAssign + Mul<Complex<T2::Real>, Output: ComplexFloat<Real = <<T1 as Mul<T2>>::Output as ComplexFloat>::Real> + MulAssign + AddAssign + MulAssign<<<T1 as Mul<T2>>::Output as ComplexFloat>::Real> + From<Complex<<<T1 as Mul<T2>>::Output as ComplexFloat>::Real>> + Sum + 'static>,
            Complex<T2::Real>: From<T2> + AddAssign + MulAssign,
            $($($w)*)?
        {
            type OutputT = <T1 as Mul<T2>>::Output;
            type Output = Vec<Vec<<T1 as Mul<T2>>::Output>>;

            #[inline]
            fn cconv(self, rhs: $rhs) -> Self::Output
            {
                self.iter()
                    .map(|c| ListOrSingle::<T1>::as_view_slice(c).cconvolve_fft(ListOrSingle::<T2>::as_view_slice(&rhs)))
                    .collect()
            }
        }
    };
}

impl_cconv!(() (), ());
impl_cconv!((<M>) (), [T2; M]);
impl_cconv!((<'b, M>) (), &'b [T2; M]);
impl_cconv!(() (), Vec<T2>);
impl_cconv!((<'b>) (), &'b [T2]);
impl_cconv!((<K, M>) (), [[T2; M]; K]);
impl_cconv!((<'b, K, M>) (), [&'b [T2; M]; K]);
impl_cconv!((<K>) (), [Vec<T2>; K]);
impl_cconv!((<'b, K>) (), [&'b [T2]; K]);
impl_cconv!((<'c, K, M>) (), &'c [[T2; M]; K]);
impl_cconv!((<'c, 'b, K, M>) (), &'c [&'b [T2; M]; K]);
impl_cconv!((<'c, K>) (), &'c [Vec<T2>; K]);
impl_cconv!((<'c, 'b, K>) (), &'c [&'b [T2]; K]);
impl_cconv!((<M>) (), Vec<[T2; M]>);
impl_cconv!((<'b, M>) (), Vec<&'b [T2; M]>);
impl_cconv!(() (), Vec<Vec<T2>>);
impl_cconv!((<'b>) (), Vec<&'b [T2]>);
impl_cconv!((<'c, M>) (), &'c [[T2; M]]);
impl_cconv!((<'c, 'b, M>) (), &'c [&'b [T2; M]]);
impl_cconv!((<'c>) (), &'c [Vec<T2>]);
impl_cconv!((<'c, 'b>) (), &'c [&'b [T2]]);

impl_cconv!((<N>) [T1; N], ());
impl_cconv!((<N, M>) [T1; N], [T2; M] [N, M]);
impl_cconv!((<'b, N, M>) [T1; N], &'b [T2; M] [N, M]);
impl_cconv!((<N>) [T1; N], Vec<T2> []);
impl_cconv!((<'b, N>) [T1; N], &'b [T2] []);
impl_cconv!((<K, N, M>) [T1; N], [[T2; M]; K] => [K] [N, M]);
impl_cconv!((<'b, K, N, M>) [T1; N], [&'b [T2; M]; K] => [K] [N, M]);
impl_cconv!((<K, N>) [T1; N], [Vec<T2>; K] => [K] []);
impl_cconv!((<'b, K, N>) [T1; N], [&'b [T2]; K] => [K] []);
impl_cconv!((<'c, K, N, M>) [T1; N], &'c [[T2; M]; K] => [K] [N, M]);
impl_cconv!((<'c, 'b, K, N, M>) [T1; N], &'c [&'b [T2; M]; K] => [K] [N, M]);
impl_cconv!((<'c, K, N>) [T1; N], &'c [Vec<T2>; K] => [K] []);
impl_cconv!((<'c, 'b, K, N>) [T1; N], &'c [&'b [T2]; K] => [K] []);
impl_cconv!((<N, M>) [T1; N], Vec<[T2; M]> => [] [N, M]);
impl_cconv!((<'b, N, M>) [T1; N], Vec<&'b [T2; M]> => [] [N, M]);
impl_cconv!((<N>) [T1; N], Vec<Vec<T2>> => [] []);
impl_cconv!((<'b, N>) [T1; N], Vec<&'b [T2]> => [] []);
impl_cconv!((<'c, N, M>) [T1; N], &'c [[T2; M]] => [] [N, M]);
impl_cconv!((<'c, 'b, N, M>) [T1; N], &'c [&'b [T2; M]] => [] [N, M]);
impl_cconv!((<'c, N>) [T1; N], &'c [Vec<T2>] => [] []);
impl_cconv!((<'c, 'b, N>) [T1; N], &'c [&'b [T2]] => [] []);

impl_cconv!((<'a, N>) &'a [T1; N], ());
impl_cconv!((<'a, N, M>) &'a [T1; N], [T2; M] [N, M]);
impl_cconv!((<'a, 'b N, M>) &'a [T1; N], &'b [T2; M] [N, M]);
impl_cconv!((<'a, N>) &'a [T1; N], Vec<T2> []);
impl_cconv!((<'a, 'b, N>) &'a [T1; N], &'b [T2] []);
impl_cconv!((<K, N, M>) &[T1; N], [[T2; M]; K] => [K] [N, M]);
impl_cconv!((<'b, K, N, M>) &[T1; N], [&'b [T2; M]; K] => [K] [N, M]);
impl_cconv!((<K, N>) &[T1; N], [Vec<T2>; K] => [K] []);
impl_cconv!((<'b, K, N>) &[T1; N], [&'b [T2]; K] => [K] []);
impl_cconv!((<'c, K, N, M>) &[T1; N], &'c [[T2; M]; K] => [K] [N, M]);
impl_cconv!((<'c, 'b, K, N, M>) &[T1; N], &'c [&'b [T2; M]; K] => [K] [N, M]);
impl_cconv!((<'c, K, N>) &[T1; N], &'c [Vec<T2>; K] => [K] []);
impl_cconv!((<'c, 'b, K, N>) &[T1; N], &'c [&'b [T2]; K] => [K] []);
impl_cconv!((<N, M>) &[T1; N], Vec<[T2; M]> => [] [N, M]);
impl_cconv!((<'b, N, M>) &[T1; N], Vec<&'b [T2; M]> => [] [N, M]);
impl_cconv!((<N>) &[T1; N], Vec<Vec<T2>> => [] []);
impl_cconv!((<'b, N>) &[T1; N], Vec<&'b [T2]> => [] []);
impl_cconv!((<'c, N, M>) &[T1; N], &'c [[T2; M]] => [] [N, M]);
impl_cconv!((<'c, 'b, N, M>) &[T1; N], &'c [&'b [T2; M]] => [] [N, M]);
impl_cconv!((<'c, N>) &[T1; N], &'c [Vec<T2>] => [] []);
impl_cconv!((<'c, 'b, N>) &[T1; N], &'c [&'b [T2]] => [] []);

impl_cconv!(() Vec<T1>, ());
impl_cconv!((<M>) Vec<T1>, [T2; M] []);
impl_cconv!((<'b, M>) Vec<T1>, &'b [T2; M] []);
impl_cconv!(() Vec<T1>, Vec<T2> []);
impl_cconv!((<'b>) Vec<T1>, &'b [T2] []);
impl_cconv!((<K, M>) Vec<T1>, [[T2; M]; K] => [K] []);
impl_cconv!((<'b, K, M>) Vec<T1>, [&'b [T2; M]; K] => [K] []);
impl_cconv!((<K>) Vec<T1>, [Vec<T2>; K] => [K] []);
impl_cconv!((<'b, K>) Vec<T1>, [&'b [T2]; K] => [K] []);
impl_cconv!((<'c, K, M>) Vec<T1>, &'c [[T2; M]; K] => [K] []);
impl_cconv!((<'c, 'b, K, M>) Vec<T1>, &'c [&'b [T2; M]; K] => [K] []);
impl_cconv!((<'c, K>) Vec<T1>, &'c [Vec<T2>; K] => [K] []);
impl_cconv!((<'c, 'b, K>) Vec<T1>, &'c [&'b [T2]; K] => [K] []);
impl_cconv!((<M>) Vec<T1>, Vec<[T2; M]> => [] []);
impl_cconv!((<'b, M>) Vec<T1>, Vec<&'b [T2; M]> => [] []);
impl_cconv!(() Vec<T1>, Vec<Vec<T2>> => [] []);
impl_cconv!((<'b>) Vec<T1>, Vec<&'b [T2]> => [] []);
impl_cconv!((<'c, M>) Vec<T1>, &'c [[T2; M]] => [] []);
impl_cconv!((<'c, 'b, M>) Vec<T1>, &'c [&'b [T2; M]] => [] []);
impl_cconv!((<'c>) Vec<T1>, &'c [Vec<T2>] => [] []);
impl_cconv!((<'c, 'b>) Vec<T1>, &'c [&'b [T2]] => [] []);

impl_cconv!((<'a>) &'a [T1], ());
impl_cconv!((<'a, M>) &'a [T1], [T2; M] []);
impl_cconv!((<'a, 'b, M>) &'a [T1], &'b [T2; M] []);
impl_cconv!((<'a>) &'a [T1], Vec<T2> []);
impl_cconv!((<'a, 'b>) &'a [T1], &'b [T2] []);
impl_cconv!((<K, M>) &[T1], [[T2; M]; K] => [K] []);
impl_cconv!((<'b, K, M>) &[T1], [&'b [T2; M]; K] => [K] []);
impl_cconv!((<K>) &[T1], [Vec<T2>; K] => [K] []);
impl_cconv!((<'b, K>) &[T1], [&'b [T2]; K] => [K] []);
impl_cconv!((<'c, K, M>) &[T1], &'c [[T2; M]; K] => [K] []);
impl_cconv!((<'c, 'b, K, M>) &[T1], &'c [&'b [T2; M]; K] => [K] []);
impl_cconv!((<'c, K>) &[T1], &'c [Vec<T2>; K] => [K] []);
impl_cconv!((<'c, 'b, K>) &[T1], &'c [&'b [T2]; K] => [K] []);
impl_cconv!((<M>) &[T1], Vec<[T2; M]> => [] []);
impl_cconv!((<'b, M>) &[T1], Vec<&'b [T2; M]> => [] []);
impl_cconv!(() &[T1], Vec<Vec<T2>> => [] []);
impl_cconv!((<'b>) &[T1], Vec<&'b [T2]> => [] []);
impl_cconv!((<'c, M>) &[T1], &'c [[T2; M]] => [] []);
impl_cconv!((<'c, 'b, M>) &[T1], &'c [&'b [T2; M]] => [] []);
impl_cconv!((<'c>) &[T1], &'c [Vec<T2>] => [] []);
impl_cconv!((<'c, 'b>) &[T1], &'c [&'b [T2]] => [] []);

impl_cconv!((<K, N>) [[T1; N]; K], ());
impl_cconv!((<K, N, M>) [[T1; N]; K] => [K], [T2; M] [N, M]);
impl_cconv!((<'b, K, N, M>) [[T1; N]; K] => [K], &'b [T2; M] [N, M]);
impl_cconv!((<K, N>) [[T1; N]; K] => [K], Vec<T2> []);
impl_cconv!((<'b, K, N>) [[T1; N]; K] => [K], &'b [T2] []);
impl_cconv!((<'c, K, N>) &'c [[T1; N]; K], ());
impl_cconv!((<K, N, M>) &[[T1; N]; K] => [K], [T2; M] [N, M]);
impl_cconv!((<'b, K, N, M>) &[[T1; N]; K] => [K], &'b [T2; M] [N, M]);
impl_cconv!((<K, N>) &[[T1; N]; K] => [K], Vec<T2> []);
impl_cconv!((<'b, K, N>) &[[T1; N]; K] => [K], &'b [T2] []);
impl_cconv!((<N>) Vec<[T1; N]>, ());
impl_cconv!((<N, M>) Vec<[T1; N]> => [], [T2; M] [N, M]);
impl_cconv!((<'b, N, M>) Vec<[T1; N]> => [], &'b [T2; M] [N, M]);
impl_cconv!((<N>) Vec<[T1; N]> => [], Vec<T2> []);
impl_cconv!((<'b, N>) Vec<[T1; N]> => [], &'b [T2] []);
impl_cconv!((<'c, N>) &'c [[T1; N]], ());
impl_cconv!((<N, M>) &[[T1; N]] => [], [T2; M] [N, M]);
impl_cconv!((<'b, N, M>) &[[T1; N]] => [], &'b [T2; M] [N, M]);
impl_cconv!((<N>) &[[T1; N]] => [], Vec<T2> []);
impl_cconv!((<'b, N>) &[[T1; N]] => [], &'b [T2] []);

impl_cconv!((<'a, K, N>) [&'a [T1; N]; K], ());
impl_cconv!((<K, N, M>) [&[T1; N]; K] => [K], [T2; M] [N, M]);
impl_cconv!((<'b, K, N, M>) [&[T1; N]; K] => [K], &'b [T2; M] [N, M]);
impl_cconv!((<K, N>) [&[T1; N]; K] => [K], Vec<T2> []);
impl_cconv!((<'b, K, N>) [&[T1; N]; K] => [K], &'b [T2] []);
impl_cconv!((<'a, 'c, K, N>) &'c [&'a [T1; N]; K], ());
impl_cconv!((<K, N, M>) &[&[T1; N]; K] => [K], [T2; M] [N, M]);
impl_cconv!((<'b, K, N, M>) &[&[T1; N]; K] => [K], &'b [T2; M] [N, M]);
impl_cconv!((<K, N>) &[&[T1; N]; K] => [K], Vec<T2> []);
impl_cconv!((<'b, K, N>) &[&[T1; N]; K] => [K], &'b [T2] []);
impl_cconv!((<'a, N>) Vec<&'a [T1; N]>, ());
impl_cconv!((<N, M>) Vec<&[T1; N]> => [], [T2; M] [N, M]);
impl_cconv!((<'b, N, M>) Vec<&[T1; N]> => [], &'b [T2; M] [N, M]);
impl_cconv!((<N>) Vec<&[T1; N]> => [], Vec<T2> []);
impl_cconv!((<'b, N>) Vec<&[T1; N]> => [], &'b [T2] []);
impl_cconv!((<'a, 'c, N>) &'c [&'a [T1; N]], ());
impl_cconv!((<N, M>) &[&[T1; N]] => [], [T2; M] [N, M]);
impl_cconv!((<'b, N, M>) &[&[T1; N]] => [], &'b [T2; M] [N, M]);
impl_cconv!((<N>) &[&[T1; N]] => [], Vec<T2> []);
impl_cconv!((<'b, N>) &[&[T1; N]] => [], &'b [T2] []);

impl_cconv!((<K>) [Vec<T1>; K], ());
impl_cconv!((<K, M>) [Vec<T1>; K] => [K], [T2; M] []);
impl_cconv!((<'b, K, M>) [Vec<T1>; K] => [K], &'b [T2; M] []);
impl_cconv!((<K>) [Vec<T1>; K] => [K], Vec<T2> []);
impl_cconv!((<'b, K>) [Vec<T1>; K] => [K], &'b [T2] []);
impl_cconv!((<'c, K>) &'c [Vec<T1>; K], ());
impl_cconv!((<K, M>) &[Vec<T1>; K] => [K], [T2; M] []);
impl_cconv!((<'b, K, M>) &[Vec<T1>; K] => [K], &'b [T2; M] []);
impl_cconv!((<K>) &[Vec<T1>; K] => [K], Vec<T2> []);
impl_cconv!((<'b, K>) &[Vec<T1>; K] => [K], &'b [T2] []);
impl_cconv!(() Vec<Vec<T1>>, ());
impl_cconv!((<M>) Vec<Vec<T1>> => [], [T2; M] []);
impl_cconv!((<'b, M>) Vec<Vec<T1>> => [], &'b [T2; M] []);
impl_cconv!(() Vec<Vec<T1>> => [], Vec<T2> []);
impl_cconv!((<'b>) Vec<Vec<T1>> => [], &'b [T2] []);
impl_cconv!((<'c>) &'c [Vec<T1>], ());
impl_cconv!((<M>) &[Vec<T1>] => [], [T2; M] []);
impl_cconv!((<'b, M>) &[Vec<T1>] => [], &'b [T2; M] []);
impl_cconv!(() &[Vec<T1>] => [], Vec<T2> []);
impl_cconv!((<'b>) &[Vec<T1>] => [], &'b [T2] []);

impl_cconv!((<'a, K>) [&'a [T1]; K], ());
impl_cconv!((<K, M>) [&[T1]; K] => [K], [T2; M] []);
impl_cconv!((<'b, K, M>) [&[T1]; K] => [K], &'b [T2; M] []);
impl_cconv!((<K>) [&[T1]; K] => [K], Vec<T2> []);
impl_cconv!((<'b, K>) [&[T1]; K] => [K], &'b [T2] []);
impl_cconv!((<'a, 'c, K>) &'c [&'a [T1]; K], ());
impl_cconv!((<K, M>) &[&[T1]; K] => [K], [T2; M] []);
impl_cconv!((<'b, K, M>) &[&[T1]; K] => [K], &'b [T2; M] []);
impl_cconv!((<K>) &[&[T1]; K] => [K], Vec<T2> []);
impl_cconv!((<'b, K>) &[&[T1]; K] => [K], &'b [T2] []);
impl_cconv!((<'a>) Vec<&'a [T1]>, ());
impl_cconv!((<M>) Vec<&[T1]> => [], [T2; M] []);
impl_cconv!((<'b, M>) Vec<&[T1]> => [], &'b [T2; M] []);
impl_cconv!(() Vec<&[T1]> => [], Vec<T2> []);
impl_cconv!((<'b>) Vec<&[T1]> => [], &'b [T2] []);
impl_cconv!((<'a, 'c>) &'c [&'a [T1]], ());
impl_cconv!((<M>) &[&[T1]] => [], [T2; M] []);
impl_cconv!((<'b, M>) &[&[T1]] => [], &'b [T2; M] []);
impl_cconv!(() &[&[T1]] => [], Vec<T2> []);
impl_cconv!((<'b>) &[&[T1]] => [], &'b [T2] []);

#[cfg(test)]
mod test
{
    use crate::operations::convolution::CConv;

    #[test]
    fn test()
    {
        let a: [f32; _] = [1.0, 2.0, 3.0];
        let b: [f32; _] = [2.0, 2.0];

        let c = a.cconv(b);

        println!("{:?}", c)
    }
}