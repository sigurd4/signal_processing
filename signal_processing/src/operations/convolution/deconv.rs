use num::{complex::ComplexFloat, One, Complex};
use core::ops::{Div, SubAssign, AddAssign, MulAssign};
use array_math::{max_len, ArrayMath, SliceMath};

use crate::quantities::{ListOrSingle, MaybeContainer};

pub trait Deconv<T, Rhs>: MaybeContainer<T>
where
    T: Div,
    Rhs: MaybeContainer<T>
{
    type Q: MaybeContainer<T>;
    type R: MaybeContainer<T>;
    type Output: ListOrSingle<Option<(Self::Q, Self::R)>>;

    fn deconv(self, rhs: Rhs) -> Self::Output;
}

macro_rules! impl_deconv {
    (($(<$($a:lifetime),* $(,)? $($c:ident),* >)?) (), $rhs:ty $(where $($w:tt)*)?) => {
        impl<$($($a,)* $(const $c: usize,)*)? T> Deconv<T, $rhs> for ()
        where
            T: Div + One,
            [T; 1]: Deconv<T, $rhs>,
            $($($w)*)?
        {
            type Q = <[T; 1] as Deconv<T, $rhs>>::Q;
            type R = <[T; 1] as Deconv<T, $rhs>>::R;
            type Output = <[T; 1] as Deconv<T, $rhs>>::Output;

            #[inline]
            fn deconv(self, rhs: $rhs) -> Self::Output
            {
                [T::one()].deconv(rhs)
            }
        }
    };
    (($(<$($a:lifetime),* $(,)? $($c:ident),* >)?) $lhs:ty, () $(where $($w:tt)*)?) => {
        impl<$($($a,)* $(const $c: usize,)*)? T> Deconv<T, ()> for $lhs
        where
            T: Div + One,
            $lhs: Deconv<T, [T; 1]>,
            $($($w)*)?
        {
            type Q = <$lhs as Deconv<T, [T; 1]>>::Q;
            type R = <$lhs as Deconv<T, [T; 1]>>::R;
            type Output = <$lhs as Deconv<T, [T; 1]>>::Output;

            #[inline]
            fn deconv(self, _: ()) -> Self::Output
            {
                self.deconv([T::one()])
            }
        }
    };
    (($(<$($a:lifetime),* $(,)? $($c:ident),* >)?) $lhs:ty, $rhs:ty [$n:tt, $m:tt] $(where $($w:tt)*)?) => {
        impl<$($($a,)* $(const $c: usize,)*)? T> Deconv<T, $rhs> for $lhs
        where
            T: ComplexFloat<Real: Into<T>> + SubAssign + AddAssign + Into<Complex<T::Real>> + 'static,
            Complex<T::Real>: AddAssign + MulAssign + MulAssign<T::Real>,
            [(); max_len($n, $m) - 1]:,
            [(); (($n + 1 - $m) + $m - 1).next_power_of_two()]:,
            [(); (($n + 1 - $m) + $m - 1).next_power_of_two() - ($n + 1 - $m)]:,
            [(); (($n + 1 - $m) + $m - 1).next_power_of_two() - $m]:,
            [(); (($n + 1 - $m) + $m - 1).next_power_of_two() - (($n + 1 - $m) + $m - 1)]:,
            $($($w)*)?
        {
            type Q = [T; $n + 1 - $m];
            type R = [T; $n];
            type Output = Option<(Self::Q, Self::R)>;
        
            fn deconv(self, rhs: $rhs) -> Self::Output
            {
                MaybeContainer::to_owned(&self)
                    .deconvolve_fft(MaybeContainer::to_owned(&rhs))
            }
        }
    };
    (($(<$($a:lifetime),* $(,)? $($c:ident),* >)?) $lhs:ty, $rhs:ty [] $(where $($w:tt)*)?) => {
        impl<$($($a,)* $(const $c: usize,)*)? T> Deconv<T, $rhs> for $lhs
        where
            T: ComplexFloat<Real: Into<T>> + SubAssign + AddAssign + Into<Complex<T::Real>> + 'static,
            Complex<T::Real>: AddAssign + MulAssign + MulAssign<T::Real>,
            Vec<T>: TryInto<<$lhs as MaybeContainer<T>>::Owned>,
            $($($w)*)?
        {
            type Q = Vec<T>;
            type R = <$lhs as MaybeContainer<T>>::Owned;
            type Output = Option<(Self::Q, Self::R)>;
        
            #[inline]
            fn deconv(self, rhs: $rhs) -> Self::Output
            {
                ListOrSingle::<T>::as_view_slice(&self)
                    .deconvolve_fft(ListOrSingle::<T>::as_view_slice(&rhs))
                    .map(|(q, r): (Vec<_>, Vec<_>)| (q, r.try_into().ok().unwrap()))
            }
        }
    };
    (($(<$($a:lifetime),* $(,)? $($c:ident),* >)?) $lhs:ty, $rhs:ty => [$k:tt] [$n:tt, $m:tt] $(where $($w:tt)*)?) => {
        impl<$($($a,)* $(const $c: usize,)*)? T> Deconv<T, $rhs> for $lhs
        where
            T: ComplexFloat<Real: Into<T>> + SubAssign + AddAssign + Into<Complex<T::Real>> + 'static,
            Complex<T::Real>: AddAssign + MulAssign + MulAssign<T::Real>,
            [(); max_len($n, $m) - 1]:,
            [(); (($n + 1 - $m) + $m - 1).next_power_of_two()]:,
            [(); (($n + 1 - $m) + $m - 1).next_power_of_two() - ($n + 1 - $m)]:,
            [(); (($n + 1 - $m) + $m - 1).next_power_of_two() - $m]:,
            [(); (($n + 1 - $m) + $m - 1).next_power_of_two() - (($n + 1 - $m) + $m - 1)]:,
            $($($w)*)?
        {
            type Q = [T; $n + 1 - $m];
            type R = [T; $n];
            type Output = [Option<(Self::Q, Self::R)>; $k];
        
            #[inline]
            fn deconv(self, rhs: $rhs) -> Self::Output
            {
                rhs.each_ref()
                    .map(|c| MaybeContainer::to_owned(&self).deconvolve_fft(MaybeContainer::to_owned(c)))
            }
        }
    };
    (($(<$($a:lifetime),* $(,)? $($c:ident),* >)?) $lhs:ty => [$k:tt], $rhs:ty [$n:tt, $m:tt] $(where $($w:tt)*)?) => {
        impl<$($($a,)* $(const $c: usize,)*)? T> Deconv<T, $rhs> for $lhs
        where
            T: ComplexFloat<Real: Into<T>> + SubAssign + AddAssign + Into<Complex<T::Real>> + 'static,
            Complex<T::Real>: AddAssign + MulAssign + MulAssign<T::Real>,
            [(); max_len($n, $m) - 1]:,
            [(); (($n + 1 - $m) + $m - 1).next_power_of_two()]:,
            [(); (($n + 1 - $m) + $m - 1).next_power_of_two() - ($n + 1 - $m)]:,
            [(); (($n + 1 - $m) + $m - 1).next_power_of_two() - $m]:,
            [(); (($n + 1 - $m) + $m - 1).next_power_of_two() - (($n + 1 - $m) + $m - 1)]:,
            $($($w)*)?
        {
            type Q = [T; $n + 1 - $m];
            type R = [T; $n];
            type Output = [Option<(Self::Q, Self::R)>; $k];
        
            #[inline]
            fn deconv(self, rhs: $rhs) -> Self::Output
            {
                self.each_ref()
                    .map(|c| MaybeContainer::to_owned(c).deconvolve_fft(MaybeContainer::to_owned(&rhs)))
            }
        }
    };
    (($(<$($a:lifetime),* $(,)? $($c:ident),* >)?) $lhs:ty, $rhs:ty => [] [$n:tt, $m:tt] $(where $($w:tt)*)?) => {
        impl<$($($a,)* $(const $c: usize,)*)? T> Deconv<T, $rhs> for $lhs
        where
            T: ComplexFloat<Real: Into<T>> + SubAssign + AddAssign + Into<Complex<T::Real>> + 'static,
            Complex<T::Real>: AddAssign + MulAssign + MulAssign<T::Real>,
            [(); max_len($n, $m) - 1]:,
            [(); (($n + 1 - $m) + $m - 1).next_power_of_two()]:,
            [(); (($n + 1 - $m) + $m - 1).next_power_of_two() - ($n + 1 - $m)]:,
            [(); (($n + 1 - $m) + $m - 1).next_power_of_two() - $m]:,
            [(); (($n + 1 - $m) + $m - 1).next_power_of_two() - (($n + 1 - $m) + $m - 1)]:,
            $($($w)*)?
        {
            type Q = [T; $n + 1 - $m];
            type R = [T; $n];
            type Output = Vec<Option<(Self::Q, Self::R)>>;
        
            #[inline]
            fn deconv(self, rhs: $rhs) -> Self::Output
            {
                rhs.iter()
                    .map(|c| MaybeContainer::to_owned(&self).deconvolve_fft(MaybeContainer::to_owned(c)))
                    .collect()
            }
        }
    };
    (($(<$($a:lifetime),* $(,)? $($c:ident),* >)?) $lhs:ty => [], $rhs:ty [$n:tt, $m:tt] $(where $($w:tt)*)?) => {
        impl<$($($a,)* $(const $c: usize,)*)? T> Deconv<T, $rhs> for $lhs
        where
            T: ComplexFloat<Real: Into<T>> + SubAssign + AddAssign + Into<Complex<T::Real>> + 'static,
            Complex<T::Real>: AddAssign + MulAssign + MulAssign<T::Real>,
            [(); max_len($n, $m) - 1]:,
            [(); (($n + 1 - $m) + $m - 1).next_power_of_two()]:,
            [(); (($n + 1 - $m) + $m - 1).next_power_of_two() - ($n + 1 - $m)]:,
            [(); (($n + 1 - $m) + $m - 1).next_power_of_two() - $m]:,
            [(); (($n + 1 - $m) + $m - 1).next_power_of_two() - (($n + 1 - $m) + $m - 1)]:,
            $($($w)*)?
        {
            type Q = [T; $n + 1 - $m];
            type R = [T; $n];
            type Output = Vec<Option<(Self::Q, Self::R)>>;
        
            #[inline]
            fn deconv(self, rhs: $rhs) -> Self::Output
            {
                self.iter()
                    .map(|c| MaybeContainer::to_owned(c).deconvolve_fft(MaybeContainer::to_owned(&rhs)))
                    .collect()
            }
        }
    };
    (($(<$($a:lifetime),* $(,)? $($c:ident),* >)?) $lhs:ty, $rhs:ty => [$k:tt] [] $(where $($w:tt)*)?) => {
        impl<$($($a,)* $(const $c: usize,)*)? T> Deconv<T, $rhs> for $lhs
        where
            T: ComplexFloat<Real: Into<T>> + SubAssign + AddAssign + Into<Complex<T::Real>> + 'static,
            Complex<T::Real>: AddAssign + MulAssign + MulAssign<T::Real>,
            Vec<T>: TryInto<<$lhs as MaybeContainer<T>>::Owned>,
            $($($w)*)?
        {
            type Q = Vec<T>;
            type R = <$lhs as MaybeContainer<T>>::Owned;
            type Output = [Option<(Self::Q, Self::R)>; $k];
        
            #[inline]
            fn deconv(self, rhs: $rhs) -> Self::Output
            {
                rhs.each_ref()
                    .map(|c| ListOrSingle::<T>::as_view_slice(&self)
                        .deconvolve_fft(ListOrSingle::<T>::as_view_slice(c))
                        .map(|(q, r): (Vec<_>, Vec<_>)| (q, r.try_into().ok().unwrap()))
                    )
            }
        }
    };
    (($(<$($a:lifetime),* $(,)? $($c:ident),* >)?) $lhs:ty => [$k:tt], $rhs:ty [] $(where $($w:tt)*)?) => {
        impl<$($($a,)* $(const $c: usize,)*)? T> Deconv<T, $rhs> for $lhs
        where
            T: ComplexFloat<Real: Into<T>> + SubAssign + AddAssign + Into<Complex<T::Real>> + 'static,
            Complex<T::Real>: AddAssign + MulAssign + MulAssign<T::Real>,
            Vec<T>: TryInto<<$lhs as MaybeContainer<T>>::Owned>,
            $($($w)*)?
        {
            type Q = Vec<T>;
            type R = <$lhs as MaybeContainer<T>>::Owned;
            type Output = [Option<(Self::Q, Self::R)>; $k];
        
            #[inline]
            fn deconv(self, rhs: $rhs) -> Self::Output
            {
                self.each_ref()
                    .map(|c| ListOrSingle::<T>::as_view_slice(c)
                        .deconvolve_fft(ListOrSingle::<T>::as_view_slice(&rhs))
                        .map(|(q, r): (Vec<_>, Vec<_>)| (q, r.try_into().ok().unwrap()))
                    )
            }
        }
    };
    (($(<$($a:lifetime),* $(,)? $($c:ident),* >)?) $lhs:ty, $rhs:ty => [] [] $(where $($w:tt)*)?) => {
        impl<$($($a,)* $(const $c: usize,)*)? T> Deconv<T, $rhs> for $lhs
        where
            T: ComplexFloat<Real: Into<T>> + SubAssign + AddAssign + Into<Complex<T::Real>> + 'static,
            Complex<T::Real>: AddAssign + MulAssign + MulAssign<T::Real>,
            Vec<T>: TryInto<<$lhs as MaybeContainer<T>>::Owned>,
            $($($w)*)?
        {
            type Q = Vec<T>;
            type R = <$lhs as MaybeContainer<T>>::Owned;
            type Output = Vec<Option<(Self::Q, Self::R)>>;
        
            #[inline]
            fn deconv(self, rhs: $rhs) -> Self::Output
            {
                rhs.iter()
                    .map(|c| ListOrSingle::<T>::as_view_slice(&self)
                        .deconvolve_fft(ListOrSingle::<T>::as_view_slice(c))
                        .map(|(q, r): (Vec<_>, Vec<_>)| (q, r.try_into().ok().unwrap()))
                    ).collect()
            }
        }
    };
    (($(<$($a:lifetime),* $(,)? $($c:ident),* >)?) $lhs:ty => [], $rhs:ty [] $(where $($w:tt)*)?) => {
        impl<$($($a,)* $(const $c: usize,)*)? T> Deconv<T, $rhs> for $lhs
        where
            T: ComplexFloat<Real: Into<T>> + SubAssign + AddAssign + Into<Complex<T::Real>> + 'static,
            Complex<T::Real>: AddAssign + MulAssign + MulAssign<T::Real>,
            Vec<T>: TryInto<<$lhs as MaybeContainer<T>>::Owned>,
            $($($w)*)?
        {
            type Q = Vec<T>;
            type R = <$lhs as MaybeContainer<T>>::Owned;
            type Output = Vec<Option<(Self::Q, Self::R)>>;

            #[inline]
            fn deconv(self, rhs: $rhs) -> Self::Output
            {
                self.iter()
                    .map(|c| ListOrSingle::<T>::as_view_slice(c)
                        .deconvolve_fft(ListOrSingle::<T>::as_view_slice(&rhs))
                        .map(|(q, r): (Vec<_>, Vec<_>)| (q, r.try_into().ok().unwrap()))
                    ).collect()
            }
        }
    };
}

impl_deconv!(() (), ());
impl_deconv!((<M>) (), [T; M]);
impl_deconv!((<'b, M>) (), &'b [T; M]);
impl_deconv!(() (), Vec<T>);
impl_deconv!((<'b>) (), &'b [T]);
impl_deconv!((<K, M>) (), [[T; M]; K]);
impl_deconv!((<'b, K, M>) (), [&'b [T; M]; K]);
impl_deconv!((<K>) (), [Vec<T>; K]);
impl_deconv!((<'b, K>) (), [&'b [T]; K]);
impl_deconv!((<'c, K, M>) (), &'c [[T; M]; K]);
impl_deconv!((<'c, 'b, K, M>) (), &'c [&'b [T; M]; K]);
impl_deconv!((<'c, K>) (), &'c [Vec<T>; K]);
impl_deconv!((<'c, 'b, K>) (), &'c [&'b [T]; K]);
impl_deconv!((<M>) (), Vec<[T; M]>);
impl_deconv!((<'b, M>) (), Vec<&'b [T; M]>);
impl_deconv!(() (), Vec<Vec<T>>);
impl_deconv!((<'b>) (), Vec<&'b [T]>);
impl_deconv!((<'c, M>) (), &'c [[T; M]]);
impl_deconv!((<'c, 'b, M>) (), &'c [&'b [T; M]]);
impl_deconv!((<'c>) (), &'c [Vec<T>]);
impl_deconv!((<'c, 'b>) (), &'c [&'b [T]]);

impl_deconv!((<N>) [T; N], ());
impl_deconv!((<N, M>) [T; N], [T; M] [N, M]);
impl_deconv!((<'b, N, M>) [T; N], &'b [T; M] [N, M]);
impl_deconv!((<N>) [T; N], Vec<T> []);
impl_deconv!((<'b, N>) [T; N], &'b [T] []);
impl_deconv!((<K, N, M>) [T; N], [[T; M]; K] => [K] [N, M]);
impl_deconv!((<'b, K, N, M>) [T; N], [&'b [T; M]; K] => [K] [N, M]);
impl_deconv!((<K, N>) [T; N], [Vec<T>; K] => [K] []);
impl_deconv!((<'b, K, N>) [T; N], [&'b [T]; K] => [K] []);
impl_deconv!((<'c, K, N, M>) [T; N], &'c [[T; M]; K] => [K] [N, M]);
impl_deconv!((<'c, 'b, K, N, M>) [T; N], &'c [&'b [T; M]; K] => [K] [N, M]);
impl_deconv!((<'c, K, N>) [T; N], &'c [Vec<T>; K] => [K] []);
impl_deconv!((<'c, 'b, K, N>) [T; N], &'c [&'b [T]; K] => [K] []);
impl_deconv!((<N, M>) [T; N], Vec<[T; M]> => [] [N, M]);
impl_deconv!((<'b, N, M>) [T; N], Vec<&'b [T; M]> => [] [N, M]);
impl_deconv!((<N>) [T; N], Vec<Vec<T>> => [] []);
impl_deconv!((<'b, N>) [T; N], Vec<&'b [T]> => [] []);
impl_deconv!((<'c, N, M>) [T; N], &'c [[T; M]] => [] [N, M]);
impl_deconv!((<'c, 'b, N, M>) [T; N], &'c [&'b [T; M]] => [] [N, M]);
impl_deconv!((<'c, N>) [T; N], &'c [Vec<T>] => [] []);
impl_deconv!((<'c, 'b, N>) [T; N], &'c [&'b [T]] => [] []);

impl_deconv!((<'a, N>) &'a [T; N], ());
impl_deconv!((<'a, N, M>) &'a [T; N], [T; M] [N, M]);
impl_deconv!((<'a, 'b N, M>) &'a [T; N], &'b [T; M] [N, M]);
impl_deconv!((<'a, N>) &'a [T; N], Vec<T> []);
impl_deconv!((<'a, 'b, N>) &'a [T; N], &'b [T] []);
impl_deconv!((<K, N, M>) &[T; N], [[T; M]; K] => [K] [N, M]);
impl_deconv!((<'b, K, N, M>) &[T; N], [&'b [T; M]; K] => [K] [N, M]);
impl_deconv!((<'a, K, N>) &'a [T; N], [Vec<T>; K] => [K] []);
impl_deconv!((<'b, 'a, K, N>) &'a [T; N], [&'b [T]; K] => [K] []);
impl_deconv!((<'c, 'a, K, N, M>) &'a [T; N], &'c [[T; M]; K] => [K] [N, M]);
impl_deconv!((<'c, 'b, 'a, K, N, M>) &'a [T; N], &'c [&'b [T; M]; K] => [K] [N, M]);
impl_deconv!((<'c, 'a, K, N>) &'a [T; N], &'c [Vec<T>; K] => [K] []);
impl_deconv!((<'c, 'b, 'a, K, N>) &'a [T; N], &'c [&'b [T]; K] => [K] []);
impl_deconv!((<'a, N, M>) &'a [T; N], Vec<[T; M]> => [] [N, M]);
impl_deconv!((<'b, 'a, N, M>) &'a [T; N], Vec<&'b [T; M]> => [] [N, M]);
impl_deconv!((<'a, N>) &'a [T; N], Vec<Vec<T>> => [] []);
impl_deconv!((<'b, 'a, N>) &'a [T; N], Vec<&'b [T]> => [] []);
impl_deconv!((<'c, 'a, N, M>) &'a [T; N], &'c [[T; M]] => [] [N, M]);
impl_deconv!((<'c, 'b, 'a, N, M>) &'a [T; N], &'c [&'b [T; M]] => [] [N, M]);
impl_deconv!((<'c, 'a, N>) &'a [T; N], &'c [Vec<T>] => [] []);
impl_deconv!((<'c, 'b, 'a, N>) &'a [T; N], &'c [&'b [T]] => [] []);

impl_deconv!(() Vec<T>, ());
impl_deconv!((<M>) Vec<T>, [T; M] []);
impl_deconv!((<'b, M>) Vec<T>, &'b [T; M] []);
impl_deconv!(() Vec<T>, Vec<T> []);
impl_deconv!((<'b>) Vec<T>, &'b [T] []);
impl_deconv!((<K, M>) Vec<T>, [[T; M]; K] => [K] []);
impl_deconv!((<'b, K, M>) Vec<T>, [&'b [T; M]; K] => [K] []);
impl_deconv!((<K>) Vec<T>, [Vec<T>; K] => [K] []);
impl_deconv!((<'b, K>) Vec<T>, [&'b [T]; K] => [K] []);
impl_deconv!((<'c, K, M>) Vec<T>, &'c [[T; M]; K] => [K] []);
impl_deconv!((<'c, 'b, K, M>) Vec<T>, &'c [&'b [T; M]; K] => [K] []);
impl_deconv!((<'c, K>) Vec<T>, &'c [Vec<T>; K] => [K] []);
impl_deconv!((<'c, 'b, K>) Vec<T>, &'c [&'b [T]; K] => [K] []);
impl_deconv!((<M>) Vec<T>, Vec<[T; M]> => [] []);
impl_deconv!((<'b, M>) Vec<T>, Vec<&'b [T; M]> => [] []);
impl_deconv!(() Vec<T>, Vec<Vec<T>> => [] []);
impl_deconv!((<'b>) Vec<T>, Vec<&'b [T]> => [] []);
impl_deconv!((<'c, M>) Vec<T>, &'c [[T; M]] => [] []);
impl_deconv!((<'c, 'b, M>) Vec<T>, &'c [&'b [T; M]] => [] []);
impl_deconv!((<'c>) Vec<T>, &'c [Vec<T>] => [] []);
impl_deconv!((<'c, 'b>) Vec<T>, &'c [&'b [T]] => [] []);

impl_deconv!((<'a>) &'a [T], ());
impl_deconv!((<'a, M>) &'a [T], [T; M] []);
impl_deconv!((<'a, 'b, M>) &'a [T], &'b [T; M] []);
impl_deconv!((<'a>) &'a [T], Vec<T> []);
impl_deconv!((<'a, 'b>) &'a [T], &'b [T] []);
impl_deconv!((<'a, K, M>) &'a [T], [[T; M]; K] => [K] []);
impl_deconv!((<'b, 'a, K, M>) &'a [T], [&'b [T; M]; K] => [K] []);
impl_deconv!((<'a, K>) &'a [T], [Vec<T>; K] => [K] []);
impl_deconv!((<'b, 'a, K>) &'a [T], [&'b [T]; K] => [K] []);
impl_deconv!((<'c, 'a, K, M>) &'a [T], &'c [[T; M]; K] => [K] []);
impl_deconv!((<'c, 'b, 'a, K, M>) &'a [T], &'c [&'b [T; M]; K] => [K] []);
impl_deconv!((<'c, 'a, K>) &'a [T], &'c [Vec<T>; K] => [K] []);
impl_deconv!((<'c, 'b, 'a, K>) &'a [T], &'c [&'b [T]; K] => [K] []);
impl_deconv!((<'a, M>) &'a [T], Vec<[T; M]> => [] []);
impl_deconv!((<'b, 'a, M>) &'a [T], Vec<&'b [T; M]> => [] []);
impl_deconv!((<'a>) &'a [T], Vec<Vec<T>> => [] []);
impl_deconv!((<'b, 'a>) &'a [T], Vec<&'b [T]> => [] []);
impl_deconv!((<'c, 'a, M>) &'a [T], &'c [[T; M]] => [] []);
impl_deconv!((<'c, 'b, 'a, M>) &'a [T], &'c [&'b [T; M]] => [] []);
impl_deconv!((<'c, 'a>) &'a [T], &'c [Vec<T>] => [] []);
impl_deconv!((<'c, 'b, 'a>) &'a [T], &'c [&'b [T]] => [] []);

impl_deconv!((<K, N>) [[T; N]; K], ());
impl_deconv!((<K, N, M>) [[T; N]; K] => [K], [T; M] [N, M]);
impl_deconv!((<'b, K, N, M>) [[T; N]; K] => [K], &'b [T; M] [N, M]);
impl_deconv!((<K, N>) [[T; N]; K] => [K], Vec<T> []);
impl_deconv!((<'b, K, N>) [[T; N]; K] => [K], &'b [T] []);
impl_deconv!((<'c, K, N>) &'c [[T; N]; K], ());
impl_deconv!((<K, N, M>) &[[T; N]; K] => [K], [T; M] [N, M]);
impl_deconv!((<'b, K, N, M>) &[[T; N]; K] => [K], &'b [T; M] [N, M]);
impl_deconv!((<'a, K, N>) &'a [[T; N]; K] => [K], Vec<T> []);
impl_deconv!((<'b, 'a, K, N>) &'a [[T; N]; K] => [K], &'b [T] []);
impl_deconv!((<N>) Vec<[T; N]>, ());
impl_deconv!((<N, M>) Vec<[T; N]> => [], [T; M] [N, M]);
impl_deconv!((<'b, N, M>) Vec<[T; N]> => [], &'b [T; M] [N, M]);
impl_deconv!((<N>) Vec<[T; N]> => [], Vec<T> []);
impl_deconv!((<'b, N>) Vec<[T; N]> => [], &'b [T] []);
impl_deconv!((<'c, N>) &'c [[T; N]], ());
impl_deconv!((<N, M>) &[[T; N]] => [], [T; M] [N, M]);
impl_deconv!((<'b, N, M>) &[[T; N]] => [], &'b [T; M] [N, M]);
impl_deconv!((<'a, N>) &'a [[T; N]] => [], Vec<T> []);
impl_deconv!((<'b, 'a, N>) &'a [[T; N]] => [], &'b [T] []);

impl_deconv!((<'a, K, N>) [&'a [T; N]; K], ());
impl_deconv!((<K, N, M>) [&[T; N]; K] => [K], [T; M] [N, M]);
impl_deconv!((<'b, K, N, M>) [&[T; N]; K] => [K], &'b [T; M] [N, M]);
impl_deconv!((<'a, K, N>) [&'a [T; N]; K] => [K], Vec<T> []);
impl_deconv!((<'b, 'a, K, N>) [&'a [T; N]; K] => [K], &'b [T] []);
impl_deconv!((<'a, 'c, K, N>) &'c [&'a [T; N]; K], ());
impl_deconv!((<K, N, M>) &[&[T; N]; K] => [K], [T; M] [N, M]);
impl_deconv!((<'b, K, N, M>) &[&[T; N]; K] => [K], &'b [T; M] [N, M]);
impl_deconv!((<'b, 'a, K, N>) &'b [&'a [T; N]; K] => [K], Vec<T> []);
impl_deconv!((<'c, 'b, 'a K, N>) &'c [&'a [T; N]; K] => [K], &'b [T] []);
impl_deconv!((<'a, N>) Vec<&'a [T; N]>, ());
impl_deconv!((<N, M>) Vec<&[T; N]> => [], [T; M] [N, M]);
impl_deconv!((<'b, N, M>) Vec<&[T; N]> => [], &'b [T; M] [N, M]);
impl_deconv!((<'a, N>) Vec<&'a [T; N]> => [], Vec<T> []);
impl_deconv!((<'b, 'a, N>) Vec<&'a [T; N]> => [], &'b [T] []);
impl_deconv!((<'a, 'c, N>) &'c [&'a [T; N]], ());
impl_deconv!((<N, M>) &[&[T; N]] => [], [T; M] [N, M]);
impl_deconv!((<'b, N, M>) &[&[T; N]] => [], &'b [T; M] [N, M]);
impl_deconv!((<'c, 'a, N>) &'c [&'a [T; N]] => [], Vec<T> []);
impl_deconv!((<'c, 'b, 'a, N>) &'c [&'a [T; N]] => [], &'b [T] []);

impl_deconv!((<K>) [Vec<T>; K], ());
impl_deconv!((<K, M>) [Vec<T>; K] => [K], [T; M] []);
impl_deconv!((<'b, K, M>) [Vec<T>; K] => [K], &'b [T; M] []);
impl_deconv!((<K>) [Vec<T>; K] => [K], Vec<T> []);
impl_deconv!((<'b, K>) [Vec<T>; K] => [K], &'b [T] []);
impl_deconv!((<'c, K>) &'c [Vec<T>; K], ());
impl_deconv!((<'c, K, M>) &'c [Vec<T>; K] => [K], [T; M] []);
impl_deconv!((<'c, 'b, K, M>) &'c [Vec<T>; K] => [K], &'b [T; M] []);
impl_deconv!((<'c, K>) &'c [Vec<T>; K] => [K], Vec<T> []);
impl_deconv!((<'c, 'b, K>) &'c [Vec<T>; K] => [K], &'b [T] []);
impl_deconv!(() Vec<Vec<T>>, ());
impl_deconv!((<M>) Vec<Vec<T>> => [], [T; M] []);
impl_deconv!((<'b, M>) Vec<Vec<T>> => [], &'b [T; M] []);
impl_deconv!(() Vec<Vec<T>> => [], Vec<T> []);
impl_deconv!((<'b>) Vec<Vec<T>> => [], &'b [T] []);
impl_deconv!((<'c>) &'c [Vec<T>], ());
impl_deconv!((<'c, M>) &'c [Vec<T>] => [], [T; M] []);
impl_deconv!((<'c, 'b, M>) &'c [Vec<T>] => [], &'b [T; M] []);
impl_deconv!((<'c>) &'c [Vec<T>] => [], Vec<T> []);
impl_deconv!((<'c, 'b>) &'c [Vec<T>] => [], &'b [T] []);

impl_deconv!((<'a, K>) [&'a [T]; K], ());
impl_deconv!((<'a, K, M>) [&'a [T]; K] => [K], [T; M] []);
impl_deconv!((<'b, 'a, K, M>) [&'a [T]; K] => [K], &'b [T; M] []);
impl_deconv!((<'a, K>) [&'a [T]; K] => [K], Vec<T> []);
impl_deconv!((<'b, 'a, K>) [&'a [T]; K] => [K], &'b [T] []);
impl_deconv!((<'a, 'c, K>) &'c [&'a [T]; K], ());
impl_deconv!((<'c, 'a, K, M>) &'c [&'a [T]; K] => [K], [T; M] []);
impl_deconv!((<'c, 'b, 'a, K, M>) &'c [&'a [T]; K] => [K], &'b [T; M] []);
impl_deconv!((<'c, 'a, K>) &'c [&'a [T]; K] => [K], Vec<T> []);
impl_deconv!((<'c, 'b, 'a, K>) &'c [&'a [T]; K] => [K], &'b [T] []);
impl_deconv!((<'a>) Vec<&'a [T]>, ());
impl_deconv!((<'a, M>) Vec<&'a [T]> => [], [T; M] []);
impl_deconv!((<'b, 'a, M>) Vec<&'a [T]> => [], &'b [T; M] []);
impl_deconv!((<'a>) Vec<&'a [T]> => [], Vec<T> []);
impl_deconv!((<'b, 'a>) Vec<&'a [T]> => [], &'b [T] []);
impl_deconv!((<'a, 'c>) &'c [&'a [T]], ());
impl_deconv!((<'c, 'a, M>) &'c [&'a [T]] => [], [T; M] []);
impl_deconv!((<'c, 'b, 'a, M>) &'c [&'a [T]] => [], &'b [T; M] []);
impl_deconv!((<'c, 'a>) &'c [&'a [T]] => [], Vec<T> []);
impl_deconv!((<'c, 'b, 'a>) &'c [&'a [T]] => [], &'b [T] []);

#[cfg(test)]
mod test
{
    use crate::operations::convolution::{Conv, Deconv};

    #[test]
    fn test()
    {
        let x: [f64; _] = [1.0, 2.0, 3.0, 4.0];
        let h: [f64; _] = [1.0, 1.0];
        let y = x.conv(h);
        let (x, r) = y.deconv(h).unwrap();

        println!("{:?}, {:?}", x, r);
    }
}