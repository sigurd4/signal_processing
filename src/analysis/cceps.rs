use core::{iter::Sum, ops::{AddAssign, MulAssign, SubAssign}};

use num::{complex::ComplexFloat, traits::FloatConst, Complex, NumCast, Zero};
use array_math::SliceMath;
use option_trait::{Maybe, StaticMaybe};
use thiserror::Error;

use crate::{List, ListOrSingle, Lists, TruncateIm};

#[derive(Debug, Clone, Copy, PartialEq, Error)]
pub enum CCepsError
{
    #[error("Sequence has one or more zero-valued fourier coefficient.")]
    ZeroInFourier
}

pub trait CCeps<'a, T, C, N>: Lists<T>
where
    T: ComplexFloat,
    N: Maybe<usize>
{
    fn cceps(&'a self, numtaps: N) -> Result<Self::RowsMapped<C>, CCepsError>;
}

impl<'a, T, C, L> CCeps<'a, T, C, <C::Length as StaticMaybe<usize>>::Opposite> for L
where
    T: ComplexFloat + AddAssign + SubAssign + Into<Complex<T::Real>> + 'static,
    Complex<T::Real>: MulAssign + AddAssign,
    T::Real: AddAssign + SubAssign + Sum + Into<Complex<T::Real>> + Into<T>,
    L: Lists<T> + 'a,
    C: List<T>,
    Vec<T>: TryInto<C>,
    <C::Length as StaticMaybe<usize>>::Opposite: Sized,
    L::RowView<'a>: List<T>
{
    fn cceps(&'a self, n: <C::Length as StaticMaybe<usize>>::Opposite) -> Result<Self::RowsMapped<C>, CCepsError>
    {
        let n = n.into_option()
            .unwrap_or(C::LENGTH);

        self.try_map_rows_to_owned(|x| {
            let x = x.as_view_slice();

            let mut f: Vec<Complex<T::Real>> = x.iter()
                .map(|&x| x.into())
                .collect();

            f.resize(n, Zero::zero());

            let zero = T::Real::zero();
            let half = n/2;
            if 2*half == n && f.dtft(T::Real::TAU()*<T::Real as NumCast>::from(half + 1).unwrap()/<T::Real as NumCast>::from(n).unwrap()).re < zero {
                f.pop();
            }

            f.fft();
            if f.iter().any(|f| f.is_zero())
            {
                return Err(CCepsError::ZeroInFourier)
            }

            let mut f_arg_prev = T::Real::zero();
            f.rotate_right(n/2);
            for f in f.iter_mut()
            {
                *f = f.ln();
                while f.im < f_arg_prev - T::Real::PI()
                {
                    f.im += T::Real::TAU()
                }
                while f.im > f_arg_prev + T::Real::PI()
                {
                    f.im -= T::Real::TAU()
                }
                f_arg_prev = f.im;
            }
            while f.iter()
                .map(|c| c.im)
                .sum::<T::Real>()/<T::Real as NumCast>::from(n).unwrap() > T::Real::PI()
            {
                for c in f.iter_mut()
                {
                    c.im -= T::Real::TAU()
                }
            }
            while f.iter()
                .map(|c| c.im)
                .sum::<T::Real>()/<T::Real as NumCast>::from(n).unwrap() < -T::Real::PI()
            {
                for c in f.iter_mut()
                {
                    c.im += T::Real::TAU()
                }
            }
            f.rotate_left(n/2);
            f.ifft();
            f.rotate_right(n/2);

            let zero = T::zero();
            let mut y: Vec<_> = f.into_iter()
                .take(n)
                .map(|y| y.truncate_im())
                .collect();
            y.resize(n, zero);
            Ok(y.try_into().ok().unwrap())
        })
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
        const T: f64 = 1.27;
        const N: usize = (T/0.01) as usize;
        let t: [_; N] = (0.0..T).linspace_array();

        let d = (N as f64*0.2/T) as usize;
        let s1 = t.map(|t| (TAU*45.0*t).sin());
        let s2 = s1.add_each(ArrayOps::fill(|i| if i >= d {0.5*s1[i - d]} else {0.0}));

        let c: [_; _] = s2.map(|s| Complex::from(s)).cceps(()).unwrap();

        plot::plot_curves("x̂(t)", "plots/x_hat_cceps.png", [&t.zip(c.map(|c| c.re))]).unwrap()
    }
}