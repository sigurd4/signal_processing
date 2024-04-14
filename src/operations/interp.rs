use core::ops::MulAssign;

use array_math::max_len;
use num::{complex::ComplexFloat, NumCast, One};
use option_trait::Maybe;

use crate::{ComplexOp, FftFilt, Fir1, Fir1Type, List, System, Tf};

pub trait Interp<T, Q, Y>: List<T>
where
    T: ComplexFloat,
    Q: Maybe<usize>,
    Y: List<T>
{
    fn interp<N, W>(self, ratio: Q, order: N, cutoff: W) -> Y
    where
        N: Maybe<usize>,
        W: Maybe<T::Real>;
}

impl<T, L> Interp<T, usize, Vec<T>> for L
where
    T: ComplexFloat<Real: ComplexOp<T, Output = T> + ComplexFloat<Real = T::Real>> + MulAssign<T::Real>,
    L: List<T>,
    (): Maybe<T::Real>,
    Tf<T::Real, Vec<T::Real>>: Fir1<usize, [T::Real; 1], T::Real, (), false> + for<'a> FftFilt<'a, T, Vec<T>, Output = Vec<T>> + System<Domain = T::Real>
{
    fn interp<N, W>(self, ratio: usize, order: N, cutoff: W) -> Vec<T>
    where
        N: Maybe<usize>,
        W: Maybe<T::Real>
    {
        (|| {
            let x = self.into_vec();
            let l = x.len();
            if l == 0
            {
                return vec![]
            }
    
            let order = order.into_option()
                .unwrap_or(4);
            let cutoff = cutoff.into_option()
                .unwrap_or_else(|| {
                    let one = T::Real::one();
                    let two = one + one;
                    two.recip()
                });
    
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
        })()
    }
}

impl<T, L, const M: usize> Interp<T, (), [T; M]> for L
where
    T: ComplexFloat,
    L: List<T, Length = usize> + Interp<T, usize, Vec<T>>,
    [(); 0 - M % max_len(L::LENGTH, 1)]:
{
    fn interp<N, W>(self, (): (), order: N, cutoff: W) -> [T; M]
    where
        N: Maybe<usize>,
        W: Maybe<<T as ComplexFloat>::Real>
    {
        self.interp(M/L::LENGTH.max(1), order, cutoff)
            .try_into()
            .ok()
            .unwrap()
    }
}

#[cfg(test)]
mod test
{
    use crate::Interp;

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