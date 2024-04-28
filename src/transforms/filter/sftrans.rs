use core::ops::{Add, Div, DivAssign, MulAssign, Sub};
use std::ops::Mul;

use num::{complex::ComplexFloat, Complex, Float, One, Zero};
use thiserror::Error;

use crate::{MaybeList, ProductSequence, System, ToZpk, Zpk};

#[derive(Debug, Clone, Copy, PartialEq, Error)]
pub enum SfTransError
{
    #[error("Non causal system, i.e. it contains one or more poles at infinity.")]
    NonCausal,
    #[error("The system must contain at least one pole.")]
    ZeroPoles,
    #[error("Frequencies must be monotonic starting at zero.")]
    FrequenciesNotNondecreasing,
}

pub trait SfTrans<const W: usize>: System
where
    [(); W - 1]:
{
    type Output: System<Domain = Self::Domain>;

    fn sftrans(
        self,
        wo: <Self::Domain as ComplexFloat>::Real,
        w: [<Self::Domain as ComplexFloat>::Real; W],
        stop: bool
    ) -> Result<Self::Output, SfTransError>;
}

impl<T, Z, P, K, const W: usize> SfTrans<W> for Zpk<T, Z, P, K>
where
    T: ComplexFloat + Mul<T::Real, Output = T> + Div<T::Real, Output = T> + Sub<T::Real, Output = T> + Into<Complex<T::Real>>,
    K: ComplexFloat<Real = T::Real> + DivAssign<T::Real> + MulAssign<T::Real>,
    T::Real: Into<T>,
    Complex<T::Real>: Add<T, Output = Complex<T::Real>>,
    Z: MaybeList<T>,
    P: MaybeList<T>,
    Self: ToZpk<T, Vec<T>, Vec<T>, K, (), ()> + System<Domain = K>,
    [(); W - 1]:,
    [(); 2 - W]:,
{
    type Output = Zpk<Complex<T::Real>, Vec<Complex<T::Real>>, Vec<Complex<T::Real>>, K>;

    fn sftrans(
        self,
        wo: <Self::Domain as ComplexFloat>::Real,
        w: [T::Real; W],
        stop: bool
    ) -> Result<Self::Output, SfTransError>
    {
        if !w.is_sorted()
        {
            return Err(SfTransError::FrequenciesNotNondecreasing)
        }

        let Zpk::<T, Vec<T>, Vec<T>, K> {z: sz, p: sp, k: mut sg} = self.to_zpk((), ());
    
        let two = T::Real::one() + T::Real::one();
        let c = wo;
        let p = sp.len();
        let z = sz.len();
        if z > p
        {
            return Err(SfTransError::NonCausal)
        }
        if p == 0
        {
            return Err(SfTransError::ZeroPoles)
        }
    
        if W == 2
        {
            let fl = w[0];
            let fh = w[1];
            if stop
            {
                if let Some(prod) = sp.iter()
                    .map(|&sp| -sp)
                    .reduce(Mul::mul)
                {
                    sg /= prod.re()
                }
                if let Some(prod) = sz.iter()
                    .map(|&sz| -sz)
                    .reduce(Mul::mul)
                {
                    sg *= prod.re()
                }
                let b_mul = c*(fh - fl)/two;
                let sp = {
                    let b = sp.into_inner()
                        .into_iter()
                        .map(|sp| Into::<T>::into(b_mul)/sp)
                        .collect::<Vec<_>>();
                    let bs = b.iter()
                        .map(|&b| Into::<Complex<T::Real>>::into(b*b - fh*fl).sqrt())
                        .collect::<Vec<_>>();
                    [
                        b.iter()
                            .zip(bs.iter())
                            .map(|(&b, &bs)| bs + b)
                            .collect::<Vec<_>>(),
                        b.into_iter()
                            .zip(bs)
                            .map(|(b, bs)| -bs + b)
                            .collect()
                    ].concat()
                };
                let mut sz = {
                    let b = sz.into_inner()
                        .into_iter()
                        .map(|sz| Into::<T>::into(b_mul)/sz)
                        .collect::<Vec<_>>();
                    let bs = b.iter()
                        .map(|&b| Into::<Complex<T::Real>>::into(b*b - fh*fl).sqrt())
                        .collect::<Vec<_>>();
                    [
                        b.iter()
                            .zip(bs.iter())
                            .map(|(&b, &bs)| bs + b)
                            .collect::<Vec<_>>(),
                        b.into_iter()
                            .zip(bs)
                            .map(|(b, bs)| -bs + b)
                            .collect()
                    ].concat()
                };
                let extend0 = Into::<Complex<T::Real>>::into(-fh*fl).sqrt();
                let extend = [extend0, -extend0];
                sz.append(&mut (1..=2*(p - z)).map(|i| extend[i % 2]).collect());
                Ok(Zpk {
                    z: ProductSequence::new(sz),
                    p: ProductSequence::new(sp),
                    k: sg
                })
            }
            else
            {
                sg *= Float::powi((fh - fl)/c, (p - z) as i32);
                let b_mul = (fh - fl)/(two*c);
                let sp = {
                    let b = sp.into_inner()
                        .into_iter()
                        .map(|sp| sp*b_mul)
                        .collect::<Vec<_>>();
                    let bs = b.iter()
                        .map(|&b| Into::<Complex<T::Real>>::into(b*b - fh*fl).sqrt())
                        .collect::<Vec<_>>();
                    [
                        b.iter()
                            .zip(bs.iter())
                            .map(|(&b, &bs)| bs + b)
                            .collect::<Vec<_>>(),
                        b.into_iter()
                            .zip(bs)
                            .map(|(b, bs)| -bs + b)
                            .collect()
                    ].concat()
                };
                let mut sz = {
                    let b = sz.into_inner()
                        .into_iter()
                        .map(|sz| sz*b_mul)
                        .collect::<Vec<_>>();
                    let bs = b.iter()
                        .map(|&b| Into::<Complex<T::Real>>::into(b*b - fh*fl).sqrt())
                        .collect::<Vec<_>>();
                    [
                        b.iter()
                            .zip(bs.iter())
                            .map(|(&b, &bs)| bs + b)
                            .collect::<Vec<_>>(),
                        b.into_iter()
                            .zip(bs)
                            .map(|(b, bs)| -bs + b)
                            .collect()
                    ].concat()
                };
                sz.append(&mut vec![Zero::zero(); p - z]);
                Ok(Zpk {
                    z: ProductSequence::new(sz),
                    p: ProductSequence::new(sp),
                    k: sg
                })
            }
        }
        else
        {
            let fc = w[0];
            if stop
            {
                if let Some(prod) = sp.iter()
                    .map(|&sp| -sp)
                    .reduce(Mul::mul)
                {
                    sg /= prod.re()
                }
                if let Some(prod) = sz.iter()
                    .map(|&sz| -sz)
                    .reduce(Mul::mul)
                {
                    sg /= prod.re()
                }
                let b_mul = c*fc;
                let sp = sp.into_inner()
                    .into_iter()
                    .map(|sp| Into::<Complex<T::Real>>::into(Into::<T>::into(b_mul)/sp))
                    .collect::<Vec<_>>();
                let mut sz = sz.into_inner()
                    .into_iter()
                    .map(|sz| Into::<Complex<T::Real>>::into(Into::<T>::into(b_mul)/sz))
                    .collect::<Vec<_>>();
                sz.append(&mut vec![Zero::zero(); p - z]);
                Ok(Zpk {
                    z: ProductSequence::new(sz),
                    p: ProductSequence::new(sp),
                    k: sg
                })
            }
            else
            {
                sg *= Float::powi(fc/c, (p - z) as i32);
                let b_mul = fc/c;
                let sp = sp.into_inner()
                    .into_iter()
                    .map(|sp| Into::<Complex<T::Real>>::into(Into::<T>::into(b_mul)*sp))
                    .collect::<Vec<_>>();
                let sz = sz.into_inner()
                    .into_iter()
                    .map(|sz| Into::<Complex<T::Real>>::into(Into::<T>::into(b_mul)*sz))
                    .collect::<Vec<_>>();
                Ok(Zpk {
                    z: ProductSequence::new(sz),
                    p: ProductSequence::new(sp),
                    k: sg
                })
            }
        }
    }
}

#[cfg(test)]
mod test
{
    use array_math::ArrayOps;
    use linspace::LinspaceArray;

    use crate::{plot, Bilinear, Butter, FilterGenPlane, FilterGenType, RealFreqZ, SfTrans, Zpk};

    #[test]
    fn test()
    {
        let h0 = Zpk::butter(6, [0.5], FilterGenType::LowPass, FilterGenPlane::S)
            .unwrap();

        let h = h0.sftrans(0.5, [0.2, 0.8], true)
            .unwrap()
            .bilinear(2.0)
            .unwrap();

        const N: usize = 1024;
        let (h_f, _): ([_; N], _) = h.real_freqz(());

        plot::plot_curves("H(e^jw)", "plots/h_z_sftrans.png", [&(0.0..1.0).linspace_array().zip(h_f.map(|h| h.norm()))]).unwrap();
    }
}