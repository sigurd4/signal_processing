use core::ops::{AddAssign, MulAssign};

use num::{complex::ComplexFloat, traits::float::FloatConst, Complex, NumCast, One, Zero};
use array_math::SliceMath;
use option_trait::Maybe;

use crate::{ContainerOrSingle, List, ListOrSingle, Lists, OwnedList};

pub trait Czt<T>: Lists<T>
where
    T: ComplexFloat
{
    fn czt<R, P>(self, ratio: R, point: P) -> Self::Mapped<Complex<T::Real>>
    where
        R: Maybe<Complex<T::Real>>,
        P: Maybe<Complex<T::Real>>;
}

impl<T, L, LL> Czt<T> for L
where
    T: ComplexFloat + Into<Complex<T::Real>>,
    L: Lists<T, RowOwned: OwnedList<T, Mapped<Complex<T::Real>> = LL>, RowsMapped<LL>: Into<L::Mapped<Complex<T::Real>>>>,
    <L::RowOwned as ContainerOrSingle<T>>::Mapped<()>: List<(), Mapped<Complex<T::Real>>: Into<LL>>,
    Complex<T::Real>: AddAssign + MulAssign + MulAssign<T::Real>
{
    fn czt<R, P>(self, ratio: R, point: P) -> Self::Mapped<Complex<T::Real>>
    where
        R: Maybe<Complex<T::Real>>,
        P: Maybe<Complex<T::Real>>
    {
        let a = point.into_option()
            .map(|a| a.into())
            .unwrap_or_else(Complex::one);
        let w = ratio.into_option()
            .map(|w| w.into());

        self.map_rows_into_owned(|x| {
            let n = x.length();
            let nfft = (2*n).saturating_sub(1);
            let nfft_pow2 = nfft.next_power_of_two();

            let y_void = x.map_to_owned(|_| ());

            let w = w.unwrap_or_else(|| Complex::cis(-T::Real::TAU()/<T::Real as NumCast>::from(n).unwrap()));
            let w_sqrt = w.sqrt();
            let w2: Vec<_> = (2..=2*n).map(|i| {
                let p = i as i32 - 1 - n as i32;
                if let Some(pp) = p.checked_mul(p)
                {
                    w_sqrt.powi(pp)
                }
                else
                {
                    w_sqrt.powi(p).powi(p)
                }
            }).collect();
            let mut fw: Vec<Complex<_>> = w2[..nfft].iter()
                .map(|&w| w.recip().into())
                .collect();
            fw.resize(nfft_pow2, Complex::zero());
            fw.fft();

            let mut fg: Vec<_> = x.into_vec()
                .into_iter()
                .map(|x| x.into())
                .collect();
            let a_recip = a.recip();
            let mut apmk = Complex::one();
            for (i, g)  in fg.iter_mut()
                .enumerate()
            {
                *g *= apmk*w2[i + n - 1];
                apmk *= a_recip;
            }
            fg.resize(nfft_pow2, Complex::zero());
            fg.fft();

            let mut gg: Vec<_> = fg.into_iter()
                .zip(fw)
                .map(|(g, w)| g*w)
                .collect();
            gg.ifft();

            let mut y = gg.into_iter()
                .zip(w2)
                .skip(n.saturating_sub(1))
                .map(|(g, w)| g*w);

            y_void.map_to_owned(|_| y.next().unwrap())
                .into()
        }).into()
    }
}

#[cfg(test)]
mod test
{
    use crate::Czt;

    #[test]
    fn test()
    {
        let x = [1.0, 2.0, 3.0, 4.0];

        let y = x.czt((), ());

        println!("{:?}", y);
    }
}