use core::ops::{Add, AddAssign, Div, DivAssign, Mul, MulAssign};

pub const trait AddAssignSpec: ~const Add<Output = Self> + Copy
{
    fn _add_assign(&mut self, rhs: Self);
}
impl<T> const AddAssignSpec for T
where
    T: ~const Add<Output = Self> + Copy
{
    default fn _add_assign(&mut self, rhs: Self)
    {
        *self = *self + rhs
    }
}
impl<T> const AddAssignSpec for T
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
impl<T> const MulAssignSpec for T
where
    T: ~const Mul<Output = Self> + Copy
{
    default fn _mul_assign(&mut self, rhs: Self)
    {
        *self = *self * rhs
    }
}
impl<T> const MulAssignSpec for T
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
impl<T, Rhs> const DivAssignSpec<Rhs> for T
where
    T: ~const Div<Rhs, Output = Self> + Copy
{
    default fn _div_assign(&mut self, rhs: Rhs)
    {
        *self = *self / rhs
    }
}
impl<T, Rhs> const DivAssignSpec<Rhs> for T
where
    T: ~const Div<Rhs, Output = Self> + Copy + ~const DivAssign<Rhs>
{
    fn _div_assign(&mut self, rhs: Rhs)
    {
        *self /= rhs
    }
}