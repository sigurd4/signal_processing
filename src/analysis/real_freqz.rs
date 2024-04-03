use core::{ops::{AddAssign, MulAssign}};

use array_math::{ArrayOps, SliceMath};
use num::{complex::ComplexFloat, traits::FloatConst, Complex, Float, NumCast, One, Zero};
use option_trait::Maybe;

use crate::{List, Lists, MaybeList, Sos, System, Tf, ToSos, Zpk};

pub trait RealFreqZ<'a, H, W, N>: System
where
    Self::Domain: Float,
    H: Lists<Complex<<Self::Domain as ComplexFloat>::Real>>,
    W: List<<Self::Domain as ComplexFloat>::Real>,
    N: Maybe<usize>,
{
    fn real_freqz(&'a self, n: N) -> (H, W);
}

impl<'a, T, B, A, const N: usize> RealFreqZ<'a, [Complex<T>; N], [T; N], ()> for Tf<T, B, A>
where
    T: Float + FloatConst,
    B: MaybeList<T>,
    A: MaybeList<T>,
    Complex<T>: AddAssign + MulAssign,
    Self: 'a,
    &'a Self: Into<Tf<T, Vec<T>, Vec<T>>>
{
    fn real_freqz(&'a self, (): ()) -> ([Complex<T>; N], [T; N])
    {
        if N == 0
        {
            return ([Zero::zero(); N], [T::zero(); N])
        }

        let Tf { mut b, mut a }: Tf<T, Vec<T>, Vec<T>> = self.into();

        let m = (N*2).saturating_sub(2)
            .max(b.len())
            .max(a.len())
            .next_power_of_two();
        let l = m/2 + 1;

        let mut bf = vec![Zero::zero(); l];
        let mut af = vec![Zero::zero(); l];

        b.resize(m, Zero::zero());
        a.resize(m, Zero::zero());
        b.real_fft(&mut bf);
        a.real_fft(&mut af);

        let nf = <T as NumCast>::from(N).unwrap();
        let w = ArrayOps::fill(|i| <T as NumCast>::from(i).unwrap()/nf*T::PI());
        let h = ArrayOps::fill(|i| {
            let i = i as f64*l as f64/N as f64;
            let p = i.fract();
            let q = <T as NumCast>::from(1.0 - p).unwrap();
            let p = <T as NumCast>::from(p).unwrap();
            let mut i0 = i.floor() as usize % m;
            let mut i1 = i.ceil() as usize % m;
            if i0 >= l
            {
                i0 = m - i0
            }
            if i1 >= l
            {
                i1 = m - i1
            }
            (bf[i0]*q + bf[i1]*p)/(af[i0]*q + af[i1]*p)
        });
        (h, w)
    }
}
impl<'a, T, B, A> RealFreqZ<'a, Vec<Complex<T>>, Vec<T>, usize> for Tf<T, B, A>
where
    T: Float + FloatConst,
    B: MaybeList<T>,
    A: MaybeList<T>,
    Complex<T>: AddAssign + MulAssign,
    Self: 'a,
    &'a Self: Into<Tf<T, Vec<T>, Vec<T>>>
{
    fn real_freqz(&'a self, n: usize) -> (Vec<Complex<T>>, Vec<T>)
    {
        let Tf { mut b, mut a }: Tf<T, Vec<T>, Vec<T>> = self.into();

        let m = (n*2).saturating_sub(2)
            .max(b.len())
            .max(a.len())
            .next_power_of_two();
        let l = m/2 + 1;

        let mut bf = vec![Zero::zero(); l];
        let mut af = vec![Zero::zero(); l];

        b.resize(m, Zero::zero());
        a.resize(m, Zero::zero());
        b.real_fft(&mut bf);
        a.real_fft(&mut af);

        let nf = <T as NumCast>::from(n).unwrap();
        let w = (0..n).map(|i| <T as NumCast>::from(i).unwrap()/nf*T::PI())
            .collect();
        let h = (0..n).map(|i| {
                let i = i as f64*l as f64/n as f64;
                let p = i.fract();
                let q = <T as NumCast>::from(1.0 - p).unwrap();
                let p = <T as NumCast>::from(p).unwrap();
                let mut i0 = i.floor() as usize % m;
                let mut i1 = i.ceil() as usize % m;
                if i0 >= l
                {
                    i0 = m - i0
                }
                if i1 >= l
                {
                    i1 = m - i1
                }
                (bf[i0]*q + bf[i1]*p)/(af[i0]*q + af[i1]*p)
            }).collect();
        (h, w)
    }
}

impl<'a, T, B, A, S, const N: usize> RealFreqZ<'a, [Complex<T>; N], [T; N], ()> for Sos<T, B, A, S>
where
    T: Float + FloatConst,
    B: Maybe<[T; 3]> + MaybeList<T>,
    A: Maybe<[T; 3]> + MaybeList<T>,
    S: MaybeList<Tf<T, B, A>>,
    Self: 'a,
    &'a Self: Into<Sos<T, B, A, &'a [Tf<T, B, A>]>>,
    Tf<T, B, A>: RealFreqZ<'a, [Complex<T>; N], [T; N], ()> + System<Domain = T>
{
    fn real_freqz(&'a self, (): ()) -> ([Complex<T>; N], [T; N])
    {
        let Sos { sos }: Sos<T, B, A, &'a [Tf<T, B, A>]> = self.into();
        let h = sos.iter()
            .map(|s| s.real_freqz(()).0)
            .reduce(|a, b| a.mul_each(b))
            .unwrap_or_else(|| [One::one(); N]);
        
        let nf = <T as NumCast>::from(N).unwrap();
        let w = ArrayOps::fill(|i| <T as NumCast>::from(i).unwrap()/nf*T::PI());
        (h, w)
    }
}
impl<'a, T, B, A, S> RealFreqZ<'a, Vec<Complex<T>>, Vec<T>, usize> for Sos<T, B, A, S>
where
    T: Float + FloatConst,
    B: Maybe<[T; 3]> + MaybeList<T>,
    A: Maybe<[T; 3]> + MaybeList<T>,
    S: MaybeList<Tf<T, B, A>>,
    Self: 'a,
    &'a Self: Into<Sos<T, B, A, &'a [Tf<T, B, A>]>>,
    Tf<T, B, A>: RealFreqZ<'a, Vec<Complex<T>>, Vec<T>, ()> + System<Domain = T>
{
    fn real_freqz(&'a self, n: usize) -> (Vec<Complex<T>>, Vec<T>)
    {
        let Sos { sos }: Sos<T, B, A, &'a [Tf<T, B, A>]> = self.into();
        let h = sos.iter()
            .map(|s| s.real_freqz(()).0)
            .reduce(|a, b| a.into_iter()
                .zip(b)
                .map(|(a, b)| a*b)
                .collect()
            ).unwrap_or_else(|| vec![One::one(); n]);
        
        let nf = <T as NumCast>::from(n).unwrap();
        let w = (0..n).map(|i| <T as NumCast>::from(i).unwrap()/nf*T::PI()).collect();
        (h, w)
    }
}

impl<'a, T, Z, P, K, H, W, N> RealFreqZ<'a, H, W, N> for Zpk<T, Z, P, K>
where
    T: ComplexFloat<Real = K>,
    Z: MaybeList<T>,
    P: MaybeList<T>,
    K: Float + FloatConst,
    H: Lists<Complex<K>>,
    W: List<K>,
    N: Maybe<usize>,
    Self: ToSos<'a, K, [K; 3], [K; 3], Vec<Tf<K, [K; 3], [K; 3]>>, (), ()> + System<Domain = K>,
    Sos<K, [K; 3], [K; 3], Vec<Tf<K, [K; 3], [K; 3]>>>: for<'b> RealFreqZ<'b, H, W, N> + System<Domain = K>
{
    fn real_freqz(&'a self, n: N) -> (H, W)
    {
        self.to_sos((), ())
            .real_freqz(n)
    }
}