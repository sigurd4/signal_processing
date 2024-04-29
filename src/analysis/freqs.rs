use core::ops::{AddAssign, MulAssign};

use array_math::SliceMath;
use num::{complex::ComplexFloat, Complex, One};
use option_trait::Maybe;

use crate::{quantities::{Lists, MaybeList, MaybeOwnedList}, systems::{Sos, Tf, Zpk}, System, transforms::system::ToSos};

pub trait FreqS<'a, S, SS>: System
where
    S: ComplexFloat,
    SS: Lists<S> + 'a
{
    fn freqs(&'a self, s: SS) -> SS::Mapped<Complex<<Self::Domain as ComplexFloat>::Real>>;
}

impl<'a, T, S, SS, B, A> FreqS<'a, S, SS> for Tf<T, B, A>
where
    S: ComplexFloat<Real = T::Real> + 'a,
    SS: Lists<S> + 'a,
    T: ComplexFloat + Into<Complex<T::Real>>,
    Complex<T::Real>: MulAssign<S> + AddAssign,
    B: MaybeList<T>,
    A: MaybeList<T>,
    Self: 'a
{
    fn freqs(&'a self, s: SS) -> SS::Mapped<Complex<T::Real>>
    {
        s.map_into_owned(|s| self.b.as_view_slice_option()
                .map(|b: &[T]| b.iter()
                    .map(|b| (*b).into())
                    .collect::<Vec<_>>()
                    .rpolynomial(s)
                ).unwrap_or_else(One::one)
            /self.a.as_view_slice_option()
                .map(|a: &[T]| a.iter()
                    .map(|a| (*a).into())
                    .collect::<Vec<_>>()
                    .rpolynomial(s)
                ).unwrap_or_else(One::one)
        )
    }
}

impl<'a, T, B, A, S, SS, SSS> FreqS<'a, SS, SSS> for Sos<T, B, A, S>
where
    T: ComplexFloat,
    B: Maybe<[T; 3]> + MaybeOwnedList<T>,
    A: Maybe<[T; 3]> + MaybeOwnedList<T>,
    S: MaybeList<Tf<T, B, A>>,
    SS: ComplexFloat<Real = T::Real> + 'a,
    SSS: Lists<SS> + 'a,
    Tf<T, B, A>: FreqS<'a, SS, [SS; 1]> + System<Domain = T>
{
    fn freqs(&'a self, s: SSS) -> SSS::Mapped<Complex<T::Real>>
    {
        s.map_into_owned(|s| self.sos.as_view_slice_option()
            .map(|sos: &[Tf<T, B, A>]| sos.iter()
                .map(|tf| tf.freqs([s]))
                .map(|[s]| s)
                .reduce(|a, b| a*b)
                .unwrap_or_else(One::one)
            ).unwrap_or_else(One::one)
        )
    }
}

impl<'a, T, Z, P, K, S, SS> FreqS<'a, S, SS> for Zpk<T, Z, P, K>
where
    T: ComplexFloat,
    Z: MaybeList<T> + 'a,
    P: MaybeList<T> + 'a,
    K: ComplexFloat<Real = T::Real>,
    S: ComplexFloat<Real = T::Real>,
    SS: Lists<S> + 'a,
    Z::View<'a>: MaybeList<T>,
    P::View<'a>: MaybeList<T>,
    Zpk<T, Z::View<'a>, P::View<'a>, K>: ToSos<K, [K; 3], [K; 3], Vec<Tf<K, [K; 3], [K; 3]>>, (), ()> + 'a + System<Domain = K>,
    Sos<K, [K; 3], [K; 3], Vec<Tf<K, [K; 3], [K; 3]>>>: for<'b> FreqS<'b, S, SS> + System<Domain = K>
{
    fn freqs(&'a self, s: SS) -> SS::Mapped<Complex<<Self::Domain as ComplexFloat>::Real>>
    {
        self.as_view()
            .to_sos((), ())
            .freqs(s)
    }
}