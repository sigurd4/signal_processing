use core::any::Any;

use num::{complex::ComplexFloat, Complex, Float};

pub trait TruncateIm: ComplexFloat
{
    fn truncate_im<T>(self) -> T
    where
        T: ComplexFloat<Real = Self::Real> + 'static,
        Self::Real: Into<T>;
}

impl<F> TruncateIm for Complex<F>
where
    F: Float + 'static,
    Self: ComplexFloat<Real = F>
{
    fn truncate_im<T>(self) -> T
    where
        T: ComplexFloat<Real = Self::Real> + 'static,
        Self::Real: Into<T>
    {
        let mut t = T::zero();
        if let Some(t) = <dyn Any>::downcast_mut::<Self>(&mut t as &mut dyn Any)
        {
            *t = self
        }
        else
        {
            t = self.re.into()
        }
        t
    }
}