use core::{iter::Product, ops::{AddAssign, DerefMut, DivAssign, MulAssign, SubAssign}};

use num::{complex::ComplexFloat, Complex, One};
use array_math::SliceMath;
use option_trait::{Maybe, MaybeOr, StaticMaybe};

use crate::{MaybeList, MaybeOwnedList, MaybeLists, Normalize, ProductSequence, Sos, Ss, SsAMatrix, SsBMatrix, SsCMatrix, SsDMatrix, System, Tf, ToSos, ToTf, Zpk};

pub trait ToZpk<T, Z, P, K, I, O>: System
where
    T: ComplexFloat,
    K: ComplexFloat<Real = T::Real>,
    Z: MaybeList<T>,
    P: MaybeList<T>,
    I: Maybe<usize>,
    O: Maybe<usize>
{
    fn to_zpk(self, input: I, output: O) -> Zpk<T, Z, P, K>;
}

impl<T1, Z1, P1, K1, T2, Z2, P2, K2> ToZpk<T2, Z2, P2, K2, (), ()> for Zpk<T1, Z1, P1, K1>
where
    T1: ComplexFloat + Into<T2>,
    T2: ComplexFloat,
    K1: ComplexFloat<Real = T1::Real> + Into<K2>,
    K2: ComplexFloat<Real = T2::Real>,
    Z1: MaybeList<T1>,
    P1: MaybeList<T1>,
    Z2: MaybeList<T2>,
    P2: MaybeList<T2>,
    Z1::MaybeMapped<T2>: MaybeList<T2>,
    P1::MaybeMapped<T2>: MaybeList<T2>,
    ProductSequence<T2, Z1::MaybeMapped<T2>>: Into<ProductSequence<T2, Z2>>,
    ProductSequence<T2, P1::MaybeMapped<T2>>: Into<ProductSequence<T2, P2>>
{
    fn to_zpk(self, (): (), (): ()) -> Zpk<T2, Z2, P2, K2>
    {
        Zpk {
            z: ProductSequence::new(self.z.into_inner().maybe_map_into_owned(|z| z.into())).into(),
            p: ProductSequence::new(self.p.into_inner().maybe_map_into_owned(|p| p.into())).into(),
            k: self.k.into()
        }
    }
}

impl<T, K, B, B2, A, A2, Z, P, O> ToZpk<Complex<<K as ComplexFloat>::Real>, Z, P, K, (), O> for Tf<T, B, A>
where
    O: Maybe<usize>,
    T: ComplexFloat,
    K: ComplexFloat + DivAssign,
    B: MaybeLists<T, MaybeSome: StaticMaybe<B::Some, Maybe<Vec<Complex<<K as ComplexFloat>::Real>>>: MaybeOr<Vec<Complex<<K as ComplexFloat>::Real>>, Z, Output = Z>>>,
    A: MaybeList<T, MaybeSome: StaticMaybe<A::Some, Maybe<Vec<Complex<<K as ComplexFloat>::Real>>>: MaybeOr<Vec<Complex<<K as ComplexFloat>::Real>>, P, Output = P>>>,
    Z: MaybeList<Complex<<K as ComplexFloat>::Real>> + StaticMaybe<Vec<Complex<<K as ComplexFloat>::Real>>, Maybe<Vec<K>> = B2>,
    P: MaybeList<Complex<<K as ComplexFloat>::Real>> + StaticMaybe<Vec<Complex<<K as ComplexFloat>::Real>>, Maybe<Vec<K>> = A2>,
    Self: Normalize<Output: ToTf<K, B2, A2, (), O>>,
    B2: MaybeList<K> + Maybe<Vec<K>>,
    A2: MaybeList<K> + Maybe<Vec<K>>,
    Complex<<K as ComplexFloat>::Real>: From<K> + AddAssign + SubAssign + MulAssign + DivAssign + DivAssign<<K as ComplexFloat>::Real>,
    K: ComplexFloat + ndarray_linalg::Lapack<Complex = Complex<<K as ComplexFloat>::Real>>
{
    fn to_zpk(self, (): (), output: O) -> Zpk<Complex<<K as ComplexFloat>::Real>, Z, P, K>
    {
        let mut tf: Tf<K, B2, A2> = self.normalize().to_tf((), output);

        let mut b_op: Option<&mut Vec<K>> = tf.b.deref_mut().as_option_mut();
        let kb = if let Some(b) = &mut b_op
        {
            let k = b.first().copied().unwrap_or(K::zero());
            for b in b.iter_mut()
            {
                *b /= k;
            }
            k
        }
        else
        {
            One::one()
        };
        let mut a_op: Option<&mut Vec<K>> = tf.a.deref_mut().as_option_mut();
        let ka = if let Some(a) = &mut a_op
        {
            let k = a.first().copied().unwrap_or(K::zero());
            for a in a.iter_mut()
            {
                *a /= k;
            }
            k
        }
        else
        {
            One::one()
        };
        let k = kb/ka;
        let z = ProductSequence::new(StaticMaybe::maybe_from_fn(|| b_op.unwrap().rpolynomial_roots()));
        let p = ProductSequence::new(StaticMaybe::maybe_from_fn(|| a_op.unwrap().rpolynomial_roots()));
        Zpk {
            z,
            p,
            k
        }
    }
}

impl<'a, T, A, B, C, D, K> ToZpk<Complex<K::Real>, Vec<Complex<K::Real>>, Vec<Complex<K::Real>>, K, usize, usize> for Ss<T, A, B, C, D>
where
    T: ComplexFloat,
    K: ComplexFloat,
    A: SsAMatrix<T, B, C, D>,
    B: SsBMatrix<T, A, C, D>,
    C: SsCMatrix<T, A, B, D>,
    D: SsDMatrix<T, A, B, C>,
    Self: ToTf<K, Vec<K>, Vec<K>, usize, ()>,
    Tf<K, Vec<K>, Vec<K>>: ToZpk<Complex<K::Real>, Vec<Complex<K::Real>>, Vec<Complex<K::Real>>, K, (), usize>
{
    fn to_zpk(self, input: usize, output: usize) -> Zpk<Complex<K::Real>, Vec<Complex<K::Real>>, Vec<Complex<K::Real>>, K>
    {
        self.to_tf(input, ()).to_zpk((), output)
    }
}

impl<'a, T, B, A, S, K> ToZpk<Complex<K::Real>, Vec<Complex<K::Real>>, Vec<Complex<K::Real>>, K, (), ()> for Sos<T, B, A, S>
where
    T: ComplexFloat,
    K: ComplexFloat,
    B: Maybe<[T; 3]> + MaybeOwnedList<T>,
    A: Maybe<[T; 3]> + MaybeOwnedList<T>,
    S: MaybeList<Tf<T, B, A>>,
    Self: ToSos<T, B, A, Vec<Tf<T, B, A>>, (), ()>,
    Tf<T, B, A>: ToZpk<Complex<K::Real>, Vec<Complex<K::Real>>, Vec<Complex<K::Real>>, K, (), ()>,
    Zpk<Complex<K::Real>, Vec<Complex<K::Real>>, Vec<Complex<K::Real>>, K>: Product
{
    fn to_zpk(self, (): (), (): ()) -> Zpk<Complex<K::Real>, Vec<Complex<K::Real>>, Vec<Complex<K::Real>>, K>
    {
        let Sos::<_, _, _, Vec<_>> {sos} = self.to_sos((), ());

        sos.into_inner()
            .into_iter()
            .map(|sos| sos.to_zpk((), ()))
            .product()
    }
}