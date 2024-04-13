use num::{complex::ComplexFloat, traits::FloatConst, Complex, Float};
use option_trait::Maybe;
use array_math::SliceMath;

use crate::{MaybeList, MaybeOwnedList, Normalize, Residue, Rpk, System, Tf};

pub trait ResidueZ<T, R, P, RP, K>: System
where
    T: ComplexFloat<Real = <Self::Domain as ComplexFloat>::Real>,
    R: ComplexFloat<Real = T::Real>,
    P: ComplexFloat<Real = T::Real>,
    RP: MaybeList<(R, P)>,
    K: MaybeList<T>
{
    fn residuez<TOL>(self, tol: TOL) -> Rpk<T, R, P, RP, K>
    where
        TOL: Maybe<T::Real>;
}

impl<T, B, B2, A, A2, R> ResidueZ<T, Complex<R>, Complex<R>, Vec<(Complex<R>, Complex<R>)>, Vec<T>> for Tf<T, B, A>
where
    T: ComplexFloat<Real = R>,
    B: MaybeList<T>,
    A: MaybeList<T>,
    B2: MaybeOwnedList<T>,
    A2: MaybeOwnedList<T>,
    R: Float + FloatConst,
    Self: Normalize<Output = Tf<T, B2, A2>> + System<Domain = T>,
    Tf<T, B2, A2>: Residue<T, Complex<R>, Complex<R>, Vec<(Complex<R>, Complex<R>)>, Vec<T>> + System<Domain = T>
{
    fn residuez<TOL>(self, tol: TOL) -> Rpk<T, Complex<R>, Complex<R>, Vec<(Complex<R>, Complex<R>)>, Vec<T>>
    where
        TOL: Maybe<R>
    {
        let mut tf = self.normalize();

        if let Some(b) = tf.b.as_mut_slice_option()
        {
            b.reverse();
            b.conj_assign_all();
        }
        if let Some(a) = tf.a.as_mut_slice_option()
        {
            a.reverse();
            a.conj_assign_all();
        }

        let mut rpk = tf.residue(tol);

        let mut p_prev = None;
        let mut m = 1;
        for (r, p) in rpk.rp.iter_mut()
        {
            if p_prev == Some(*p)
            {
                m += 1
            }
            else
            {
                m = 1;
                p_prev = Some(*p)
            }

            *p = p.recip();
            *r = *r*((-*p).powi(m));
        }
        rpk.k.reverse();
        rpk.k.conj_assign_all();

        rpk
    }
}

#[cfg(test)]
mod test
{
    use crate::{ResidueZ, Tf};

    #[test]
    fn test()
    {
        let h = Tf::new(
            [1.0, 2.0, 3.0],
            [4.0, 5.0, 6.0]
        );
        let rpk = h.residuez(());
        println!("{:?}", rpk)
    }
}