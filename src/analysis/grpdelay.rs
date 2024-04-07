use core::ops::{AddAssign, Mul, MulAssign};

use array_math::{ArrayOps, SliceMath};
use num::{complex::ComplexFloat, traits::FloatConst, Float, Complex, NumCast, One, Zero};
use option_trait::Maybe;

use crate::{Conv, List, Lists, MaybeList, MaybeLists, System, Tf};

pub trait GrpDelay<'a, H, W, N>: System
where
    H: Lists<<Self::Domain as ComplexFloat>::Real>,
    W: List<<Self::Domain as ComplexFloat>::Real>,
    N: Maybe<usize>,
{
    fn grpdelay<FS>(&'a self, n: N, sampling_rate: FS) -> (H, W)
    where
        FS: Maybe<<Self::Domain as ComplexFloat>::Real>;
}

impl<'a, T, B, A, const N: usize> GrpDelay<'a, B::RowsMapped<[T::Real; N]>, [T::Real; N], ()> for Tf<T, B, A>
where
    T: ComplexFloat,
    B: MaybeLists<T>,
    A: MaybeList<T>,
    B::RowsMapped<[T::Real; N]>: Lists<T::Real>,
    Vec<Complex<T::Real>>: for<'b> Conv<Complex<T::Real>, Complex<T::Real>, &'b [Complex<T::Real>], Output = Vec<Complex<T::Real>>>,
    Complex<T::Real>: AddAssign + MulAssign + Mul<T::Real, Output = Complex<T::Real>>,
    Self: 'a,
    &'a Self: Into<Tf<Complex<T::Real>, Vec<Vec<Complex<T::Real>>>, Vec<Complex<T::Real>>>>
{
    fn grpdelay<FS>(&'a self, (): (), sampling_rate: FS) -> (B::RowsMapped<[T::Real; N]>, [T::Real; N])
    where
        FS: Maybe<<Self::Domain as ComplexFloat>::Real>
    {
        let Tf { b, a }: Tf<Complex<T::Real>, Vec<Vec<Complex<T::Real>>>, Vec<Complex<T::Real>>> = self.into();

        let fs = sampling_rate.into_option()
            .unwrap_or(T::Real::TAU());

        let nf = <T::Real as NumCast>::from(N).unwrap();
        let w = ArrayOps::fill(|i| <T::Real as NumCast>::from(i).unwrap()/nf*fs);

        let oa = a.len().saturating_sub(1);
        
        let mut a_rev_conj: Vec<_> = a.iter()
            .map(|&a| a.conj())
            .collect();
        a_rev_conj.reverse();

        let one = T::Real::one();
        let zero = T::Real::zero();

        let mut b = b.into_inner().into_iter();
        let gd = self.b.map_rows_to_owned(|_| {
            let b = b.next().unwrap();
            
            let c = b.conv(a_rev_conj.as_slice());
            let cr: Vec<_> = c.iter()
                .enumerate()
                .map(|(i, &c)| c*<T::Real as NumCast>::from(i).unwrap())
                .collect();

            let mut num = cr;
            num.resize(num.len().max(N), zero.into());
            num.fft();
            let mut num: [_; N] = num.try_into()
                .unwrap_or_else(|num: Vec<_>| {
                    let l = num.len();
                    let lf = <T::Real as NumCast>::from(l).unwrap();
                    ArrayOps::fill(|i| {
                        let j = <T::Real as NumCast>::from(i).unwrap()*lf/nf;
                        let p = j.fract();
                        let q = one - p;
                        let j0 = <usize as NumCast>::from(j.floor()).unwrap() % l;
                        let j1 = <usize as NumCast>::from(j.ceil()).unwrap() % l;
                        num[j0]*q + num[j1]*p
                    })
                });
            let mut den = c;
            den.resize(den.len().max(N), zero.into());
            den.fft();
            let mut den: [_; N] = den.try_into()
                .unwrap_or_else(|den: Vec<_>| {
                    let l = den.len();
                    let lf = <T::Real as NumCast>::from(l).unwrap();
                    ArrayOps::fill(|i| {
                        let j = <T::Real as NumCast>::from(i).unwrap()*lf/nf;
                        let p = j.fract();
                        let q = one - p;
                        let j0 = <usize as NumCast>::from(j.floor()).unwrap() % l;
                        let j1 = <usize as NumCast>::from(j.ceil()).unwrap() % l;
                        den[j0]*q + den[j1]*p
                    })
                });
            let oaf = <T::Real as NumCast>::from(oa).unwrap();
            let minmag = <T::Real as NumCast>::from(100u8).unwrap()*T::Real::epsilon();
            for b in 0..N
            {
                if den[b].abs() < minmag
                {
                    let db = den[b];
                    let arg = db.arg();
                    if !Float::is_finite(arg) || num[b].abs() < one
                    {
                        num[b] = oaf.into();
                        den[b] = one.into();
                    }
                    else
                    {
                        den[b] = Complex::cis(arg)*minmag*num[b].abs()
                    }
                }
            }

            let gd = num.comap(den, |n, d| (n/d).re - oaf);

            gd
        });

        (gd, w)
    }
}

#[cfg(test)]
mod test
{
    use array_math::ArrayOps;

    use crate::{plot, Butter, FilterGenPlane, GrpDelay, Tf};

    #[test]
    fn test()
    {
        let fs = 1000.0;

        let (n, wp, _ws, t) = crate::buttord(
            [40.0],
            [150.0],
            3.0,
            60.0,
            FilterGenPlane::Z { sampling_frequency: Some(fs) }
        ).unwrap();

        let h = Tf::butter(n, wp, t, FilterGenPlane::Z { sampling_frequency: None })
            .unwrap();

        const N: usize = 1024;
        let (h_t, w): ([_; N], _) = h.grpdelay((), ());

        plot::plot_curves("t(e^jw)", "plots/t_z_grpdelay.png", [&w.zip(h_t)]).unwrap();
    }
}