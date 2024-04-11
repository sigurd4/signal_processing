use core::any::Any;

use num::complex::ComplexFloat;

pub trait TruncateIm: ComplexFloat
{
    fn truncate_im<T>(self) -> T
    where
        T: ComplexFloat<Real = Self::Real> + 'static,
        Self::Real: Into<T>;
}

impl<C> TruncateIm for C
where
    Self: ComplexFloat + 'static
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
            t = self.re().into()
        }
        t
    }
}