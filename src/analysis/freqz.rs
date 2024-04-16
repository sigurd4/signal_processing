use core::ops::{AddAssign, Mul, MulAssign};

use array_math::{ArrayOps, SliceMath};
use num::{complex::ComplexFloat, traits::FloatConst, Complex, NumCast, One, Zero};
use option_trait::Maybe;

use crate::{List, Lists, MaybeList, MaybeLists, Sos, System, Tf, ToSos, Zpk};

pub trait FreqZ<'a, H, W, N>: System
where
    H: Lists<Complex<<Self::Domain as ComplexFloat>::Real>>,
    W: List<<Self::Domain as ComplexFloat>::Real>,
    N: Maybe<usize>,
{
    fn freqz(&'a self, n: N, shift: bool) -> (H, W);
}

impl<'a, T, B, A, const N: usize> FreqZ<'a, B::RowsMapped<[Complex<T::Real>; N]>, [T::Real; N], ()> for Tf<T, B, A>
where
    T: ComplexFloat,
    B: MaybeLists<T>,
    A: MaybeList<T>,
    B::RowsMapped<[Complex<T::Real>; N]>: Lists<Complex<T::Real>>,
    Complex<T::Real>: AddAssign + MulAssign + Mul<T::Real, Output = Complex<T::Real>>,
    Self: 'a,
    &'a Self: Into<Tf<Complex<T::Real>, Vec<Vec<Complex<T::Real>>>, Vec<Complex<T::Real>>>>
{
    fn freqz(&'a self, (): (), shift: bool) -> (B::RowsMapped<[Complex<T::Real>; N]>, [T::Real; N])
    {
        let Tf { b, mut a }: Tf<Complex<T::Real>, Vec<Vec<Complex<T::Real>>>, Vec<Complex<T::Real>>> = self.into();

        let nf = <T::Real as NumCast>::from(N).unwrap();
        let w = ArrayOps::fill(|i| <T::Real as NumCast>::from(i).unwrap()/nf*T::Real::TAU() - if shift {T::Real::PI()} else {T::Real::zero()});

        let mut b = b.into_inner()
            .into_iter();
        let h = self.b.map_rows_to_owned(|_| {
            let mut b = b.next().unwrap();

            let m = N.max(b.len())
                .max(a.len())
                .next_power_of_two();
            b.resize(m, Zero::zero());
            a.resize(m, Zero::zero());
            b.fft();
            a.fft(); 
            
            ArrayOps::fill(|mut i| {
                if shift
                {
                    i += N - N/2
                }
                let i = i as f64*m as f64/N as f64;
                let p = i.fract();
                let q = <T::Real as NumCast>::from(1.0 - p).unwrap();
                let p = <T::Real as NumCast>::from(p).unwrap();
                let i0 = i.floor() as usize % m;
                let i1 = i.ceil() as usize % m;
                (b[i0]*q + b[i1]*p)/(a[i0]*q + a[i1]*p)
            })
        });
        (h, w)
    }
}

impl<'a, T, B, A> FreqZ<'a, B::RowsMapped<Vec<Complex<T::Real>>>, Vec<T::Real>, usize> for Tf<T, B, A>
where
    T: ComplexFloat,
    B: MaybeLists<T>,
    A: MaybeList<T>,
    B::RowsMapped<Vec<Complex<T::Real>>>: Lists<Complex<T::Real>>,
    Complex<T::Real>: AddAssign + MulAssign + Mul<T::Real, Output = Complex<T::Real>>,
    Self: 'a,
    &'a Self: Into<Tf<Complex<T::Real>, Vec<Vec<Complex<T::Real>>>, Vec<Complex<T::Real>>>>
{
    fn freqz(&'a self, n: usize, shift: bool) -> (B::RowsMapped<Vec<Complex<T::Real>>>, Vec<T::Real>)
    {
        let Tf { b, mut a }: Tf<Complex<T::Real>, Vec<Vec<Complex<T::Real>>>, Vec<Complex<T::Real>>> = self.into();

        let nf = <T::Real as NumCast>::from(n).unwrap();
        let w = (0..n).map(|i| <T::Real as NumCast>::from(i).unwrap()/nf*T::Real::TAU() - if shift {T::Real::PI()} else {T::Real::zero()})
            .collect();

        let mut b = b.into_inner()
            .into_iter();
        let h = self.b.map_rows_to_owned(|_| {
            let mut b = b.next().unwrap();
            let m = n.max(b.len())
                .max(a.len())
                .next_power_of_two();
            b.resize(m, Zero::zero());
            a.resize(m, Zero::zero());
            b.fft();
            a.fft();

            
            (0..n).map(|mut i| {
                    if shift
                    {
                        i += n - n/2
                    }
                    let i = i as f64*m as f64/n as f64;
                    let p = i.fract();
                    let q = <T::Real as NumCast>::from(1.0 - p).unwrap();
                    let p = <T::Real as NumCast>::from(p).unwrap();
                    let i0 = i.floor() as usize % m;
                    let i1 = i.ceil() as usize % m;
                    (b[i0]*q + b[i1]*p)/(a[i0]*q + a[i1]*p)
                }).collect()
        });
        (h, w)
    }
}

impl<'a, T, B, A, S, const N: usize> FreqZ<'a, [Complex<T::Real>; N], [T::Real; N], ()> for Sos<T, B, A, S>
where
    T: ComplexFloat,
    B: Maybe<[T; 3]> + MaybeList<T>,
    A: Maybe<[T; 3]> + MaybeList<T>,
    S: MaybeList<Tf<T, B, A>>,
    Self: 'a,
    &'a Self: Into<Sos<T, B, A, &'a [Tf<T, B, A>]>>,
    Tf<T, B, A>: FreqZ<'a, [Complex<T::Real>; N], [T::Real; N], ()> + System<Domain = T>
{
    fn freqz(&'a self, (): (), shift: bool) -> ([Complex<T::Real>; N], [T::Real; N])
    {
        let Sos { sos }: Sos<T, B, A, &'a [Tf<T, B, A>]> = self.into();
        let h = sos.iter()
            .map(|s| s.freqz((), shift).0)
            .reduce(|a, b| a.mul_each(b))
            .unwrap_or_else(|| [One::one(); N]);
        
        let nf = <T::Real as NumCast>::from(N).unwrap();
        let w = ArrayOps::fill(|i| <T::Real as NumCast>::from(i).unwrap()/nf*T::Real::TAU() - if shift {T::Real::PI()} else {T::Real::zero()});
        (h, w)
    }
}
impl<'a, T, B, A, S> FreqZ<'a, Vec<Complex<T::Real>>, Vec<T::Real>, usize> for Sos<T, B, A, S>
where
    T: ComplexFloat,
    B: Maybe<[T; 3]> + MaybeList<T>,
    A: Maybe<[T; 3]> + MaybeList<T>,
    S: MaybeList<Tf<T, B, A>>,
    Self: 'a,
    &'a Self: Into<Sos<T, B, A, &'a [Tf<T, B, A>]>>,
    Tf<T, B, A>: FreqZ<'a, Vec<Complex<T::Real>>, Vec<T::Real>, ()> + System<Domain = T>
{
    fn freqz(&'a self, n: usize, shift: bool) -> (Vec<Complex<T::Real>>, Vec<T::Real>)
    {
        let Sos { sos }: Sos<T, B, A, &'a [Tf<T, B, A>]> = self.into();
        let h = sos.iter()
            .map(|s| s.freqz((), shift).0)
            .reduce(|a, b| a.into_iter()
                .zip(b)
                .map(|(a, b)| a*b)
                .collect()
            ).unwrap_or_else(|| vec![One::one(); n]);
        
        let nf = <T::Real as NumCast>::from(n).unwrap();
        let w = (0..n).map(|i| <T::Real as NumCast>::from(i).unwrap()/nf*T::Real::TAU() - if shift {T::Real::PI()} else {T::Real::zero()}).collect();
        (h, w)
    }
}

impl<'a, T, Z, P, K, H, W, N> FreqZ<'a, H, W, N> for Zpk<T, Z, P, K>
where
    T: ComplexFloat,
    Z: MaybeList<T> + 'a,
    P: MaybeList<T> + 'a,
    K: ComplexFloat<Real = T::Real>,
    H: Lists<Complex<K::Real>>,
    W: List<K::Real>,
    N: Maybe<usize>,
    Z::View<'a>: MaybeList<T>,
    P::View<'a>: MaybeList<T>,
    Zpk<T, Z::View<'a>, P::View<'a>, K>: ToSos<K, [K; 3], [K; 3], Vec<Tf<K, [K; 3], [K; 3]>>, (), ()> + System<Domain = K>,
    Sos<K, [K; 3], [K; 3], Vec<Tf<K, [K; 3], [K; 3]>>>: for<'b> FreqZ<'b, H, W, N> + System<Domain = K>
{
    fn freqz(&'a self, n: N, shift: bool) -> (H, W)
    {
        self.as_view()
            .to_sos((), ())
            .freqz(n, shift)
    }
}