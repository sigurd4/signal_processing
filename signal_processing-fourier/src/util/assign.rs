use core::ops::{Add, AddAssign, Div, DivAssign, Mul, MulAssign};

use num_complex::ComplexFloat;

use crate::util::TruncateIm;

pub trait RealMul: ComplexFloat
{
    fn _real_mul(self, real: Self::Real) -> Self;
}
impl<C> RealMul for C
where
    C: ComplexFloat
{
    default fn _real_mul(self, real: Self::Real) -> Self
    {
        self*TruncateIm::from_real(real)
    }
}
impl<C> RealMul for C
where
    C: ComplexFloat + Mul<C::Real, Output = Self>
{
    fn _real_mul(self, real: Self::Real) -> Self
    {
        self*real
    }
}

pub trait RealDiv: ComplexFloat
{
    fn _real_div(self, real: Self::Real) -> Self;
}
impl<C> RealDiv for C
where
    C: ComplexFloat
{
    default fn _real_div(self, real: Self::Real) -> Self
    {
        self/TruncateIm::from_real(real)
    }
}
impl<C> RealDiv for C
where
    C: ComplexFloat + Div<C::Real, Output = Self>
{
    fn _real_div(self, real: Self::Real) -> Self
    {
        self/real
    }
}

pub const trait AddAssignSpec: ~const Add<Output = Self> + Copy
{
    fn _add_assign(&mut self, rhs: Self);
}
const impl<T> AddAssignSpec for T
where
    T: ~const Add<Output = Self> + Copy
{
    default fn _add_assign(&mut self, rhs: Self)
    {
        *self = *self + rhs
    }
}
const impl<T> AddAssignSpec for T
where
    T: ~const Add<Output = Self> + Copy + ~const AddAssign
{
    fn _add_assign(&mut self, rhs: Self)
    {
        *self += rhs
    }
}

pub const trait MulAssignSpec: ~const Mul<Output = Self> + Copy
{
    fn _mul_assign(&mut self, rhs: Self);
}
const impl<T> MulAssignSpec for T
where
    T: ~const Mul<Output = Self> + Copy
{
    default fn _mul_assign(&mut self, rhs: Self)
    {
        *self = *self * rhs
    }
}
const impl<T> MulAssignSpec for T
where
    T: ~const Mul<Output = Self> + Copy + ~const MulAssign
{
    fn _mul_assign(&mut self, rhs: Self)
    {
        *self *= rhs
    }
}

pub const trait DivAssignSpec<Rhs = Self>: ~const Div<Rhs, Output = Self> + Copy
{
    fn _div_assign(&mut self, rhs: Rhs);
}
const impl<T, Rhs> DivAssignSpec<Rhs> for T
where
    T: ~const Div<Rhs, Output = Self> + Copy
{
    default fn _div_assign(&mut self, rhs: Rhs)
    {
        *self = *self / rhs
    }
}
const impl<T, Rhs> DivAssignSpec<Rhs> for T
where
    T: ~const Div<Rhs, Output = Self> + Copy + ~const DivAssign<Rhs>
{
    fn _div_assign(&mut self, rhs: Rhs)
    {
        *self /= rhs
    }
}