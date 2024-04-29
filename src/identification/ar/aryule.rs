use num::complex::ComplexFloat;
use option_trait::Maybe;

use crate::{systems::Ar, identification::ar::Levinson, quantities::{List, ListOrSingle, Lists, MaybeList, MaybeLists}, System, analysis::{XCorr, XCorrScale}};

pub trait ArYule<X, O, K>: System + Sized
where
    X: Lists<Self::Set>,
    O: Maybe<usize>,
    K: Lists<Self::Set>
{
    fn aryule(x: X, order: O) -> (Self, K);
}

impl<T, O, X, K, A, AV> ArYule<X, O, K> for Ar<T, A, AV>
where
    T: ComplexFloat<Real: Into<T>>,
    A: MaybeList<T>,
    AV: ListOrSingle<(A, T::Real)>,
    X: Lists<T>,
    O: Maybe<usize>,
    K: Lists<T>,
    X::RowOwned: XCorr<T, T, (), T> + List<T, RowsMapped<<X::RowOwned as MaybeLists<T>>::RowsMapped<Vec<T>>> = Vec<T>>,
    X::RowsMapped<Vec<T>>: Lists<T>,
    Self: Levinson<X::RowsMapped<Vec<T>>, O, K> + System<Set = T>
{
    fn aryule(x: X, order: O) -> (Self, K)
    {
        let n = order.as_option()
            .copied()
            .map(|o| o + 1)
            .unwrap_or(A::LENGTH)
            .max(2);
        let c = x.map_rows_into_owned(|x| {
            let (mut c, _): (Vec<T>, _) = x.xcorr((), XCorrScale::Biased, n);
            c.reverse();
            c.truncate(c.len() - n);
            c.reverse();
            if let Some(c) = c.get_mut(0)
            {
                *c = c.re().into();
            }
            c
        });
        Self::levinson(c, order)
    }
}

#[cfg(test)]
mod test
{
    use crate::{systems::Ar, identification::ar::ArYule};

    #[test]
    fn test()
    {
        let x = [1.0, 2.0, 3.0, 4.0, 5.0];

        const N: usize = 3;
        let (ar, k) = Ar::<_, [_; N], _>::aryule(x, ());

        println!("{:?}", ar);
        println!("{:?}", k);
    }
}