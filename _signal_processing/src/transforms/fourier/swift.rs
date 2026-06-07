use num::{complex::ComplexFloat, traits::MulAddAssign, Complex};

use crate::quantities::ListOrSingle;

pub trait Swift<T, X>: ComplexFloat<Real = T::Real>
where
    Complex<T::Real>: Into<Self>,
    T: ComplexFloat,
    X: ListOrSingle<T>,
{
    fn swift(&mut self, x: X, omega: T::Real, tau: T::Real);
}

impl<T, X> Swift<T, X> for Complex<T::Real>
where
    T: ComplexFloat,
    X: ListOrSingle<T>,
    Complex<T::Real>: MulAddAssign<Complex<T::Real>, T>
{
    fn swift(&mut self, x: X, omega: T::Real, tau: T::Real)
    {
        let w = Complex::from_polar((-tau.recip()).exp(), omega);
        for &x in x.as_view_slice()
        {
            self.mul_add_assign(w, x)
        }
    }
}