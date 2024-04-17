


use num::{complex::ComplexFloat, Complex};

use crate::{Container, Lists, Dft};

pub trait Dht<T>: Lists<T>
where
    T: ComplexFloat
{
    fn dht(self) -> Self::Mapped<T::Real>;
}

impl<T, L> Dht<T> for L
where
    T: ComplexFloat,
    L: Lists<T>,
    Self: Dft<T>,
    Self::Mapped<Complex<T::Real>>: Lists<Complex<T::Real>, Mapped<T::Real> = Self::Mapped<T::Real>>,
{
    fn dht(self) -> Self::Mapped<T::Real>
    {
        let y = self.dft();
        y.map_into_owned(|y| y.re - y.im)
    }
}

#[cfg(test)]
mod test
{
    use core::f64::consts::TAU;

    use array_math::ArrayOps;
    use linspace::LinspaceArray;

    use crate::{plot, Dht};

    #[test]
    fn test()
    {
        const N: usize = 1024;
        const T: f64 = 1.0;
        const F: f64 = 220.0;
        
        let x: [_; N] = ArrayOps::fill(|i| (TAU*F*i as f64/N as f64*T).sin());

        let xf = x.dht();

        let w = (0.0..TAU).linspace_array();

        plot::plot_curves("X(e^jw)", "plots/x_z_dht.png", [&w.zip(xf)])
            .unwrap()
    }
}