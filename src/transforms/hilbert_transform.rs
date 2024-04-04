use core::ops::{AddAssign, MulAssign};

use num::{complex::ComplexFloat, traits::FloatConst, Complex, Float, NumCast, One, Zero};
use array_math::SliceMath;

use crate::{Lists, TruncateIm};

pub trait HilbertTransform<T>: Lists<T>
where
    T: ComplexFloat
{
    fn hilbert_transform(&self) -> Self::Mapped<T>;
}

impl<T, L> HilbertTransform<T> for L
where
    T: ComplexFloat + Into<Complex<T::Real>> + 'static,
    T::Real: Into<T> + Into<Complex<T::Real>>,
    Complex<T::Real>: AddAssign + MulAssign,
    L: Lists<T>
{
    fn hilbert_transform(&self) -> Self::Mapped<T>
    {
        let x: Vec<Vec<T>> = self.as_view_slices()
            .into_iter()
            .map(|x| x.to_vec())
            .collect();
        let zero = T::Real::zero();
        let one = T::Real::one();
        let mut y = x.into_iter()
            .map(|x| {
                let mut y: Vec<Complex<_>> = x.into_iter()
                    .map(|x| x.into())
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
                y.truncate(n);
                y.into_iter()
                    .map(|y| y.truncate_im::<T>())
                    .collect::<Vec<_>>()
            }).flatten();

        self.map_to_owned(|_| y.next().unwrap())
    }
}

#[cfg(test)]
mod test
{
    use core::f64::consts::TAU;

    use array_math::ArrayOps;
    use linspace::LinspaceArray;

    use crate::{plot, HilbertTransform};

    #[test]
    fn test()
    {
        const N: usize = 1024;
        const F: f64 = 220.0;
        const T: f64 = 4.0/F;
        
        let t = (0.0..T).linspace_array();
        let x: [_; N] = ArrayOps::fill(|i| (TAU*F*i as f64/N as f64*T).sin());

        let y = x.hilbert_transform();

        plot::plot_curves("x(t), y(t)", "plots/xy_t_hilbert_transform.png", [&t.zip(x), &t.zip(y)])
            .unwrap()
    }
}