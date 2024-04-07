use core::ops::{AddAssign, MulAssign};

use num::{complex::ComplexFloat, Complex, One, Zero};
use array_math::SliceMath;

use crate::{Lists, OwnedLists, TruncateIm};

pub trait Hilbert<T>: Lists<T>
where
    T: ComplexFloat
{
    fn hilbert(self) -> Self::Owned;
}

impl<T, L> Hilbert<T> for L
where
    T: ComplexFloat + Into<Complex<T::Real>> + 'static,
    T::Real: Into<T> + Into<Complex<T::Real>>,
    Complex<T::Real>: AddAssign + MulAssign,
    L: Lists<T>,
    L::Owned: OwnedLists<T>
{
    fn hilbert(self) -> Self::Owned
    {
        let zero = T::Real::zero();
        let one = T::Real::one();
        
        let mut x = self.to_owned();
        for x in x.as_mut_slice2()
        {
            let mut y: Vec<Complex<_>> = x.iter()
                .map(|&x| x.into())
                .collect();
            y.fft();

            let n = y.len();
            let nhalf = n/2;

            for (i, y) in y.iter_mut()
                .enumerate()
            {
                if i == 0
                {
                    *y = zero.into()
                }
                else
                {
                    *y *= Complex::new(zero, if i > nhalf {one} else {-one})
                }
            }
            y.ifft();
            
            for (x, y) in x.iter_mut()
                .zip(y.into_iter())
            {
                *x = y.truncate_im()
            }
        }

        x
    }
}

#[cfg(test)]
mod test
{
    use core::f64::consts::TAU;

    use array_math::ArrayOps;
    use linspace::LinspaceArray;

    use crate::{plot, Hilbert, MaybeContainer};

    #[test]
    fn test()
    {
        const N: usize = 1024;
        const F: f64 = 220.0;
        const T: f64 = 4.0/F;
        
        let t = (0.0..T).linspace_array();
        let x: [_; N] = ArrayOps::fill(|i| (TAU*F*i as f64/N as f64*T).sin());

        let y = x.as_view().hilbert();

        plot::plot_curves("x(t), y(t)", "plots/xy_t_hilbert.png", [&t.zip(x), &t.zip(y)])
            .unwrap()
    }
}