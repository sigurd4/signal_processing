use core::ops::{AddAssign, MulAssign};

use array_math::ArrayMath;
use num::{complex::ComplexFloat, Complex};
use option_trait::{Maybe, MaybeOr, NotVoid, StaticMaybe};

use crate::{MaybeList, MaybeLists, MaybeOwnedList, Polynomial, ProductSequence, Sos, System, Tf, ToTf, ToZpk, Zpk};

pub trait ToSos<T, B, A, S, I, O>: System
where
    T: ComplexFloat,
    B: Maybe<[T; 3]> + MaybeOwnedList<T>,
    A: Maybe<[T; 3]> + MaybeOwnedList<T>,
    S: MaybeList<Tf<T, B, A>>,
    I: Maybe<usize>,
    O: Maybe<usize>
{
    fn to_sos(self, input: I, output: O) -> Sos<T, B, A, S>;
}

impl<T1, T2, B1, B2, A1, A2, S1, S2> ToSos<T2, B2, A2, S2, (), ()> for Sos<T1, B1, A1, S1>
where
    T1: ComplexFloat,
    T2: ComplexFloat,
    B1: Maybe<[T1; 3]> + MaybeOwnedList<T1> + Clone,
    B2: Maybe<[T2; 3]> + MaybeOwnedList<T2>,
    A1: Maybe<[T1; 3]> + MaybeOwnedList<T1> + Clone,
    A2: Maybe<[T2; 3]> + MaybeOwnedList<T2>,
    S1: MaybeList<Tf<T1, B1, A1>>,
    S2: MaybeList<Tf<T2, B2, A2>>,
    Tf<T1, B1, A1>: ToTf<T2, B2, A2, (), ()>,
    S1::MaybeMapped<Tf<T2, B2, A2>>: MaybeList<Tf<T2, B2, A2>>,
    ProductSequence<Tf<T2, B2, A2>, S1::MaybeMapped<Tf<T2, B2, A2>>>: Into<ProductSequence<Tf<T2, B2, A2>, S2>>
{
    fn to_sos(self, (): (), (): ()) -> Sos<T2, B2, A2, S2>
    {
        Sos {
            sos: ProductSequence::new(self.sos.into_inner().maybe_map_into_owned(|sos| sos.to_tf((), ()))).into()
        }
    }
}

impl<T1, T2, Z, P, K, B, A, S> ToSos<T2, B, A, S, (), ()> for Zpk<T1, Z, P, K>
where
    T1: ComplexFloat + Into<Complex<T1::Real>>,
    T2: ComplexFloat + Into<Complex<T2::Real>> + AddAssign + MulAssign + 'static,
    K: ComplexFloat<Real = T1::Real> + Into<T2>,
    Z: MaybeList<T1, MaybeSome: StaticMaybe<Z::Some, Maybe<[T2; 3]>: MaybeOr<[T2; 3], B, Output = B>>>,
    P: MaybeList<T1, MaybeSome: StaticMaybe<P::Some, Maybe<[T2; 3]>: MaybeOr<[T2; 3], A, Output = A>>>,
    B: StaticMaybe<[T2; 3]> + MaybeOwnedList<T2> + MaybeOr<[T2; 3], A, Output: StaticMaybe<[T2; 3], Maybe<Vec<Tf<T2, B, A>>>: MaybeOr<Vec<Tf<T2, B, A>>, S, Output = S>>>,
    A: StaticMaybe<[T2; 3]> + MaybeOwnedList<T2>,
    S: StaticMaybe<Vec<Tf<T2, B, A>>> + MaybeList<Tf<T2, B, A>>,
    Complex<T1::Real>: Into<Complex<T2::Real>>,
    Complex<T2::Real>: AddAssign,
    T1::Real: Into<T2> + NotVoid,
    T2::Real: Into<T2>,
    Self: ToZpk<T1, Vec<T1>, Vec<T1>, K, (), ()>
{
    fn to_sos(self, (): (), (): ()) -> Sos<T2, B, A, S>
    {
        Sos {
            sos: ProductSequence::new(StaticMaybe::maybe_from_fn(|| {
                let (zc, pc, zr, pr, k) = self.complex_real(()).unwrap();
        
                let mut p = vec![];
                let mut z = vec![];
        
                let zero = T2::zero();
                let one = T2::one();
                let cone: Complex<T2::Real> = one.into();
        
                for (m, c, r) in [(&mut p, pc, pr), (&mut z, zc, zr)]
                {
                    for [z1, z2] in c
                    {
                        m.push(Polynomial::new([cone, -z1.into()].convolve_direct(&[cone, -z2.into()])).truncate_im())
                    }
                    let r = r.array_chunks::<2>();
                    for &z in r.remainder()
                    {
                        m.push(Polynomial::new([zero, one, -z.into()]))
                    }
                    for &[z1, z2] in r
                    {
                        m.push(Polynomial::new([one, -z1.into()].convolve_direct(&[one, -z2.into()])))
                    }
                }
        
                if let Some(z) = z.first_mut()
                {
                    for z in z.iter_mut()
                    {
                        *z *= k.into()
                    }
                }
                else
                {
                    z.push(Polynomial::new([zero, zero, k.into()]))
                }
        
                let l = p.len().max(z.len());
                p.resize(l, Polynomial::new([zero, zero, one]));
                z.resize(l, Polynomial::new([zero, zero, one]));
        
                z.into_iter()
                    .zip(p)
                    .map(|(z, p)| Tf {
                        b: Polynomial::new(StaticMaybe::maybe_from_fn(|| z.into_inner())),
                        a: Polynomial::new(StaticMaybe::maybe_from_fn(|| p.into_inner()))
                    }).collect()
            }))
        }
    }
}

impl<T1, T2, B1, B2, A1, A2, Z, P, O> ToSos<T2, B2, A2, Vec<Tf<T2, B2, A2>>, (), O> for Tf<T1, B1, A1>
where
    T1: ComplexFloat,
    T2: ComplexFloat,
    B1: MaybeLists<T1, Some: NotVoid, MaybeSome: StaticMaybe<B1::Some, Maybe<[T2; 3]>: MaybeOr<[T2; 3], B2, Output = B2>> + StaticMaybe<B1::Some, Maybe<Vec<Complex<T1::Real>>> = Z>>,
    A1: MaybeList<T1, Some: NotVoid, MaybeSome: StaticMaybe<B1::Some, Maybe<[T2; 3]>: MaybeOr<[T2; 3], A2, Output = A2>> + StaticMaybe<A1::Some, Maybe<Vec<Complex<T1::Real>>> = P>>,
    B2: StaticMaybe<[T2; 3]> + MaybeOwnedList<T2>,
    A2: StaticMaybe<[T2; 3]> + MaybeOwnedList<T2>,
    O: Maybe<usize>,
    Z: MaybeList<Complex<T1::Real>>,
    P: MaybeList<Complex<T1::Real>>,
    Self: ToZpk<Complex<T1::Real>, Z, P, T1, (), O>,
    Zpk<Complex<T1::Real>, Z, P, T1>: ToSos<T2, B2, A2, Vec<Tf<T2, B2, A2>>, (), ()>
{
    fn to_sos(self, (): (), output: O) -> Sos<T2, B2, A2, Vec<Tf<T2, B2, A2>>>
    {
        self.to_zpk((), output)
            .to_sos((), ())
    }
}