use core::ops::MulAssign;

use num::{complex::ComplexFloat, Complex, One};
use option_trait::Maybe;

use crate::{MaybeList, MaybeLists, MaybeOwnedList, Sos, System, Tf, ToSos, ToTf, ToZpk, Zpk};

pub trait MaxPhase: System
{
    type OutputMax: System<Domain = Self::Domain>;
    type OutputAp: System<Domain: ComplexFloat<Real = <Self::Domain as ComplexFloat>::Real>>;

    fn maxphase(self) -> (Self::OutputMax, Self::OutputAp);
}

impl<T, Z, P, K> MaxPhase for Zpk<T, Z, P, K>
where
    T: ComplexFloat,
    Z: MaybeList<T, Owned: MaybeOwnedList<T>>,
    P: MaybeList<T, Owned: MaybeOwnedList<T>>,
    K: ComplexFloat<Real = T::Real> + MulAssign<T::Real>,
{
    type OutputMax = Zpk<T, Z::Owned, P::Owned, K>;
    type OutputAp = Zpk<T, Vec<T>, Vec<T>, K::Real>;

    fn maxphase(self) -> (Self::OutputMax, Self::OutputAp)
    {
        let Zpk {mut z, mut p, k} = self.to_owned();

        let mut ap = Zpk::new(vec![], vec![], K::Real::one());

        let one = T::Real::one();

        if let Some(z) = z.as_mut_slice_option()
        {
            for z in z.iter_mut()
            {
                if z.abs() < one
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
                if p.abs() < one
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

impl<T, B, A> MaxPhase for Tf<T, B, A>
where
    T: ComplexFloat + MulAssign<T::Real>,
    B: MaybeLists<T>,
    A: MaybeList<T>,
    Self: ToZpk<Complex<T::Real>, Vec<Complex<T::Real>>, Vec<Complex<T::Real>>, T, (), ()> + System<Domain = T>,
    Zpk<Complex<T::Real>, Vec<Complex<T::Real>>, Vec<Complex<T::Real>>, T>: ToTf<T, Vec<T>, Vec<T>, (), ()>,
    Zpk<Complex<T::Real>, Vec<Complex<T::Real>>, Vec<Complex<T::Real>>, T::Real>: ToTf<T, Vec<T>, Vec<T>, (), ()>
{
    type OutputMax = Tf<T, Vec<T>, Vec<T>>;
    type OutputAp = Tf<T, Vec<T>, Vec<T>>;

    fn maxphase(self) -> (Self::OutputMax, Self::OutputAp)
    {
        let (hmin, hap) = self.to_zpk((), ()).maxphase();
        (hmin.to_tf((), ()), hap.to_tf((), ()))
    }
}

impl<T, B, A, S> MaxPhase for Sos<T, B, A, S>
where
    T: ComplexFloat + MulAssign<T::Real>,
    B: Maybe<[T; 3]> + MaybeList<T>,
    A: Maybe<[T; 3]> + MaybeList<T>,
    S: MaybeList<Tf<T, B, A>>,
    Self: ToZpk<Complex<T::Real>, Vec<Complex<T::Real>>, Vec<Complex<T::Real>>, T, (), ()> + System<Domain = T>,
    Zpk<Complex<T::Real>, Vec<Complex<T::Real>>, Vec<Complex<T::Real>>, T>: ToSos<T, [T; 3], [T; 3], Vec<Tf<T, [T; 3], [T; 3]>>, (), ()>,
    Zpk<Complex<T::Real>, Vec<Complex<T::Real>>, Vec<Complex<T::Real>>, T::Real>: ToSos<T, [T; 3], [T; 3], Vec<Tf<T, [T; 3], [T; 3]>>, (), ()>
{
    type OutputMax = Sos<T, [T; 3], [T; 3], Vec<Tf<T, [T; 3], [T; 3]>>>;
    type OutputAp = Sos<T, [T; 3], [T; 3], Vec<Tf<T, [T; 3], [T; 3]>>>;

    fn maxphase(self) -> (Self::OutputMax, Self::OutputAp)
    {
        let (hmin, hap) = self.to_zpk((), ()).maxphase();
        (hmin.to_sos((), ()), hap.to_sos((), ()))
    }
}

#[cfg(test)]
mod test
{
    use crate::{tf, MaxPhase};

    #[test]
    fn test()
    {
        let h1 = tf!(f64[s] = (s + 3)/(s + 0.5));

        let (h2, h2ap) = h1.maxphase();
        let h3 = h2*h2ap;
        
        println!("b = {:?}", h3.b);
        println!("a = {:?}", h3.a);
    }
}