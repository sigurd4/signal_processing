use core::marker::PhantomData;

use num::complex::ComplexFloat;
use option_trait::Maybe;

use crate::{LenEq, Matrix};

pub trait SsAMatrix<T, B: Matrix<T>, C: Matrix<T>, D: Matrix<T>>: Matrix<T> {}
impl<T, A, B, C, D, N> SsAMatrix<T, B, C, D> for A
where
    N: Maybe<usize>,
    A: Matrix<T, Height = N, Width = N>,
    B: Matrix<T, Height = A::Height, Width = D::Width>,
    C: Matrix<T, Height = D::Height, Width = A::Width>,
    D: Matrix<T>,
    usize: LenEq<{A::HEIGHT}, {A::WIDTH}, true> + LenEq<{B::HEIGHT}, {A::HEIGHT}, true> + LenEq<{B::WIDTH}, {D::WIDTH}, true> + LenEq<{C::HEIGHT}, {D::HEIGHT}, true> + LenEq<{C::WIDTH}, {A::WIDTH}, true>
{}
pub trait SsBMatrix<T, A: Matrix<T>, C: Matrix<T>, D: Matrix<T>>: Matrix<T> {}
impl<T, A, B, C, D, N> SsBMatrix<T, A, C, D> for B
where
    N: Maybe<usize>,
    A: Matrix<T, Height = N, Width = N>,
    B: Matrix<T, Height = A::Height, Width = D::Width>,
    C: Matrix<T, Height = D::Height, Width = A::Width>,
    D: Matrix<T>,
    usize: LenEq<{A::HEIGHT}, {A::WIDTH}, true> + LenEq<{B::HEIGHT}, {A::HEIGHT}, true> + LenEq<{B::WIDTH}, {D::WIDTH}, true> + LenEq<{C::HEIGHT}, {D::HEIGHT}, true> + LenEq<{C::WIDTH}, {A::WIDTH}, true>
{}
pub trait SsCMatrix<T, A: Matrix<T>, B: Matrix<T>, D: Matrix<T>>: Matrix<T> {}
impl<T, A, B, C, D, N> SsCMatrix<T, A, B, D> for C
where
    N: Maybe<usize>,
    A: Matrix<T, Height = N, Width = N>,
    B: Matrix<T, Height = A::Height, Width = D::Width>,
    C: Matrix<T, Height = D::Height, Width = A::Width>,
    D: Matrix<T>,
    usize: LenEq<{A::HEIGHT}, {A::WIDTH}, true> + LenEq<{B::HEIGHT}, {A::HEIGHT}, true> + LenEq<{B::WIDTH}, {D::WIDTH}, true> + LenEq<{C::HEIGHT}, {D::HEIGHT}, true> + LenEq<{C::WIDTH}, {A::WIDTH}, true>
{}
pub trait SsDMatrix<T, A: Matrix<T>, B: Matrix<T>, C: Matrix<T>>: Matrix<T> {}
impl<T, A, B, C, D, N> SsDMatrix<T, A, B, C> for D
where
    N: Maybe<usize>,
    A: Matrix<T, Height = N, Width = N>,
    B: Matrix<T, Height = A::Height, Width = D::Width>,
    C: Matrix<T, Height = D::Height, Width = A::Width>,
    D: Matrix<T>,
    usize: LenEq<{A::HEIGHT}, {A::WIDTH}, true> + LenEq<{B::HEIGHT}, {A::HEIGHT}, true> + LenEq<{B::WIDTH}, {D::WIDTH}, true> + LenEq<{C::HEIGHT}, {D::HEIGHT}, true> + LenEq<{C::WIDTH}, {A::WIDTH}, true>
{}


#[derive(Debug, Clone, Copy)]
pub struct Ss<T, A, B, C, D>
where
    T: ComplexFloat,
    A: SsAMatrix<T, B, C, D>,
    B: SsBMatrix<T, A, C, D>,
    C: SsCMatrix<T, A, B, D>,
    D: SsDMatrix<T, A, B, C>
{
    pub a: A,
    pub b: B,
    pub c: C,
    pub d: D,
    phantom: PhantomData<T>
}

impl<T, A, B, C, D> Ss<T, A, B, C, D>
where
    T: ComplexFloat,
    A: SsAMatrix<T, B, C, D>,
    B: SsBMatrix<T, A, C, D>,
    C: SsCMatrix<T, A, B, D>,
    D: SsDMatrix<T, A, B, C>
{
    pub fn new(a: A, b: B, c: C, d: D) -> Self
    {
        Self {
            a,
            b,
            c,
            d,
            phantom: PhantomData
        }
    }
}

#[allow(unused)]
macro abcd {
    (A, B, C, D) => {},
}

pub macro ss {
    ($t:path[]
        let $a:ident = [$($([$($($am:literal),+$(,)?)?]),+$(,)?)?],
        let $b:ident = [$($([$($($bm:literal),+$(,)?)?]),+$(,)?)?],
        let $c:ident = [$($([$($($cm:literal),+$(,)?)?]),+$(,)?)?],
        let $d:ident = [$($([$($($dm:literal),+$(,)?)?]),+$(,)?)?]$(,)?
    ) => {
        {
            const N: usize = [$($([$($($am),*)?]),*)?].len();
            const P: usize = [$($([$($($dm),*)?]),*)?][0].len();
            const Q: usize = [$($([$($($dm),*)?]),*)?].len();

            #[allow(non_snake_case)]
            let $a : [[$t; N]; N] = [$($([$($(<$t as num::NumCast>::from($am).unwrap()),*)?]),*)?];
            #[allow(non_snake_case)]
            let $b : [[$t; P]; N] = [$($([$($(<$t as num::NumCast>::from($bm).unwrap()),*)?]),*)?];
            #[allow(non_snake_case)]
            let $c : [[$t; N]; Q] = [$($([$($(<$t as num::NumCast>::from($cm).unwrap()),*)?]),*)?];
            #[allow(non_snake_case)]
            let $d : [[$t; P]; Q] = [$($([$($(<$t as num::NumCast>::from($dm).unwrap()),*)?]),*)?];
            abcd!($a, $b, $c, $d);
            Ss::<$t, [[$t; N]; N], [[$t; P]; N], [[$t; N]; Q], [[$t; P]; Q]>::new($a, $b, $c, $d)
        }
    }
}

#[cfg(test)]
mod test
{
    use crate::ss;

    #[test]
    fn test()
    {
        let h = ss!(f64[]
            let A = [
                [1, 2],
                [3, 4]
            ],
            let B = [
                [1],
                [2]
            ],
            let C = [
                [1, 2]
            ],
            let D = [
                [1]
            ]
        );

        println!("{:?}", h);
    }
}
