use core::{borrow::BorrowMut, f64::consts::{FRAC_PI_2, SQRT_2}};

use crate::{Dft, SpectrumScaling, temp, util::{self, AddAssignSpec, IntoComplex, MulAssignSpec, RealDiv, RealMul, TruncateIm}};

use array_trait::length::{self, LengthValue};
use bulks::{AsBulk, Bulk, CollectNearest, IntoBulk};
use num_complex::{Complex, ComplexFloat};
use num_traits::{Float, FloatConst, NumCast, Zero};

pub fn fct_iii_unscaled<B, C, T>(sequence: &mut B, mut temp: Option<&mut [C]>)
where
    for<'a> &'a mut B: IntoBulk<Item: BorrowMut<C>>,
    B: ?Sized,
    C: ComplexFloat<Real = T> + 'static,
    T: Float + FloatConst + 'static
{
    if //fct_iii_8_unscaled(sequence) ||
        fct_iii_radix2_unscaled(sequence, &mut temp) ||
        dct_iii_fft_unscaled(sequence, &mut temp)
    {
        return
    }
    dct_iii_direct_unscaled(sequence, &mut temp);
}

// Algorithm by Arai, Agui, Nakajima, 1988. For details, see:
// https://web.stanford.edu/class/ee398a/handouts/lectures/07-TransformCoding.pdf#page=30
#[allow(unused)]
pub fn fct_iii_8_unscaled<B, C, T>(sequence: &mut B) -> bool
where
    for<'a> &'a mut B: IntoBulk<Item: BorrowMut<C>>,
    B: ?Sized,
    C: ComplexFloat<Real = T> + 'static,
    T: Float + FloatConst + 'static
{
    const S: [f64; 8] = [
        0.353553390593273762200422/SQRT_2.next_down(),
        0.254897789552079584470970,
        0.270598050073098492199862,
        0.300672443467522640271861,
        0.353553390593273762200422,
        0.449988111568207852319255,
        0.653281482438188263928322,
        1.281457723870753089398043,
    ];

    const A: [f64; 5] = [
        0.707106781186547524400844,
        0.541196100146196984399723,
        0.707106781186547524400844,
        1.306562964876376527856643,
        0.382683432365089771728460,
    ];

    let len = sequence.bulk_mut().length();

    if length::value::eq(len, [(); 8])
        && let Some(mut sequence) = sequence.bulk_mut()
        .map(Some)
        .resize_with([(); 8], || None)
        .try_collect::<[_; 8], _>()
    {
        let [v15, v26, v21, v28, v16, v25, v22, v27] = sequence.bulk_mut()
            .map(|x| *(*x).borrow_mut())
            .zip(S)
            .map(|(v, s)| v._real_div(T::from(s).unwrap()))
            .collect();

        let one = T::one();
        let two = one + one;

        let v19 = (v25 - v28)._real_div(two);
        let v20 = (v26 - v27)._real_div(two);
        let v23 = (v26 + v27)._real_div(two);
        let v24 = (v25 + v28)._real_div(two);
        
        let v7  = (v23 + v24)._real_div(two);
        let v11 = (v21 + v22)._real_div(two);
        let v13 = (v23 - v24)._real_div(two);
        let v17 = (v21 - v22)._real_div(two);
        
        let v8 = (v15 + v16)._real_div(two);
        let v9 = (v15 - v16)._real_div(two);
        
        let v18 = (v19 - v20)._real_mul(T::from(A[4]).unwrap());  // Different from original
        let v12 = (v19._real_mul(T::from(A[3]).unwrap()) - v18)._real_div(T::from(A[1] * A[4] - A[1] * A[3] - A[3] * A[4]).unwrap());
        let v14 = (v18 - v20._real_mul(T::from(A[1]).unwrap()))._real_div(T::from(A[1] * A[4] - A[1] * A[3] - A[3] * A[4]).unwrap());
        
        let v6 = v14 - v7;
        let v5 = v13._real_div(T::from(A[2]).unwrap()) - v6;
        let v4 = -v5 - v12;
        let v10 = v17._real_div(T::from(A[0]).unwrap()) - v11;
        
        let v0 = (v8 + v11)._real_div(two);
        let v1 = (v9 + v10)._real_div(two);
        let v2 = (v9 - v10)._real_div(two);
        let v3 = (v8 - v11)._real_div(two);

        let y1 = [v0, v1, v2, v3];
        let y2 = [v4, v5, v6, v7];

        bulks::chain(
            y1.into_bulk()
                .zip(y2.into_bulk().rev())
                .map(|(y1, y2)| (y1 + y2)._real_div(two)),
            y1.into_bulk()
                .rev()
                .zip(y2)
                .map(|(y1, y2)| (y1 - y2)._real_div(two))
        )
            .zip(sequence)
            .for_each(|(x, mut y)| *y.borrow_mut() = x);
        return true
    }
    false
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

/// Algorithm by Byeong Gi Lee, 1984. For details, see:
/// https://www.nayuki.io/res/fast-discrete-cosine-transform-algorithms/lee-new-algo-discrete-cosine-transform.pdf
pub fn fct_iii_radix2_unscaled<B, C, T>(sequence: &mut B, temp: &mut Option<&mut [C]>) -> bool
where
    for<'a> &'a mut B: IntoBulk<Item: BorrowMut<C>>,
    B: ?Sized,
    C: ComplexFloat<Real = T> + 'static,
    T: Float + FloatConst + 'static
{
    let len = sequence.bulk_mut().length();
    if length::value::eq(length::value::rem(len, [(); 2]), [(); 0])
    {
        // Recursive FFT

        temp!(temp for len);

        let ldiv = length::value::len(len)/2;
        {
            let mut x = temp.chunks_mut(ldiv);
            let x = bulks::repeat_n_with(|| x.next().unwrap(), [(); 2])
                .collect_nearest();
            {
                let [p, q] = sequence.bulk_mut()
                    .map(|mut x| *x.borrow_mut())
                    .map(Some)
                    .resize_with([(); _], || None)
                    .try_collect_array()
                    .unwrap();

                x[0][0] = p;
                x[1][0] = q;
            }
            for k in 1..ldiv
            {
                let [p, q, r] = sequence.bulk_mut()
                    .skip(2*k - 1)
                    .map(|mut x| *x.borrow_mut())
                    .map(Some)
                    .resize_with([(); _], || None)
                    .try_collect_array()
                    .unwrap();

                x[0][k] = q;
                x[1][k] = p + r;
            }
        }
        partial_fct_iii_unscaled(sequence, temp, [(); 2]);

        let wn_half = Complex::cis(<T as NumCast>::from(FRAC_PI_2/length::value::len(len) as f64).unwrap());
        let wn = wn_half*wn_half;
        let mut wn_pk = wn_half;

        let mut x = temp.chunks(ldiv);
        let x = bulks::repeat_n_with(|| x.next().unwrap(), [(); 2])
            .collect_nearest();
        for k in 0..ldiv
        {
            let p = x[0][k];
            let q = x[1][k]._real_div(wn_pk.re + wn_pk.re);

            let [mut x1, mut x2] = sequence.bulk_mut()
                .skip(k)
                .step_by(length::value::len(len) - k*2 - 1)
                .map(Some)
                .resize_with([(); _], || None)
                .try_collect_array()
                .unwrap();
            
            *x1.borrow_mut() = p + q;
            *x2.borrow_mut() = p - q;

            wn_pk._mul_assign(wn);
        }
        return true;
    }
    false
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
    let four = two + two;

    let m = bulks::range([(); 1], len)
        .map(|i| {
            let i = <<T as ComplexFloat>::Real as NumCast>::from(i).unwrap();
            Complex::cis(i*frac_pi_2/lenf)
        });

    sequence.bulk_mut()
        .skip([(); 1])
        .map(|mut x| (*x.borrow_mut(), x))
        .into_bulk()
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
    bulks::zip(y1, y2.rev())
        .map(|(y1, y2)| C::truncate_im(y1 + y2)._real_div(four))
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
        .for_each(|(mut src, dst)| { *dst = core::mem::replace(src.borrow_mut(), first_term)._real_div(two); });
        
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