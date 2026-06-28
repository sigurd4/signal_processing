use core::{borrow::BorrowMut, f64::consts::PI};

use crate::{Dft, SpectrumScaling, temp, util::{AddAssignSpec, IntoComplex, MulAssignSpec, RealDiv, RealMul, TruncateIm}};

use array_trait::length;
use bulks::{AsBulk, Bulk, IntoBulk};
use num_complex::{Complex, ComplexFloat};
use num_traits::{Float, FloatConst, NumCast, One, Zero};

pub fn fst_i_unscaled<B, C, T>(sequence: &mut B, mut temp: Option<&mut [C]>)
where
    for<'a> &'a mut B: IntoBulk<Item: BorrowMut<C>>,
    B: ?Sized,
    C: ComplexFloat<Real = T>,
    T: Float + FloatConst + 'static
{
    if dst_i_fft_unscaled(sequence, &mut temp)
    {
        return
    }
    dst_i_direct_unscaled(sequence, &mut temp);
}

pub fn dst_i_fft_unscaled<B, C, T>(sequence: &mut B, temp: &mut Option<&mut [C]>) -> bool
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
    
    let len_p1 = length::value::add(len, [(); 1]);
    let len_p2 = length::value::add(len, [(); 2]);
    let len_buf = length::value::mul(len_p1, [(); 2]);

    let mut temp = temp.as_mut()
        .map(|temp| unsafe {
            core::slice::from_raw_parts_mut(temp.as_mut_ptr().cast::<Complex<T>>(), temp.len()/(std::mem::size_of::<Complex<T>>()/std::mem::size_of::<C>()).max(1))
        });
    temp!(temp for len_buf);

    let one = T::one();
    let two = one + one;

    sequence.bulk_mut()
        .map(|mut x| (*x.borrow_mut(), x))
        .for_each(|(x, mut y)| *y.borrow_mut() = x._real_div(two));
    temp[0] = Complex::zero();
    sequence.bulk_mut()
        .map(|mut x| *x.borrow_mut())
        .zip(temp.bulk_mut().skip([(); 1]).take(len))
        .for_each(|(x, y)| *y = x.into_complex());
    temp[length::value::len(len_p1)] = Complex::zero();
    sequence.bulk_mut()
        .map(|mut x| *x.borrow_mut())
        .zip(temp.bulk_mut().skip(len_p2).rev())
        .for_each(|(x, y)| *y = -x.into_complex());
    temp.dft_scaled(SpectrumScaling::Summed);
    let (y1, y2) = (*temp).bulk()
        .split_at([(); 1]).1
        .split_at(len);
    
    y1.zip(y2.rev())
        .map(|(y1, y2)| C::truncate_im((*y1 - *y2)*Complex::i())._real_div(two))
        .zip(sequence)
        .for_each(|(y, mut x)| *x.borrow_mut() = y);
    true
}

pub fn dst_i_direct_unscaled<B, C, T>(sequence: &mut B, temp: &mut Option<&mut [C]>)
where
    for<'a> &'a mut B: IntoBulk<Item: BorrowMut<C>>,
    B: ?Sized,
    C: ComplexFloat<Real = T>,
    T: Float + 'static
{
    let len = sequence.bulk_mut().length();
    let len_p1 = length::value::add(len, [(); 1]);

    temp!(temp for len);

    let wn = Complex::cis(<T as NumCast>::from(
        PI/length::value::len(len_p1) as f64
    ).unwrap());
    let mut wnk = wn;

    sequence.bulk_mut()
        .zip(temp.borrow_mut())
        .for_each(|(mut src, dst)| { *dst = core::mem::replace(src.borrow_mut(), Zero::zero()); });
        
    sequence.bulk_mut()
        .for_each(|mut y| {
            let mut wnki = wnk;
            (*temp).bulk()
                .for_each(|x| {
                    y.borrow_mut()._add_assign(x._real_mul(wnki.im));
                    wnki._mul_assign(wnk);
                });
            wnk._mul_assign(wn);
        });
}