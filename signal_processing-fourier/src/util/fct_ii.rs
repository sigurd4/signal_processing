use core::{borrow::{Borrow, BorrowMut}, f64::consts::{FRAC_PI_2, PI}};

use crate::{Dft, Permute, SpectrumScaling, scratch_space::ScratchLength, temp, util::{self, AddAssignSpec, IntoComplex, MulAssignSpec, RealDiv, RealMul, TruncateIm, fft}};

use array_trait::length::{self, LengthValue};
use bulks::{AsBulk, Bulk, CollectNearest, IntoBulk};
use num_complex::{Complex, ComplexFloat};
use num_traits::{Float, FloatConst, NumCast, One, Zero};

pub fn fct_ii_unscaled<B, C, T>(sequence: &mut B, mut temp: Option<&mut [C]>)
where
    for<'a> &'a mut B: IntoBulk<Item: BorrowMut<C>>,
    B: ?Sized,
    C: ComplexFloat<Real = T> + 'static,
    T: Float + FloatConst + 'static
{
    if  fct_ii_radix2_unscaled(sequence, &mut temp) ||
        dct_ii_fft_unscaled(sequence, &mut temp)
    {
        return
    }
    dct_ii_direct_unscaled(sequence, &mut temp);
}

pub fn partial_fct_ii_unscaled<B, C, T, M>(sequence: &mut B, temp: &mut [C], m: M)
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
            fct_ii_unscaled::<[_], C, T>(
                core::slice::from_raw_parts_mut(temp, length::value::len(n)),
                Some(core::slice::from_raw_parts_mut(bulk, length::value::len(n)))
            )
        }
    }
    if length::value::gt(r, [(); 0]) && let Some((temp, bulk)) = iter.next()
    {
        unsafe {
            fct_ii_unscaled::<[_], C, T>(
                core::slice::from_raw_parts_mut(temp, length::value::len(r)),
                Some(core::slice::from_raw_parts_mut(bulk, length::value::len(r)))
            )
        }
    }
}

/// Algorithm by Byeong Gi Lee, 1984. For details, see:
/// See: http://citeseerx.ist.psu.edu/viewdoc/download?doi=10.1.1.118.3056&rep=rep1&type=pdf#page=34
pub fn fct_ii_radix2_unscaled<B, C, T>(sequence: &mut B, temp: &mut Option<&mut [C]>) -> bool
where
    for<'a> &'a mut B: IntoBulk<Item: BorrowMut<C>>,
    B: ?Sized,
    C: ComplexFloat<Real = T> + 'static,
    T: Float + FloatConst + 'static
{
    let len = sequence.bulk_mut().length();
    if length::value::eq(length::value::rem(len, [(); 2]), [(); 0])
    {
        /*if util::is_power_of(len, [(); 2])
        {
            // In-place DCT II
            sequence.bit_rev_permute();
            
            for s in 0..length::value::len(len).ilog2()
            {
                let m = 2usize << s;
                let wm_half = Complex::cis(<T as NumCast>::from(FRAC_PI_2/m as f64).unwrap());
                let wm = wm_half*wm_half;
                for k in (0..length::value::len(len)).step_by(m)
                {
                    let mut w = wm_half;
                    for j in 0..m/2
                    {
                        let [mut x1, mut x2] = sequence.bulk_mut()
                            .skip(k + j)
                            .step_by(m/2)
                            .map(Some)
                            .resize_with([(); _], || None)
                            .try_collect_array()
                            .unwrap();

                        let p = *x1.borrow();
                        let q = x2.borrow()._real_mul(w.re);

                        *x1.borrow_mut() = p + q;
                        *x2.borrow_mut() = p - q;
                        w._mul_assign(wm);
                    }
                }
            }
            return true
        }*/
        // Recursive FFT

        temp!(temp for len);

        let ldiv = length::value::len(len)/2;
        let wn_half = Complex::cis(<T as NumCast>::from(FRAC_PI_2/length::value::len(len) as f64).unwrap());
        let wn = wn_half*wn_half;
        let mut wn_pk = wn_half;

        {
            let mut x = temp.chunks_mut(ldiv);
            let x = bulks::repeat_n_with(|| x.next().unwrap(), [(); 2])
                .collect_nearest();
            for k in 0..ldiv
            {
                let [x1, x2] = sequence.bulk_mut()
                    .map(|mut x| *x.borrow_mut())
                    .skip(k)
                    .step_by(length::value::len(len) - k*2 - 1)
                    .map(Some)
                    .resize_with([(); _], || None)
                    .try_collect_array()
                    .unwrap();
                
                let p = x1 + x2;
                let q = x1 - x2;

                x[0][k] = p;
                x[1][k] = q._real_div(wn_pk.re + wn_pk.re);

                wn_pk._mul_assign(wn);
            }
        }
        partial_fct_ii_unscaled(sequence, temp, [(); 2]);
        let mut x = temp.chunks(ldiv);
        let x = bulks::repeat_n_with(|| x.next().unwrap(), [(); 2])
            .collect_nearest();
        for k in 0..ldiv.saturating_sub(1)
        {
            let p = x[0][k];
            let q = x[1][k];
            let r = x[1][k + 1];

            let [mut x1, mut x2] = sequence.bulk_mut()
                .skip(k*2)
                .map(Some)
                .resize_with([(); _], || None)
                .try_collect_array()
                .unwrap();
            
            *x1.borrow_mut() = p;
            *x2.borrow_mut() = q + r;
        }
        if let Some(k) = ldiv.checked_sub(1)
        {
            let p = x[0][k];
            let q = x[1][k];

            let [mut x1, mut x2] = sequence.bulk_mut()
                .skip(k*2)
                .map(Some)
                .resize_with([(); _], || None)
                .try_collect_array()
                .unwrap();
            
            *x1.borrow_mut() = p;
            *x2.borrow_mut() = q;
        }
        return true;
    }
    false
}
pub fn dct_ii_fft_unscaled<B, C, T>(sequence: &mut B, temp: &mut Option<&mut [C]>) -> bool
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
    sequence.bulk_mut()
        .map(|mut x| *x.borrow_mut())
        .zip(temp.bulk_mut().skip(len).rev())
        .for_each(|(x, y)| *y = x.into_complex());
    temp.dft_scaled(SpectrumScaling::Summed);
    let (y1, y2) = (*temp).bulk()
        .split_at(len);
    let (y0, y1) = y1.split_at([(); 1]);
    y0.map(|y| C::truncate_im(*y))
        .chain(
            bulks::zip(y1, y2.rev())
                .zip(m)
                .map(|((y1, y2), m)| C::truncate_im(y1*m.conj() + y2*m)._real_div(two))
        )
        .zip(sequence)
        .for_each(|(y, mut x)| *x.borrow_mut() = y);

    true
}

pub fn dct_ii_direct_unscaled<B, C, T>(sequence: &mut B, temp: &mut Option<&mut [C]>)
where
    for<'a> &'a mut B: IntoBulk<Item: BorrowMut<C>>,
    B: ?Sized,
    C: ComplexFloat<Real = T> + 'static,
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
        .for_each(|(mut src, dst)| { *dst = core::mem::replace(src.borrow_mut(), Zero::zero()); });
        
    sequence.bulk_mut()
        .for_each(|mut y| {
            let mut wnki = wnk_half;
            let wnk = wnk_half*wnk_half;
            (*temp).bulk()
                .for_each(|x| {
                    y.borrow_mut()._add_assign(x._real_mul(wnki.re));
                    wnki._mul_assign(wnk);
                });
            wnk_half._mul_assign(wn_half);
        });
}