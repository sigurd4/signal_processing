use core::ops::{AddAssign, MulAssign};

use array_math::SliceMath;
use num::{One, Zero};

use crate::{MaybeList, MaybeLists, Polynomial};

impl<T, C, X> FnOnce<(X,)> for Polynomial<T, C>
where
    C: MaybeLists<T>,
    T: One + Zero + AddAssign + MulAssign<X> + Copy,
    X: Copy
{
    type Output = C::RowsMapped<T>;

    extern "rust-call" fn call_once(self, args: (X,)) -> Self::Output
    {
        self.call(args)
    }
}

impl<T, C, X> FnMut<(X,)> for Polynomial<T, C>
where
    C: MaybeLists<T>,
    T: One + Zero + AddAssign + MulAssign<X> + Copy,
    X: Copy
{
    extern "rust-call" fn call_mut(&mut self, args: (X,)) -> Self::Output
    {
        self.call(args)
    }
}

impl<T, C, X> Fn<(X,)> for Polynomial<T, C>
where
    C: MaybeLists<T>,
    T: One + Zero + AddAssign + MulAssign<X> + Copy,
    X: Copy
{
    extern "rust-call" fn call(&self, (x,): (X,)) -> Self::Output
    {
        self.map_rows_to_owned(|p| p.as_view_slice_option()
            .map(|p| p.rpolynomial(x))
            .unwrap_or_else(One::one)
        )
    }
}