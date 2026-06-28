use core::{borrow::BorrowMut, f64::consts::PI};

use crate::{Dft, SpectrumScaling, temp, util::{AddAssignSpec, IntoComplex, MulAssignSpec, RealDiv, RealMul, TruncateIm}};

use array_trait::length;
use bulks::{AsBulk, Bulk, IntoBulk};
use num_complex::{Complex, ComplexFloat};
use num_traits::{Float, FloatConst, NumCast, One};

pub fn fct_i_unscaled<B, C, T>(sequence: &mut B, mut temp: Option<&mut [C]>)
where
    for<'a> &'a mut B: IntoBulk<Item: BorrowMut<C>>,
    B: ?Sized,
    C: ComplexFloat<Real = T>,
    T: Float + FloatConst + 'static
{
    if dct_i_fft_unscaled(sequence, &mut temp)
    {
        return
    }
    dct_i_direct_unscaled(sequence, &mut temp);
}

pub fn dct_i_fft_unscaled<B, C, T>(sequence: &mut B, temp: &mut Option<&mut [C]>) -> bool
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
    
    let len_m1 = length::value::saturating_sub(len, [(); 1]);
    let len_buf = length::value::mul(len_m1, [(); 2]);

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
    sequence.bulk_mut()
        .map(|mut x| *x.borrow_mut())
        .zip(temp.bulk_mut().take(len))
        .for_each(|(x, y)| *y = x.into_complex());
    sequence.bulk_mut()
        .take(len_m1)
        .skip([(); 1])
        .map(|mut x| *x.borrow_mut())
        .zip(temp.bulk_mut().skip(len).rev())
        .for_each(|(x, y)| *y = x.into_complex());
    temp.dft_scaled(SpectrumScaling::Summed);
    let (y1, y2) = (*temp).bulk()
        .split_at(len);
    let (y1, y3) = y1.split_at(len_m1);
    let (y0, y1) = y1.split_at([(); 1]);
    y0.map(|&y| C::truncate_im(y))
        .chain(
            y1.zip(y2.rev())
                .map(|(y1, y2)| C::truncate_im(*y1 + *y2)._real_div(two))
        )
        .chain(y3.map(|&y| C::truncate_im(y)))
        .zip(sequence)
        .for_each(|(y, mut x)| *x.borrow_mut() = y);
    true
}

pub fn dct_i_direct_unscaled<B, C, T>(sequence: &mut B, temp: &mut Option<&mut [C]>)
where
    for<'a> &'a mut B: IntoBulk<Item: BorrowMut<C>>,
    B: ?Sized,
    C: ComplexFloat<Real = T>,
    T: Float + 'static
{
    let len = sequence.bulk_mut().length();
    let len_m1 = length::value::saturating_sub(len, [(); 1]);
    
    let (x_first, x_last) = match (
        sequence.bulk_mut().first().map(|mut x| *x.borrow_mut()),
        sequence.bulk_mut().last().map(|mut x| *x.borrow_mut())
    )
    {
        (Some(x_first), Some(x_last)) => (x_first, x_last),
        _ => return
    };

    temp!(temp for len);

    let wn = Complex::cis(<T as NumCast>::from(
        PI/length::value::len(len_m1) as f64
    ).unwrap());
    let mut wnk = Complex::one();
    let two = <T as NumCast>::from(2).unwrap();

    let mut first_term = [
        (x_first + x_last)._real_div(two),
        (x_first - x_last)._real_div(two)
    ].into_iter()
        .cycle();

    sequence.bulk_mut()
        .zip(temp.borrow_mut())
        .for_each(|(mut src, dst)| { *dst = core::mem::replace(src.borrow_mut(), first_term.next().unwrap()); });
        
    sequence.bulk_mut()
        .for_each(|mut y| {
            let mut wnki = wnk;
            (*temp).bulk()
                .take(len_m1)
                .skip([(); 1])
                .for_each(|x| {
                    y.borrow_mut()._add_assign(x._real_mul(wnki.re));
                    wnki._mul_assign(wnk);
                });
            wnk._mul_assign(wn);
        });
}