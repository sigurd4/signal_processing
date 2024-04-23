use core::ops::{AddAssign, MulAssign, SubAssign};

use num::{complex::ComplexFloat, traits::float::FloatConst, Complex, NumCast, One, Zero};

use crate::{List, OwnedList};

pub trait SdftKw<T, X, W>: OwnedList<Complex<T::Real>>
where
    T: ComplexFloat,
    X: OwnedList<T>,
    W: List<Complex<T::Real>>
{
    fn sdft_kw(&mut self, x: &mut X, buffer: &mut Vec<T>, window_kernel: W);
}

impl<T, X, W, Z> SdftKw<T, X, W> for Z
where
    T: ComplexFloat,
    X: OwnedList<T>,
    Z: OwnedList<Complex<T::Real>>,
    W: List<Complex<T::Real>>,
    Complex<T::Real>: AddAssign<T> + SubAssign<T> + MulAssign
{
    fn sdft_kw(&mut self, xx: &mut X, buffer: &mut Vec<T>, window_kernel: W)
    {
        let n = self.length();
        buffer.truncate(n);
        let nf = <T::Real as NumCast>::from(n).unwrap();
        let w = Complex::cis(T::Real::TAU()/nf);
        let cone = Complex::one();

        let kn = window_kernel.length();

        let xn = xx.length();
        if xn == 0
        {
            return;
        }
        let bn = buffer.len();

        let xbn = (bn.min(n) + xn).saturating_sub(n);

        for x in xx.as_mut_slice()[..xn - xbn]
            .iter_mut()
        {
            let mut wn = cone;
            for (z, k) in self.as_mut_slice()[..kn]
                .iter_mut()
                .zip(window_kernel.as_view_slice()
                    .iter()
                )
            {
                *z += *x;
                *z *= wn*k;
                wn *= w;
            }
            if kn < n
            {
                for z in self.as_mut_slice()[kn..]
                    .iter_mut()
                {
                    *z = Complex::zero()
                }
            }
            let mut y = T::zero();
            core::mem::swap(x, &mut y);
            buffer.push(y);
        }
        let bnn = bn + xn - xbn;
        if bnn > 0
        {
            buffer.rotate_right((xn - xbn) % bnn);
            buffer[..xn - xbn].reverse();
        }
        let mut i = xn - xbn;
        while i < xn
        {
            let j = (i + n).min(xn);
            for (x, y) in xx.as_mut_slice()[i..j]
                .iter_mut()
                .zip(buffer.as_mut_slice()  
                    .iter_mut()
                    .rev()
                    .take(j - i)
                )
            {
                let mut wn = cone;
                for (z, k) in self.as_mut_slice()[..kn]
                    .iter_mut()
                    .zip(window_kernel.as_view_slice()
                        .iter()
                    )
                {
                    *z += *x;
                    *z -= *y;
                    *z *= wn*k;
                    wn *= w;
                }
                if kn < n
                {
                    for z in self.as_mut_slice()[kn..]
                        .iter_mut()
                    {
                        *z = Complex::zero()
                    }
                }
                std::mem::swap(x, y);
            }
            buffer.rotate_right(j - i);
            i = j;
        }
    }
}

