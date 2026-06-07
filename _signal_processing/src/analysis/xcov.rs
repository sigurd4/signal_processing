use core::{iter::Sum, ops::{RangeInclusive, SubAssign}};

use num::complex::ComplexFloat;
use option_trait::{Maybe, StaticMaybe};

use crate::{util::ComplexOp, quantities::{Lists, MaybeContainer, MaybeList, MaybeOwnedList, OwnedLists}, analysis::{XCorr, XCorrScale}};

pub trait XCov<X, Y, YY, Z>: Lists<X>
where
    X: ComplexFloat + ComplexOp<Y, Output = Z>,
    Y: ComplexFloat<Real = X::Real> + Into<Z>,
    YY: MaybeList<Y, MaybeSome: StaticMaybe<YY::Some, MaybeOr<Self::RowOwned, Self::Owned> = Self::Owned>>,
    Z: ComplexFloat<Real = X::Real>
{
    fn xcov<SC, ML>(
        self,
        y: YY,
        scale: SC,
        max_lag: ML
    ) -> (Self::RowsMapped<Self::RowsMapped<Vec<Z>>>, RangeInclusive<isize>)
    where
        SC: Maybe<XCorrScale>,
        ML: Maybe<usize>;
}

impl<X, XX, Y, YY, Z> XCov<X, Y, YY, Z> for XX
where
    XX: Lists<X>,
    X: ComplexFloat + ComplexOp<Y, Output = Z> + Sum + SubAssign,
    Y: ComplexFloat<Real = X::Real> + Into<Z> + Sum + SubAssign,
    YY: MaybeList<Y, MaybeSome: StaticMaybe<<YY as MaybeContainer<Y>>::Some, MaybeOr<Self::RowOwned, Self::Owned> = Self::Owned>>,
    Z: ComplexFloat<Real = X::Real>,
    XX::Owned: OwnedLists<X, Owned = XX::Owned, RowOwned = XX::RowOwned, RowsMapped<Vec<Z>> = XX::RowsMapped<Vec<Z>>> + OwnedLists<X, RowsMapped<XX::RowsMapped<Vec<Z>>> = XX::RowsMapped<XX::RowsMapped<Vec<Z>>>> + XCorr<X, Y, YY::Owned, Z>,
    YY::Owned: MaybeOwnedList<Y>,
    YY::Owned: MaybeList<Y, MaybeSome: StaticMaybe<<YY::Owned as MaybeContainer<Y>>::Some, MaybeOr<Self::RowOwned, Self::Owned> = Self::Owned>>,
{
    fn xcov<SC, ML>(
        self,
        y: YY,
        scale: SC,
        max_lag: ML
    ) -> (Self::RowsMapped<Self::RowsMapped<Vec<Z>>>, RangeInclusive<isize>)
    where
        SC: Maybe<XCorrScale>,
        ML: Maybe<usize>
    {
        let mut x = self.into_owned();
        let mut y = y.into_owned();

        {
            let x = x.as_mut_slices();
            for x in x
            {
                let xlen = x.len();
                let xmean = x.iter()
                    .map(|&x| x)
                    .sum::<X>()
                    /X::from(xlen).unwrap();
                for x in x.iter_mut()
                {
                    *x -= xmean
                }
            }
        }
        if let Some(y) = y.as_mut_slice_option()
        {
            let ylen = y.len();
            let ymean = y.iter()
                .map(|&y| y)
                .sum::<Y>()
                /Y::from(ylen).unwrap();
            for y in y.iter_mut()
            {
                *y -= ymean
            }
        }
            

        x.xcorr(y, scale, max_lag)
    }
}
