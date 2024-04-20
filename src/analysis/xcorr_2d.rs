use num::complex::ComplexFloat;

use crate::{ComplexOp, Matrix, MaybeMatrix};

pub trait XCorr2D<X, Y, YY, Z>: Matrix<X>
where
    X: ComplexFloat + ComplexOp<Y, Output = Z>,
    Y: ComplexFloat<Real = X::Real> + Into<Z>,
    YY: MaybeMatrix<Y>,
    Z: ComplexFloat<Real = X::Real>
{
    fn xcorr_2d();
}