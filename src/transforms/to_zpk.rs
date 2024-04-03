use core::{iter::Product, ops::{AddAssign, DivAssign, MulAssign, SubAssign}};

use num::{complex::ComplexFloat, Complex};
use array_math::SliceMath;
use option_trait::Maybe;

use crate::{List, Matrix, MaybeList, MaybeLists, Normalize, ProductSequence, Sos, Ss, System, Tf, ToTf, Zpk};

pub trait ToZpk<'a, T, Z, P, K, I, O>: System
where
    T: ComplexFloat,
    K: ComplexFloat<Real = T::Real>,
    Z: MaybeList<T>,
    P: MaybeList<T>,
    I: Maybe<usize>,
    O: Maybe<usize>
{
    fn to_zpk(&'a self, input: I, output: O) -> Zpk<T, Z, P, K>;
}

impl<'a, T1, Z1, P1, K1, T2, Z2, P2, K2> ToZpk<'a, T2, Z2, P2, K2, (), ()> for Zpk<T1, Z1, P1, K1>
where
    T1: ComplexFloat,
    T2: ComplexFloat,
    K1: ComplexFloat<Real = T1::Real>,
    K2: ComplexFloat<Real = T2::Real>,
    Z1: List<T1>,
    P1: List<T1>,
    Z2: List<T2>,
    P2: List<T2>,
    Self: 'a,
    &'a Self: Into<Zpk<T2, Z2, P2, K2>>
{
    fn to_zpk(&'a self, (): (), (): ()) -> Zpk<T2, Z2, P2, K2>
    {
        self.into()
    }
}

impl<'a, T, K, B, A> ToZpk<'a, Complex<<K as ComplexFloat>::Real>, Vec<Complex<<K as ComplexFloat>::Real>>, Vec<Complex<<K as ComplexFloat>::Real>>, K, (), usize> for Tf<T, B, A>
where
    T: ComplexFloat + 'static,
    K: ComplexFloat + DivAssign,
    B: MaybeLists<T>,
    A: MaybeList<T>,
    Self: ToTf<'a, K, Vec<K>, Vec<K>, (), usize>,
    Complex<<K as ComplexFloat>::Real>: From<K> + AddAssign + SubAssign + MulAssign + DivAssign + DivAssign<<K as ComplexFloat>::Real>,
    K: ComplexFloat + ndarray_linalg::Lapack<Complex = Complex<<K as ComplexFloat>::Real>>
{
    fn to_zpk(&'a self, (): (), output: usize) -> Zpk<Complex<<K as ComplexFloat>::Real>, Vec<Complex<<K as ComplexFloat>::Real>>, Vec<Complex<<K as ComplexFloat>::Real>>, K>
    {
        let mut tf: Tf<K, Vec<K>, Vec<K>> = self.to_tf((), output);
        tf.normalize();
        let k = tf.b.first().copied().unwrap_or(K::zero());
        for b in tf.b.iter_mut()
        {
            *b /= k;
        }
        let z = ProductSequence::new(tf.b.rpolynomial_roots());
        let p = ProductSequence::new(tf.a.rpolynomial_roots());
        Zpk {
            z,
            p,
            k
        }
    }
}
impl<'a, T, K, B, A> ToZpk<'a, Complex<<K as ComplexFloat>::Real>, Vec<Complex<<K as ComplexFloat>::Real>>, Vec<Complex<<K as ComplexFloat>::Real>>, K, (), ()> for Tf<T, B, A>
where
    T: ComplexFloat,
    K: ComplexFloat + DivAssign,
    B: MaybeList<T>,
    A: MaybeList<T>,
    Self: ToTf<'a, K, Vec<K>, Vec<K>, (), ()>,
    Complex<<K as ComplexFloat>::Real>: From<K> + AddAssign + SubAssign + MulAssign + DivAssign + DivAssign<<K as ComplexFloat>::Real>,
    K: ComplexFloat + ndarray_linalg::Lapack<Complex = Complex<<K as ComplexFloat>::Real>>
{
    fn to_zpk(&'a self, (): (), (): ()) -> Zpk<Complex<<K as ComplexFloat>::Real>, Vec<Complex<<K as ComplexFloat>::Real>>, Vec<Complex<<K as ComplexFloat>::Real>>, K>
    {
        let mut tf: Tf<K, Vec<K>, Vec<K>> = self.to_tf((), ());
        tf.normalize();
        let k = tf.b.first().copied().unwrap_or(K::zero());
        for b in tf.b.iter_mut()
        {
            *b /= k;
        }
        let z = ProductSequence::new(tf.b.rpolynomial_roots());
        let p = ProductSequence::new(tf.a.rpolynomial_roots());
        Zpk {
            z,
            p,
            k
        }
    }
}

impl<'a, T, A, B, C, D, K> ToZpk<'a, Complex<K::Real>, Vec<Complex<K::Real>>, Vec<Complex<K::Real>>, K, usize, usize> for Ss<T, A, B, C, D>
where
    T: ComplexFloat,
    K: ComplexFloat,
    A: Matrix<T>,
    B: Matrix<T>,
    C: Matrix<T>,
    D: Matrix<T>,
    Self: ToTf<'a, K, Vec<K>, Vec<K>, usize, ()>,
    Tf<K, Vec<K>, Vec<K>>: for<'b> ToZpk<'b, Complex<K::Real>, Vec<Complex<K::Real>>, Vec<Complex<K::Real>>, K, (), usize>
{
    fn to_zpk(&'a self, input: usize, output: usize) -> Zpk<Complex<K::Real>, Vec<Complex<K::Real>>, Vec<Complex<K::Real>>, K>
    {
        self.to_tf(input, ()).to_zpk((), output)
    }
}

impl<'a, T, B, A, S, K> ToZpk<'a, Complex<K::Real>, Vec<Complex<K::Real>>, Vec<Complex<K::Real>>, K, (), ()> for Sos<T, B, A, S>
where
    T: ComplexFloat,
    K: ComplexFloat,
    B: Maybe<[T; 3]> + MaybeList<T>,
    A: Maybe<[T; 3]> + MaybeList<T>,
    S: MaybeList<Tf<T, B, A>>,
    Tf<T, B, A>: ToZpk<'a, Complex<K::Real>, Vec<Complex<K::Real>>, Vec<Complex<K::Real>>, K, (), ()>,
    Zpk<Complex<K::Real>, Vec<Complex<K::Real>>, Vec<Complex<K::Real>>, K>: Product
{
    fn to_zpk(&'a self, (): (), (): ()) -> Zpk<Complex<K::Real>, Vec<Complex<K::Real>>, Vec<Complex<K::Real>>, K>
    {
        self.sos.as_view_slice_option()
            .unwrap_or_default()
            .iter()
            .map(|sos| sos.to_zpk((), ()))
            .product()
    }
}