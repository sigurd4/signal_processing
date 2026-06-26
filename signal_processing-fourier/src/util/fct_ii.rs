use core::{borrow::BorrowMut, f64::consts::{FRAC_PI_2, PI}};

use crate::{Dft, SpectrumScaling, temp, util::{AddAssignSpec, IntoComplex, MulAssignSpec, RealDiv, RealMul, TruncateIm}};

use array_trait::length;
use bulks::{AsBulk, Bulk, IntoBulk};
use num_complex::{Complex, ComplexFloat};
use num_traits::{Float, FloatConst, NumCast, One, Zero};

pub fn fct_ii_unscaled<B, C, T>(sequence: &mut B, mut temp: Option<&mut [Complex<T>]>)
where
    for<'a> &'a mut B: IntoBulk<Item: BorrowMut<C>>,
    B: ?Sized,
    C: ComplexFloat<Real = T>,
    T: Float + FloatConst + 'static
{
    /*if dct_ii_fft_unscaled(sequence, &mut temp)
    {
        return
    }*/
    dct_ii_direct_unscaled(sequence, &mut temp);
}

pub fn dct_ii_direct_unscaled<B, C, T>(sequence: &mut B, temp: &mut Option<&mut [Complex<T>]>)
where
    for<'a> &'a mut B: IntoBulk<Item: BorrowMut<C>>,
    B: ?Sized,
    C: ComplexFloat<Real = T>,
    T: Float + 'static
{
    let len = sequence.bulk_mut().length();

    temp!(temp for len);

    let wn_half = Complex::cis(<T as NumCast>::from(
        FRAC_PI_2/length::value::len(len) as f64
    ).unwrap());
    let mut wnk_half = Complex::one();

    sequence.bulk_mut()
        .zip(temp.borrow_mut())
        .for_each(|(mut src, dst)| { *dst = core::mem::replace(src.borrow_mut(), Zero::zero()).into_complex(); });
        
    sequence.bulk_mut()
        .for_each(|mut y| {
            let mut wnki = wnk_half;
            let wnk = wnk_half*wnk_half;
            (*temp).bulk()
                .for_each(|x| {
                    y.borrow_mut()._add_assign(C::truncate_im(*x)._real_mul(wnki.re));
                    wnki._mul_assign(wnk);
                });
            wnk_half._mul_assign(wn_half);
        });
}