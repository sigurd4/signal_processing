use core::{iter::Sum, ops::{AddAssign, MulAssign, SubAssign}};

use num::{complex::ComplexFloat, traits::float::FloatConst, Complex, NumCast, One, Zero};

use crate::{Matrix, OwnedList};

pub trait SdftKw<T, X, W>: OwnedList<Complex<T::Real>>
where
    T: ComplexFloat,
    X: OwnedList<T>,
    W: Matrix<Complex<T::Real>>
{
    fn sdft_kw(&mut self, x: &mut X, xbuffer: &mut Vec<T>, zbuffer: &mut Vec<Complex<T::Real>>, window_kernel: W);
}

impl<T, X, W, Z> SdftKw<T, X, W> for Z
where
    T: ComplexFloat,
    X: OwnedList<T>,
    Z: OwnedList<Complex<T::Real>>,
    W: Matrix<Complex<T::Real>>,
    Complex<T::Real>: AddAssign<T> + SubAssign<T> + MulAssign + Sum
{
    fn sdft_kw(&mut self, xx: &mut X, xbuffer: &mut Vec<T>, zbuffer: &mut Vec<Complex<T::Real>>, window_kernel: W)
    {
        let n = self.length();
        xbuffer.truncate(n);
        zbuffer.resize(n, Complex::zero());
        let nf = <T::Real as NumCast>::from(n).unwrap();
        let w = Complex::cis(T::Real::TAU()/nf);
        let cone = Complex::one();

        let k = window_kernel.as_view_slices();

        let xn = xx.length();
        if xn == 0
        {
            return;
        }
        let bn = xbuffer.len();

        let xbn = (bn.min(n) + xn).saturating_sub(n);

        for x in xx.as_mut_slice()[..xn - xbn]
            .iter_mut()
        {
            let mut wn = cone;
            for z in self.as_mut_slice()
                .iter_mut()
            {
                *z += *x;
                *z *= wn;
                wn *= w;
            }
            for (zb, k) in zbuffer.as_mut_slice()
                .iter_mut()
                .zip(k.iter())
            {
                *zb = self.as_mut_slice()
                    .iter()
                    .zip(k.iter())
                    .map(|(&z, &k)| z*k)
                    .sum();
            }
            for (z, zb) in self.as_mut_slice()
                .iter_mut()
                .zip(zbuffer.as_mut_slice()
                    .iter()
                )
            {
                *z = *zb;
            }

            let mut y = T::zero();
            core::mem::swap(x, &mut y);
            xbuffer.push(y);
        }
        let bnn = bn + xn - xbn;
        if bnn > 0
        {
            xbuffer.rotate_right((xn - xbn) % bnn);
            xbuffer[..xn - xbn].reverse();
        }
        let mut i = xn - xbn;
        while i < xn
        {
            let j = (i + n).min(xn);
            for (x, y) in xx.as_mut_slice()[i..j]
                .iter_mut()
                .zip(xbuffer.as_mut_slice()  
                    .iter_mut()
                    .rev()
                    .take(j - i)
                )
            {
                let mut wn = cone;
                for z in self.as_mut_slice()
                    .iter_mut()
                {
                    *z += *x;
                    *z -= *y;
                    *z *= wn;
                    wn *= w;
                }
                for (zb, k) in zbuffer.as_mut_slice()
                    .iter_mut()
                    .zip(k.iter())
                {
                    *zb = self.as_mut_slice()
                        .iter()
                        .zip(k.iter())
                        .map(|(&z, &k)| z*k)
                        .sum();
                }
                for (z, zb) in self.as_mut_slice()
                    .iter_mut()
                    .zip(zbuffer.as_mut_slice()
                        .iter()
                    )
                {
                    *z = *zb;
                }
                std::mem::swap(x, y);
            }
            xbuffer.rotate_right(j - i);
            i = j;
        }
    }
}

