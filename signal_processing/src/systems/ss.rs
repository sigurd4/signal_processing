use core::marker::PhantomData;

use num::complex::ComplexFloat;
use option_trait::Maybe;

use crate::{util::LenEq, quantities::Matrix};

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
    pub type View<'a> = Ss<T, A::View<'a>, B::View<'a>, C::View<'a>, D::View<'a>>
    where
        A: 'a,
        B: 'a,
        C: 'a,
        D: 'a,
        A::View<'a>: SsAMatrix<T, B::View<'a>, C::View<'a>, D::View<'a>>,
        B::View<'a>: SsBMatrix<T, A::View<'a>, C::View<'a>, D::View<'a>>,
        C::View<'a>: SsCMatrix<T, A::View<'a>, B::View<'a>, D::View<'a>>,
        D::View<'a>: SsDMatrix<T, A::View<'a>, B::View<'a>, C::View<'a>>;
    pub type Owned = Ss<T, A::Owned, B::Owned, C::Owned, D::Owned>
    where
        A::Owned: SsAMatrix<T, B::Owned, C::Owned, D::Owned>,
        B::Owned: SsBMatrix<T, A::Owned, C::Owned, D::Owned>,
        C::Owned: SsCMatrix<T, A::Owned, B::Owned, D::Owned>,
        D::Owned: SsDMatrix<T, A::Owned, B::Owned, C::Owned>;

    pub fn as_view<'a>(&'a self) -> Ss<T, A::View<'a>, B::View<'a>, C::View<'a>, D::View<'a>>
    where
        A: 'a,
        B: 'a,
        C: 'a,
        D: 'a,
        A::View<'a>: SsAMatrix<T, B::View<'a>, C::View<'a>, D::View<'a>>,
        B::View<'a>: SsBMatrix<T, A::View<'a>, C::View<'a>, D::View<'a>>,
        C::View<'a>: SsCMatrix<T, A::View<'a>, B::View<'a>, D::View<'a>>,
        D::View<'a>: SsDMatrix<T, A::View<'a>, B::View<'a>, C::View<'a>>
    {
        Ss::new(self.a.as_view(), self.b.as_view(), self.c.as_view(), self.d.as_view())
    }
    pub fn to_owned(&self) -> Ss<T, A::Owned, B::Owned, C::Owned, D::Owned>
    where
        T: Clone,
        A::Owned: SsAMatrix<T, B::Owned, C::Owned, D::Owned>,
        B::Owned: SsBMatrix<T, A::Owned, C::Owned, D::Owned>,
        C::Owned: SsCMatrix<T, A::Owned, B::Owned, D::Owned>,
        D::Owned: SsDMatrix<T, A::Owned, B::Owned, C::Owned>
    {
        Ss::new(self.a.to_owned(), self.b.to_owned(), self.c.to_owned(), self.d.to_owned())
    }
    pub fn into_owned(self) -> Ss<T, A::Owned, B::Owned, C::Owned, D::Owned>
    where
        T: Clone,
        A::Owned: SsAMatrix<T, B::Owned, C::Owned, D::Owned>,
        B::Owned: SsBMatrix<T, A::Owned, C::Owned, D::Owned>,
        C::Owned: SsCMatrix<T, A::Owned, B::Owned, D::Owned>,
        D::Owned: SsDMatrix<T, A::Owned, B::Owned, C::Owned>
    {
        Ss::new(self.a.into_owned(), self.b.into_owned(), self.c.into_owned(), self.d.into_owned())
    }

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

#[allow(unused)]
macro s {
    (s) => {},
    (z) => {},
}

pub macro ss {
    ($t:path[$s:ident]
        let $a:ident = [$($([$($($am:literal),+$(,)?)?]),+$(,)?)?],
        let $b:ident = [$($([$($($bm:literal),+$(,)?)?]),+$(,)?)?],
        let $c:ident = [$($([$($($cm:literal),+$(,)?)?]),+$(,)?)?],
        let $d:ident = [$($([$($($dm:literal),+$(,)?)?]),+$(,)?)?]$(,)?
    ) => {
        {
            s!($s);

            const N: usize = [$($([$($({let _ = $am; ()}),*)?]),*)?].len();
            const P: usize = [$($([$($({let _ = $dm; ()}),*)?]),*)?][0].len();
            const Q: usize = [$($([$($({let _ = $dm; ()}),*)?]),*)?].len();

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
    use crate::systems::ss;

    #[test]
    fn test()
    {
        let h = ss!(f64[z]
            let A = [
                [0.0000, 0.1716],
                [-1.0000, 0]
            ],
            let B = [
                [-0.2426],
                [0.5858]
            ],
            let C = [
                [0, 1]
            ],
            let D = [
                [0.2929]
            ]
        );

        println!("{:?}", h);
    }
}
