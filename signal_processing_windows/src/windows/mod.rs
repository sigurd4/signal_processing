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

use array_trait::length::LengthValue;
use bulks::Bulk;
use num::complex::ComplexFloat;

pub enum WindowRange
{
    Symmetric,
    Periodic
}

pub const trait WindowGen<T, N>
where
    T: ComplexFloat,
    N: LengthValue
{
    type Output: Bulk<Item = T>;

    fn window_gen(&self, numtaps: N, range: WindowRange) -> Self::Output;
}