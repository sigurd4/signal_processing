use core::ops::{AddAssign, MulAssign};

use array_math::ArrayMath;
use num::{complex::ComplexFloat, Complex};
use option_trait::Maybe;

use crate::{Lists, MaybeList, Polynomial, ProductSequence, Sos, System, Tf, ToZpk, Zpk};

pub trait ToSos<T, B, A, S, I, O>: System
where
    T: ComplexFloat,
    B: Maybe<[T; 3]> + MaybeList<T>,
    A: Maybe<[T; 3]> + MaybeList<T>,
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
    B1: Maybe<[T1; 3]> + MaybeList<T1>,
    B2: Maybe<[T2; 3]> + MaybeList<T2>,
    A1: Maybe<[T1; 3]> + MaybeList<T1>,
    A2: Maybe<[T2; 3]> + MaybeList<T2>,
    S1: MaybeList<Tf<T1, B1, A1>>,
    S2: MaybeList<Tf<T2, B2, A2>>,
    ProductSequence<Tf<T1, B1, A1>, S1>: Into<ProductSequence<Tf<T2, B2, A2>, S2>>
{
    fn to_sos(self, (): (), (): ()) -> Sos<T2, B2, A2, S2>
    {
        Sos {
            sos: self.sos.into()
        }
    }
}

impl<T1, T2, Z, P, K> ToSos<T2, [T2; 3], [T2; 3], Vec<Tf<T2, [T2; 3], [T2; 3]>>, (), ()> for Zpk<T1, Z, P, K>
where
    T1: ComplexFloat + Into<Complex<T1::Real>>,
    T2: ComplexFloat + Into<Complex<T2::Real>> + AddAssign + MulAssign + 'static,
    K: ComplexFloat<Real = T1::Real> + Into<T2>,
    Z: MaybeList<T1>,
    P: MaybeList<T1>,
    (): Maybe<T1::Real>,
    Complex<T1::Real>: Into<Complex<T2::Real>>,
    Complex<T2::Real>: AddAssign,
    T1::Real: Into<T2>,
    T2::Real: Into<T2>,
    Self: ToZpk<T1, Vec<T1>, Vec<T1>, K, (), ()>
{
    fn to_sos(self, (): (), (): ()) -> Sos<T2, [T2; 3], [T2; 3], Vec<Tf<T2, [T2; 3], [T2; 3]>>>
    {
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

        Sos {
            sos: ProductSequence::new(
                z.into_iter()
                    .zip(p)
                    .map(|(z, p)| Tf {
                        b: z,
                        a: p
                    }).collect()
            )
        }
    }
}

impl<T1, T2, B, A, O> ToSos<T2, [T2; 3], [T2; 3], Vec<Tf<T2, [T2; 3], [T2; 3]>>, (), O> for Tf<T1, B, A>
where
    T1: ComplexFloat,
    T2: ComplexFloat,
    B: Lists<T1>,
    A: MaybeList<T1>,
    O: Maybe<usize>,
    Self: ToZpk<Complex<T1::Real>, Vec<Complex<T1::Real>>, Vec<Complex<T1::Real>>, T1, (), O>,
    Zpk<Complex<T1::Real>, Vec<Complex<T1::Real>>, Vec<Complex<T1::Real>>, T1>: ToSos<T2, [T2; 3], [T2; 3], Vec<Tf<T2, [T2; 3], [T2; 3]>>, (), ()>
{
    fn to_sos(self, (): (), output: O) -> Sos<T2, [T2; 3], [T2; 3], Vec<Tf<T2, [T2; 3], [T2; 3]>>>
    {
        self.to_zpk((), output)
            .to_sos((), ())
    }
}