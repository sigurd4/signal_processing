use num::{complex::ComplexFloat, traits::Inv};

use crate::{MaybeList, Tf};

impl<T, B, A> Inv for Tf<T, B, A>
where
    T: ComplexFloat,
    B: MaybeList<T>,
    A: MaybeList<T>
{
    type Output = Tf<T, A, B>;

    fn inv(self) -> Self::Output
    {
        Tf {
            b: self.a,
            a: self.b
        }
    }
}