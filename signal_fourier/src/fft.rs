use core::borrow::{Borrow, BorrowMut};
use std::{f64::consts::TAU};

use array_trait::length::{self, LengthValue, Value};
use bulks::{AsBulk, Bulk, CollectNearest, InplaceBulk};
use num_complex::Complex;
use num_traits::{Float, NumCast, One, Zero};

use crate::{permute::Permute, temp, util::{self, MulAssignSpec, AddAssignSpec, LengthAsBulk}};

pub fn fft_unscaled<B, T, const I: bool>(bulk: &mut B, mut temp: Option<&mut [Complex<T>]>)
where
    B: InplaceBulk<ItemPointee = Complex<T>>,
    T: Float
{
    let len = bulk.len();
    if len <= 1
    {
        return;
    }
    if !(
        fft_radix2_unscaled::<_, _, I>(bulk, &mut temp)
        || fft_radix3_unscaled::<_, _, I>(bulk, &mut temp)
        || fft_radix5_unscaled::<_, _, I>(bulk, &mut temp)
        || fft_radix7_unscaled::<_, _, I>(bulk, &mut temp)
        || fft_radix_p_unscaled::<_, _, _, I>(bulk, &mut temp, [(); 11])
        || fft_radix_p_unscaled::<_, _, _, I>(bulk, &mut temp, [(); 13])
        || fft_radix_p_unscaled::<_, _, _, I>(bulk, &mut temp, [(); 17])
        || fft_radix_p_unscaled::<_, _, _, I>(bulk, &mut temp, [(); 19])
        || fft_radix_p_unscaled::<_, _, _, I>(bulk, &mut temp, [(); 23])
        || fft_radix_p_unscaled::<_, _, _, I>(bulk, &mut temp, [(); 29])
        || fft_radix_p_unscaled::<_, _, _, I>(bulk, &mut temp, [(); 31])
        || fft_radix_p_unscaled::<_, _, _, I>(bulk, &mut temp, [(); 37])
        || fft_radix_p_unscaled::<_, _, _, I>(bulk, &mut temp, [(); 41])
        || fft_radix_p_unscaled::<_, _, _, I>(bulk, &mut temp, [(); 43])
        || fft_radix_p_unscaled::<_, _, _, I>(bulk, &mut temp, [(); 47])
        || fft_radix_p_unscaled::<_, _, _, I>(bulk, &mut temp, [(); 53])
        || fft_radix_p_unscaled::<_, _, _, I>(bulk, &mut temp, [(); 59])
        || fft_radix_p_unscaled::<_, _, _, I>(bulk, &mut temp, [(); 61])
        || fft_radix_p_unscaled::<_, _, _, I>(bulk, &mut temp, [(); 67])
        || fft_radix_p_unscaled::<_, _, _, I>(bulk, &mut temp, [(); 71])
        || fft_radix_p_unscaled::<_, _, _, I>(bulk, &mut temp, [(); 73])
        || fft_radix_p_unscaled::<_, _, _, I>(bulk, &mut temp, [(); 79])
        || fft_radix_p_unscaled::<_, _, _, I>(bulk, &mut temp, [(); 83])
        || fft_radix_p_unscaled::<_, _, _, I>(bulk, &mut temp, [(); 89])
        || fft_radix_p_unscaled::<_, _, _, I>(bulk, &mut temp, [(); 97])
        || fft_radix_n_sqrt_unscaled::<_, _, I>(bulk, &mut temp)
    )
    {
        dft_unscaled::<_, _, I>(bulk, &mut temp)
    }
}

pub fn partial_fft_unscaled<B, T, const I: bool, M>(bulk: &mut B, temp: &mut [Complex<T>], m: M) -> bool
where
    B: InplaceBulk<ItemPointee = Complex<T>>,
    T: Float,
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
        for x in bulk.each_ref()
            .skip(k)
            .step_by(m)
        {
            temp[i] = *x;
            i += 1;
        }
    }

    let len = length::value::or_len::<Value<B::Length>>(bulk.len());
    let n = length::value::div(len, length::value::max(m, [(); 1]));
    let r = length::value::rem(len, length::value::max(m, [(); 1]));

    let mut iter = temp.bulk_mut()
        .zip(bulk.each_mut())
        .step_by(n)
        .into_iter();
    
    for (temp, bulk) in iter.by_ref()
        .take(length::value::len(m))
    {
        unsafe {
            fft_unscaled::<_, T, I>(
                &mut length::from_mut_ptr_len(temp, n).as_bulk_mut(),
                Some(core::slice::from_raw_parts_mut(bulk, length::value::len(n)))
            )
        }
    }
    if length::value::ne(r, [(); 0]) && let Some((temp, bulk)) = iter.next()
    {
        unsafe {
            fft_unscaled::<_, T, I>(
                &mut length::from_mut_ptr_len(temp, r).as_bulk_mut(),
                Some(core::slice::from_raw_parts_mut(bulk, length::value::len(n)))
            )
        }
    }
    true
}


pub fn fft_radix2_unscaled<T, B, const I: bool>(slice: &mut B, temp: &mut Option<&mut [Complex<T>]>) -> bool
where
    B: InplaceBulk<ItemPointee = Complex<T>>,
    T: Float
{
    let len = slice.length();
    if length::value::eq(length::value::rem(len, [(); 2]), [(); 0])
    {
        if util::is_power_of(len, [(); 2])
        {
            // In-place FFT

            slice.bit_rev_permute();
            
            for s in 0..length::value::len(len).ilog2()
            {
                let m = 2usize << s;
                let wm = Complex::cis(<T as NumCast>::from(if I {TAU} else {-TAU}/m as f64).unwrap());
                for k in (0..length::value::len(len)).step_by(m)
                {
                    let mut w = Complex::<T>::one();
                    for j in 0..m/2
                    {
                        let [x1, x2] = slice.get_many_mut([k + j, k + j + m/2]).map(Option::unwrap);

                        let p = *x1;
                        let q = w**x2;

                        *x1 = p + q;
                        *x2 = p - q;
                        w._mul_assign(wm);
                    }
                }
            }
            return true
        }
        // Recursive FFT

        temp!(temp for len);

        let ldiv = length::value::len(len)/2;
        partial_fft_unscaled::<_, _, I, _>(slice, temp, 2);
        let mut x = temp.chunks(ldiv);
        let x = bulks::repeat_n_with(|| x.next().unwrap(), [(); 2])
            .collect_nearest();

        let wn = Complex::cis(<T as NumCast>::from(if I {TAU} else {-TAU}/length::value::len(len) as f64).unwrap());
        let mut wn_pk = Complex::one();
        for k in 0..ldiv
        {
            let p = x[0][k];
            let q = wn_pk*x[1][k];

            let [x1, x2] = slice.get_many_mut([k, k + ldiv]).map(Option::unwrap);
            *x1 = p + q;
            *x2 = p - q;

            wn_pk._mul_assign(wn);
        }
        return true;
    }
    false
}

pub fn fft_radix3_unscaled<T, B, const I: bool>(slice: &mut B, temp: &mut Option<&mut [Complex<T>]>) -> bool
where
    B: InplaceBulk<ItemPointee = Complex<T>>,
    T: Float
{
    let len = slice.length();

    const P: usize = 3;

    if length::value::eq(length::value::rem(len, [(); P]), [(); 0])
    {
        let w3 = Complex::cis(<T as NumCast>::from(if I {TAU} else {-TAU}/P as f64).unwrap());
        let w3_p2 = w3*w3;

        if util::is_power_of(len, [(); P])
        {
            // In-place FFT

            slice.digit_rev_permute([(); P]);

            let mut m = P;
            for s in 0..length::value::len(len).ilog(P)
            {
                let wm = Complex::cis(<T as NumCast>::from(if I {TAU} else {-TAU}/m as f64).unwrap());
                for k in (0..length::value::len(len)).step_by(m)
                {
                    let mut w = Complex::one();
                    for j in 0..m/P
                    {
                        let [x1, x2, x3] = slice.get_many_mut([k + j, k + j + m/P, k + j + m/P*2]).map(Option::unwrap);
                            
                        let p = *x1 + (*x2 + *x3*w)*w;
                        let q = *x1 + (*x2*w3 + *x3*w3_p2*w)*w;
                        let r = *x1 + (*x2*w3_p2 + *x3*w3*w)*w;

                        *x1 = p;
                        *x2 = q;
                        *x3 = r;
                        
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
        partial_fft_unscaled::<_, _, I, _>(slice, temp, P);
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

            let [x1, x2, x3] = slice.get_many_mut([k, k + ldiv, k + ldiv*2]).map(Option::unwrap);
            *x1 = p;
            *x2 = q;
            *x3 = r;

            w._mul_assign(wn);
        }
        return true;
    }
    false
}

pub fn fft_radix5_unscaled<T, B, const I: bool>(slice: &mut B, temp: &mut Option<&mut [Complex<T>]>) -> bool
where
    B: InplaceBulk<ItemPointee = Complex<T>>,
    T: Float
{
    let len = slice.length();

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

            slice.digit_rev_permute([(); P]);

            let mut m = P;
            for s in 0..length::value::len(len).ilog(P)
            {
                let wm = Complex::cis(<T as NumCast>::from(if I {TAU} else {-TAU}/m as f64).unwrap());
                for k in (0..length::value::len(len)).step_by(m)
                {
                    let mut w = Complex::one();
                    for j in 0..m/P
                    {
                        let [x1, x2, x3, x4, x5] = slice.get_many_mut([k + j, k + j + m/P, k + j + m/P*2, k + j + m/P*3, k + j + m/P*4]).map(Option::unwrap);
                            
                        let p = *x1 + (*x2 + (*x3 + (*x4 + *x5*w)*w)*w)*w;
                        let q = *x1 + (*x2*w5 + (*x3*w5_p2 + (*x4*w5_p3 + *x5*w5_p4*w)*w)*w)*w;
                        let r = *x1 + (*x2*w5_p2 + (*x3*w5_p4 + (*x4*w5 + *x5*w5_p3*w)*w)*w)*w;
                        let s = *x1 + (*x2*w5_p3 + (*x3*w5 + (*x4*w5_p4 + *x5*w5_p2*w)*w)*w)*w;
                        let t = *x1 + (*x2*w5_p4 + (*x3*w5_p3 + (*x4*w5_p2 + *x5*w5*w)*w)*w)*w;

                        *x1 = p;
                        *x2 = q;
                        *x3 = r;
                        *x4 = s;
                        *x5 = t;
                        
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
        partial_fft_unscaled::<_, _, I, _>(slice, temp, P);
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

            let [x1, x2, x3, x4, x5] = slice.get_many_mut([k, k + ldiv, k + ldiv*2, k + ldiv*3, k + ldiv*4]).map(Option::unwrap);

            *x1 = p;
            *x2 = q;
            *x3 = r;
            *x4 = s;
            *x5 = t;

            w._mul_assign(wn);
        }
        return true;
    }
    false
}

pub fn fft_radix7_unscaled<T, B, const I: bool>(slice: &mut B, temp: &mut Option<&mut [Complex<T>]>) -> bool
where
    B: InplaceBulk<ItemPointee = Complex<T>>,
    T: Float
{
    let len = slice.length();

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

            slice.digit_rev_permute([(); P]);

            let mut m = P;
            for s in 0..length::value::len(len).ilog(P)
            {
                let wm = Complex::cis(<T as NumCast>::from(if I {TAU} else {-TAU}/m as f64).unwrap());
                for k in (0..length::value::len(len)).step_by(m)
                {
                    let mut w = Complex::one();
                    for j in 0..m/P
                    {
                        let [x1, x2, x3, x4, x5, x6, x7] = slice.get_many_mut([k + j, k + j + m/P, k + j + m/P*2, k + j + m/P*3, k + j + m/P*4, k + j + m/P*5, k + j + m/P*6]).map(Option::unwrap);
                            
                        let p = *x1 + (*x2 + (*x3 + (*x4 + (*x5 + (*x6 + *x7*w)*w)*w)*w)*w)*w;
                        let q = *x1 + (*x2*w7 + (*x3*w7_p2 + (*x4*w7_p3 + (*x5*w7_p4 + (*x6*w7_p5 + *x7*w7_p6*w)*w)*w)*w)*w)*w;
                        let r = *x1 + (*x2*w7_p2 + (*x3*w7_p4 + (*x4*w7_p6 + (*x5*w7 + (*x6*w7_p3 + *x7*w7_p5*w)*w)*w)*w)*w)*w;
                        let s = *x1 + (*x2*w7_p3 + (*x3*w7_p6 + (*x4*w7_p2 + (*x5*w7_p5 + (*x6*w7 + *x7*w7_p4*w)*w)*w)*w)*w)*w;
                        let t = *x1 + (*x2*w7_p4 + (*x3*w7 + (*x4*w7_p5 + (*x5*w7_p2 + (*x6*w7_p6 + *x7*w7_p3*w)*w)*w)*w)*w)*w;
                        let u = *x1 + (*x2*w7_p5 + (*x3*w7_p3 + (*x4*w7 + (*x5*w7_p6 + (*x6*w7_p4 + *x7*w7_p2*w)*w)*w)*w)*w)*w;
                        let v = *x1 + (*x2*w7_p6 + (*x3*w7_p5 + (*x4*w7_p4 + (*x5*w7_p3 + (*x6*w7_p2 + *x7*w7*w)*w)*w)*w)*w)*w;

                        *x1 = p;
                        *x2 = q;
                        *x3 = r;
                        *x4 = s;
                        *x5 = t;
                        *x6 = u;
                        *x7 = v;
                        
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
        partial_fft_unscaled::<_, _, I, _>(slice, temp, P);
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

            let [x1, x2, x3, x4, x5, x6, x7] = slice.get_many_mut([k, k + ldiv, k + ldiv*2, k + ldiv*3, k + ldiv*4, k + ldiv*5, k + ldiv*6]).map(Option::unwrap);

            *x1 = p;
            *x2 = q;
            *x3 = r;
            *x4 = s;
            *x5 = t;
            *x6 = u;
            *x7 = v;

            w._mul_assign(wn);
        }
        return true;
    }
    false
}

pub fn fft_radix_p_unscaled<T, B, P, const I: bool>(bulk: &mut B, temp: &mut Option<&mut [Complex<T>]>, p: P) -> bool
where
    B: InplaceBulk<ItemPointee = Complex<T>>,
    P: LengthValue,
    T: Float
{
    assert!(length::value::ne(p, [(); 0]));

    let len = bulk.length();
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

            bulk.digit_rev_permute(p);

            let mut x = bulks::repeat_n_with(|| core::ptr::null_mut::<Complex<T>>(), p)
                .collect_nearest();
            
            let mut m = length::value::len(p);
            for s in 0..length::value::len(len).ilog(length::value::len(p))
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

                        for (r, x) in bulk.each_mut()
                            .skip(k + j)
                            .step_by(mp)
                            .zip(x.borrow_mut())
                        {
                            *x = r as *mut _
                        }

                        for (i, y) in y.borrow_mut()
                            .bulk_mut()
                            .enumerate()
                        {
                            *y = x.borrow()
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

        partial_fft_unscaled::<_, _, I, _>(bulk, temp, p);
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
                *y = x.iter()
                    .enumerate()
                    .map(|(j, x)| x[k]*wp[(j*i) % length::value::len(p)])
                    .fold(Complex::zero(), |y, z| {
                        z + y*w
                    });
            }

            for (x, y) in bulk.each_mut()
                .skip(k)
                .step_by(m)
                .zip(y.borrow())
            {
                *x = *y
            }
            
            w._mul_assign(wn);
        }
        return true;
    }
    false
}

pub fn fft_radix_n_sqrt_unscaled<B, T, const I: bool>(bulk: &mut B, temp: &mut Option<&mut [Complex<T>]>) -> bool
where
    B: InplaceBulk<ItemPointee = Complex<T>>,
    T: Float
{
    let len = length::value::or_len::<Value<B::Length>>(bulk.len());
    let p = {
        util::radix(len)
    };
    if let Some(p) = p && length::value::ne(p, [(); 0])
    {
        return fft_radix_p_unscaled::<_, _, _, I>(bulk, temp, p)
    }
    false
}

pub fn dft_unscaled<B, T, const I: bool>(bulk: &mut B, temp: &mut Option<&mut [Complex<T>]>)
where
    B: InplaceBulk<ItemPointee = Complex<T>>,
    T: Float
{
    let len = bulk.length();

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

    
    bulk.each_mut()
        .zip(temp.borrow_mut())
        .for_each(|(src, dst)| { *dst = core::mem::replace(src, Zero::zero()); });
        
    bulk.each_mut()
        .for_each(|y| {
            let mut wnki = Complex::one();
            (*temp).bulk()
                .for_each(|x| {
                    y._add_assign(*x*wnki);
                    wnki._mul_assign(wnk);
                });
            wnk._mul_assign(wn);
        });
}

#[cfg(test)]
mod test
{
    use bulks::{Bulk, IntoBulk};
    use num_complex::Complex;

    use crate::FourierInplace;

    #[test]
    fn it_works()
    {
        let mut bulk = [1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11].into_bulk()
            .map(|x| Complex::from(x as f32))
            .collect_array()
            .into_bulk();

        bulk.dft_inplace();
        bulk.idft_inplace();

        let a = bulk.collect_array();

        println!("{a:?}")
    }
}