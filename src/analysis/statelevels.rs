use core::{iter::Sum, ops::RangeInclusive};

use num::{traits::FloatConst, Float, NumCast};
use option_trait::{Maybe, StaticMaybe};

use crate::{IntoList, List, ListOrSingle, OwnedList, OwnedListOrSingle};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum StateLevelsMethod
{
    Mean,
    Mode
}

pub trait StateLevels<T, L, N, NN, B>: List<T>
where
    T: Float + FloatConst,
    L: List<T>,
    N: Maybe<NN> + Maybe<usize>,
    NN: Maybe<usize>,
    B: Maybe<RangeInclusive<T>>,
    RangeInclusive<T>: IntoList<T, L, NN>
{
    fn state_levels<M, H, HV>(self, num_bins: N, method: M, bounds: B) -> (RangeInclusive<T>, H, HV)
    where
        M: Maybe<StateLevelsMethod>,
        H: StaticMaybe<L::Mapped<usize>>,
        HV: StaticMaybe<L>;
}

impl<T, X, L, N, NN, B> StateLevels<T, L, N, NN, B> for X
where
    T: Float + FloatConst + Sum,
    X: List<T>,
    L: List<T>,
    N: Maybe<NN> + Maybe<usize>,
    NN: StaticMaybe<usize>,
    B: Maybe<RangeInclusive<T>>,
    RangeInclusive<T>: IntoList<T, L, NN>,
    L::Mapped<usize>: OwnedList<usize>
{
    fn state_levels<M, H, HV>(self, num_bins: N, method: M, bounds: B) -> (RangeInclusive<T>, H, HV)
    where
        M: Maybe<StateLevelsMethod>,
        H: StaticMaybe<L::Mapped<usize>>,
        HV: StaticMaybe<L>
    {
        let num_bins = NN::maybe_from_fn(|| num_bins.into_option()
            .unwrap_or(100)
        );
        let method = method.into_option()
            .unwrap_or(StateLevelsMethod::Mode);

        let x = self.into_vec();

        let bounds = bounds.into_option()
            .unwrap_or_else(|| {
                let xmax = x.iter()
                    .map(|&x| x)
                    .reduce(Float::max)
                    .unwrap_or_else(T::zero) + T::epsilon();
                let xmin = x.iter()
                    .map(|&x| x)
                    .reduce(Float::min)
                    .unwrap_or_else(T::zero) - T::epsilon();
                xmin..=xmax
            });
        let lower = *bounds.start();
        let upper = *bounds.end();
        let nbins = num_bins.as_option()
            .map(|&n| n)
            .unwrap_or(L::LENGTH);
        let nbinsf = T::from(nbins).unwrap();

        let zero = T::zero();
        let one = T::one();
        let two = one + one;
        let half = two.recip();

        let qh = (upper - lower)/((nbinsf - one)*two);

        let histvec = (lower + qh..=upper - qh).into_list(num_bins);

        let mut histogram = histvec.map_to_owned(|_| 0);

        for idx in x.iter()
            .filter_map(|&x| <usize as NumCast>::from((nbinsf * (x - lower)/(upper - lower)).ceil() - one))
            .filter(|&idx| idx < nbins)
        {
            histogram.as_mut_slice()[idx] += 1
        }
        let ilow = histogram.as_view_slice()
            .iter()
            .enumerate()
            .filter(|(_, &n)| n > 0)
            .map(|(i, _)| i)
            .next()
            .unwrap_or(0);
        let ihigh = histogram.as_view_slice()
            .iter()
            .enumerate()
            .rev()
            .filter(|(_, &n)| n > 0)
            .map(|(i, _)| i)
            .next()
            .unwrap_or(0);

        let llow = ilow;
        let lhigh = ilow + (ihigh - ilow)/2;
        let ulow = ilow + (ihigh - ilow)/2;
        let uhigh = ihigh;

        let lhist = &histogram.as_view_slice()[llow..=lhigh];
        let uhist = &histogram.as_view_slice()[ulow..=uhigh];

        let dy = (upper - lower)/nbinsf;

        let levels = match method
        {
            StateLevelsMethod::Mean => {
                let lsum = T::from(lhist.iter()
                        .map(|&h| h)
                        .sum::<usize>()
                    ).unwrap();
                let ldot = (llow..=lhigh).map(|i| {
                    if lsum.is_zero()
                    {
                        return zero
                    }
                    (T::from(i).unwrap() - half)
                        *T::from(histogram.as_view_slice()[i]).unwrap()
                        /lsum
                }).sum::<T>();
                let ls = lower + dy*ldot;
                
                let usum = T::from(uhist.iter()
                        .map(|&h| h)
                        .sum::<usize>()
                    ).unwrap();
                let udot = (ulow..=uhigh).map(|i| {
                    if usum.is_zero()
                    {
                        return zero
                    }
                    (T::from(i).unwrap() - half)
                        *T::from(histogram.as_view_slice()[i]).unwrap()
                        /usum
                }).sum::<T>();
                let us = lower + dy*udot;

                ls..=us
            },
            StateLevelsMethod::Mode => {
                let imax = T::from(lhist.iter()
                        .map(|&h| h)
                        .enumerate()
                        .reduce(|a, b| if a.1 >= b.1 {a} else {b})
                        .map(|(i, _)| i)
                        .unwrap_or(0)
                    ).unwrap();
                let imin = T::from(uhist.iter()
                        .map(|&h| h)
                        .enumerate()
                        .reduce(|a, b| if a.1 >= b.1 {a} else {b})
                        .map(|(i, _)| i)
                        .unwrap_or(0)
                    ).unwrap();

                let ls = lower + dy*(T::from(llow).unwrap() + imax + half);
                let us = lower + dy*(T::from(ulow).unwrap() + imin + half);

                ls..=us
            },
        };

        (
            levels,
            H::maybe_from_fn(|| histogram),
            HV::maybe_from_fn(|| histvec)
        )
    }
}

#[cfg(test)]
mod test
{
    use array_math::ArrayOps;
    use linspace::LinspaceArray;

    use crate::{plot, StateLevels};

    #[test]
    fn test()
    {
        const N: usize = 1024;
        let n: [_; N] = (0.0..N as f64).linspace_array();
        let x: [_; N] = core::array::from_fn(|i| ((i/100) % 2) as f64*3.3);

        const M: usize = 32;
        let (levels, histogram, histvec): (_, [_; M], [_; M]) = x.state_levels((), (), ());

        plot::plot_curves("n(x)", "plots/n_x_state_levels.png", [
                &histvec.zip(histogram.map(|n| n as f64))
            ]).unwrap();
        
        plot::plot_curves("x(n)", "plots/x_n_state_levels.png", [
                &n.zip(x),
                &n.zip([*levels.start(); _]),
                &n.zip([*levels.end(); _])
            ]).unwrap();
    }
}