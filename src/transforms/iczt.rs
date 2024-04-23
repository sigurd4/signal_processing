use core::ops::{AddAssign, DivAssign, Mul, MulAssign};

use num::{complex::ComplexFloat, traits::float::FloatConst, Complex, NumCast, One, Zero};
use array_math::SliceMath;
use option_trait::Maybe;

use crate::{ContainerOrSingle, List, ListOrSingle, Lists, OwnedList};

pub trait Iczt<T>: Lists<T>
where
    T: ComplexFloat
{
    fn iczt<R, P>(self, ratio: R, point: P) -> Self::Mapped<Complex<T::Real>>
    where
        R: Maybe<Complex<T::Real>>,
        P: Maybe<Complex<T::Real>>;
}

impl<T, L, LL> Iczt<T> for L
where
    T: ComplexFloat + Into<Complex<T::Real>>,
    L: Lists<T, RowOwned: OwnedList<T, Mapped<Complex<T::Real>> = LL>, RowsMapped<LL>: Into<L::Mapped<Complex<T::Real>>>>,
    <L::RowOwned as ContainerOrSingle<T>>::Mapped<()>: List<(), Mapped<Complex<T::Real>>: Into<LL>>,
    Complex<T::Real>: AddAssign + MulAssign + DivAssign + MulAssign<T::Real> + Mul<T, Output = Complex<T::Real>> + Mul<Output = Complex<T::Real>>,
    T::Real: AddAssign
{
    fn iczt<R, P>(self, ratio: R, point: P) -> Self::Mapped<Complex<T::Real>>
    where
        R: Maybe<Complex<T::Real>>,
        P: Maybe<Complex<T::Real>>
    {
        let czero = Complex::zero();
        let cone = Complex::one();

        let a = point.into_option()
            .map(|a| a.into())
            .unwrap_or(cone);
        let w = ratio.into_option()
            .map(|w| w.into());

        self.map_rows_into_owned(|y| {
            let n = y.length();
            let nfft = (2*n).saturating_sub(1);
            let nfft_pow2 = nfft.next_power_of_two();

            let x_void = y.map_to_owned(|_| ());

            let w = w.unwrap_or_else(|| Complex::cis(-T::Real::TAU()/<T::Real as NumCast>::from(n).unwrap()));
            let w_recip = w.recip();
            let mut c0 = cone;
            let mut wp_recip = w_recip;
            for _ in 1..n
            {
                c0 /= cone - wp_recip;
                wp_recip *= w_recip;
            }
            
            let mut ck = c0;
            let wn_recip = w_recip.powi(n as i32);
            let mut wk = cone;
            let mut cyz: Vec<_> = core::iter::once((c0, a))
                .chain((1..n).map(|_| {
                    wk *= w;
                    ck *= (cone - wn_recip*wk)/(cone - wk);
                    (ck, a/wk)
                })).zip(y.into_vec())
                .map(|((c, z), y)| (c*y, z))
                .collect();

            let mut h: Vec<_> = (0..n).map(|_| {
                let hn = cyz.iter_mut()
                    .map(|(cy, z)| {
                        let cyn = *cy;
                        *cy *= *z;
                        cyn
                    }).sum::<Complex<_>>();
                hn
            }).collect();
            
            let mut m: Vec<_> = cyz.into_iter()
                .map(|(_, z)| vec![cone, -z])
                .reduce(|a, b| a.convolve_direct(&b))
                .unwrap_or_else(|| vec![cone]);

            h.resize(nfft_pow2, czero);
            h.fft();

            m.resize(nfft_pow2, czero);
            m.fft();

            let mut x: Vec<_> = h.into_iter()
                .zip(m.into_iter())
                .map(|(h, m)| h*m)
                .collect();
            x.ifft();
            let mut x = x.into_iter();

            x_void.map_to_owned(|_| x.next().unwrap())
                .into()
        }).into()
    }
}

#[cfg(test)]
mod test
{
    use num::Complex;

    use crate::{Czt, Iczt};

    #[test]
    fn test()
    {
        let x = [1.0, 2.0, 3.0, 4.0];

        let a = Complex::new(1.0, -4.0);
        let w = Complex::new(-0.5, 0.1);

        let y = x.czt(w, a);
        let x = y.iczt(w, a);

        println!("{:?}", x);
    }
}