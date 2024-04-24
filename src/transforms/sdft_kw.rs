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
        let kn = k.len();
        if kn < n
        {
            zbuffer[kn..].fill(Complex::zero())
        }

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

#[cfg(test)]
mod test
{
    use linspace::LinspaceArray;
    use num::{Complex, Zero};

    use crate::{plot, window::{Hann, Sine, WindowGen, WindowRange}, Chirp, ChirpCurve, MaybeContainer, Sdft, SdftKw, ToKw};

    #[test]
    fn test()
    {
        const T: f64 = 1.0;
        const N: usize = 256;
        const M: usize = 10;
        const FS: f64 = N as f64/T;
        let f: [_; M] = (0.0..FS).linspace_array();
        let (x, t): ([_; N], _) = (0.0..T).chirp((), M as f64/T..FS/4.0, 0.0..1.0, ChirpCurve::Logarithmic, 0.0);

        const W: usize = 128;
        let w: [f64; W] = Sine.window_gen((), WindowRange::Symmetric);
        let kw: [_; M] = w.to_kw(());
        //let kw_avg = kw.norm();

        //println!("{:?}", kw_avg);

        let mut z = [Complex::zero(); M];
        let mut xb = vec![];
        let mut zb: Vec<Complex<f64>> = vec![];

        let s: Vec<_> = x.into_iter()
            .map(|x| {
                z.sdft_kw(&mut [x], &mut xb, &mut zb, &kw);
                z.clone()
            }).collect();
        plot::plot_parametric_curve_2d("|X(e^jw)|(t)", "plots/x_z_sdft_kw.svg",
            core::array::from_fn::<_, {M/2 + 1}, _>(|i| i as f64),
            core::array::from_fn::<_, N, _>(|i| i as f64),
            |i, j| [f[i as usize], t[j as usize], s[j as usize][i as usize].norm().log10()*20.0]
        ).unwrap()
    }
}