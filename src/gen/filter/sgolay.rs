use core::ops::{Deref, DerefMut, Mul, MulAssign};

use ndarray::Array2;
use ndarray_linalg::Lapack;
use num::{complex::ComplexFloat, Float, NumCast};
use option_trait::{Maybe, MaybeAnd, StaticMaybe};
use thiserror::Error;

use crate::{util, quantities::{ListOrSingle, OwnedList}, util::MaybeLenEq, System, systems::Tf};

#[derive(Debug, Clone, Copy, PartialEq, Error)]
pub enum SGolayError
{
    #[error("Order must be less than number of taps.")]
    OrderOutOfRange,
    #[error("Derivative must be less than number of taps.")]
    DerivativeOutOfRange,
    #[error("Number of taps must be odd.")]
    EvenFilterLength
}

pub trait SGolay<L, N>: System + Sized
where
    L: ListOrSingle<Self> + Sized,
    N: Maybe<usize>
{
    fn sgolay<TS, M>(order: usize, numtaps: N, derivative: M, scale: TS) -> Result<L, SGolayError>
    where
        TS: Maybe<Self::Domain>,
        M: Maybe<usize>;
}

impl<L, T, B, N, NN> SGolay<L, N> for Tf<T, B>
where
    T: ComplexFloat<Real: Lapack<Real = <T as ComplexFloat>::Real> + Into<T>> + Mul<<T as ComplexFloat>::Real, Output = T> + MulAssign,
    Vec<T>: TryInto<B>,
    Vec<Tf<T, B>>: TryInto<L>,
    L: OwnedList<Tf<T, B>> + MaybeLenEq<B, true>,
    B: OwnedList<T>,
    <L::Length as StaticMaybe<usize>>::Opposite: MaybeAnd<usize, <B::Length as StaticMaybe<usize>>::Opposite, Output = NN>,
    NN: MaybeAnd<usize, N, Output = N, Opposite: Sized>,
    N: StaticMaybe<usize>,
    [(); (L::LENGTH % 2) - 1]:,
    [(); (B::LENGTH % 2) - 1]:
{
    fn sgolay<TS, M>(order: usize, numtaps: N, derivative: M, scale: TS) -> Result<L, SGolayError>
    where
        TS: Maybe<T>,
        M: Maybe<usize>
    {
        let n = numtaps.into_option()
            .unwrap_or_else(|| {
                NN::Opposite::maybe_from_fn(|| L::LENGTH.min(B::LENGTH))
                    .into_option()
                    .unwrap_or_else(|| order + 3 - order % 2)
            });
        let m = derivative.into_option()
            .unwrap_or(0);

        if n % 2 != 1
        {
            return Err(SGolayError::EvenFilterLength)
        }
        if order >= n
        {
            return Err(SGolayError::OrderOutOfRange)
        }
        if m >= n
        {
            return Err(SGolayError::DerivativeOutOfRange)
        }

        let scale = scale.into_option()
            .unwrap_or_else(T::one);

        let mut f = (0..n).map(|_| Tf::new(vec![T::zero(); n].try_into().ok().unwrap(), ()))
            .collect::<Vec<_>>()
            .try_into()
            .ok()
            .unwrap();

        let k = n/2;
        for row in 0..k + 1
        {
            let c = Array2::from_shape_fn((n, order + 1), |(n, p)| {
                let n = <T::Real as NumCast>::from(n).unwrap();
                let r = <T::Real as NumCast>::from(row).unwrap();
                Float::powi(n - r, p as i32)
            });
            let a = util::pinv(c);
            for (b, &a) in f.as_mut_slice()[row]
                .b
                .as_mut_slice()
                .iter_mut()
                .zip(a.row(m).into_iter())
            {
                *b = a.into()
            }
        }
        let s = <T::Real as NumCast>::from(1 - (m % 2) as i8*2).unwrap();
        for row in k + 1..n
        {
            let f: &mut [Tf<T, B>] = f.as_mut_slice();
            let a = f[n - 1 - row]
                .b
                .deref()
                .as_view_slice()
                .iter()
                .rev()
                .map(|&a: &T| a)
                .collect::<Vec<_>>();
            for (b, a) in f[row]
                .b
                .deref_mut()
                .as_mut_slice()
                .iter_mut()
                .zip(a)
            {
                *b = a*s
            }
        }

        let scale = util::factorial::<<T as ComplexFloat>::Real, _>(m).into()/scale.powi(m as i32);

        for f in f.as_mut_slice()
            .iter_mut()
        {
            for b in f.b.as_mut_slice()
                .iter_mut()
            {
                *b *= scale
            }
        }

        Ok(f)
    }
}

#[cfg(test)]
mod test
{
    use crate::{plot, analysis::RealFreqZ, gen::filter::SGolay, systems::Tf};

    #[test]
    fn test()
    {
        const N: usize = 21;
        let h: [Tf::<_, Vec<_>>; N] = Tf::sgolay(4, (), (), ())
            .unwrap();

        const M: usize = 1024;
        let h_fw: [(Vec<_>, _); N] = h.map(|h| h.real_freqz(M));

        plot::plot_curves("H(e^jw)", "plots/h_z_sgolay.png",
                h_fw.map(|(h_f, w)| w.into_iter().zip(h_f.into_iter().map(|h| h.norm())).collect::<Vec<_>>()).each_ref().map(|wh| wh.as_slice())
            ).unwrap();
    }
}