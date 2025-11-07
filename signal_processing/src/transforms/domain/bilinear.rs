use core::{any::Any, iter::Product, ops::{Add, AddAssign, Div, DivAssign, Mul, MulAssign}};

use array_math::{SliceMath, SliceOps};
use num::{complex::ComplexFloat, NumCast, One};
use option_trait::{Maybe, StaticMaybe};
use thiserror::Error;

use crate::{quantities::{ListOrSingle, MaybeList, MaybeOwnedList}, systems::{Sos, Tf, Zpk}, transforms::system::ToZpk, util::{self, Overlay}, System};

#[derive(Debug, Clone, Copy, PartialEq, Error)]
pub enum BilinearError
{
    #[error("Non causal transfer function, i.e. it contains one or more poles at infinity.")]
    NonCausal,
    #[error("The system must contain at least one pole.")]
    ZeroPoles
}

pub trait Bilinear: System
{
    type Output: Sized;

    fn bilinear(self, sampling_frequency: <Self::Set as ComplexFloat>::Real) -> Result<Self::Output, BilinearError>;
}

impl<'a, T, Z, P, K> Bilinear for Zpk<T, Z, P, K>
where
    T: ComplexFloat + Add<T::Real, Output = T> + Div<T::Real, Output = T> + Mul<T::Real, Output = T> + MulAssign + Product + 'static,
    K: ComplexFloat<Real = T::Real> + MulAssign<K::Real> + Mul<K::Real, Output = K> + Div<K::Real, Output = K> + 'static,
    Z: MaybeList<T>,
    P: MaybeList<T>,
    T::Real: Into<T>,
    Self: ToZpk<T, Vec<T>, Vec<T>, K, (), ()> + System<Set = K>
{
    type Output = Zpk<T, Vec<T>, Vec<T>, K>;

    fn bilinear(self, sampling_frequency: K::Real) -> Result<Self::Output, BilinearError>
    {
        let Zpk::<T, Vec<T>, Vec<T>, K> {z: sz, p: sp, k: sg} = self.to_zpk((), ());
    
        let one = K::Real::one();
        let two = one + one;
        
        let t = sampling_frequency.recip();

        let p = sp.len();
        let z = sz.len();
        if z > p
        {
            return Err(BilinearError::NonCausal)
        }
        if p == 0
        {
            return Err(BilinearError::ZeroPoles)
        }

        let mut zg = sg;
        let n = sz.iter()
            .map(|&sz| -sz + two/t)
            .product::<T>();
        let d = sp.iter()
            .map(|&sp| -sp + two/t)
            .product::<T>();
        let gmul = n/d;
        if let Some(zg) = <dyn Any>::downcast_mut::<T>(&mut zg as &mut dyn Any)
        {
            if gmul.is_infinite()
            {
                *zg = *zg*n/d;
            }
            else
            {
                *zg *= gmul;
            }
        }
        else if gmul.is_infinite()
        {
            zg = zg*n.re()/d.re();
        }
        else
        {
            zg *= gmul.re();
        }
        let zp: Vec<_> = sp.into_inner()
            .into_iter()
            .map(|sp| (sp*t + two)/(-sp*t + two))
            .collect();
        let mut zz: Vec<_> = sz.into_inner()
            .into_iter()
            .map(|sz| (sz*t + two)/(-sz*t + two))
            .collect();
        zz.resize(p, -T::one());
        Ok(Zpk::new(zz, zp, zg))
    }
}

impl<T, B, A, BA2> Bilinear for Tf<T, B, A>
where
    T: ComplexFloat + AddAssign + DivAssign + Mul<T::Real, Output = T>,
    B: MaybeList<T> + Overlay<T, A, Output = BA2>,
    A: MaybeList<T>,
    BA2: MaybeOwnedList<T, Some: ListOrSingle<T, Length: StaticMaybe<usize, Opposite: Sized>>>
{
    type Output = Tf<T, BA2, BA2>;

    fn bilinear(self, sampling_frequency: T::Real) -> Result<Self::Output, BilinearError>
    {
        let one = T::Real::one();
        let two = one + one;

        let fs = sampling_frequency;

        let b = self.b.into_inner()
            .into_vec_option()
            .unwrap_or_else(|| vec![T::one()]);
        let a = self.a.into_inner()
            .into_vec_option()
            .unwrap_or_else(|| vec![T::one()]);
        
        let nb = b.len();
        let na = a.len();
        let m = nb.max(na);

        let mut bb = BA2::maybe_from_len_fn(StaticMaybe::maybe_from_fn(|| m), |j| {
            let mut val = T::zero();

            for i in 0..nb
            {
                for k in 0..=i
                {
                    for l in 0..m - i
                    {
                        if k + l == j
                        {
                            val += b[nb - 1 - i]*(
                                util::bincoeff::<T::Real, _>(i, k)*util::bincoeff::<T::Real, _>(m - 1 - i, l)
                                *(two*fs).powi(i as i32)
                                *<T::Real as NumCast>::from(1 - (k % 2) as i8*2).unwrap()
                            )
                        }
                    }
                }
            }

            val
        });
        let mut aa = BA2::maybe_from_len_fn(StaticMaybe::maybe_from_fn(|| m), |j| {
            let mut val = T::zero();

            for i in 0..na
            {
                for k in 0..=i
                {
                    for l in 0..m - i
                    {
                        if k + l == j
                        {
                            val += a[na - 1 - i]*(
                                util::bincoeff::<T::Real, _>(i, k)*util::bincoeff::<T::Real, _>(m - 1 - i, l)
                                *(two*fs).powi(i as i32)
                                *<T::Real as NumCast>::from(1 - (k % 2) as i8*2).unwrap()
                            )
                        }
                    }
                }
            }

            val
        });

        if let Some(aa) = aa.as_mut_slice_option() && let Some(bb) = bb.as_mut_slice_option()
        {
            if let Some(a0) = aa.trim_zeros_front()
                .first()
                .copied()
            {
                bb.div_assign_all(a0);
                aa.div_assign_all(a0);
            }
        }

        Ok(Tf::new(bb, aa))
    }
}

impl<T, B, A, B2, A2, S> Bilinear for Sos<T, B, A, S>
where
    T: ComplexFloat,
    B: Maybe<[T; 3]> + MaybeOwnedList<T> + Clone,
    A: Maybe<[T; 3]> + MaybeOwnedList<T> + Clone,
    B2: Maybe<[T; 3]> + MaybeOwnedList<T>,
    A2: Maybe<[T; 3]> + MaybeOwnedList<T>,
    S: MaybeList<Tf<T, B, A>>,
    S::MaybeMapped<Tf<T, B2, A2>>: MaybeList<Tf<T, B2, A2>>,
    Tf<T, B, A>: Bilinear<Output = Tf<T, B2, A2>> + System<Set = T>
{
    type Output = Sos<T, B2, A2, S::MaybeMapped<Tf<T, B2, A2>>>;

    fn bilinear(self, sampling_frequency: T::Real) -> Result<Self::Output, BilinearError>
    {
        Ok(Sos::new(self.sos.into_inner()
            .maybe_try_map_into_owned(|sos| sos.bilinear(sampling_frequency))?
        ))
    }
}

#[cfg(test)]
mod test
{
    use array_math::ArrayOps;

    use crate::{analysis::RealFreqZ, gen::filter::{Butter, FilterGenPlane, FilterGenType}, plot, systems::{Tf, Zpk}, transforms::domain::Bilinear};

    #[test]
    fn test()
    {
        const M: usize = 3;
        let h = Tf::butter(M, [250.0], FilterGenType::LowPass, FilterGenPlane::S)
            .unwrap();
        let h2 = Zpk::butter(M, [250.0], FilterGenType::LowPass, FilterGenPlane::S)
            .unwrap();

        let h = h.bilinear(1000.0).unwrap();
        let h2 = h2.bilinear(1000.0).unwrap();

        const N: usize = 1024;
        let (hf, _w): (_, [_; N]) = h.real_freqz(());
        let (hf2, w): (_, [_; N]) = h2.real_freqz(());

        plot::plot_curves("H(e^jw)", "plots/h_z_bilinear.png", [&w.zip(hf.map(|h| h.norm())), &w.zip(hf2.map(|h| h.norm()))])
            .unwrap();
    }
}