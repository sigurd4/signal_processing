use core::{borrow::{Borrow, BorrowMut}, f64::consts::{FRAC_PI_2, PI}};

use crate::{Dft, Permute, SpectrumScaling, scratch_space::ScratchLength, temp, util::{self, AddAssignSpec, IntoComplex, MulAssignSpec, RealDiv, RealMul, TruncateIm, fft}};

use array_trait::length::{self, LengthValue};
use bulks::{AsBulk, Bulk, CollectNearest, IntoBulk};
use num_complex::{Complex, ComplexFloat};
use num_traits::{Float, FloatConst, NumCast, One, ToPrimitive, Zero};

pub fn fct_iii_unscaled<B, C, T>(sequence: &mut B, mut temp: Option<&mut [C]>)
where
    for<'a> &'a mut B: IntoBulk<Item: BorrowMut<C>>,
    B: ?Sized,
    C: ComplexFloat<Real = T> + 'static,
    T: Float + FloatConst + 'static
{
    if  //fct_iii_radix2_unscaled(sequence, &mut temp) ||
        dct_iii_fft_unscaled(sequence, &mut temp)
    {
        return
    }
    dct_iii_direct_unscaled(sequence, &mut temp);
}

pub fn partial_fct_iii_unscaled<B, C, T, M>(sequence: &mut B, temp: &mut [C], m: M)
where
    for<'a> &'a mut B: IntoBulk<Item: BorrowMut<C>>,
    B: ?Sized,
    C: ComplexFloat<Real = T> + 'static,
    T: Float + FloatConst + 'static,
    M: LengthValue
{
    let len = sequence.bulk_mut().length();
    let n = length::value::div(len, length::value::max(m, [(); 1]));
    let r = length::value::rem(len, length::value::max(m, [(); 1]));
    
    let mut buffer = util::recurse_buffer(sequence);
    temp!(buffer for len);

    let mut iter = temp.bulk_mut()
        .zip(buffer)
        .step_by(n)
        .into_iter();
    
    for (temp, bulk) in iter.by_ref()
        .take(length::value::len(m))
    {
        unsafe {
            fct_iii_unscaled::<[_], C, T>(
                core::slice::from_raw_parts_mut(temp, length::value::len(n)),
                Some(core::slice::from_raw_parts_mut(bulk, length::value::len(n)))
            )
        }
    }
    if length::value::gt(r, [(); 0]) && let Some((temp, bulk)) = iter.next()
    {
        unsafe {
            fct_iii_unscaled::<[_], C, T>(
                core::slice::from_raw_parts_mut(temp, length::value::len(r)),
                Some(core::slice::from_raw_parts_mut(bulk, length::value::len(r)))
            )
        }
    }
}

pub fn dct_iii_fft_unscaled<B, C, T>(sequence: &mut B, temp: &mut Option<&mut [C]>) -> bool
where
    for<'a> &'a mut B: IntoBulk<Item: BorrowMut<C>>,
    B: ?Sized,
    C: ComplexFloat<Real = T> + 'static,
    T: Float + FloatConst + 'static
{
    let len = sequence.bulk_mut().length();
    if length::value::le(len, [(); 1])
    {
        return false
    }
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
    temp[length::value::len(len)] = Complex::zero();
    sequence.bulk_mut()
        .skip([(); 1])
        .map(|mut x| *x.borrow_mut())
        .zip(temp.bulk_mut().skip(length::value::add(len, [(); 1])).rev())
        .for_each(|(x, y)| *y = x.into_complex());
    let (y0, y1) = (*temp).bulk_mut()
        .split_at(len);
    y0.skip([(); 1])
        .zip(y1.skip([(); 1]).rev())
        .zip(m)
        .for_each(|((y0, y1), m)| {
            y0._mul_assign(m.conj());
            y1._mul_assign(m);
        });
    temp.dft_scaled(SpectrumScaling::Summed);
    let (y1, y2) = (*temp).bulk()
        .split_at(len);
    /*bulks::zip(y1, y2.rev())
        .map(|(y1, y2)| C::truncate_im(y1 + y2)._real_div(two))*/
        y1.map(|y| C::truncate_im(*y))
        .zip(sequence)
        .for_each(|(y, mut x)| *x.borrow_mut() = y);

    true
}

pub fn dct_iii_direct_unscaled<B, C, T>(sequence: &mut B, temp: &mut Option<&mut [C]>)
where
    for<'a> &'a mut B: IntoBulk<Item: BorrowMut<C>>,
    B: ?Sized,
    C: ComplexFloat<Real = T> + 'static,
    T: Float + 'static
{
    let len = sequence.bulk_mut().length();

    let x_first = match sequence.bulk_mut().first().map(|mut x| *x.borrow_mut())
    {
        Some(x_first) => x_first,
        None => return
    };

    temp!(temp for len);

    let wn_half = Complex::cis(<T as NumCast>::from(
        FRAC_PI_2/length::value::len(len) as f64
    ).unwrap());
    let wn = wn_half*wn_half;
    let mut wnk = wn_half;
    let two = <T as NumCast>::from(2).unwrap();

    let first_term = x_first._real_div(two);

    sequence.bulk_mut()
        .zip(temp.borrow_mut())
        .for_each(|(mut src, dst)| { *dst = core::mem::replace(src.borrow_mut(), first_term); });
        
    sequence.bulk_mut()
        .for_each(|mut y| {
            let mut wnki = wnk;
            (*temp).bulk()
                .skip([(); 1])
                .for_each(|x| {
                    y.borrow_mut()._add_assign(x._real_mul(wnki.re));
                    wnki._mul_assign(wnk);
                });
            wnk._mul_assign(wn);
        });
}