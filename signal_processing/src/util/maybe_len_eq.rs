use ndarray::{Array1, ArrayView1};

pub trait MaybeLenEq<Rhs, const MAYBE_EQ: bool>
where
    Rhs: ?Sized {}

impl<T1, const N1: usize> MaybeLenEq<(), true> for [T1; N1] {}
impl<T1, T2, const N1: usize, const N2: usize> MaybeLenEq<[T2; N2], {N1 == N2}> for [T1; N1] {}
impl<T1, T2, const N1: usize, const N2: usize> MaybeLenEq<&[T2; N2], {N1 == N2}> for [T1; N1] {}
impl<T1, T2, const N1: usize> MaybeLenEq<Vec<T2>, true> for [T1; N1] {}
impl<T1, T2, const N1: usize> MaybeLenEq<&[T2], true> for [T1; N1] {}
impl<T1, T2, const N1: usize> MaybeLenEq<Array1<T2>, true> for [T1; N1] {}
impl<'b, T1, T2, const N1: usize> MaybeLenEq<ArrayView1<'b, T2>, true> for [T1; N1] {}

impl<T1, const N1: usize> MaybeLenEq<(), true> for &[T1; N1] {}
impl<T1, T2, const N1: usize, const N2: usize> MaybeLenEq<[T2; N2], {N1 == N2}> for &[T1; N1] {}
impl<T1, T2, const N1: usize, const N2: usize> MaybeLenEq<&[T2; N2], {N1 == N2}> for &[T1; N1] {}
impl<T1, T2, const N1: usize> MaybeLenEq<Vec<T2>, true> for &[T1; N1] {}
impl<T1, T2, const N1: usize> MaybeLenEq<&[T2], true> for &[T1; N1] {}
impl<T1, T2, const N1: usize> MaybeLenEq<Array1<T2>, true> for &[T1; N1] {}
impl<'b, T1, T2, const N1: usize> MaybeLenEq<ArrayView1<'b, T2>, true> for &[T1; N1] {}

impl<T1> MaybeLenEq<(), true> for Vec<T1> {}
impl<T1, T2, const N2: usize> MaybeLenEq<[T2; N2], true> for Vec<T1> {}
impl<T1, T2, const N2: usize> MaybeLenEq<&[T2; N2], true> for Vec<T1> {}
impl<T1, T2> MaybeLenEq<Vec<T2>, true> for Vec<T1> {}
impl<T1, T2> MaybeLenEq<&[T2], true> for Vec<T1> {}
impl<T1, T2> MaybeLenEq<Array1<T2>, true> for Vec<T1> {}
impl<'b, T1, T2> MaybeLenEq<ArrayView1<'b, T2>, true> for Vec<T1> {}

impl<T1> MaybeLenEq<(), true> for &[T1] {}
impl<T1, T2, const N2: usize> MaybeLenEq<[T2; N2], true> for &[T1] {}
impl<T1, T2, const N2: usize> MaybeLenEq<&[T2; N2], true> for &[T1] {}
impl<T1, T2> MaybeLenEq<Vec<T2>, true> for &[T1] {}
impl<T1, T2> MaybeLenEq<&[T2], true> for &[T1] {}
impl<T1, T2> MaybeLenEq<Array1<T2>, true> for &[T1] {}
impl<'b, T1, T2> MaybeLenEq<ArrayView1<'b, T2>, true> for &[T1] {}

impl<T1> MaybeLenEq<(), true> for Array1<T1> {}
impl<T1, T2, const N2: usize> MaybeLenEq<[T2; N2], true> for Array1<T1> {}
impl<T1, T2, const N2: usize> MaybeLenEq<&[T2; N2], true> for Array1<T1> {}
impl<T1, T2> MaybeLenEq<Vec<T2>, true> for Array1<T1> {}
impl<T1, T2> MaybeLenEq<&[T2], true> for Array1<T1> {}
impl<T1, T2> MaybeLenEq<Array1<T2>, true> for Array1<T1> {}
impl<'b, T1, T2> MaybeLenEq<ArrayView1<'b, T2>, true> for Array1<T1> {}

impl<'a, T1> MaybeLenEq<(), true> for ArrayView1<'a, T1> {}
impl<'a, T1, T2, const N2: usize> MaybeLenEq<[T2; N2], true> for ArrayView1<'a, T1> {}
impl<'a, T1, T2, const N2: usize> MaybeLenEq<&[T2; N2], true> for ArrayView1<'a, T1> {}
impl<'a, T1, T2> MaybeLenEq<Vec<T2>, true> for ArrayView1<'a, T1> {}
impl<'a, T1, T2> MaybeLenEq<&[T2], true> for ArrayView1<'a, T1> {}
impl<'a, T1, T2> MaybeLenEq<Array1<T2>, true> for ArrayView1<'a, T1> {}
impl<'a, 'b, T1, T2> MaybeLenEq<ArrayView1<'b, T2>, true> for ArrayView1<'a, T1> {}

impl MaybeLenEq<(), true> for () {}
impl<T2, const N2: usize> MaybeLenEq<[T2; N2], true> for () {}
impl<T2, const N2: usize> MaybeLenEq<&[T2; N2], true> for () {}
impl<T2> MaybeLenEq<Vec<T2>, true> for () {}
impl<T2> MaybeLenEq<&[T2], true> for () {}
impl<T2> MaybeLenEq<Array1<T2>, true> for () {}
impl<'b, T2> MaybeLenEq<ArrayView1<'b, T2>, true> for () {}