use core::ops::{Range, RangeInclusive};

use array_math::ArrayOps;
use num::{traits::FloatConst, Complex, Float};
use option_trait::Maybe;

use crate::{List, NotRange};

pub trait CMorWavF<'a, T, L, N>
where
    T: Float,
    L: List<T>,
    N: Maybe<usize>
{
    fn cmorwavf(&'a self, numtaps: N, fb: T, fc: T) -> (L::Mapped<Complex<T>>, L);
}

impl<'a, T, L> CMorWavF<'a, T, L::View<'a>, ()> for L
where
    T: Float + FloatConst,
    L: List<T> + NotRange,
    L::View<'a>: List<T, Mapped<Complex<T>> = L::Mapped<Complex<T>>>
{
    fn cmorwavf(&'a self, (): (), fb: T, fc: T) -> (L::Mapped<Complex<T>>, L::View<'a>)
    {
        let psi = self.map_to_owned(|&x| {
            Complex::from(T::PI()*fb).inv().sqrt()*Complex::cis(T::TAU()*fc*x)*(-x*x/fb).exp()
        });

        (psi, self.as_view())
    }
}

impl<'a, T, const N: usize> CMorWavF<'a, T, [T; N], ()> for Range<T>
where
    T: Float + FloatConst,
    [T; N]: NotRange
{
    fn cmorwavf(&self, (): (), fb: T, fc: T) -> ([Complex<T>; N], [T; N])
    {
        let x: [_; N] = ArrayOps::fill(|i| {
            let p = T::from(i).unwrap()/T::from(N).unwrap();
            self.start + (self.end - self.start)*p
        });
        
        (x.cmorwavf((), fb, fc).0, x)
    }
}
impl<'a, T> CMorWavF<'a, T, Vec<T>, usize> for Range<T>
where
    T: Float + FloatConst,
    Vec<T>: NotRange
{
    fn cmorwavf(&self, n: usize, fb: T, fc: T) -> (Vec<Complex<T>>, Vec<T>)
    {
        let x: Vec<_> = (0..n).map(|i| {
                let p = T::from(i).unwrap()/T::from(n).unwrap();
                self.start + (self.end - self.start)*p
            }).collect();

        (x.cmorwavf((), fb, fc).0, x)
    }
}

impl<'a, T, const N: usize> CMorWavF<'a, T, [T; N], ()> for RangeInclusive<T>
where
    T: Float + FloatConst,
    [T; N]: NotRange
{
    fn cmorwavf(&self, (): (), fb: T, fc: T) -> ([Complex<T>; N], [T; N])
    {
        let x: [_; N] = ArrayOps::fill(|i| {
            let p = T::from(i).unwrap()/T::from(N - 1).unwrap();
            *self.start() + (*self.end() - *self.start())*p
        });
        
        (x.cmorwavf((), fb, fc).0, x)
    }
}
impl<'a, T> CMorWavF<'a, T, Vec<T>, usize> for RangeInclusive<T>
where
    T: Float + FloatConst,
    Vec<T>: NotRange
{
    fn cmorwavf(&self, n: usize, fb: T, fc: T) -> (Vec<Complex<T>>, Vec<T>)
    {
        let x: Vec<_> = (0..n).map(|i| {
                let p = T::from(i).unwrap()/T::from(n - 1).unwrap();
                *self.start() + (*self.end() - *self.start())*p
            }).collect();

        (x.cmorwavf((), fb, fc).0, x)
    }
}

#[cfg(test)]
mod test
{
    use array_math::ArrayOps;

    use crate::{plot, CMorWavF};

    #[test]
    fn test()
    {
        const N: usize = 1024;
        let (psi, t): ([_; N], _) = (-8.0..=8.0).cmorwavf((), 1.5, 1.0);

        plot::plot_curves("ψ(t)", "plots/psi_t_cmorwavf.png", [&t.zip(psi.map(|psi| psi.re)), &t.zip(psi.map(|psi| psi.im))])
            .unwrap()
    }
}