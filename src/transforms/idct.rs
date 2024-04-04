use core::{cell::LazyCell, ops::{AddAssign, Mul, MulAssign}};

use num::{complex::ComplexFloat, traits::FloatConst, Complex, NumCast, One, Zero};
use array_math::SliceMath;

use crate::{Container, List, ListOrSingle, Lists, OwnedLists, DFT, TruncateIm};

pub trait IDCT<'a, T>: Lists<T>
where
    T: ComplexFloat
{
    fn idct(&'a self) -> Self::Owned;
}

impl<'a, T, L> IDCT<'a, T> for L
where
    T: ComplexFloat + Into<Complex<T::Real>> + Mul<T::Real, Output = T> + 'static,
    L: Lists<T, IndexView<'a>: List<T>> + 'a,
    L::Owned: OwnedLists<T>,
    L::RowsMapped<Vec<Complex<T::Real>>>: OwnedLists<Complex<T::Real>>,
    L::Mapped<Complex<T::Real>>: OwnedLists<Complex<T::Real>>,
    Complex<T::Real>: AddAssign + MulAssign,
    T::Real: Into<T> + Into<Complex<T::Real>>,
    Self: DFT<T>,
{
    fn idct(&'a self) -> Self::Owned
    {
        let zero = T::Real::zero();
        let one = T::Real::one();
        let two = one + one;
        let four = two + two;
        let half = two.recip();
        let quarter = four.recip();

        let xreal = core::intrinsics::type_id::<T>() == core::intrinsics::type_id::<T::Real>();
        let mut y = self.to_owned();
        for y in y.as_mut_slice2()
            .into_iter()
        {
            let n = y.len();
            let nf = <T::Real as NumCast>::from(n).unwrap();
            let mut x: Vec<_> = y.iter()
                .map(|&x| x.into())
                .collect();
            if n % 2 == 0 && xreal
            {
                for (i, x) in x.iter_mut()
                    .enumerate()
                {
                    let w = if i == 0
                    {
                        (quarter*nf).sqrt().into()
                    }
                    else
                    {
                        Complex::cis(T::Real::FRAC_PI_2()/nf*NumCast::from(i).unwrap())*(half*nf).sqrt()
                    };
                    *x *= w;
                }
                x.ifft();

                for (i, x) in x.into_iter()
                    .enumerate()
                {
                    let j = if i < n/2 {i*2} else {(n - i)*2 - 1};
                    y[j] = x.truncate_im::<T>()*two
                }
            }
            else
            {
                for (i, x) in x.iter_mut()
                    .enumerate()
                {
                    let w = if i == 0
                    {
                        (four*nf).sqrt().into()
                    }
                    else
                    {
                        Complex::cis(T::Real::FRAC_PI_2()/nf*NumCast::from(i).unwrap())*(two*nf).sqrt()
                    };
                    *x *= w;
                }

                let mut x2 = x[1..].iter()
                    .enumerate()
                    .map(|(i, &x)| {
                        let w = Complex::cis(-T::Real::PI()/nf*NumCast::from(i + 1).unwrap());
                        x*w
                    }).rev()
                    .collect();
                x.push(zero.into());
                x.append(&mut x2);
                x.ifft();

                for (x, y) in x.into_iter()
                    .zip(y.iter_mut())
                {
                    *y = x.truncate_im()
                }
            }
        }
        y
    }
}

#[cfg(test)]
mod test
{
    use core::f64::consts::TAU;

    use array_math::ArrayOps;
    use linspace::LinspaceArray;

    use crate::{plot, DCT, IDCT};

    #[test]
    fn test()
    {
        const N: usize = 1024;
        const F: f64 = 220.0;
        const T: f64 = 4.0/F;
        
        let x: [_; N] = ArrayOps::fill(|i| (TAU*F*i as f64/N as f64*T).sin());

        let y = x.dct().idct();

        let t = (0.0..T).linspace_array();

        plot::plot_curves("x(t)", "plots/x_t_idct.png", [&t.zip(x), &t.zip(y)])
            .unwrap()
    }
}