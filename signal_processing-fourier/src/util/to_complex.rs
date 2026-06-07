use num_complex::{Complex, ComplexFloat};

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