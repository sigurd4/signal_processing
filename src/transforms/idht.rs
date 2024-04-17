


use num::{complex::ComplexFloat, Complex};

use crate::{Container, Lists, Idft};

pub trait Idht<T>: Lists<T>
where
    T: ComplexFloat
{
    fn idht(self) -> Self::Mapped<T::Real>;
}

impl<T, L> Idht<T> for L
where
    T: ComplexFloat,
    L: Lists<T>,
    Self: Idft<T>,
    Self::Mapped<Complex<T::Real>>: Lists<Complex<T::Real>, Mapped<T::Real> = Self::Mapped<T::Real>>,
{
    fn idht(self) -> Self::Mapped<T::Real>
    {
        let y = self.idft();
        y.map_into_owned(|y| y.re + y.im)
    }
}

#[cfg(test)]
mod test
{
    use core::f64::consts::TAU;

    use array_math::ArrayOps;
    use linspace::LinspaceArray;

    use crate::{plot, Dht, Idht};

    #[test]
    fn test()
    {
        const N: usize = 1024;
        const T: f64 = 0.1;
        const F: f64 = 220.0;
        
        let t: [_; N] = (0.0..T).linspace_array();
        let x = t.map(|t| (TAU*F*t).sin());

        let xf = x.dht();
        let y = xf.idht();

        plot::plot_curves("x(t)", "plots/x_t_idht.png", [&t.zip(y.map(|y| y))])
            .unwrap()
    }
}