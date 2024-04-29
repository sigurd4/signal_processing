use core::ops::{AddAssign, DivAssign, MulAssign, SubAssign};

use array_math::{max_len, SliceMath};
use ndarray_linalg::Lapack;
use num::{complex::ComplexFloat, Complex, One, Zero};
use option_trait::Maybe;

use crate::{quantities::{MaybeList, MaybeLists, MaybeOwnedList, Polynomial}, Plane, systems::{Sos, Tf, Zpk}, System, util::TruncateIm};

pub trait Stabilize: System
{
    type Output: System<Set = Self::Set>;

    fn stabilize(self, plane: Plane) -> Self::Output;
}

impl<T, B, A> Stabilize for Tf<T, B, A>
where
    T: ComplexFloat<Real: Into<T>> + Lapack<Complex = Complex<<T as ComplexFloat>::Real>> + 'static,
    B: MaybeLists<T>,
    A: MaybeList<T, Owned: MaybeOwnedList<T>>,
    Complex<<T as ComplexFloat>::Real>: ComplexFloat<Real = <T as ComplexFloat>::Real> + AddAssign + SubAssign + MulAssign + DivAssign + DivAssign<<T as ComplexFloat>::Real> + From<T> + TruncateIm,
    [(); max_len(max_len(B::WIDTH, A::WIDTH), 1)]:
{
    type Output = Tf<T, B, A::Owned>;

    fn stabilize(self, plane: Plane) -> Self::Output
    {
        let zero = <T as ComplexFloat>::Real::zero();
        let one = <T as ComplexFloat>::Real::one();

        let mut tf = Tf::new(
            self.b.into_inner(),
            self.a.into_inner()
                .into_owned()
        );

        if let Some(a) = tf.a.as_mut_slice_option()
        {
            let mut p: Vec<_> = a.rpolynomial_roots();
            let mut changed = false;

            for p in p.iter_mut()
            {
                match plane
                {
                    Plane::Z => {
                        if (*p).abs() > one
                        {
                            *p = (*p).conj().recip();
                            changed = true
                        }
                    },
                    Plane::S => {
                        if p.re > zero
                        {
                            *p = -(*p).conj();
                            changed = true
                        }
                    }
                }
            }

            if changed
            {
                let a0 = a.trim_zeros_front()
                    .first()
                    .map(|&a0| a0)
                    .unwrap_or_else(T::one);
                a.fill(T::zero());
                let a_new = p.into_iter()
                    .map(|p| Polynomial::new([Complex::one(), -p]))
                    .product::<Polynomial<_, Vec<_>>>()
                    .into_inner();
                for (a, a_new) in a.iter_mut()
                    .rev()
                    .zip(a_new.into_iter()
                        .map(|a| a.truncate_im())
                        .rev()
                    )
                {
                    *a = a0*a_new
                }
            }
        }

        tf
    }
}

impl<T, B, A, S> Stabilize for Sos<T, B, A, S>
where
    T: ComplexFloat,
    B: Maybe<[T; 3]> + MaybeOwnedList<T> + Clone,
    A: Maybe<[T; 3]> + MaybeOwnedList<T> + Clone,
    S: MaybeList<Tf<T, B, A>, MaybeMapped<Tf<T, B, A>>: Into<S::Owned>>,
    S::Owned: MaybeList<Tf<T, B, A>>,
    Tf<T, B, A>: Stabilize<Output = Tf<T, B, A>>
{
    type Output = Sos<T, B, A, S::Owned>;

    fn stabilize(self, plane: Plane) -> Self::Output
    {
        Sos::new(self.sos.into_inner()
            .maybe_map_into_owned(|sos| sos.stabilize(plane))
            .into()
        )
    }
}

impl<T, Z, P, K> Stabilize for Zpk<T, Z, P, K>
where
    T: ComplexFloat,
    Z: MaybeList<T, Owned: MaybeOwnedList<T>>,
    P: MaybeList<T, Owned: MaybeOwnedList<T>>,
    K: ComplexFloat<Real = T::Real>
{
    type Output = Zpk<T, Z::Owned, P::Owned, K>;

    fn stabilize(self, plane: Plane) -> Self::Output
    {
        let zero = T::Real::zero();
        let one = T::Real::one();

        let mut zpk = self.to_owned();
        if let Some(p) = zpk.p.as_mut_slice_option()
        {
            for p in p.iter_mut()
            {
                match plane
                {
                    Plane::Z => {
                        if (*p).abs() > one
                        {
                            *p = (*p).conj().recip()
                        }
                    },
                    Plane::S => {
                        if p.re() > zero
                        {
                            *p = -(*p).conj()
                        }
                    }
                }
            }
        }
        zpk
    }
}

#[cfg(test)]
mod test
{
    use crate::{analysis::IsStable, Plane, transforms::filter::Stabilize, systems::Tf};

    #[test]
    fn test()
    {
        let h = Tf::new([1.0], [1.0, -1.5, -3.0]);
        println!("{}", h.is_stable((), Plane::Z));

        let h_stab = h.stabilize(Plane::Z);
        println!("{}", h_stab.is_stable((), Plane::Z));

        println!("{:?}", h_stab);
        
        let h = Tf::new([1.0], [2.0, -1.0, 1.0]);
        println!("{}", h.is_stable((), Plane::S));
        
        let h_stab = h.stabilize(Plane::S);
        println!("{}", h_stab.is_stable((), Plane::S));

        println!("{:?}", h_stab);
    }
}