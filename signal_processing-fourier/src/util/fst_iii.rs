use core::{borrow::BorrowMut, f64::consts::{FRAC_PI_2, PI}};

use crate::{Dft, SpectrumScaling, temp, util::{AddAssignSpec, IntoComplex, MulAssignSpec, RealDiv, RealMul, TruncateIm}};

use array_trait::length;
use bulks::{AsBulk, Bulk, IntoBulk};
use num_complex::{Complex, ComplexFloat};
use num_traits::{Float, FloatConst, NumCast, One, Zero};

pub fn fst_iii_unscaled<B, C, T>(sequence: &mut B, mut temp: Option<&mut [C]>)
where
    for<'a> &'a mut B: IntoBulk<Item: BorrowMut<C>>,
    B: ?Sized,
    C: ComplexFloat<Real = T>,
    T: Float + FloatConst + 'static
{
    if dst_iii_fft_unscaled(sequence, &mut temp)
    {
        return
    }
    dst_iii_direct_unscaled(sequence, &mut temp);
}

pub fn dst_iii_fft_unscaled<B, C, T>(sequence: &mut B, temp: &mut Option<&mut [C]>) -> bool
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
    let len_buf = length::value::mul(len, [(); 2]);
    let mut temp = temp.as_mut()
        .map(|temp| unsafe {
            core::slice::from_raw_parts_mut(temp.as_mut_ptr().cast::<Complex<T>>(), temp.len()/(std::mem::size_of::<Complex<T>>()/std::mem::size_of::<C>()).max(1))
        });
    temp!(temp for len_buf);

    let lenf = <T as NumCast>::from(length::value::len(len)).unwrap();

    let frac_pi_2 = T::FRAC_PI_2();
    let one = T::one();
    let two = one + one;
    let four = two + two;

    let m = bulks::range([(); 1], len)
        .map(|i| {
            let i = <<T as ComplexFloat>::Real as NumCast>::from(i).unwrap();
            Complex::cis(i*frac_pi_2/lenf)
        });

    sequence.bulk_mut()
        .map(|mut x| (*x.borrow_mut(), x))
        .into_bulk()
        .for_each(|(x, mut y)| *y.borrow_mut() = x._real_div(two));
    temp[0] = Complex::zero();
    sequence.bulk_mut()
        .map(|mut x| *x.borrow_mut())
        .zip(temp.bulk_mut().skip([(); 1]).take(len))
        .for_each(|(x, y)| *y = x.into_complex());
    sequence.bulk_mut()
        .skip([(); 1])
        .map(|mut x| *x.borrow_mut())
        .zip(temp.bulk_mut().skip(length::value::add(len, [(); 1])).rev())
        .for_each(|(x, y)| *y = x.into_complex());
    let (y1, y2) = (*temp).bulk_mut()
        .skip([(); 1])
        .split_at(len);
    let (y1, y3) = y1.split_at(len_m1);
    y3.for_each(|y| y._mul_assign(Complex::i()));
    y1.zip(y2.rev())
        .zip(m)
        .for_each(|((y1, y2), m)| {
            y1._mul_assign(m);
            y2._mul_assign(-m.conj());
        });
    temp.idft_scaled(SpectrumScaling::Averaged);
    let (y1, y2) = (*temp).bulk()
        .split_at(len);
    bulks::zip(y1, y2.rev())
        .map(|(y1, y2)| C::truncate_im(-(y1 - y2)*Complex::i())._real_div(four))
        .zip(sequence)
        .for_each(|(y, mut x)| *x.borrow_mut() = y);

    true
}

pub fn dst_iii_direct_unscaled<B, C, T>(sequence: &mut B, temp: &mut Option<&mut [C]>)
where
    for<'a> &'a mut B: IntoBulk<Item: BorrowMut<C>>,
    B: ?Sized,
    C: ComplexFloat<Real = T>,
    T: Float + 'static
{
    let len = sequence.bulk_mut().length();
    let len_m1 = length::value::saturating_sub(len, [(); 1]);
    
    let x_last = match sequence.bulk_mut().last().map(|mut x| *x.borrow_mut())
    {
        Some(x_last) => x_last,
        None => return
    };

    temp!(temp for len);

    let wn_half = Complex::cis(<T as NumCast>::from(
        FRAC_PI_2/length::value::len(len) as f64
    ).unwrap());
    let wn = wn_half*wn_half;
    let mut wnk = wn_half;
    let two = <T as NumCast>::from(2).unwrap();

    let mut first_term = [
        x_last._real_div(two),
        -x_last._real_div(two)
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
                .for_each(|x| {
                    y.borrow_mut()._add_assign(x._real_mul(wnki.im));
                    wnki._mul_assign(wnk);
                });
            wnk._mul_assign(wn);
        });
}