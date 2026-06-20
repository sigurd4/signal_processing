use num_complex::{Complex, ComplexFloat};
use num_traits::{Float, FloatConst, NumCast};

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

pub trait TruncateIm: ComplexFloat
{
    fn from_real(r: Self::Real) -> Self;
    fn truncate_im(c: Complex<Self::Real>) -> Self;
}
impl<T> TruncateIm for T
where
    T: ComplexFloat
{
    default fn from_real(r: Self::Real) -> Self
    {
        NumCast::from(r).expect("ComplexFloat cannot contain real.")
    }
    default fn truncate_im(c: Complex<Self::Real>) -> Self
    {
        Self::from_real(c.re)
    }
}
impl<T> TruncateIm for T
where
    T: ComplexFloat<Real: Into<Self>>
{
    default fn from_real(r: Self::Real) -> Self
    {
        r.into()
    }
    default fn truncate_im(c: Complex<Self::Real>) -> Self
    {
        Self::from_real(c.re)
    }
}
impl<T> TruncateIm for Complex<T>
where
    T: Float + FloatConst
{
    fn from_real(r: Self::Real) -> Self
    {
        r.into()
    }
    fn truncate_im(c: Complex<Self::Real>) -> Self
    {
        c
    }
}