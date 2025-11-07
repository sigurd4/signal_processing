use core::ops::MulAssign;

use num::{complex::ComplexFloat, Complex, One};
use option_trait::Maybe;

use crate::{quantities::{MaybeList, MaybeLists, MaybeOwnedList}, System, systems::{Sos, Tf, Zpk}, transforms::system::{ToSos, ToTf, ToZpk}};

pub trait MinPhase: System
{
    type OutputMin: System<Set = Self::Set>;
    type OutputAp: System<Set: ComplexFloat<Real = <Self::Set as ComplexFloat>::Real>>;

    fn minphase(self) -> (Self::OutputMin, Self::OutputAp);
}

impl<T, Z, P, K> MinPhase for Zpk<T, Z, P, K>
where
    T: ComplexFloat,
    Z: MaybeList<T, Owned: MaybeOwnedList<T>>,
    P: MaybeList<T, Owned: MaybeOwnedList<T>>,
    K: ComplexFloat<Real = T::Real> + MulAssign<T::Real>,
{
    type OutputMin = Zpk<T, Z::Owned, P::Owned, K>;
    type OutputAp = Zpk<T, Vec<T>, Vec<T>, K::Real>;

    fn minphase(self) -> (Self::OutputMin, Self::OutputAp)
    {
        let Zpk {mut z, mut p, k} = self.to_owned();

        let mut ap = Zpk::new(vec![], vec![], K::Real::one());

        let one = T::Real::one();

        if let Some(z) = z.as_mut_slice_option()
        {
            for z in z.iter_mut()
            {
                if z.abs() > one
                {
                    ap.z.push(*z);
                    *z = z.recip().conj();
                    ap.p.push(*z);
                }
            }
        }
        if let Some(p) = p.as_mut_slice_option()
        {
            for p in p.iter_mut()
            {
                if p.abs() > one
                {
                    ap.p.push(*p);
                    *p = p.recip().conj();
                    ap.z.push(*p);
                }
            }
        }

        (Zpk {z, p, k}, ap)
    }
}

impl<T, B, A> MinPhase for Tf<T, B, A>
where
    T: ComplexFloat + MulAssign<T::Real>,
    B: MaybeLists<T>,
    A: MaybeList<T>,
    Self: ToZpk<Complex<T::Real>, Vec<Complex<T::Real>>, Vec<Complex<T::Real>>, T, (), ()> + System<Set = T>,
    Zpk<Complex<T::Real>, Vec<Complex<T::Real>>, Vec<Complex<T::Real>>, T>: ToTf<T, Vec<T>, Vec<T>, (), ()>,
    Zpk<Complex<T::Real>, Vec<Complex<T::Real>>, Vec<Complex<T::Real>>, T::Real>: ToTf<T, Vec<T>, Vec<T>, (), ()>
{
    type OutputMin = Tf<T, Vec<T>, Vec<T>>;
    type OutputAp = Tf<T, Vec<T>, Vec<T>>;

    fn minphase(self) -> (Self::OutputMin, Self::OutputAp)
    {
        let (hmin, hap) = self.to_zpk((), ()).minphase();
        (hmin.to_tf((), ()), hap.to_tf((), ()))
    }
}

impl<T, B, A, S> MinPhase for Sos<T, B, A, S>
where
    T: ComplexFloat + MulAssign<T::Real>,
    B: Maybe<[T; 3]> + MaybeOwnedList<T>,
    A: Maybe<[T; 3]> + MaybeOwnedList<T>,
    S: MaybeList<Tf<T, B, A>>,
    Self: ToZpk<Complex<T::Real>, Vec<Complex<T::Real>>, Vec<Complex<T::Real>>, T, (), ()> + System<Set = T>,
    Zpk<Complex<T::Real>, Vec<Complex<T::Real>>, Vec<Complex<T::Real>>, T>: ToSos<T, [T; 3], [T; 3], Vec<Tf<T, [T; 3], [T; 3]>>, (), ()>,
    Zpk<Complex<T::Real>, Vec<Complex<T::Real>>, Vec<Complex<T::Real>>, T::Real>: ToSos<T, [T; 3], [T; 3], Vec<Tf<T, [T; 3], [T; 3]>>, (), ()>
{
    type OutputMin = Sos<T, [T; 3], [T; 3], Vec<Tf<T, [T; 3], [T; 3]>>>;
    type OutputAp = Sos<T, [T; 3], [T; 3], Vec<Tf<T, [T; 3], [T; 3]>>>;

    fn minphase(self) -> (Self::OutputMin, Self::OutputAp)
    {
        let (hmin, hap) = self.to_zpk((), ()).minphase();
        (hmin.to_sos((), ()), hap.to_sos((), ()))
    }
}

#[cfg(test)]
mod test
{
    use crate::{systems::tf, decompositions::MinPhase};

    #[test]
    fn test()
    {
        let h1 = tf!(f64[s] = (s + 3)/(s + 0.5));

        let (h2, h2ap) = h1.minphase();
        let h3 = h2*h2ap;
        
        println!("b = {:?}", h3.b);
        println!("a = {:?}", h3.a);
    }
}