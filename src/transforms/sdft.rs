use core::ops::{AddAssign, MulAssign, SubAssign};

use num::{complex::ComplexFloat, traits::FloatConst, Complex, NumCast, One};

use crate::OwnedList;

pub trait Sdft<T, X>: OwnedList<Complex<T::Real>>
where
    T: ComplexFloat,
    X: OwnedList<T>
{
    #[doc(alias = "sliding_dft")]
    fn sdft(&mut self, x: &mut X, buffer: &mut Vec<T>);
}

impl<T, X, Z> Sdft<T, X> for Z
where
    T: ComplexFloat,
    X: OwnedList<T>,
    Z: OwnedList<Complex<T::Real>>,
    Complex<T::Real>: AddAssign<T> + SubAssign<T> + MulAssign
{
    fn sdft(&mut self, xx: &mut X, buffer: &mut Vec<T>)
    {
        let n = self.length();
        buffer.truncate(n);
        let nf = <T::Real as NumCast>::from(n).unwrap();
        let w = Complex::cis(T::Real::TAU()/nf);
        let cone = Complex::one();

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
            for z in self.as_mut_slice()
                .iter_mut()
            {
                *z += *x;
                *z *= wn;
                wn *= w;
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
                for z in self.as_mut_slice()
                    .iter_mut()
                {
                    *z += *x;
                    *z -= *y;
                    *z *= wn;
                    wn *= w;
                }
                std::mem::swap(x, y);
            }
            buffer.rotate_right(j - i);
            i = j;
        }
    }
}

#[cfg(test)]
mod test
{
    use linspace::LinspaceArray;
    use num::{Complex, Zero};

    use crate::{plot, Chirp, ChirpCurve, Sdft};

    #[test]
    fn test()
    {
        const T: f64 = 1.0;
        const N: usize = 256;
        const M: usize = 16;
        const FS: f64 = N as f64/T;
        let f: [_; M] = (0.0..FS).linspace_array();
        let (x, t): ([_; N], _) = (0.0..T).chirp((), M as f64/T..FS/4.0, 0.0..1.0, ChirpCurve::Logarithmic, 0.0);

        let mut z = [Complex::zero(); M];
        let mut xb = vec![];

        let s: Vec<_> = x.into_iter()
            .map(|x| {
                z.sdft(&mut [x], &mut xb);
                z.clone()
            }).collect();
        plot::plot_parametric_curve_2d("|X(e^jw)|(t)", "plots/x_z_sdft.svg",
            core::array::from_fn::<_, {M/2 + 1}, _>(|i| i as f64),
            core::array::from_fn::<_, N, _>(|i| i as f64),
            |i, j| [f[i as usize], t[j as usize], s[j as usize][i as usize].norm().log10()*20.0]
        ).unwrap()
    }
}