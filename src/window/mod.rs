use num::{complex::ComplexFloat, Complex};
use option_trait::Maybe;

use crate::List;

moddef::moddef!(
    flat(pub) mod {
        barthann,
        blackman_harris,
        blackman_nuttall,
        blackman,
        bohman,
        boxcar,
        confined_gaussian,
        confined_normal,
        dolph_chebyshev,
        flattop,
        gaussian,
        hamming,
        hann,
        kaiser,
        normal,
        nuttall,
        parzen,
        planck_taper,
        power_of_sine,
        sine,
        triangular,
        tukey,
        ultraspherical,
        welch
    }
);

pub trait WindowGen<T, W, N>
where
    T: ComplexFloat,
    N: Maybe<usize>,
    W: List<T>
{
    type Output: Maybe<W>;

    fn window_gen(&self, numtaps: N, range: WindowRange) -> Self::Output;
}

pub enum WindowRange
{
    Symmetric,
    Periodic
}