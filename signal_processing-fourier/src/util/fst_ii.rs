use core::{borrow::BorrowMut, f64::consts::FRAC_PI_2};

use crate::{Dft, SpectrumScaling, temp, util::{AddAssignSpec, IntoComplex, MulAssignSpec, RealDiv, RealMul, TruncateIm}};

use array_trait::length;
use bulks::{AsBulk, Bulk, IntoBulk};
use num_complex::{Complex, ComplexFloat};
use num_traits::{Float, FloatConst, NumCast, Zero};

pub fn fst_ii_unscaled<B, C, T>(sequence: &mut B, mut temp: Option<&mut [C]>)
where
    for<'a> &'a mut B: IntoBulk<Item: BorrowMut<C>>,
    B: ?Sized,
    C: ComplexFloat<Real = T>,
    T: Float + FloatConst + 'static
{
    if dst_ii_fft_unscaled(sequence, &mut temp)
    {
        return
    }
    dst_ii_direct_unscaled(sequence, &mut temp);
}

pub fn dst_ii_fft_unscaled<B, C, T>(sequence: &mut B, temp: &mut Option<&mut [C]>) -> bool
where
    for<'a> &'a mut B: IntoBulk<Item: BorrowMut<C>>,
    B: ?Sized,
    C: ComplexFloat<Real = T>,
    T: Float + FloatConst + 'static
{
    let len = sequence.bulk_mut().length();
    if length::value::le(len, [(); 1])
    {
        return false
    }
    let lenf = <<T as ComplexFloat>::Real as NumCast>::from(length::value::len(len)).unwrap();
    
    let len_buf = length::value::mul(len, [(); 2]);

    let mut temp = temp.as_mut()
        .map(|temp| unsafe {
            core::slice::from_raw_parts_mut(temp.as_mut_ptr().cast::<Complex<T>>(), temp.len()/(std::mem::size_of::<Complex<T>>()/std::mem::size_of::<C>()).max(1))
        });
    temp!(temp for len_buf);

    let frac_pi_2 = <T as ComplexFloat>::Real::FRAC_PI_2();
    let one = T::one();
    let two = one + one;

    let m = bulks::range([(); 1], len)
        .map(|i| {
            let i = <<T as ComplexFloat>::Real as NumCast>::from(i).unwrap();
            Complex::cis(i*frac_pi_2/lenf)
        });

    sequence.bulk_mut()
        .map(|mut x| (*x.borrow_mut(), x))
        .for_each(|(x, mut y)| *y.borrow_mut() = x._real_div(two));
    sequence.bulk_mut()
        .map(|mut x| *x.borrow_mut())
        .zip(temp.bulk_mut().take(len))
        .for_each(|(x, y)| *y = x.into_complex());
    sequence.bulk_mut()
        .map(|mut x| *x.borrow_mut())
        .zip(temp.bulk_mut().skip(len).rev())
        .for_each(|(x, y)| *y = -x.into_complex());
    temp.dft_scaled(SpectrumScaling::Summed);
    let (y1, y2) = (*temp).bulk()
        .split_at(len);
    let (_, y1) = y1.split_at([(); 1]);
    let (y3, y2) = y2.split_at([(); 1]);
    
    y1.zip(y2.rev())
        .zip(m)
        .map(|((y1, y2), m)| C::truncate_im((*y1*m.conj() - *y2*m)*Complex::i())._real_div(two))
        .chain(y3.map(|y| C::from_real(y.re)))
        .zip(sequence)
        .for_each(|(y, mut x)| *x.borrow_mut() = y);
    true
}

pub fn dst_ii_direct_unscaled<B, C, T>(sequence: &mut B, temp: &mut Option<&mut [C]>)
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
    let mut wnk_half = wn_half;

    sequence.bulk_mut()
        .zip(temp.borrow_mut())
        .for_each(|(mut src, dst)| { *dst = core::mem::replace(src.borrow_mut(), Zero::zero()); });
        
    sequence.bulk_mut()
        .for_each(|mut y| {
            let mut wnki = wnk_half;
            let wnk = wnk_half*wnk_half;
            (*temp).bulk()
                .for_each(|x| {
                    y.borrow_mut()._add_assign(x._real_mul(wnki.im));
                    wnki._mul_assign(wnk);
                });
            wnk_half._mul_assign(wn_half);
        });
}