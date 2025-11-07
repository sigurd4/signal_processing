use num::{complex::ComplexFloat, rational::Ratio, NumCast, One};
use option_trait::{Maybe, StaticMaybe};

use crate::{operations::resampling::{Downsample, Interp}, quantities::{List, ListOrSingle, Lists, MaybeLists}};

pub trait Resample<T, M, PQ, Y>: Lists<T>
where
    T: ComplexFloat,
    M: Maybe<<PQ::Maybe<usize> as StaticMaybe<usize>>::Opposite>,
    PQ: StaticMaybe<Ratio<usize>>,
    Y: List<T>
{
    fn resample<N, W>(self, length: M, ratio: PQ, order: N, cutoff: W) -> Self::RowsMapped<Y>
    where
        N: Maybe<usize>,
        W: Maybe<T::Real>;
}

impl<T, L> Resample<T, (), Ratio<usize>, Vec<T>> for L
where
    L: Lists<T> + Interp<T, usize, Vec<T>>,
    L::RowsMapped<Vec<T>>: Downsample<T, usize, Vec<T>> + Lists<T, RowsMapped<Vec<T>> = L::RowsMapped<Vec<T>>>,
    T: ComplexFloat
{
    fn resample<N, W>(self, (): (), mut ratio: Ratio<usize>, order: N, cutoff: W) -> Self::RowsMapped<Vec<T>>
    where
        N: Maybe<usize>,
        W: Maybe<<T as ComplexFloat>::Real>
    {
        let one = T::Real::one();
        let two = one + one;

        ratio = Ratio::new(*ratio.numer(), (*ratio.denom()).max(1));

        let cutoff = cutoff.into_option()
            .unwrap_or_else(|| <T::Real as NumCast>::from(*ratio.numer()).unwrap()/(two*<T::Real as NumCast>::from((*ratio.numer()).max(*ratio.denom())).unwrap()));

        let z = self.interp(*ratio.numer(), order, cutoff);
        let y = z.downsample(*ratio.denom(), 0);

        y
    }
}

impl<T, L> Resample<T, usize, (), Vec<T>> for L
where
    L: Lists<T>,
    T: ComplexFloat,
    L::RowOwned: Resample<T, (), Ratio<usize>, Vec<T>> + List<T, RowsMapped<Vec<T>> = Vec<T>>
{
    fn resample<N, W>(self, length: usize, (): (), order: N, cutoff: W) -> Self::RowsMapped<Vec<T>>
    where
        N: Maybe<usize>,
        W: Maybe<<T as ComplexFloat>::Real>
    {
        let order = order.into_option();
        let cutoff = cutoff.into_option();

        self.map_rows_into_owned(|x| {
            let ratio = Ratio::new(length, x.length().max(1));
    
            x.resample((), ratio, order, cutoff)
        })
    }
}

impl<T, L, const M: usize> Resample<T, (), (), [T; M]> for L
where
    L: Lists<T> + Resample<T, usize, (), Vec<T>>,
    L::RowsMapped<Vec<T>>: Lists<T, RowOwned = Vec<T>, RowsMapped<[T; M]> = L::RowsMapped<[T; M]>>,
    T: ComplexFloat
{
    fn resample<N, W>(self, (): (), (): (), order: N, cutoff: W) -> Self::RowsMapped<[T; M]>
    where
        N: Maybe<usize>,
        W: Maybe<<T as ComplexFloat>::Real>
    {
        self.resample(M, (), order, cutoff)
            .map_rows_into_owned(|y| {
                TryInto::<[T; M]>::try_into(y)
                    .ok()
                    .unwrap()
            })
    }
}

#[cfg(test)]
mod test
{
    use core::f64::consts::TAU;

    use array_math::ArrayOps;
    use linspace::LinspaceArray;

    use crate::{plot, operations::resampling::Resample};

    #[test]
    fn test()
    {
        const N: usize = 32;
        const M: usize = 1024;
        const T: f64 = 5.0;
        const F: f64 = 1.0;

        let tx: [_; N] = (0.0..=T).linspace_array();
        let x = tx.map(|t| (TAU*F*t).cos());

        let ty: [_; M] = (0.0..=T).linspace_array();
        let y = x.resample((), (), (), ());

        plot::plot_curves("x(t)", "plots/x_t_resample.png", [&tx.zip(x), &ty.zip(y)])
            .unwrap()
    }
}