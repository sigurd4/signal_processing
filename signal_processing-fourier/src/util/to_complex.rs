use num_complex::{Complex, ComplexFloat};
use num_traits::{Float, FloatConst};

pub trait IntoComplex: ComplexFloat
{
    fn into_complex(self) -> Complex<Self::Real>;
}
impl<T> IntoComplex for T
where
    T: ComplexFloat
{
    default fn into_complex(self) -> Complex<T::Real>
    {
        Complex {
            re: self.re(),
            im: self.im()
        }
    }
}
impl<T> IntoComplex for T
where
    T: ComplexFloat + Into<Complex<T::Real>>
{
    fn into_complex(self) -> Complex<T::Real>
    {
        self.into()
    }
}

pub trait TruncateIm: ComplexFloat<Real: Into<Self>>
{
    fn truncate_im(c: Complex<Self::Real>) -> Self;
}
impl<T> TruncateIm for T
where
    T: ComplexFloat<Real: Into<Self>>
{
    default fn truncate_im(c: Complex<Self::Real>) -> Self
    {
        c.re.into()
    }
}
impl<T> TruncateIm for Complex<T>
where
    T: Float + FloatConst
{
    fn truncate_im(c: Complex<Self::Real>) -> Self
    {
        c
    }
}