use core::borrow::{Borrow, BorrowMut};
use std::{f64::consts::TAU};

use array_trait::length::{self, LengthValue};
use bulks::{AsBulk, Bulk, CollectNearest, IntoBulk};
use num_complex::Complex;
use num_traits::{Float, NumCast, One, Zero};

use crate::{permute::Permute, temp, util::{self, MulAssignSpec, AddAssignSpec}};

pub fn fft_unscaled<B, T, const I: bool>(sequence: &mut B, mut temp: Option<&mut [Complex<T>]>)
where
    for<'a> &'a mut B: IntoBulk<Item: BorrowMut<Complex<T>>>,
    B: ?Sized,
    T: Float + 'static
{
    let len = sequence.bulk_mut().len();
    if len <= 1
    {
        return;
    }
    if !(
        fft_radix2_unscaled::<_, _, I>(sequence, &mut temp)
        || fft_radix3_unscaled::<_, _, I>(sequence, &mut temp)
        || fft_radix5_unscaled::<_, _, I>(sequence, &mut temp)
        || fft_radix7_unscaled::<_, _, I>(sequence, &mut temp)
        || fft_radix_p_unscaled::<_, _, _, I>(sequence, &mut temp, [(); 11])
        || fft_radix_p_unscaled::<_, _, _, I>(sequence, &mut temp, [(); 13])
        || fft_radix_p_unscaled::<_, _, _, I>(sequence, &mut temp, [(); 17])
        || fft_radix_p_unscaled::<_, _, _, I>(sequence, &mut temp, [(); 19])
        || fft_radix_p_unscaled::<_, _, _, I>(sequence, &mut temp, [(); 23])
        || fft_radix_p_unscaled::<_, _, _, I>(sequence, &mut temp, [(); 29])
        || fft_radix_p_unscaled::<_, _, _, I>(sequence, &mut temp, [(); 31])
        || fft_radix_p_unscaled::<_, _, _, I>(sequence, &mut temp, [(); 37])
        || fft_radix_p_unscaled::<_, _, _, I>(sequence, &mut temp, [(); 41])
        || fft_radix_p_unscaled::<_, _, _, I>(sequence, &mut temp, [(); 43])
        || fft_radix_p_unscaled::<_, _, _, I>(sequence, &mut temp, [(); 47])
        || fft_radix_p_unscaled::<_, _, _, I>(sequence, &mut temp, [(); 53])
        || fft_radix_p_unscaled::<_, _, _, I>(sequence, &mut temp, [(); 59])
        || fft_radix_p_unscaled::<_, _, _, I>(sequence, &mut temp, [(); 61])
        || fft_radix_p_unscaled::<_, _, _, I>(sequence, &mut temp, [(); 67])
        || fft_radix_p_unscaled::<_, _, _, I>(sequence, &mut temp, [(); 71])
        || fft_radix_p_unscaled::<_, _, _, I>(sequence, &mut temp, [(); 73])
        || fft_radix_p_unscaled::<_, _, _, I>(sequence, &mut temp, [(); 79])
        || fft_radix_p_unscaled::<_, _, _, I>(sequence, &mut temp, [(); 83])
        || fft_radix_p_unscaled::<_, _, _, I>(sequence, &mut temp, [(); 89])
        || fft_radix_p_unscaled::<_, _, _, I>(sequence, &mut temp, [(); 97])
        || fft_radix_n_sqrt_unscaled::<_, _, I>(sequence, &mut temp)
    )
    {
        dft_unscaled::<_, _, I>(sequence, &mut temp)
    }
}

pub fn partial_fft_unscaled<B, T, const I: bool, M>(sequence: &mut B, temp: &mut [Complex<T>], m: M) -> bool
where
    for<'a> &'a mut B: IntoBulk<Item: BorrowMut<Complex<T>>>,
    B: ?Sized,
    T: Float + 'static,
    M: LengthValue
{
    if length::value::eq(m, [(); 0])
    {
        return false
    }
    let mut i = 0;

    for (k, ()) in bulks::repeat_n((), m)
        .enumerate()
    {
        for x in sequence.bulk_mut()
            .skip(k)
            .step_by(m)
        {
            temp[i] = *x.borrow();
            i += 1;
        }
    }

    trait Recurse<T>
    where
        T: Float + 'static
    {
        fn buffer(&mut self) -> Option<&mut [Complex<T>]>;
    }
    impl<B, T> Recurse<T> for B
    where
        B: ?Sized,
        T: Float + 'static,
    {
        default fn buffer(&mut self) -> Option<&mut [Complex<T>]>
        {
            None
        }
    }
    impl<B, T> Recurse<T> for B
    where
        B: BorrowMut<[Complex<T>]> + ?Sized,
        T: Float + 'static,
    {
        fn buffer(&mut self) -> Option<&mut [Complex<T>]>
        {
            Some(self.borrow_mut())
        }
    }

    let len = sequence.bulk_mut().length();
    let n = length::value::div(len, length::value::max(m, [(); 1]));
    let r = length::value::rem(len, length::value::max(m, [(); 1]));

    let mut buffer = Recurse::<T>::buffer(sequence);
    temp!(buffer for len);

    let mut iter = temp.bulk_mut()
        .zip(buffer)
        .step_by(n)
        .into_iter();
    
    for (temp, bulk) in iter.by_ref()
        .take(length::value::len(m))
    {
        unsafe {
            fft_unscaled::<[_], T, I>(
                core::slice::from_raw_parts_mut(temp, length::value::len(n)),
                Some(core::slice::from_raw_parts_mut(bulk, length::value::len(n)))
            )
        }
    }
    if length::value::gt(r, [(); 0]) && let Some((temp, bulk)) = iter.next()
    {
        unsafe {
            fft_unscaled::<[_], T, I>(
                core::slice::from_raw_parts_mut(temp, length::value::len(r)),
                Some(core::slice::from_raw_parts_mut(bulk, length::value::len(r)))
            )
        }
    }
    true
}


pub fn fft_radix2_unscaled<T, B, const I: bool>(sequence: &mut B, temp: &mut Option<&mut [Complex<T>]>) -> bool
where
    for<'a> &'a mut B: IntoBulk<Item: BorrowMut<Complex<T>>>,
    B: ?Sized,
    T: Float + 'static
{
    let len = sequence.bulk_mut().length();
    if length::value::eq(length::value::rem(len, [(); 2]), [(); 0])
    {
        if util::is_power_of(len, [(); 2])
        {
            // In-place FFT

            sequence.bit_rev_permute();
            
            for s in 0..length::value::len(len).ilog2()
            {
                let m = 2usize << s;
                let wm = Complex::cis(<T as NumCast>::from(if I {TAU} else {-TAU}/m as f64).unwrap());
                for k in (0..length::value::len(len)).step_by(m)
                {
                    let mut w = Complex::<T>::one();
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
                        let q = w**x2.borrow();

                        *x1.borrow_mut() = p + q;
                        *x2.borrow_mut() = p - q;
                        w._mul_assign(wm);
                    }
                }
            }
            return true
        }
        // Recursive FFT

        temp!(temp for len);

        let ldiv = length::value::len(len)/2;
        partial_fft_unscaled::<_, _, I, _>(sequence, temp, 2);
        let mut x = temp.chunks(ldiv);
        let x = bulks::repeat_n_with(|| x.next().unwrap(), [(); 2])
            .collect_nearest();

        let wn = Complex::cis(<T as NumCast>::from(if I {TAU} else {-TAU}/length::value::len(len) as f64).unwrap());
        let mut wn_pk = Complex::one();
        for k in 0..ldiv
        {
            let p = x[0][k];
            let q = wn_pk*x[1][k];

            let [mut x1, mut x2] = sequence.bulk_mut()
                .skip(k)
                .step_by(ldiv)
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

pub fn fft_radix3_unscaled<T, B, const I: bool>(sequence: &mut B, temp: &mut Option<&mut [Complex<T>]>) -> bool
where
    for<'a> &'a mut B: IntoBulk<Item: BorrowMut<Complex<T>>>,
    B: ?Sized,
    T: Float + 'static
{
    let len = sequence.bulk_mut().length();

    const P: usize = 3;

    if length::value::eq(length::value::rem(len, [(); P]), [(); 0])
    {
        let w3 = Complex::cis(<T as NumCast>::from(if I {TAU} else {-TAU}/P as f64).unwrap());
        let w3_p2 = w3*w3;

        if util::is_power_of(len, [(); P])
        {
            // In-place FFT

            sequence.digit_rev_permute([(); P]);

            let mut m = P;
            for _s in 0..length::value::len(len).ilog(P)
            {
                let wm = Complex::cis(<T as NumCast>::from(if I {TAU} else {-TAU}/m as f64).unwrap());
                for k in (0..length::value::len(len)).step_by(m)
                {
                    let mut w = Complex::one();
                    for j in 0..m/P
                    {
                        let [mut x1, mut x2, mut x3] = sequence.bulk_mut()
                            .skip(k + j)
                            .step_by(m/P)
                            .map(Some)
                            .resize_with([(); _], || None)
                            .try_collect_array()
                            .unwrap();
                            
                        let p = *x1.borrow() + (*x2.borrow() + *x3.borrow()*w)*w;
                        let q = *x1.borrow() + (*x2.borrow()*w3 + *x3.borrow()*w3_p2*w)*w;
                        let r = *x1.borrow() + (*x2.borrow()*w3_p2 + *x3.borrow()*w3*w)*w;

                        *x1.borrow_mut() = p;
                        *x2.borrow_mut() = q;
                        *x3.borrow_mut() = r;
                        
                        w._mul_assign(wm);
                    }
                }
                m *= P
            }
            return true
        }
        // Recursive FFT

        temp!(temp for len);

        let ldiv = length::value::len(len)/P;
        partial_fft_unscaled::<_, _, I, _>(sequence, temp, P);
        let mut x = temp.chunks(ldiv);
        let x = bulks::repeat_n_with(|| x.next().unwrap(), [(); P])
            .collect_nearest();

        let wn = Complex::cis(<T as NumCast>::from(if I {TAU} else {-TAU}/length::value::len(len) as f64).unwrap()).into();
        let mut w = Complex::one();
        for k in 0..ldiv
        {
            let x1 = &x[0][k];
            let x2 = &x[1][k];
            let x3 = &x[2][k];
            
            let p = *x1 + (*x2 + *x3*w)*w;
            let q = *x1 + (*x2*w3 + *x3*w3_p2*w)*w;
            let r = *x1 + (*x2*w3_p2 + *x3*w3*w)*w;

            let [mut x1, mut x2, mut x3] = sequence.bulk_mut()
                .skip(k)
                .step_by(ldiv)
                .map(Some)
                .resize_with([(); _], || None)
                .try_collect_array()
                .unwrap();
            *x1.borrow_mut() = p;
            *x2.borrow_mut() = q;
            *x3.borrow_mut() = r;

            w._mul_assign(wn);
        }
        return true;
    }
    false
}

pub fn fft_radix5_unscaled<T, B, const I: bool>(sequence: &mut B, temp: &mut Option<&mut [Complex<T>]>) -> bool
where
    for<'a> &'a mut B: IntoBulk<Item: BorrowMut<Complex<T>>>,
    B: ?Sized,
    T: Float + 'static
{
    let len = sequence.bulk_mut().length();

    const P: usize = 5;

    if length::value::eq(length::value::rem(len, [(); P]), [(); 0])
    {
        let w5 = Complex::cis(<T as NumCast>::from(if I {TAU} else {-TAU}/P as f64).unwrap());
        let w5_p2 = w5*w5;
        let w5_p3 = w5_p2*w5;
        let w5_p4 = w5_p3*w5;

        if util::is_power_of(len, [(); P])
        {
            // In-place FFT

            sequence.digit_rev_permute([(); P]);

            let mut m = P;
            for _s in 0..length::value::len(len).ilog(P)
            {
                let wm = Complex::cis(<T as NumCast>::from(if I {TAU} else {-TAU}/m as f64).unwrap());
                for k in (0..length::value::len(len)).step_by(m)
                {
                    let mut w = Complex::one();
                    for j in 0..m/P
                    {
                        let [mut x1, mut x2, mut x3, mut x4, mut x5] = sequence.bulk_mut()
                            .skip(k + j)
                            .step_by(m/P)
                            .map(Some)
                            .resize_with([(); _], || None)
                            .try_collect_array()
                            .unwrap();

                        let p = *x1.borrow() + (*x2.borrow() + (*x3.borrow() + (*x4.borrow() + *x5.borrow()*w)*w)*w)*w;
                        let q = *x1.borrow() + (*x2.borrow()*w5 + (*x3.borrow()*w5_p2 + (*x4.borrow()*w5_p3 + *x5.borrow()*w5_p4*w)*w)*w)*w;
                        let r = *x1.borrow() + (*x2.borrow()*w5_p2 + (*x3.borrow()*w5_p4 + (*x4.borrow()*w5 + *x5.borrow()*w5_p3*w)*w)*w)*w;
                        let s = *x1.borrow() + (*x2.borrow()*w5_p3 + (*x3.borrow()*w5 + (*x4.borrow()*w5_p4 + *x5.borrow()*w5_p2*w)*w)*w)*w;
                        let t = *x1.borrow() + (*x2.borrow()*w5_p4 + (*x3.borrow()*w5_p3 + (*x4.borrow()*w5_p2 + *x5.borrow()*w5*w)*w)*w)*w;

                        *x1.borrow_mut() = p;
                        *x2.borrow_mut() = q;
                        *x3.borrow_mut() = r;
                        *x4.borrow_mut() = s;
                        *x5.borrow_mut() = t;
                        
                        w._mul_assign(wm);
                    }
                }
                m *= P
            }
            return true
        }
        // Recursive FFT

        temp!(temp for len);

        let ldiv = length::value::len(len)/P;
        partial_fft_unscaled::<_, _, I, _>(sequence, temp, P);
        let mut x = temp.chunks(ldiv);
        let x = bulks::repeat_n_with(|| x.next().unwrap(), [(); P])
            .collect_nearest();

        let wn = Complex::cis(<T as NumCast>::from(if I {TAU} else {-TAU}/length::value::len(len) as f64).unwrap());
        let mut w = Complex::one();
        for k in 0..ldiv
        {
            let x1 = &x[0][k];
            let x2 = &x[1][k];
            let x3 = &x[2][k];
            let x4 = &x[3][k];
            let x5 = &x[4][k];
                
            let p = *x1 + (*x2 + (*x3 + (*x4 + *x5*w)*w)*w)*w;
            let q = *x1 + (*x2*w5 + (*x3*w5_p2 + (*x4*w5_p3 + *x5*w5_p4*w)*w)*w)*w;
            let r = *x1 + (*x2*w5_p2 + (*x3*w5_p4 + (*x4*w5 + *x5*w5_p3*w)*w)*w)*w;
            let s = *x1 + (*x2*w5_p3 + (*x3*w5 + (*x4*w5_p4 + *x5*w5_p2*w)*w)*w)*w;
            let t = *x1 + (*x2*w5_p4 + (*x3*w5_p3 + (*x4*w5_p2 + *x5*w5*w)*w)*w)*w;

            let [mut x1, mut x2, mut x3, mut x4, mut x5] = sequence.bulk_mut()
                .skip(k)
                .step_by(ldiv)
                .map(Some)
                .resize_with([(); _], || None)
                .try_collect_array()
                .unwrap();
            
            *x1.borrow_mut() = p;
            *x2.borrow_mut() = q;
            *x3.borrow_mut() = r;
            *x4.borrow_mut() = s;
            *x5.borrow_mut() = t;

            w._mul_assign(wn);
        }
        return true;
    }
    false
}

pub fn fft_radix7_unscaled<T, B, const I: bool>(sequence: &mut B, temp: &mut Option<&mut [Complex<T>]>) -> bool
where
    for<'a> &'a mut B: IntoBulk<Item: BorrowMut<Complex<T>>>,
    B: ?Sized,
    T: Float + 'static
{
    let len = sequence.bulk_mut().length();

    const P: usize = 7;

    if length::value::eq(length::value::rem(len, [(); P]), [(); 0])
    {       
        let w7 = Complex::cis(<T as NumCast>::from(if I {TAU} else {-TAU}/P as f64).unwrap());
        let w7_p2 = w7*w7;
        let w7_p3 = w7_p2*w7;
        let w7_p4 = w7_p3*w7;
        let w7_p5 = w7_p4*w7;
        let w7_p6 = w7_p5*w7;

        if util::is_power_of(len, [(); P])
        {
            // In-place FFT

            sequence.digit_rev_permute([(); P]);

            let mut m = P;
            for _s in 0..length::value::len(len).ilog(P)
            {
                let wm = Complex::cis(<T as NumCast>::from(if I {TAU} else {-TAU}/m as f64).unwrap());
                for k in (0..length::value::len(len)).step_by(m)
                {
                    let mut w = Complex::one();
                    for j in 0..m/P
                    {
                        let [mut x1, mut x2, mut x3, mut x4, mut x5, mut x6, mut x7] = sequence.bulk_mut()
                            .skip(k + j)
                            .step_by(m/P)
                            .map(Some)
                            .resize_with([(); _], || None)
                            .try_collect_array()
                            .unwrap();

                        let p = *x1.borrow() + (*x2.borrow() + (*x3.borrow() + (*x4.borrow() + (*x5.borrow() + (*x6.borrow() + *x7.borrow()*w)*w)*w)*w)*w)*w;
                        let q = *x1.borrow() + (*x2.borrow()*w7 + (*x3.borrow()*w7_p2 + (*x4.borrow()*w7_p3 + (*x5.borrow()*w7_p4 + (*x6.borrow()*w7_p5 + *x7.borrow()*w7_p6*w)*w)*w)*w)*w)*w;
                        let r = *x1.borrow() + (*x2.borrow()*w7_p2 + (*x3.borrow()*w7_p4 + (*x4.borrow()*w7_p6 + (*x5.borrow()*w7 + (*x6.borrow()*w7_p3 + *x7.borrow()*w7_p5*w)*w)*w)*w)*w)*w;
                        let s = *x1.borrow() + (*x2.borrow()*w7_p3 + (*x3.borrow()*w7_p6 + (*x4.borrow()*w7_p2 + (*x5.borrow()*w7_p5 + (*x6.borrow()*w7 + *x7.borrow()*w7_p4*w)*w)*w)*w)*w)*w;
                        let t = *x1.borrow() + (*x2.borrow()*w7_p4 + (*x3.borrow()*w7 + (*x4.borrow()*w7_p5 + (*x5.borrow()*w7_p2 + (*x6.borrow()*w7_p6 + *x7.borrow()*w7_p3*w)*w)*w)*w)*w)*w;
                        let u = *x1.borrow() + (*x2.borrow()*w7_p5 + (*x3.borrow()*w7_p3 + (*x4.borrow()*w7 + (*x5.borrow()*w7_p6 + (*x6.borrow()*w7_p4 + *x7.borrow()*w7_p2*w)*w)*w)*w)*w)*w;
                        let v = *x1.borrow() + (*x2.borrow()*w7_p6 + (*x3.borrow()*w7_p5 + (*x4.borrow()*w7_p4 + (*x5.borrow()*w7_p3 + (*x6.borrow()*w7_p2 + *x7.borrow()*w7*w)*w)*w)*w)*w)*w;

                        *x1.borrow_mut() = p;
                        *x2.borrow_mut() = q;
                        *x3.borrow_mut() = r;
                        *x4.borrow_mut() = s;
                        *x5.borrow_mut() = t;
                        *x6.borrow_mut() = u;
                        *x7.borrow_mut() = v;
                        
                        w._mul_assign(wm);
                    }
                }
                m *= P
            }
            return true
        }
        // Recursive FFT

        temp!(temp for len);

        let ldiv = length::value::len(len)/P;
        partial_fft_unscaled::<_, _, I, _>(&mut *sequence, temp, P);
        let mut x = temp.chunks(ldiv);
        let x = bulks::repeat_n_with(|| x.next().unwrap(), [(); P])
            .collect_nearest();

        let wn = Complex::cis(<T as NumCast>::from(if I {TAU} else {-TAU}/length::value::len(len) as f64).unwrap());
        let mut w = Complex::one();
        for k in 0..ldiv
        {
            let x1 = &x[0][k];
            let x2 = &x[1][k];
            let x3 = &x[2][k];
            let x4 = &x[3][k];
            let x5 = &x[4][k];
            let x6 = &x[5][k];
            let x7 = &x[6][k];
                
            let p = *x1 + (*x2 + (*x3 + (*x4 + (*x5 + (*x6 + *x7*w)*w)*w)*w)*w)*w;
            let q = *x1 + (*x2*w7 + (*x3*w7_p2 + (*x4*w7_p3 + (*x5*w7_p4 + (*x6*w7_p5 + *x7*w7_p6*w)*w)*w)*w)*w)*w;
            let r = *x1 + (*x2*w7_p2 + (*x3*w7_p4 + (*x4*w7_p6 + (*x5*w7 + (*x6*w7_p3 + *x7*w7_p5*w)*w)*w)*w)*w)*w;
            let s = *x1 + (*x2*w7_p3 + (*x3*w7_p6 + (*x4*w7_p2 + (*x5*w7_p5 + (*x6*w7 + *x7*w7_p4*w)*w)*w)*w)*w)*w;
            let t = *x1 + (*x2*w7_p4 + (*x3*w7 + (*x4*w7_p5 + (*x5*w7_p2 + (*x6*w7_p6 + *x7*w7_p3*w)*w)*w)*w)*w)*w;
            let u = *x1 + (*x2*w7_p5 + (*x3*w7_p3 + (*x4*w7 + (*x5*w7_p6 + (*x6*w7_p4 + *x7*w7_p2*w)*w)*w)*w)*w)*w;
            let v = *x1 + (*x2*w7_p6 + (*x3*w7_p5 + (*x4*w7_p4 + (*x5*w7_p3 + (*x6*w7_p2 + *x7*w7*w)*w)*w)*w)*w)*w;

            let [mut x1, mut x2, mut x3, mut x4, mut x5, mut x6, mut x7]: [_; _] = sequence.bulk_mut()
                .skip(k)
                .step_by(ldiv)
                .map(Some)
                .resize_with([(); _], || None)
                .try_collect_array()
                .unwrap();

            *x1.borrow_mut() = p;
            *x2.borrow_mut() = q;
            *x3.borrow_mut() = r;
            *x4.borrow_mut() = s;
            *x5.borrow_mut() = t;
            *x6.borrow_mut() = u;
            *x7.borrow_mut() = v;

            w._mul_assign(wn);
        }
        return true;
    }
    false
}

pub fn fft_radix_p_unscaled<T, B, P, const I: bool>(sequence: &mut B, temp: &mut Option<&mut [Complex<T>]>, p: P) -> bool
where
    for<'a> &'a mut B: IntoBulk<Item: BorrowMut<Complex<T>>>,
    B: ?Sized,
    P: LengthValue,
    T: Float + 'static
{
    assert!(length::value::ne(p, [(); 0]));

    let len = sequence.bulk_mut().length();
    if length::value::eq(length::value::rem(len, length::value::max(p, [(); 1])), [(); 0])
    {
        let pf = length::value::len(p) as f64;
        let wp = bulks::repeat_n((), p)
            .enumerate()
            .map(|(i, ())| {
                if i == 0
                {
                    Complex::one()
                }
                else
                {
                    Complex::cis(<T as NumCast>::from(if I {TAU} else {-TAU}*i as f64/pf).unwrap()).into()
                }
            }).collect_nearest();
        let mut y = bulks::repeat_n(Complex::zero(), p)
            .collect_nearest();

        if util::is_power_of(len, p)
        {
            // In-place FFT

            sequence.digit_rev_permute(p);

            let mut x = bulks::repeat_n_with(|| core::ptr::null_mut::<Complex<T>>(), p)
                .collect_nearest();
            
            let mut m = length::value::len(p);
            for _s in 0..length::value::len(len).ilog(length::value::len(p))
            {
                let wm = Complex::cis(<T as NumCast>::from(
                    if I
                    {
                        TAU
                    }
                    else
                    {
                        -TAU
                    }/m as f64
                ).unwrap()).into();

                let mp = length::value::div(m, p);
                for (k, ()) in bulks::repeat_n((), len)
                    .enumerate()
                    .step_by(m)
                {
                    let mut w = Complex::one();
                    for (j, ()) in bulks::repeat_n((), mp)
                        .enumerate()
                    {
                        for (mut r, x) in sequence.bulk_mut()
                            .skip(k + j)
                            .step_by(mp)
                            .zip(x.borrow_mut() as &mut [*mut Complex<T>])
                        {
                            *x = r.borrow_mut() as *mut _
                        }

                        for (i, y) in y.borrow_mut()
                            .bulk_mut()
                            .enumerate()
                        {
                            *y.borrow_mut() = x.borrow()
                                .bulk()
                                .enumerate()
                                .map(|(j, x)| unsafe {**x}*wp[(j*i) % length::value::len(p)])
                                .fold(Complex::zero(), |y, z| {
                                    y*w + z
                                });
                        }
                        
                        for (x, y) in BorrowMut::<[*mut Complex<T>]>::borrow_mut(&mut x)
                            .bulk_mut()
                            .zip(Borrow::<[Complex<T>]>::borrow(&y))
                        {
                            unsafe {
                                **x = *y
                            }
                        }
                        
                        w._mul_assign(wm);
                    }
                }
                m *= length::value::len(p)
            }
            return true
        }
        // Recursive FFT

        temp!(temp for len);

        partial_fft_unscaled::<_, _, I, _>(sequence, temp, p);
        let m = length::value::div(len, length::value::max(p, [(); 1]));
        let x: Vec<_> = temp.chunks(length::value::len(m)).collect();

        let wn = Complex::cis(<T as NumCast>::from(
            if I
            {
                TAU
            }
            else
            {
                -TAU
            }/length::value::len(len) as f64
        ).unwrap()).into();

        let mut w = Complex::one();
        for k in 0..length::value::len(m)
        {
            for (i, y) in y.borrow_mut()
                .bulk_mut()
                .enumerate()
            {
                *y.borrow_mut() = x.iter()
                    .enumerate()
                    .map(|(j, x)| x[k]*wp[(j*i) % length::value::len(p)])
                    .fold(Complex::zero(), |y, z| {
                        z + y*w
                    });
            }

            for (mut x, y) in sequence.bulk_mut()
                .skip(k)
                .step_by(m)
                .zip(y.borrow())
            {
                *x.borrow_mut() = *y
            }
            
            w._mul_assign(wn);
        }
        return true;
    }
    false
}

pub fn fft_radix_n_sqrt_unscaled<B, T, const I: bool>(sequence: &mut B, temp: &mut Option<&mut [Complex<T>]>) -> bool
where
    for<'a> &'a mut B: IntoBulk<Item: BorrowMut<Complex<T>>>,
    B: ?Sized,
    T: Float + 'static
{
    let len = sequence.bulk_mut().length();
    let p = {
        util::radix(len)
    };
    if let Some(p) = p && length::value::ne(p, [(); 0])
    {
        return fft_radix_p_unscaled::<_, _, _, I>(sequence, temp, p)
    }
    false
}

pub fn dft_unscaled<B, T, const I: bool>(sequence: &mut B, temp: &mut Option<&mut [Complex<T>]>)
where
    for<'a> &'a mut B: IntoBulk<Item: BorrowMut<Complex<T>>>,
    B: ?Sized,
    T: Float + 'static
{
    let len = sequence.bulk_mut().length();

    temp!(temp for len);

    let wn = Complex::cis(<T as NumCast>::from(
        if I
        {
            TAU
        }
        else
        {
            -TAU
        }/length::value::len(len) as f64
    ).unwrap()).into();
    let mut wnk = Complex::one();

    
    sequence.bulk_mut()
        .zip(temp.borrow_mut())
        .for_each(|(mut src, dst)| { *dst = core::mem::replace(src.borrow_mut(), Zero::zero()); });
        
    sequence.bulk_mut()
        .for_each(|mut y| {
            let mut wnki = Complex::one();
            (*temp).bulk()
                .for_each(|x| {
                    y.borrow_mut()._add_assign(*x*wnki);
                    wnki._mul_assign(wnk);
                });
            wnk._mul_assign(wn);
        });
}