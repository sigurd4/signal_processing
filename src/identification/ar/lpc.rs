use num::complex::ComplexFloat;
use option_trait::Maybe;

use crate::{systems::Ar, identification::ar::Levinson, quantities::{List, ListOrSingle, Lists}, System, analysis::{XCorr, XCorrScale}};

pub trait Lpc<X, O>: System + Sized
where
    X: Lists<Self::Set>,
    O: Maybe<usize>
{
    fn lpc(x: X, order: O) -> Self;
}

impl<T, X, XX, const N: usize> Lpc<X, ()> for Ar<T, [T; N], X::RowsMapped<([T; N], T::Real)>>
where
    T: ComplexFloat<Real: Into<T>>,
    X: Lists<T, RowOwned = XX>,
    X::RowsMapped<([T; N], T::Real)>: ListOrSingle<([T; N], T::Real)>,
    XX: XCorr<T, T, (), T> + List<T, RowsMapped<Vec<T>> = Vec<T>>,
    Ar<T, [T; N], ([T; N], T::Real)>: Levinson<Vec<T>, (), [T; N - 1]> + System<Set = T>
{
    fn lpc(x: X, (): ()) -> Self
    {
        Ar::new(x.map_rows_into_owned(|x| {
            let (mut r, _): (Vec<T>, _) = x.xcorr((), XCorrScale::Biased, N);
            let mut r = r.split_off(N);
            if let Some(r) = r.get_mut(0)
            {
                *r = r.re().into()
            }
            let (Ar {av, ..}, _) = Ar::levinson(r, ()); 
            av
        }))
    }
}

impl<T, X, XX> Lpc<X, usize> for Ar<T, Vec<T>, X::RowsMapped<(Vec<T>, T::Real)>>
where
    T: ComplexFloat<Real: Into<T>>,
    X: Lists<T, RowOwned = XX>,
    X::RowsMapped<(Vec<T>, T::Real)>: ListOrSingle<(Vec<T>, T::Real)>,
    XX: XCorr<T, T, (), T> + List<T, RowsMapped<Vec<T>> = Vec<T>>,
    Ar<T, Vec<T>, (Vec<T>, T::Real)>: Levinson<Vec<T>, usize, Vec<T>> + System<Set = T>
{
    fn lpc(x: X, order: usize) -> Self
    {
        Ar::new(x.map_rows_into_owned(|x| {
            let (mut r, _): (Vec<T>, _) = x.xcorr((), XCorrScale::Biased, order + 1);
            let mut r = r.split_off(order + 1);
            if let Some(r) = r.get_mut(0)
            {
                *r = r.re().into()
            }
            let (Ar {av, ..}, _) = Ar::levinson(r, order); 
            av
        }))
    }
}