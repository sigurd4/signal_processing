use core::{iter::Sum, ops::{AddAssign, MulAssign, SubAssign}};

use num::{complex::ComplexFloat, traits::FloatConst, Complex, Float, NumCast, Zero};
use array_math::SliceMath;
use option_trait::Maybe;
use thiserror::Error;

use crate::{List, TruncateIm};

#[derive(Debug, Clone, Copy, PartialEq, Error)]
pub enum CCepsError
{
    #[error("Sequence has one or more zero-valued fourier coefficient")]
    ZeroInFourier
}

pub trait CCeps<T, C, N>: List<T::Real>
where
    T: ComplexFloat,
    N: Maybe<usize>
{
    fn cceps(&self, numtaps: N) -> Result<C, CCepsError>;
}

impl<T, C, L> CCeps<C, Vec<C>, usize> for L
where
    C: ComplexFloat<Real = T> + From<T> + 'static,
    T: Float + FloatConst + AddAssign + SubAssign + Sum + 'static,
    Complex<T>: ComplexFloat<Real = T> + MulAssign + AddAssign,
    L: List<T>
{
    fn cceps(&self, n: usize) -> Result<Vec<C>, CCepsError>
    {
        let x = self.as_view_slice();

        let mut f: Vec<Complex<T>> = x.iter()
            .map(|&x| From::from(x))
            .collect();

        f.resize(n, Zero::zero());

        let zero = T::zero();
        let half = n/2;
        if 2*half == n && f.dtft(T::PI()).re < zero {
            f.pop();
        }

        let l = f.len().next_power_of_two();
        f.resize(l, Complex::zero());
        f.fft();
        if f.iter().any(|f| f.is_zero())
        {
            return Err(CCepsError::ZeroInFourier)
        }

        let mut f_arg_prev = T::zero();
        let mut cep: Vec<_> = f.into_iter()
            .map(|f| {
                let mut f_ln = f.ln();
                while f_ln.im < f_arg_prev - T::PI()
                {
                    f_ln.im += T::TAU()
                }
                while f_ln.im > f_arg_prev + T::PI()
                {
                    f_ln.im -= T::TAU()
                }
                f_arg_prev = f_ln.im;
                f_ln
            })
            .collect();
        while cep.iter()
            .map(|c| c.im)
            .sum::<T>()/T::from(l).unwrap() > T::PI()
        {
            for c in cep.iter_mut()
            {
                c.im -= T::TAU()
            }
        }
        while cep.iter()
            .map(|c| c.im)
            .sum::<T>()/T::from(l).unwrap() < -T::PI()
        {
            for c in cep.iter_mut()
            {
                c.im += T::TAU()
            }
        }
        cep.ifft();

        let zero = C::zero();
        let mut i = 0;
        Ok((0..n).map(|_| {
            let j = i as f64/n as f64*l as f64;
            let p = j.fract();
            let q = NumCast::from(1.0 - p).unwrap();
            let p = NumCast::from(p).unwrap();
            let j0 = j.floor() as usize;
            let j1 = j.ceil() as usize;
            let c = cep.get(j0).map(|c| c.truncate_im()).unwrap_or(zero)*q + cep.get(j1).map(|c| c.truncate_im()).unwrap_or(zero)*p;

            i += 1;

            c
        }).collect())
    }
}

impl<T, C, L, const N: usize> CCeps<C, [C; N], ()> for L
where
    C: ComplexFloat<Real = T> + From<T> + 'static,
    T: Float + FloatConst + AddAssign + SubAssign + Sum + 'static,
    Complex<T>: ComplexFloat<Real = T> + MulAssign + AddAssign,
    L: List<T>
{
    fn cceps(&self, (): ()) -> Result<[C; N], CCepsError>
    {
        Ok(self.cceps(N)?.try_into().map_err(|_| ()).unwrap())
    }
}

#[cfg(test)]
mod test
{
    use core::f64::consts::TAU;

    use array_math::ArrayOps;
    use linspace::LinspaceArray;
    use num::Complex;

    use crate::{plot, CCeps};

    #[test]
    fn test()
    {
        const N: usize = 1024;
        const T: f64 = 1.27;
        let t: [_; N] = (0.0..T).linspace_array();

        let d = (N as f64*0.2/T) as usize;
        let s1 = t.map(|t| (TAU*45.0*t).sin());
        let s2 = s1.add_each(ArrayOps::fill(|i| if i >= d {0.5*s1[i - d]} else {0.0}));

        let c: [Complex<_>; _] = s2.cceps(()).unwrap();

        plot::plot_curves("c(t)", "plots/c_cceps.png", [&t.zip(c.map(|c| c.re))]).unwrap()
    }
}