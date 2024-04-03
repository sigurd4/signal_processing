use num::{complex::ComplexFloat};

use crate::{MaybeList, MaybeLists, Polynomial, Tf};

impl<T, B, A> Default for Tf<T, B, A>
where
    T: ComplexFloat,
    B: MaybeLists<T>,
    A: MaybeList<T>,
    Polynomial<T, B>: From<Polynomial<T, ()>>,
    Polynomial<T, A>: From<Polynomial<T, ()>>
{
    fn default() -> Self
    {
        Self {
            b: Polynomial::new(()).into(),
            a: Polynomial::new(()).into()
        }
    }
}