use core::ops::{AddAssign, Div, DivAssign};
use std::ops::Mul;

use array_math::{ArrayOps, SliceOps};
use num::{complex::ComplexFloat, traits::FloatConst, Float, NumCast, One};
use option_trait::Maybe;
use thiserror::Error;

use crate::{MaybeList, MaybeLists, Polynomial, System, Tf, ToTf};

#[derive(Debug, Clone, Copy, PartialEq, Error)]
pub enum ZfTransError
{
    #[error("Non causal system, i.e. it contains one or more poles at infinity.")]
    NonCausal,
    #[error("The system must contain at least one pole.")]
    ZeroPoles,
    #[error("Frequencies must be monotonic starting at zero.")]
    FrequenciesNotNondecreasing,
    #[error("Frequencies must be positive, and if the filter is digital; less than 1/2 the sampling frequency, or if no sampling frequency is specified, between 0 and 1.")]
    FrequenciesOutOfRange,
}

pub trait ZfTrans<const W: usize>: System
where
    [(); W - 1]:
{
    type Output;

    fn zftrans<FS>(
        self,
        wo: <Self::Domain as ComplexFloat>::Real,
        wt: [<Self::Domain as ComplexFloat>::Real; W],
        sampling_frequency: FS,
        stop: bool
    ) -> Result<Self::Output, ZfTransError>
    where
        FS: Maybe<<Self::Domain as ComplexFloat>::Real>;
}

impl<T, B, A, const W: usize> ZfTrans<W> for Tf<T, B, A>
where
    T: ComplexFloat + Mul<T::Real, Output = T> + Div<T::Real, Output = T> + AddAssign + DivAssign,
    B: MaybeLists<T>,
    A: MaybeList<T>,
    B::RowsMapped<Vec<T>>: MaybeLists<T, RowsMapped<Tf<T, Vec<T>, Vec<T>>> = B::RowsMapped<Tf<T, Vec<T>, Vec<T>>>, RowOwned = Vec<T>>,
    Self: ToTf<T, B::RowsMapped<Vec<T>>, Vec<T>, (), ()> + System<Domain = T>,
    [(); W - 1]:
{
    type Output = (B::RowsMapped<Tf<T, Vec<T>, Vec<T>>>, Tf<T, Vec<T>, Vec<T>>);

    fn zftrans<FS>(
        self,
        mut wo: T::Real,
        mut w: [T::Real; W],
        sampling_frequency: FS,
        stop: bool
    ) -> Result<Self::Output, ZfTransError>
    where
        FS: Maybe<T::Real>
    {
        if !w.is_sorted()
        {
            return Err(ZfTransError::FrequenciesNotNondecreasing)
        }

        let one = T::Real::one();
        let two = one + one;

        let t = sampling_frequency.into_option()
            .unwrap_or(two);
        for wc in w.iter_mut()
        {
            if *wc > t/two
            {
                return Err(ZfTransError::FrequenciesOutOfRange)
            }
            *wc = *wc*two/t
        }
        if wo > t/two
        {
            return Err(ZfTransError::FrequenciesOutOfRange)
        }
        wo = wo*two/t;

        let Tf::<T, B::RowsMapped<Vec<_>>, Vec<_>> {b, a} = self.to_tf((), ());

        let pass_stop = if stop {1i8} else {-1};
        
        fn pk<T>(pkm1: Polynomial<T, Vec<T>>, k: usize, phik: T::Real) -> Polynomial<T, Vec<T>>
        where
            T: ComplexFloat + Mul<T::Real, Output = T> + Div<T::Real, Output = T> + AddAssign + DivAssign
        {
            let one = T::Real::one();
            let two = one + one;

            let sin_k = Float::sin(phik/two);
            let cos_k = Float::cos(phik/two);
            let mut pk = Polynomial::new(vec![T::zero(); k + 1]);
            let s = <T::Real as NumCast>::from(1 - (k % 2) as i8*2).unwrap();
            for i in 0..k
            {
                pk[i] += pkm1[i]*sin_k - pkm1[k - 1 - i]*cos_k*s;
                pk[i + 1] += pkm1[i]*sin_k + pkm1[k - 1 - i]*cos_k*s;
            }
            let pk1 = pk[0];
            pk.div_assign_all(pk1);
            pk
        }

        fn apd<T>(phi: &[T::Real]) -> Polynomial<T, Vec<T>>
        where
            T: ComplexFloat + Mul<T::Real, Output = T> + Div<T::Real, Output = T> + AddAssign + DivAssign
        {
            let mut p = Polynomial::new(vec![T::one()]);
            for (i, &phi) in phi.into_iter()
                .enumerate()
            {
                p = pk(p, i + 1, phi)
            }
            p
        }

        let k = apd::<T>(&[wo*T::Real::PI()]);

        let phi = w.mul_all(T::Real::PI());
        let p = apd::<T>(&phi);
        let mut pp = p.clone();
        pp.reverse();

        let mut p: Polynomial<T, Vec<T>> = p - (pp*Polynomial::new([k[1]]));
        let pdiv: T = p[0];
        p.div_assign_all(pdiv);
        let mut pp = p.as_view()*Polynomial::new([T::from(pass_stop).unwrap()]);
        pp.reverse();

        Ok((
            b.into_inner().map_rows_into_owned(|b: Vec<_>| {
                let na = a.len();
                let nb = b.len();
                let n = na.max(nb);
                //let np = p.len();
                //let powcols = np + np*n + 2 - n - np*2;
                let mut ptemp = Polynomial::new(vec![T::one()]);
                let ppower: Vec<_> = (0..n).map(|i| {
                    let ppower = ptemp.clone();
                    if i + 1 < n
                    {
                        ptemp = ptemp.as_view()*p.as_view();
                    }
                    ppower
                }).collect();

                let mut num = Polynomial::new(vec![]);
                let mut den = Polynomial::new(vec![]);

                for i in 0..n
                {
                    let p_pow = ppower[n - 1 - i].as_view();
                    let mut pp_pow = ppower[i].clone();
                    pp_pow.reverse();

                    let s = <T::Real as NumCast>::from(pass_stop.pow(i as u32)).unwrap();
                    let ppp = pp_pow*p_pow;
                    if i < nb
                    {
                        num = num + ppp.as_view()*Polynomial::new([b[i]*s])
                    }
                    if i < na
                    {
                        den = den + ppp*Polynomial::new([a[i]*s])
                    }
                }

                if let Some(&temp) = den.get(0)
                {
                    den.div_assign_all(temp);
                    num.div_assign_all(temp);
                }

                Tf {
                    b: num,
                    a: den
                }
            }),
            Tf {
                b: pp,
                a: p
            }
        ))
    }
}

#[cfg(test)]
mod test
{
    use array_math::ArrayOps;
    use linspace::LinspaceArray;

    use crate::{plot, Ellip, FilterGenPlane, FilterGenType, RealFreqZ, Tf, ZfTrans};

    #[test]
    fn test()
    {
        let h0 = Tf::ellip(3, 0.1, 30.0, [0.409], FilterGenType::LowPass, FilterGenPlane::Z {sampling_frequency: None})
            .unwrap();

        let (h, _) = h0.zftrans(0.5, [0.2, 0.4, 0.6, 0.8], (), false)
            .unwrap();

        const N: usize = 1024;
        let (h_f, _): ([_; N], _) = h.real_freqz(());

        plot::plot_curves("H(e^jw)", "plots/h_z_zftrans.png", [&(0.0..1.0).linspace_array().zip(h_f.map(|h| h.norm()))]).unwrap();
    }
}