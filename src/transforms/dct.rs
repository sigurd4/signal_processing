use core::ops::{AddAssign, MulAssign};

use num::{complex::ComplexFloat, traits::FloatConst, Complex, NumCast, One};
use array_math::SliceMath;

use crate::{Container, List, ListOrSingle, Lists, OwnedLists, DFT, TruncateIm};

pub trait DCT<'a, T>: Lists<T>
where
    T: ComplexFloat
{
    fn dct(&'a self) -> Self::Owned;
}

impl<'a, T, L> DCT<'a, T> for L
where
    T: ComplexFloat + Into<Complex<T::Real>> + MulAssign<T::Real> + 'static,
    L: Lists<T, IndexView<'a>: List<T>> + 'a,
    L::Owned: OwnedLists<T>,
    L::RowsMapped<Vec<Complex<T::Real>>>: OwnedLists<Complex<T::Real>>,
    L::Mapped<Complex<T::Real>>: OwnedLists<Complex<T::Real>>,
    Complex<T::Real>: AddAssign + MulAssign,
    T::Real: Into<T> + Into<Complex<T::Real>>,
    Self: DFT<T>,
{
    fn dct(&'a self) -> Self::Owned
    {
        let one = T::Real::one();
        let two = one + one;
        let half = two.recip();
        let quarter = half*half;

        let w = self.map_rows_to_owned(|x| {
            let x: &[T] = x.as_view_slice();
            let n = x.len();
            let nf = <T::Real as NumCast>::from(n).unwrap();
            
            let mut i = 0;
            x.map_into_owned(|_| {
                let w = if i == 0
                {
                    (quarter/nf).sqrt().into()
                }
                else
                {
                    Complex::cis(-T::Real::FRAC_PI_2()/nf*NumCast::from(i).unwrap())*(half/nf).sqrt()
                };
                i += 1;
                w
            })
        });
        let xreal = core::intrinsics::type_id::<T>() == core::intrinsics::type_id::<T::Real>();
        let mut y = self.to_owned();
        for (x, w) in y.as_mut_slice2()
            .into_iter()
            .zip(w.as_view_slices())
        {
            let n = x.len();
            let mut y: Vec<_> = if n % 2 == 0 && xreal
            {
                x[0..n].iter()
                    .step_by(2)
                    .map(|&x| x)
                    .chain(x[1..].iter()
                        .step_by(2)
                        .rev()
                        .map(|&x| x)
                    ).map(|x| x.into())
                    .collect()
            }
            else
            {
                x.iter()
                    .map(|&x| x)
                    .chain(x.iter()
                        .rev()
                        .map(|&x| x)
                    ).map(|x| x.into())
                    .collect()
            };
            y.fft();
            for ((x, y), w) in x.iter_mut()
                .zip(y.into_iter())
                .zip(w.into_iter())
            {
                *x = (y**w).truncate_im();
                if n % 2 == 0 && xreal
                {
                    *x *= two
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

    use array_math::{ArrayOps};
    use linspace::LinspaceArray;

    use crate::{plot, DCT};

    #[test]
    fn test()
    {
        const N: usize = 1024;
        const T: f64 = 1.0;
        const F: f64 = 220.0;
        
        let x: [_; N] = ArrayOps::fill(|i| (TAU*F*i as f64/N as f64*T).sin());

        let xf = x.dct();

        let w = (0.0..TAU).linspace_array();

        plot::plot_curves("X(e^jw)", "plots/x_z_dct.png", [&w.zip(xf)])
            .unwrap()
    }
}