use core::ops::MulAssign;

use array_math::max_len;
use num::{complex::ComplexFloat, NumCast, One};
use option_trait::Maybe;

use crate::{util::ComplexOp, operations::filtering::FftFilt, gen::filter::{Fir1, Fir1Type}, quantities::{List, ListOrSingle, Lists, MaybeLists}, System, systems::Tf};

pub trait Interp<T, Q, Y>: Lists<T>
where
    T: ComplexFloat,
    Q: Maybe<usize>,
    Y: List<T>
{
    fn interp<N, W>(self, ratio: Q, order: N, cutoff: W) -> Self::RowsMapped<Y>
    where
        N: Maybe<usize>,
        W: Maybe<T::Real>;
}

impl<T, L> Interp<T, usize, Vec<T>> for L
where
    T: ComplexFloat<Real: ComplexOp<T, Output = T> + ComplexFloat<Real = T::Real>> + MulAssign<T::Real>,
    L: Lists<T, RowOwned: List<T>>,
    (): Maybe<T::Real>,
    Tf<T::Real, Vec<T::Real>>: Fir1<usize, [T::Real; 1], T::Real, (), false> + for<'a> FftFilt<'a, T, Vec<T>, Output = Vec<T>> + System<Set = T::Real>
{
    fn interp<N, W>(self, ratio: usize, order: N, cutoff: W) -> Self::RowsMapped<Vec<T>>
    where
        N: Maybe<usize>,
        W: Maybe<T::Real>
    {
        let order = order.into_option()
            .unwrap_or(4);
        let cutoff = cutoff.into_option()
            .unwrap_or_else(|| {
                let one = T::Real::one();
                let two = one + one;
                two.recip()
            });
            
        self.map_rows_into_owned(|x| {
            let x = x.into_vec();
            let l = x.len();
            if l == 0
            {
                return vec![]
            }
    
            let mut y = vec![T::zero(); l*ratio + ratio*order + 1];
    
            for (i, x) in x.into_iter()
                .enumerate()
            {
                y[i*ratio] = x;
            }
            if l >= 2
            {
                y[l*ratio] = y[(l - 1)*ratio] + (y[(l - 1)*ratio] - y[(l - 2)*ratio]);
            }
            else if l >= 1
            {
                y[l*ratio] = y[(l - 1)*ratio];
            }
            let ratiof = NumCast::from(ratio).unwrap();
            let b = Tf::<T::Real, _, _>::fir1(
                2*ratio*order,
                [cutoff/ratiof],
                Fir1Type::LowPass,
                (),
                false,
                ()
            ).unwrap();
            y = b.fftfilt(y, ());
            for y in y.iter_mut()
            {
                *y *= ratiof
            }
            let mut y = y.split_off((ratio*order).saturating_sub(ratio/2));
            y.truncate(ratio*l);
            y
        })
    }
}

impl<T, L, const M: usize> Interp<T, (), [T; M]> for L
where
    T: ComplexFloat,
    L: Lists<T, Width = usize> + Interp<T, usize, Vec<T>>,
    Self::RowsMapped<Vec<T>>: Lists<T, RowOwned = Vec<T>, RowsMapped<[T; M]> = L::RowsMapped<[T; M]>>,
    [(); 0 - M % max_len(L::WIDTH, 1)]:
{
    fn interp<N, W>(self, (): (), order: N, cutoff: W) -> Self::RowsMapped<[T; M]>
    where
        N: Maybe<usize>,
        W: Maybe<<T as ComplexFloat>::Real>
    {
        self.interp(M/L::WIDTH.max(1), order, cutoff)
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
    use crate::operations::resampling::Interp;

    #[test]
    fn test()
    {
        const N: usize = 3;
        const Q: usize = 3;

        let x: [f64; N] = [1.0, 2.0, 3.0];
        let y: [_; N*Q] = x.interp((), (), ());

        println!("{:?}", y)
    }
}