use core::{iter::Sum, ops::{AddAssign, Mul, MulAssign}};

use array_math::{MatrixMath, ArrayOps};
use ndarray::{Array1, Array2, ArrayView1, ArrayView2};
use num::{complex::ComplexFloat, Complex, Zero};

use crate::{Dft2d, Idft2d, Matrix, MaybeContainer, MaybeLists, MaybeMatrix, TruncateIm, Container, Conv};

pub trait Conv2d<T1, T2, Rhs>: MaybeMatrix<T1>
where
    T1: Mul<T2>,
    Rhs: MaybeMatrix<T2>
{
    type Output: MaybeMatrix<<T1 as Mul<T2>>::Output>;

    fn conv_2d(self, rhs: Rhs) -> Self::Output;
}

macro_rules! impl_conv_1d {
    (($(<$($a:lifetime),* $(,)? $($c:ident),*>)?) $lhs:ty, $rhs:ty $(where $($where:tt)+)?) => {
        impl<$($($a,)* $(const $c: usize,)*)? T1, T2, T3> Conv2d<T1, T2, $rhs> for $lhs
        where
            T1: Mul<T2, Output = T3>,
            $lhs: Conv<T1, T2, $rhs>,
            <$lhs as Conv<T1, T2, $rhs>>::Output: MaybeMatrix<T3>
        {
            type Output = <$lhs as Conv<T1, T2, $rhs>>::Output;

            fn conv_2d(self, rhs: $rhs) -> Self::Output
            {
                self.conv(rhs)
            }
        }
    };
}

impl_conv_1d!(() (), ());
impl_conv_1d!((<N2>) (), [T2; N2]);
impl_conv_1d!((<'b, N2>) (), &'b [T2; N2]);
impl_conv_1d!(() (), Vec<T2>);
impl_conv_1d!((<'b>) (), &'b [T2]);
impl_conv_1d!(() (), Array1<T2>);
impl_conv_1d!((<'b>) (), ArrayView1<'b, T2>);

impl_conv_1d!((<N1>) [T1; N1], ());
impl_conv_1d!((<N1, N2>) [T1; N1], [T2; N2]);
impl_conv_1d!((<'b, N1, N2>) [T1; N1], &'b [T2; N2]);
impl_conv_1d!((<N1>) [T1; N1], Vec<T2>);
impl_conv_1d!((<'b, N1>) [T1; N1], &'b [T2]);
impl_conv_1d!((<N1>) [T1; N1], Array1<T2>);
impl_conv_1d!((<'b, N1>) [T1; N1], ArrayView1<'b, T2>);

impl_conv_1d!((<'a, N1>) &'a [T1; N1], ());
impl_conv_1d!((<'a, N1, N2>) &'a [T1; N1], [T2; N2]);
impl_conv_1d!((<'a, 'b, N1, N2>) &'a [T1; N1], &'b [T2; N2]);
impl_conv_1d!((<'a, N1>) &'a [T1; N1], Vec<T2>);
impl_conv_1d!((<'a, 'b, N1>) &'a [T1; N1], &'b [T2]);
impl_conv_1d!((<'a, N1>) &'a [T1; N1], Array1<T2>);
impl_conv_1d!((<'a, 'b, N1>) &'a [T1; N1], ArrayView1<'b, T2>);

impl_conv_1d!(() Vec<T1>, ());
impl_conv_1d!((<N2>) Vec<T1>, [T2; N2]);
impl_conv_1d!((<'b, N2>) Vec<T1>, &'b [T2; N2]);
impl_conv_1d!(() Vec<T1>, Vec<T2>);
impl_conv_1d!((<'b>) Vec<T1>, &'b [T2]);
impl_conv_1d!(() Vec<T1>, Array1<T2>);
impl_conv_1d!((<'b>) Vec<T1>, ArrayView1<'b, T2>);

impl_conv_1d!((<'a>) &'a [T1], ());
impl_conv_1d!((<'a, N2>) &'a [T1], [T2; N2]);
impl_conv_1d!((<'a, 'b, N2>) &'a [T1], &'b [T2; N2]);
impl_conv_1d!((<'a>) &'a [T1], Vec<T2>);
impl_conv_1d!((<'a, 'b>) &'a [T1], &'b [T2]);
impl_conv_1d!((<'a>) &'a [T1], Array1<T2>);
impl_conv_1d!((<'a, 'b>) &'a [T1], ArrayView1<'b, T2>);

impl_conv_1d!((<N2, M2>) (), [[T2; N2]; M2]);
impl_conv_1d!((<'bn, N2, M2>) (), [&'bn [T2; N2]; M2]);
impl_conv_1d!((<'bm, N2, M2>) (), &'bm [[T2; N2]; M2]);
impl_conv_1d!((<'bm, 'bn, N2, M2>) (), &'bm [&'bn [T2; N2]; M2]);
impl_conv_1d!((<N2>) (), Vec<[T2; N2]>);
impl_conv_1d!((<'bn, N2>) (), Vec<&'bn [T2; N2]>);
impl_conv_1d!((<'bm, N2>) (), &'bm [[T2; N2]]);
impl_conv_1d!((<'bm, 'bn, N2>) (), &'bm [&'bn [T2; N2]]);
impl_conv_1d!(() (), Array2<T2>);
impl_conv_1d!((<'b>) (), ArrayView2<'b, T2>);

impl_conv_1d!((<N1, M1>) [[T1; N1]; M1], ());
impl_conv_1d!((<'bn, N1, M1>) [&'bn [T1; N1]; M1], ());
impl_conv_1d!((<'bm, N1, M1>) &'bm [[T1; N1]; M1], ());
impl_conv_1d!((<'bm, 'bn, N1, M1>) &'bm [&'bn [T1; N1]; M1], ());
impl_conv_1d!((<N1>) Vec<[T1; N1]>, ());
impl_conv_1d!((<'bn, N1>) Vec<&'bn [T1; N1]>, ());
impl_conv_1d!((<'bm, N1>) &'bm [[T1; N1]], ());
impl_conv_1d!((<'bm, 'bn, N1>) &'bm [&'bn [T1; N1]], ());
impl_conv_1d!(() Array2<T1>, ());
impl_conv_1d!((<'b>) ArrayView2<'b, T1>, ());

macro_rules! impl_conv_2d {
    (($(<$($a:lifetime),* $(,)? $($c:ident),*>)?) $lhs:ty, $rhs:ty, [[]] $(where $($where:tt)+)?) => {
        /*impl<$($($a,)* $(const $c: usize,)*)? T1, T2, T3> Conv2d<T1, T2, $rhs> for $lhs
        where
            T1: ComplexFloat + Mul<T2, Output = T3>,
            T2: ComplexFloat,
            T3: ComplexFloat,
            $lhs: MaybeMatrix<T1>,
            $rhs: MaybeMatrix<T2>,
            <$lhs as MaybeContainer<T1>>::Owned: MaybeMatrix<T1> + MaybeLists<T1> + Conv2d<T1, T2, <$rhs as MaybeContainer<T2>>::Owned>,
            <$rhs as MaybeContainer<T2>>::Owned: MaybeMatrix<T2> + MaybeLists<T2>,
            $($($where)+)?
        {
            type Output = <<$lhs as MaybeContainer<T1>>::Owned as Conv2d<T1, T2, <$rhs as MaybeContainer<T2>>::Owned>>::Output;

            fn conv_2d(self, rhs: $rhs) -> Self::Output
            {
                let lhs: <$lhs as MaybeContainer<T1>>::Owned = self.into_owned();
                let rhs: <$rhs as MaybeContainer<T2>>::Owned = rhs.into_owned();
                
                lhs.conv_2d(rhs)
            }
        }*/
    };
    (($(<$($a:lifetime),* $(,)? $($c:ident),*>)?) $lhs:ty, $rhs:ty, ([$n:expr]) $(where $($where:tt)+)?) => {
        impl<$($($a,)* $(const $c: usize,)*)? T1, T2, T3> Conv2d<T1, T2, $rhs> for $lhs
        where
            T1: ComplexFloat + Mul<T2, Output = T3> + Into<Complex<T1::Real>>,
            T2: ComplexFloat + Into<Complex<T2::Real>>,
            T3: ComplexFloat<Real: Into<T3>> + 'static,
            $lhs: Matrix<T1, Index = (usize, usize)>,
            $rhs: Matrix<T2, Index = (usize, usize)>,
            Array2<Complex<T1::Real>>: Dft2d<Complex<T1::Real>, Mapped<Complex<T1::Real>> = Array2<Complex<T1::Real>>> + Mul<Array2<Complex<T2::Real>>, Output = Array2<Complex<T3::Real>>>,
            Array2<Complex<T2::Real>>: Dft2d<Complex<T2::Real>, Mapped<Complex<T2::Real>> = Array2<Complex<T2::Real>>>,
            Array2<Complex<T3::Real>>: Idft2d<Complex<T3::Real>, Mapped<Complex<T3::Real>> = Array2<Complex<T3::Real>>>,
            Vec<[T3; $n]>: MaybeMatrix<T3>,
            $($($where)+)?
        {
            type Output = Vec<[T3; $n]>;

            fn conv_2d(self, rhs: $rhs) -> Self::Output
            {
                let (m1, n1) = Matrix::<T1>::matrix_dim(&self);
                let (m2, n2) = Matrix::<T2>::matrix_dim(&rhs);
                let dim = ((m1 + m2).saturating_sub(1), (n1 + n2).saturating_sub(1));
                let dim_fft = (dim.0.next_power_of_two(), dim.1.next_power_of_two());
        
                let x: Array2<Complex<T1::Real>> = Array2::from_shape_fn(dim_fft, |(i, j)| Container::<T1>::index_get(&self, (i, j)).map(|&x| Into::<Complex<T1::Real>>::into(x)).unwrap_or_else(Zero::zero))
                    .dft_2d();
                let h: Array2<Complex<T2::Real>> = Array2::from_shape_fn(dim_fft, |(i, j)| Container::<T2>::index_get(&rhs, (i, j)).map(|&h| Into::<Complex<T2::Real>>::into(h)).unwrap_or_else(Zero::zero))
                    .dft_2d();
        
                let y: Array2<Complex<T3::Real>> = x*h;
        
                let y: Array2<Complex<T3::Real>> = y.idft_2d();
        
                (0..dim.0).map(|i| core::array::from_fn(|j| y[(i, j)].truncate_im()))
                    .collect()
            }
        }
    };
    (($(<$($a:lifetime),* $(,)? $($c:ident),*>)?) $lhs:ty, $rhs:ty, (()) $(where $($where:tt)+)?) => {
        impl<$($($a,)* $(const $c: usize,)*)? T1, T2, T3> Conv2d<T1, T2, $rhs> for $lhs
        where
            T1: ComplexFloat + Mul<T2, Output = T3> + Into<Complex<T1::Real>>,
            T2: ComplexFloat + Into<Complex<T2::Real>>,
            T3: ComplexFloat<Real: Into<T3>> + 'static,
            $lhs: Matrix<T1, Index = (usize, usize)>,
            $rhs: Matrix<T2, Index = (usize, usize)>,
            Array2<Complex<T1::Real>>: Dft2d<Complex<T1::Real>, Mapped<Complex<T1::Real>> = Array2<Complex<T1::Real>>> + Mul<Array2<Complex<T2::Real>>, Output = Array2<Complex<T3::Real>>>,
            Array2<Complex<T2::Real>>: Dft2d<Complex<T2::Real>, Mapped<Complex<T2::Real>> = Array2<Complex<T2::Real>>>,
            Array2<Complex<T3::Real>>: Idft2d<Complex<T3::Real>, Mapped<Complex<T3::Real>> = Array2<Complex<T3::Real>>>,
            $($($where)+)?
        {
            type Output = Array2<T3>;
        
            fn conv_2d(self, rhs: $rhs) -> Self::Output
            {
                let (m1, n1) = Matrix::<T1>::matrix_dim(&self);
                let (m2, n2) = Matrix::<T2>::matrix_dim(&rhs);
                let dim = ((m1 + m2).saturating_sub(1), (n1 + n2).saturating_sub(1));
                let dim_fft = (dim.0.next_power_of_two(), dim.1.next_power_of_two());
        
                let x: Array2<Complex<T1::Real>> = Array2::from_shape_fn(dim_fft, |(i, j)| Container::<T1>::index_get(&self, (i, j)).map(|&x| Into::<Complex<T1::Real>>::into(x)).unwrap_or_else(Zero::zero))
                    .dft_2d();
                let h: Array2<Complex<T2::Real>> = Array2::from_shape_fn(dim_fft, |(i, j)| Container::<T2>::index_get(&rhs, (i, j)).map(|&h| Into::<Complex<T2::Real>>::into(h)).unwrap_or_else(Zero::zero))
                    .dft_2d();
        
                let y: Array2<Complex<T3::Real>> = x*h;
        
                let y: Array2<Complex<T3::Real>> = y.idft_2d();
        
                Array2::from_shape_fn(dim, |(i, j)| y[(i, j)].truncate_im())
            }
        }
    };
}

impl<T1, T2, T3, const N1: usize, const N2: usize, const M1: usize, const M2: usize> Conv2d<T1, T2, [[T2; N2]; M2]> for [[T1; N1]; M1]
where
    T1: ComplexFloat + Mul<T2, Output = T3> + Into<Complex<T1::Real>>,
    T2: ComplexFloat + Into<Complex<T2::Real>>,
    T3: ComplexFloat<Real: Into<T3>> + 'static,
    Complex<T1::Real>: AddAssign + MulAssign + Mul<Complex<T2::Real>, Output: ComplexFloat<Real = T3::Real> + MulAssign + AddAssign + MulAssign<T3::Real> + Sum + 'static>,
    Complex<T2::Real>: AddAssign + MulAssign,
    Complex<T3::Real>: Into<<Complex<T1::Real> as Mul<Complex<T2::Real>>>::Output>,
    [(); (M1 + M2 - 1).next_power_of_two()]:,
    [(); (M1 + M2 - 1).next_power_of_two() - M1]:,
    [(); (M1 + M2 - 1).next_power_of_two() - M2]:,
    [(); (M1 + M2 - 1).next_power_of_two() - (M1 + M2 - 1)]:,
    [(); (M1 + M2 - 1).next_power_of_two()/2 + 1]:,
    [(); (N1 + N2 - 1).next_power_of_two()]:,
    [(); (N1 + N2 - 1).next_power_of_two() - N1]:,
    [(); (N1 + N2 - 1).next_power_of_two() - N2]:,
    [(); (N1 + N2 - 1).next_power_of_two() - (N1 + N2 - 1)]:,
    [(); (N1 + N2 - 1).next_power_of_two()/2 + 1]:,
    [(); N1 + N2 - 1]:
{
    type Output = [[T3; N1 + N2 - 1]; M1 + M2 - 1];

    fn conv_2d(self, rhs: [[T2; N2]; M2]) -> Self::Output
    {
        let mut x: [[Complex<T1::Real>; (N1 + N2 - 1).next_power_of_two()]; (M1 + M2 - 1).next_power_of_two()] = self
            .map(|x| x.map(|x| x.into()).resize(|_| Zero::zero()))
            .resize(|_| [Zero::zero(); (N1 + N2 - 1).next_power_of_two()]);
        let mut h: [[Complex<T2::Real>; (N1 + N2 - 1).next_power_of_two()]; (M1 + M2 - 1).next_power_of_two()] = rhs
            .map(|h| h.map(|h| h.into()).resize(|_| Zero::zero()))
            .resize(|_| [Zero::zero(); (N1 + N2 - 1).next_power_of_two()]);

        x.fft_2d();
        h.fft_2d();
        
        let mut y = x.comap(h, |x_f, h_f| x_f.comap(h_f, |x_f, h_f| x_f*h_f));

        y.ifft_2d();

        y.truncate()
            .map(|y| y.map(|y| y.truncate_im()).truncate())
    }
}

impl_conv_2d!((<'bn, N1, N2, M1, M2>) [[T1; N1]; M1], [&'bn [T2; N2]; M2], [[]]);
impl_conv_2d!((<'bm, N1, N2, M1, M2>) [[T1; N1]; M1], &'bm [[T2; N2]; M2], [[]]);
impl_conv_2d!((<'bm, 'bn, N1, N2, M1, M2>) [[T1; N1]; M1], &'bm [&'bn [T2; N2]; M2], [[]]);
impl_conv_2d!((<N1, N2, M1>) [[T1; N1]; M1], Vec<[T2; N2]>, ([N1 + N2 - 1]));
impl_conv_2d!((<'bn, N1, N2, M1>) [[T1; N1]; M1], Vec<&'bn [T2; N2]>, ([N1 + N2 - 1]));
impl_conv_2d!((<'bm, N1, N2, M1>) [[T1; N1]; M1], &'bm [[T2; N2]], ([N1 + N2 - 1]));
impl_conv_2d!((<'bm, 'bn, N1, N2, M1>) [[T1; N1]; M1], &'bm [&'bn [T2; N2]], ([N1 + N2 - 1]));
impl_conv_2d!((<N1, M1>) [[T1; N1]; M1], Array2<T2>, (()));
impl_conv_2d!((<'b, N1, M1>) [[T1; N1]; M1], ArrayView2<'b, T2>, (()));

impl_conv_2d!((<'an, N1, N2, M1, M2>) [&'an [T1; N1]; M1], [[T2; N2]; M2], [[]]);
impl_conv_2d!((<'an, 'bn, N1, N2, M1, M2>) [&'an [T1; N1]; M1], [&'bn [T2; N2]; M2], [[]]);
impl_conv_2d!((<'an, 'bm, N1, N2, M1, M2>) [&'an [T1; N1]; M1], &'bm [[T2; N2]; M2], [[]]);
impl_conv_2d!((<'an, 'bm, 'bn, N1, N2, M1, M2>) [&'an [T1; N1]; M1], &'bm [&'bn [T2; N2]; M2], [[]]);
impl_conv_2d!((<'an, N1, N2, M1>) [&'an [T1; N1]; M1], Vec<[T2; N2]>, ([N1 + N2 - 1]));
impl_conv_2d!((<'an, 'bn, N1, N2, M1>) [&'an [T1; N1]; M1], Vec<&'bn [T2; N2]>, ([N1 + N2 - 1]));
impl_conv_2d!((<'an, 'bm, N1, N2, M1>) [&'an [T1; N1]; M1], &'bm [[T2; N2]], ([N1 + N2 - 1]));
impl_conv_2d!((<'an, 'bm, 'bn, N1, N2, M1>) [&'an [T1; N1]; M1], &'bm [&'bn [T2; N2]], ([N1 + N2 - 1]));
impl_conv_2d!((<'an, N1, M1>) [&'an [T1; N1]; M1], Array2<T2>, (()));
impl_conv_2d!((<'an, 'b, N1, M1>) [&'an [T1; N1]; M1], ArrayView2<'b, T2>, (()));

impl_conv_2d!((<'am, 'bn, N1, N2, M1, M2>) &'am [[T1; N1]; M1], [[T2; N2]; M2], [[]]);
impl_conv_2d!((<'am, 'bn, N1, N2, M1, M2>) &'am [[T1; N1]; M1], [&'bn [T2; N2]; M2], [[]]);
impl_conv_2d!((<'am, 'bm, N1, N2, M1, M2>) &'am [[T1; N1]; M1], &'bm [[T2; N2]; M2], [[]]);
impl_conv_2d!((<'am, 'bm, 'bn, N1, N2, M1, M2>) &'am [[T1; N1]; M1], &'bm [&'bn [T2; N2]; M2], [[]]);
impl_conv_2d!((<'am, N1, N2, M1>) &'am [[T1; N1]; M1], Vec<[T2; N2]>, ([N1 + N2 - 1]));
impl_conv_2d!((<'am, 'bn, N1, N2, M1>) &'am [[T1; N1]; M1], Vec<&'bn [T2; N2]>, ([N1 + N2 - 1]));
impl_conv_2d!((<'am, 'bm, N1, N2, M1>) &'am [[T1; N1]; M1], &'bm [[T2; N2]], ([N1 + N2 - 1]));
impl_conv_2d!((<'am, 'bm, 'bn, N1, N2, M1>) &'am [[T1; N1]; M1], &'bm [&'bn [T2; N2]], ([N1 + N2 - 1]));
impl_conv_2d!((<'am, N1, M1>) &'am [[T1; N1]; M1], Array2<T2>, (()));
impl_conv_2d!((<'am, 'b, N1, M1>) &'am [[T1; N1]; M1], ArrayView2<'b, T2>, (()));

impl_conv_2d!((<'am, 'an, N1, N2, M1, M2>) &'am [&'an [T1; N1]; M1], [[T2; N2]; M2], [[]]);
impl_conv_2d!((<'am, 'an, 'bn, N1, N2, M1, M2>) &'am [&'an [T1; N1]; M1], [&'bn [T2; N2]; M2], [[]]);
impl_conv_2d!((<'am, 'an, 'bm, N1, N2, M1, M2>) &'am [&'an [T1; N1]; M1], &'bm [[T2; N2]; M2], [[]]);
impl_conv_2d!((<'am, 'an, 'bm, 'bn, N1, N2, M1, M2>) &'am [&'an [T1; N1]; M1], &'bm [&'bn [T2; N2]; M2], [[]]);
impl_conv_2d!((<'am, 'an, N1, N2, M1>) &'am [&'an [T1; N1]; M1], Vec<[T2; N2]>, ([N1 + N2 - 1]));
impl_conv_2d!((<'am, 'an, 'bn, N1, N2, M1>) &'am [&'an [T1; N1]; M1], Vec<&'bn [T2; N2]>, ([N1 + N2 - 1]));
impl_conv_2d!((<'am, 'an, 'bm, N1, N2, M1>) &'am [&'an [T1; N1]; M1], &'bm [[T2; N2]], ([N1 + N2 - 1]));
impl_conv_2d!((<'am, 'an, 'bm, 'bn, N1, N2, M1>) &'am [&'an [T1; N1]; M1], &'bm [&'bn [T2; N2]], ([N1 + N2 - 1]));
impl_conv_2d!((<'am, 'an, N1, M1>) &'am [&'an [T1; N1]; M1], Array2<T2>, (()));
impl_conv_2d!((<'am, 'an, 'b, N1, M1>) &'am [&'an [T1; N1]; M1], ArrayView2<'b, T2>, (()));

impl_conv_2d!((<N1, N2, M2>) Vec<[T1; N1]>, [[T2; N2]; M2], ([N1 + N2 - 1]));
impl_conv_2d!((<'bn, N1, N2, M2>) Vec<[T1; N1]>, [&'bn [T2; N2]; M2], ([N1 + N2 - 1]));
impl_conv_2d!((<'bm, N1, N2, M2>) Vec<[T1; N1]>, &'bm [[T2; N2]; M2], ([N1 + N2 - 1]));
impl_conv_2d!((<'bm, 'bn, N1, N2, M2>) Vec<[T1; N1]>, &'bm [&'bn [T2; N2]; M2], ([N1 + N2 - 1]));
impl_conv_2d!((<N1, N2>) Vec<[T1; N1]>, Vec<[T2; N2]>, ([N1 + N2 - 1]));
impl_conv_2d!((<'bn, N1, N2>) Vec<[T1; N1]>, Vec<&'bn [T2; N2]>, ([N1 + N2 - 1]));
impl_conv_2d!((<'bm, N1, N2>) Vec<[T1; N1]>, &'bm [[T2; N2]], ([N1 + N2 - 1]));
impl_conv_2d!((<'bm, 'bn, N1, N2>) Vec<[T1; N1]>, &'bm [&'bn [T2; N2]], ([N1 + N2 - 1]));
impl_conv_2d!((<N1>) Vec<[T1; N1]>, Array2<T2>, (()));
impl_conv_2d!((<'b, N1>) Vec<[T1; N1]>, ArrayView2<'b, T2>, (()));

impl_conv_2d!((<'an, N1, N2, M2>) Vec<&'an [T1; N1]>, [[T2; N2]; M2], ([N1 + N2 - 1]));
impl_conv_2d!((<'an, 'bn, N1, N2, M2>) Vec<&'an [T1; N1]>, [&'bn [T2; N2]; M2], ([N1 + N2 - 1]));
impl_conv_2d!((<'an, 'bm, N1, N2, M2>) Vec<&'an [T1; N1]>, &'bm [[T2; N2]; M2], ([N1 + N2 - 1]));
impl_conv_2d!((<'an, 'bm, 'bn, N1, N2, M2>) Vec<&'an [T1; N1]>, &'bm [&'bn [T2; N2]; M2], ([N1 + N2 - 1]));
impl_conv_2d!((<'an, N1, N2>) Vec<&'an [T1; N1]>, Vec<[T2; N2]>, ([N1 + N2 - 1]));
impl_conv_2d!((<'an, 'bn, N1, N2>) Vec<&'an [T1; N1]>, Vec<&'bn [T2; N2]>, ([N1 + N2 - 1]));
impl_conv_2d!((<'an, 'bm, N1, N2>) Vec<&'an [T1; N1]>, &'bm [[T2; N2]], ([N1 + N2 - 1]));
impl_conv_2d!((<'an, 'bm, 'bn, N1, N2>) Vec<&'an [T1; N1]>, &'bm [&'bn [T2; N2]], ([N1 + N2 - 1]));
impl_conv_2d!((<'an, N1>) Vec<&'an [T1; N1]>, Array2<T2>, (()));
impl_conv_2d!((<'an, 'b, N1>) Vec<&'an [T1; N1]>, ArrayView2<'b, T2>, (()));

impl_conv_2d!((<N2, M2>) Array2<T1>, [[T2; N2]; M2], (()));
impl_conv_2d!((<'bn, N2, M2>) Array2<T1>, [&'bn [T2; N2]; M2], (()));
impl_conv_2d!((<'bm, N2, M2>) Array2<T1>, &'bm [[T2; N2]; M2], (()));
impl_conv_2d!((<'bm, 'bn, N2, M2>) Array2<T1>, &'bm [&'bn [T2; N2]; M2], (()));
impl_conv_2d!((<N2>) Array2<T1>, Vec<[T2; N2]>, (()));
impl_conv_2d!((<'bn, N2>) Array2<T1>, Vec<&'bn [T2; N2]>, (()));
impl_conv_2d!((<'bm, N2>) Array2<T1>, &'bm [[T2; N2]], (()));
impl_conv_2d!((<'bm, 'bn, N2>) Array2<T1>, &'bm [&'bn [T2; N2]], (()));
impl_conv_2d!(() Array2<T1>, Array2<T2>, (()));
impl_conv_2d!((<'b>) Array2<T1>, ArrayView2<'b, T2>, (()));

impl_conv_2d!((<'a, N2, M2>) ArrayView2<'a, T1>, [[T2; N2]; M2], (()));
impl_conv_2d!((<'a, 'bn, N2, M2>) ArrayView2<'a, T1>, [&'bn [T2; N2]; M2], (()));
impl_conv_2d!((<'a, 'bm, N2, M2>) ArrayView2<'a, T1>, &'bm [[T2; N2]; M2], (()));
impl_conv_2d!((<'a, 'bm, 'bn, N2, M2>) ArrayView2<'a, T1>, &'bm [&'bn [T2; N2]; M2], (()));
impl_conv_2d!((<'a, N2>) ArrayView2<'a, T1>, Vec<[T2; N2]>, (()));
impl_conv_2d!((<'a, 'bn, N2>) ArrayView2<'a, T1>, Vec<&'bn [T2; N2]>, (()));
impl_conv_2d!((<'a, 'bm, N2>) ArrayView2<'a, T1>, &'bm [[T2; N2]], (()));
impl_conv_2d!((<'a, 'bm, 'bn, N2>) ArrayView2<'a, T1>, &'bm [&'bn [T2; N2]], (()));
impl_conv_2d!((<'a>) ArrayView2<'a, T1>, Array2<T2>, (()));
impl_conv_2d!((<'a, 'b>) ArrayView2<'a, T1>, ArrayView2<'b, T2>, (()));

#[cfg(test)]
mod test
{
    use crate::Conv2d;

    #[test]
    fn test()
    {
        let a = [
            [1.0, 2.0],
            [3.0, 4.0]
        ];
        let b = ndarray::arr2(&[
            [1.0, 0.0, 0.0],
            [0.0, 0.0, 0.0],
            [0.0, 0.0, 2.0]
        ]);
        let y = a.conv_2d(b);

        println!("{:?}", y);
    }
}