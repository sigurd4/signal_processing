use core::ops::{AddAssign, MulAssign};

use num::{complex::ComplexFloat, traits::FloatConst, Complex, Float, NumCast, One, Zero};
use array_math::SliceMath;

use crate::Lists;

pub trait Hilbert<T>: Lists<T>
where
    T: Float
{
    fn hilbert(&self) -> Self::Mapped<Complex<T>>;
}

impl<T, L> Hilbert<T> for L
where
    T: Float + FloatConst + Into<Complex<T>>,
    Complex<T>: AddAssign + MulAssign,
    L: Lists<T>
{
    fn hilbert(&self) -> Self::Mapped<Complex<T>>
    {
        let x: Vec<Vec<T>> = self.as_view_slices()
            .into_iter()
            .map(|x| x.to_vec())
            .collect();
        let zero = T::zero();
        let one = T::one();
        let two = one + one;
        let mut y = x.into_iter()
            .map(|x| {
                let mut y: Vec<Complex<_>> = x.into_iter()
                    .map(|x| x.into())
                    .collect();

                let n = y.len();
                if n > 2
                {
                    y.fft();
                    let mut y: Vec<_> = if n % 2 == 0
                    {
                        core::iter::once(y[0])
                            .chain(y[1..n/2].iter()
                                .map(|&y| y*two)
                            ).chain(core::iter::once(y[n/2]))
                            .chain(vec![zero.into(); n/2 - 1])
                            .collect()
                    }
                    else
                    {
                        core::iter::once(y[0])
                            .chain(y[1..(n + 1)/2].iter()
                                .map(|&y| y*two)
                            ).chain(vec![zero.into(); (n - 1)/2])
                            .collect()
                    };
                    y.ifft();
                    y.truncate(n);
                    y
                }
                else
                {
                    y
                }
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

    use crate::{plot, Hilbert};

    #[test]
    fn test()
    {
        const N: usize = 1024;
        const F: f64 = 220.0;
        const T: f64 = 4.0/F;
        
        let t = (0.0..T).linspace_array();
        let x: [_; N] = ArrayOps::fill(|i| (TAU*F*i as f64/N as f64*T).sin());

        let y = x.hilbert();

        plot::plot_curves("y(t)", "plots/y_t_hilbert.png", [&t.zip(y.map(|y| y.re)), &t.zip(y.map(|y| y.im))])
            .unwrap()
    }
}