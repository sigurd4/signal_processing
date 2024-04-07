use core::{any::Any, iter::Product, ops::{Add, Div, Mul, MulAssign}};

use num::{complex::ComplexFloat, Complex, One};
use thiserror::Error;

use crate::{MaybeList, ProductSequence, System, Tf, ToTf, ToZpk, Zpk};

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

    fn bilinear(self, sampling_frequency: <Self::Domain as ComplexFloat>::Real) -> Result<Self::Output, BilinearError>;
}

impl<'a, T, Z, P, K> Bilinear for Zpk<T, Z, P, K>
where
    T: ComplexFloat + Add<T::Real, Output = T> + Div<T::Real, Output = T> + Mul<T::Real, Output = T> + MulAssign + Product + 'static,
    K: ComplexFloat<Real = T::Real> + MulAssign<K::Real> + Mul<K::Real, Output = K> + Div<K::Real, Output = K> + 'static,
    Z: MaybeList<T>,
    P: MaybeList<T>,
    T::Real: Into<T>,
    Self: ToZpk<T, Vec<T>, Vec<T>, K, (), ()> + System<Domain = K>
{
    type Output = Zpk<T, Vec<T>, Vec<T>, K>;

    fn bilinear(self, t: K::Real) -> Result<Self::Output, BilinearError>
    {
        let Zpk::<T, Vec<T>, Vec<T>, K> {z: sz, p: sp, k: sg} = self.to_zpk((), ());
    
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
    
        let one = K::Real::one();
        let two = one + one;

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
        Ok(Zpk {
            z: ProductSequence::new(zz),
            p: ProductSequence::new(zp),
            k: zg
        })
    }
}

impl<T, B, A> Bilinear for Tf<T, B, A>
where
    T: ComplexFloat,
    B: MaybeList<T>,
    A: MaybeList<T>,
    Self: ToZpk<Complex<T::Real>, Vec<Complex<T::Real>>, Vec<Complex<T::Real>>, T, (), ()> + System<Domain = T>,
    Zpk<Complex<T::Real>, Vec<Complex<T::Real>>, Vec<Complex<T::Real>>, T>: Bilinear<Output: ToTf<T, Vec<T>, Vec<T>, (), ()>> + System<Domain = T>,
{
    type Output = Tf<T, Vec<T>, Vec<T>>;

    fn bilinear(self, t: T::Real) -> Result<Self::Output, BilinearError>
    {
        Ok(self.to_zpk((), ()).bilinear(t)?.to_tf((), ()))
    }
}