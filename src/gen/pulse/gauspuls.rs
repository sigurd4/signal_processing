use num::{traits::FloatConst, Float};
use option_trait::Maybe;

use crate::{IntoList, ListOrSingle};

pub trait GausPuls<T, L, N>: IntoList<T, L, N>
where
    T: Float,
    L: ListOrSingle<T>,
    N: Maybe<usize>
{
    fn gauspuls(self, numtaps: N, fc: T, bw: T) -> (L::Mapped<T>, L::Mapped<T>, L::Mapped<T>, L);
}

impl<T, L, R, N> GausPuls<T, L, N> for R
where
    T: Float + FloatConst,
    L: ListOrSingle<T>,
    R: IntoList<T, L, N>,
    N: Maybe<usize>
{
    fn gauspuls(self, n: N, fc: T, bw: T) -> (L::Mapped<T>, L::Mapped<T>, L::Mapped<T>, L)
    {
        let t = self.into_list(n);

        let fv = -bw*bw*fc*fc/(T::from(8u8).unwrap()*(T::from(10u8).unwrap().powf(-T::from(3u8).unwrap()/T::from(10u8).unwrap())).ln());
        let tv = (T::TAU()*T::TAU()*fv).recip();
        let yi = t.map_to_owned(|&t| {
            (-t*t/(T::from(2u8).unwrap()*tv)).exp()*(T::TAU()*fc*t).cos()
        });
        let yq = t.map_to_owned(|&t| {
            (-t*t/(T::from(2u8).unwrap()*tv)).exp()*(T::TAU()*fc*t).sin()
        });
        let ye = t.map_to_owned(|&t| {
            (-t*t/(T::from(2u8).unwrap()*tv)).exp()
        });
        (yi, yq, ye, t)
    }
}

#[cfg(test)]
mod test
{
    use array_math::ArrayOps;

    use crate::{plot, GausPuls};

    #[test]
    fn test()
    {
        const N: usize = 1024;
        let (yi, yq, ye, t): ([_; N], _, _, _) = (-0.003..=0.003).gauspuls((), 1e3, 0.5);

        plot::plot_curves("y(t)", "plots/y_t_gauspuls.png", [&t.zip(yi), &t.zip(yq), &t.zip(ye)])
            .unwrap()
    }
}