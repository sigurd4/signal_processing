use core::{borrow::BorrowMut, mem::MaybeUninit, ops::DerefMut};
use std::{f64::consts::TAU, iter::Sum, ops::{AddAssign, MulAssign}};

use array_trait::length::Nearest;
use bulks::{Bulk, IntoBulk};
use num_complex::{Complex, ComplexFloat};
use num_traits::{Float, NumCast, One, Zero};

pub fn fft_unscaled<T, const I: bool>(slice: &mut [T], mut temp: Option<&mut [T]>)
where
    T: ComplexFloat<Real: Float> + MulAssign + AddAssign + Sum,
    Complex<T::Real>: Into<T>
{
    let len = slice.len();
    if len <= 1
    {
        return;
    }
    if !(
        fft_radix2_unscaled::<_, I>(slice, &mut temp)
        || fft_radix3_unscaled::<_, I>(slice, &mut temp)
        || fft_radix5_unscaled::<_, I>(slice, &mut temp)
        || fft_radix7_unscaled::<_, I>(slice, &mut temp)
        || fft_radix_p_unscaled::<_, 11, I>(slice, &mut temp)
        || fft_radix_p_unscaled::<_, 13, I>(slice, &mut temp)
        || fft_radix_p_unscaled::<_, 17, I>(slice, &mut temp)
        || fft_radix_p_unscaled::<_, 19, I>(slice, &mut temp)
        || fft_radix_p_unscaled::<_, 23, I>(slice, &mut temp)
        || fft_radix_p_unscaled::<_, 29, I>(slice, &mut temp)
        || fft_radix_p_unscaled::<_, 31, I>(slice, &mut temp)
        || fft_radix_p_unscaled::<_, 37, I>(slice, &mut temp)
        || fft_radix_p_unscaled::<_, 41, I>(slice, &mut temp)
        || fft_radix_p_unscaled::<_, 43, I>(slice, &mut temp)
        || fft_radix_p_unscaled::<_, 47, I>(slice, &mut temp)
        || fft_radix_p_unscaled::<_, 53, I>(slice, &mut temp)
        || fft_radix_p_unscaled::<_, 59, I>(slice, &mut temp)
        || fft_radix_p_unscaled::<_, 61, I>(slice, &mut temp)
        || fft_radix_p_unscaled::<_, 67, I>(slice, &mut temp)
        || fft_radix_p_unscaled::<_, 71, I>(slice, &mut temp)
        || fft_radix_p_unscaled::<_, 73, I>(slice, &mut temp)
        || fft_radix_p_unscaled::<_, 79, I>(slice, &mut temp)
        || fft_radix_p_unscaled::<_, 83, I>(slice, &mut temp)
        || fft_radix_p_unscaled::<_, 89, I>(slice, &mut temp)
        || fft_radix_p_unscaled::<_, 97, I>(slice, &mut temp)
        || fft_radix_n_sqrt_unscaled::<_, I>(slice, &mut temp)
    )
    {
        dft_unscaled::<_, I>(slice, &mut temp)
    }
}

pub fn partial_fft_unscaled<T, const I: bool>(slice: &mut [T], temp: &mut [T], m: usize)
where
    T: ComplexFloat<Real: Float> + MulAssign + AddAssign + Sum,
    Complex<T::Real>: Into<T>
{
    let mut i = 0;
    let ind: Vec<_> = (0..m).map(|k| {
        let j = i;
        for x in slice[k..].iter()
            .step_by(m)
        {
            temp[i] = *x;
            i += 1;
        }
        j..i
    }).collect();
    for ind in ind
    {
        fft_unscaled::<_, I>(&mut temp[ind.clone()], Some(&mut slice[ind]))
    }
}

pub fn fft_radix2_unscaled<T, const I: bool>(slice: &mut [T], temp: &mut Option<&mut [T]>) -> bool
where
    T: ComplexFloat<Real: Float> + MulAssign + AddAssign + Sum,
    Complex<T::Real>: Into<T>
{
    let len = slice.len();
    if len.is_power_of_two()
    {
        // In-place FFT

        slice.bit_rev_permutation();
        
        for s in 0..len.ilog2()
        {
            let m = 2usize << s;
            let wm = Complex::cis(<T::Real as NumCast>::from(if I {TAU} else {-TAU}/m as f64).unwrap()).into();
            for k in (0..len).step_by(m)
            {
                let mut w = T::one();
                for j in 0..m/2
                {
                    let p = slice[k + j];
                    let q = w*slice[k + j + m/2];

                    slice[k + j] = p + q;
                    slice[k + j + m/2] = p - q;
                    w *= wm;
                }
            }
        }
        return true
    }
    if len % 2 == 0
    {
        // Recursive FFT

        let mut tempvec;
        let temp = if let Some(temp) = temp.take()
        {
            temp
        }
        else
        {
            tempvec = Some(vec![T::zero(); len]);
            tempvec.as_mut().unwrap()
        };

        partial_fft_unscaled::<_, I>(slice, temp, 2);
        let x: Vec<_> = temp.chunks(len/2).collect();

        let wn = Complex::cis(<T::Real as NumCast>::from(if I {TAU} else {-TAU}/len as f64).unwrap()).into();
        let mut wn_pk = T::one();
        for k in 0..len/2
        {
            let p = x[0][k];
            let q = wn_pk*x[1][k];

            slice[k] = p + q;
            slice[k + len/2] = p - q;

            wn_pk *= wn;
        }
        return true;
    }
    false
}

pub fn fft_radix3_unscaled<T, const I: bool>(slice: &mut [T], temp: &mut Option<&mut [T]>) -> bool
where
    T: ComplexFloat<Real: Float> + MulAssign + AddAssign + Sum,
    Complex<T::Real>: Into<T>
{
    let len = slice.len();

    const P: usize = 3;

    if len % P == 0
    {
        let w3 = Complex::cis(<T::Real as NumCast>::from(if I {TAU} else {-TAU}/P as f64).unwrap()).into();
        let w3_p2 = w3*w3;

        if is_power_of(len, P)
        {
            // In-place FFT

            slice.digit_rev_permutation(P);

            for s in 0..len.ilog(P)
            {
                let m = P.pow(s + 1);
                let wm = Complex::cis(<T::Real as NumCast>::from(if I {TAU} else {-TAU}/m as f64).unwrap()).into();
                for k in (0..len).step_by(m)
                {
                    let mut w = T::one();
                    for j in 0..m/P
                    {
                        unsafe {
                            let x1: &mut T = core::mem::transmute(&mut slice[k + j]);
                            let x2: &mut T = core::mem::transmute(&mut slice[k + j + m/P]);
                            let x3: &mut T = core::mem::transmute(&mut slice[k + j + m/P*2]);
                                
                            let p = *x1 + (*x2 + *x3*w)*w;
                            let q = *x1 + (*x2*w3 + *x3*w3_p2*w)*w;
                            let r = *x1 + (*x2*w3_p2 + *x3*w3*w)*w;

                            *x1 = p;
                            *x2 = q;
                            *x3 = r;
                        }
                        
                        w *= wm;
                    }
                }
            }
            return true
        }
        // Recursive FFT

        let mut tempvec;
        let temp = if let Some(temp) = temp.take()
        {
            temp
        }
        else
        {
            tempvec = Some(vec![T::zero(); len]);
            tempvec.as_mut().unwrap()
        };

        partial_fft_unscaled::<_, I>(slice, temp, P);
        let x: Vec<_> = temp.chunks(len/P).collect();

        let wn = Complex::cis(<T::Real as NumCast>::from(if I {TAU} else {-TAU}/len as f64).unwrap()).into();
        let mut w = T::one();
        for k in 0..len/P
        {
            let x1: &T = &x[0][k];
            let x2: &T = &x[1][k];
            let x3: &T = &x[2][k];
            
            let p = *x1 + (*x2 + *x3*w)*w;
            let q = *x1 + (*x2*w3 + *x3*w3_p2*w)*w;
            let r = *x1 + (*x2*w3_p2 + *x3*w3*w)*w;

            slice[k] = p;
            slice[k + len/P] = q;
            slice[k + len/P*2] = r;

            w *= wn;
        }
        return true;
    }
    false
}

pub fn fft_radix5_unscaled<T, const I: bool>(slice: &mut [T], temp: &mut Option<&mut [T]>) -> bool
where
    T: ComplexFloat<Real: Float> + MulAssign + AddAssign + Sum,
    Complex<T::Real>: Into<T>
{
    let len = slice.len();

    const P: usize = 5;

    if len % P == 0
    {
        if is_power_of(len, P)
        {
            // In-place FFT

            slice.digit_rev_permutation(P);
            
            let w5 = Complex::cis(<T::Real as NumCast>::from(if I {TAU} else {-TAU}/P as f64).unwrap()).into();
            let w5_p2 = w5*w5;
            let w5_p3 = w5_p2*w5;
            let w5_p4 = w5_p3*w5;

            for s in 0..len.ilog(P)
            {
                let m = P.pow(s + 1);
                let wm = Complex::cis(<T::Real as NumCast>::from(if I {TAU} else {-TAU}/m as f64).unwrap()).into();
                for k in (0..len).step_by(m)
                {
                    let mut w = T::one();
                    for j in 0..m/P
                    {
                        unsafe {
                            let x1: &mut T = core::mem::transmute(&mut slice[k + j]);
                            let x2: &mut T = core::mem::transmute(&mut slice[k + j + m/P]);
                            let x3: &mut T = core::mem::transmute(&mut slice[k + j + m/P*2]);
                            let x4: &mut T = core::mem::transmute(&mut slice[k + j + m/P*3]);
                            let x5: &mut T = core::mem::transmute(&mut slice[k + j + m/P*4]);

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
                        }
                        
                        w *= wm;
                    }
                }
            }
            return true
        }
        // Recursive FFT

        let mut tempvec;
        let temp = if let Some(temp) = temp.take()
        {
            temp
        }
        else
        {
            tempvec = Some(vec![T::zero(); len]);
            tempvec.as_mut().unwrap()
        };

        partial_fft_unscaled::<_, I>(slice, temp, P);
        let x: Vec<_> = temp.chunks(len/P).collect();

        let w5 = Complex::cis(<T::Real as NumCast>::from(if I {TAU} else {-TAU}/P as f64).unwrap()).into();
        let w5_p2 = w5*w5;
        let w5_p3 = w5_p2*w5;
        let w5_p4 = w5_p3*w5;
        let wn = Complex::cis(<T::Real as NumCast>::from(if I {TAU} else {-TAU}/len as f64).unwrap()).into();
        let mut w = T::one();
        for k in 0..len/P
        {
            let x1: &T = &x[0][k];
            let x2: &T = &x[1][k];
            let x3: &T = &x[2][k];
            let x4: &T = &x[3][k];
            let x5: &T = &x[4][k];
            
            let p = *x1 + (*x2 + (*x3 + (*x4 + *x5*w)*w)*w)*w;
            let q = *x1 + (*x2*w5 + (*x3*w5_p2 + (*x4*w5_p3 + *x5*w5_p4*w)*w)*w)*w;
            let r = *x1 + (*x2*w5_p2 + (*x3*w5_p4 + (*x4*w5 + *x5*w5_p3*w)*w)*w)*w;
            let s = *x1 + (*x2*w5_p3 + (*x3*w5 + (*x4*w5_p4 + *x5*w5_p2*w)*w)*w)*w;
            let t = *x1 + (*x2*w5_p4 + (*x3*w5_p3 + (*x4*w5_p2 + *x5*w5*w)*w)*w)*w;

            slice[k] = p;
            slice[k + len/P] = q;
            slice[k + len/P*2] = r;
            slice[k + len/P*3] = s;
            slice[k + len/P*4] = t;
            w *= wn;
        }
        return true;
    }
    false
}

pub fn fft_radix7_unscaled<T, const I: bool>(slice: &mut [T], temp: &mut Option<&mut [T]>) -> bool
where
    T: ComplexFloat<Real: Float> + MulAssign + AddAssign + Sum,
    Complex<T::Real>: Into<T>
{
    let len = slice.len();

    const P: usize = 7;

    if len % P == 0
    {
        if is_power_of(len, P)
        {
            // In-place FFT

            slice.digit_rev_permutation(P);
            
            let w7 = Complex::cis(<T::Real as NumCast>::from(if I {TAU} else {-TAU}/P as f64).unwrap()).into();
            let w7_p2 = w7*w7;
            let w7_p3 = w7_p2*w7;
            let w7_p4 = w7_p3*w7;
            let w7_p5 = w7_p4*w7;
            let w7_p6 = w7_p5*w7;

            for s in 0..len.ilog(P)
            {
                let m = P.pow(s + 1);
                let wm = Complex::cis(<T::Real as NumCast>::from(if I {TAU} else {-TAU}/m as f64).unwrap()).into();
                for k in (0..len).step_by(m)
                {
                    let mut w = T::one();
                    for j in 0..m/P
                    {
                        unsafe {
                            let x1: &mut T = core::mem::transmute(&mut slice[k + j]);
                            let x2: &mut T = core::mem::transmute(&mut slice[k + j + m/P]);
                            let x3: &mut T = core::mem::transmute(&mut slice[k + j + m/P*2]);
                            let x4: &mut T = core::mem::transmute(&mut slice[k + j + m/P*3]);
                            let x5: &mut T = core::mem::transmute(&mut slice[k + j + m/P*4]);
                            let x6: &mut T = core::mem::transmute(&mut slice[k + j + m/P*5]);
                            let x7: &mut T = core::mem::transmute(&mut slice[k + j + m/P*6]);
                            
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
                        }
                        
                        w *= wm;
                    }
                }
            }
            return true
        }
        // Recursive FFT

        let mut tempvec;
        let temp = if let Some(temp) = temp.take()
        {
            temp
        }
        else
        {
            tempvec = Some(vec![T::zero(); len]);
            tempvec.as_mut().unwrap()
        };

        partial_fft_unscaled::<_, I>(slice, temp, P);
        let x: Vec<_> = temp.chunks(len/P).collect();

        let w7 = Complex::cis(<T::Real as NumCast>::from(if I {TAU} else {-TAU}/P as f64).unwrap()).into();
        let w7_p2 = w7*w7;
        let w7_p3 = w7_p2*w7;
        let w7_p4 = w7_p3*w7;
        let w7_p5 = w7_p4*w7;
        let w7_p6 = w7_p5*w7;
        let wn = Complex::cis(<T::Real as NumCast>::from(if I {TAU} else {-TAU}/len as f64).unwrap()).into();
        let mut w = T::one();
        for k in 0..len/P
        {
            let x1: &T = &x[0][k];
            let x2: &T = &x[1][k];
            let x3: &T = &x[2][k];
            let x4: &T = &x[3][k];
            let x5: &T = &x[4][k];
            let x6: &T = &x[5][k];
            let x7: &T = &x[6][k];
            
            let p = *x1 + (*x2 + (*x3 + (*x4 + (*x5 + (*x6 + *x7*w)*w)*w)*w)*w)*w;
            let q = *x1 + (*x2*w7 + (*x3*w7_p2 + (*x4*w7_p3 + (*x5*w7_p4 + (*x6*w7_p5 + *x7*w7_p6*w)*w)*w)*w)*w)*w;
            let r = *x1 + (*x2*w7_p2 + (*x3*w7_p4 + (*x4*w7_p6 + (*x5*w7 + (*x6*w7_p3 + *x7*w7_p5*w)*w)*w)*w)*w)*w;
            let s = *x1 + (*x2*w7_p3 + (*x3*w7_p6 + (*x4*w7_p2 + (*x5*w7_p5 + (*x6*w7 + *x7*w7_p4*w)*w)*w)*w)*w)*w;
            let t = *x1 + (*x2*w7_p4 + (*x3*w7 + (*x4*w7_p5 + (*x5*w7_p2 + (*x6*w7_p6 + *x7*w7_p3*w)*w)*w)*w)*w)*w;
            let u = *x1 + (*x2*w7_p5 + (*x3*w7_p3 + (*x4*w7 + (*x5*w7_p6 + (*x6*w7_p4 + *x7*w7_p2*w)*w)*w)*w)*w)*w;
            let v = *x1 + (*x2*w7_p6 + (*x3*w7_p5 + (*x4*w7_p4 + (*x5*w7_p3 + (*x6*w7_p2 + *x7*w7*w)*w)*w)*w)*w)*w;

            slice[k] = p;
            slice[k + len/P] = q;
            slice[k + len/P*2] = r;
            slice[k + len/P*3] = s;
            slice[k + len/P*4] = t;
            slice[k + len/P*5] = u;
            slice[k + len/P*6] = v;

            w *= wn;
        }
        return true;
    }
    false
}

pub fn fft_radix_p_unscaled<T, const P: usize, const I: bool>(slice: &mut [T], temp: &mut Option<&mut [T]>) -> bool
where
    T: ComplexFloat<Real: Float> + MulAssign + AddAssign + Sum,
    Complex<T::Real>: Into<T>,
    [(); P - 1]:
{
    let len = slice.len();

    if len % P == 0
    {
        if is_power_of(len, P)
        {
            // In-place FFT

            slice.digit_rev_permutation(P);
            
            let wp: [_; P] = {
                core::array::from_fn(|i| {
                    if i == 0
                    {
                        One::one()
                    }
                    else
                    {
                        Complex::cis(<T::Real as NumCast>::from(if I {TAU} else {-TAU}*i as f64/P as f64).unwrap()).into()
                    }
                })
            };

            let mut x: [&mut T; P] = unsafe {MaybeUninit::uninit().assume_init()};
            let mut y: [T; P] = [T::zero(); P];
            
            for s in 0..len.ilog(P)
            {
                let m = P.pow(s + 1);
                let wm = Complex::cis(<T::Real as NumCast>::from(if I {TAU} else {-TAU}/m as f64).unwrap()).into();
                for k in (0..len).step_by(m)
                {
                    let mut w = T::one();
                    for j in 0..m/P
                    {
                        for i in 0..P
                        {
                            x[i] = unsafe {
                                core::mem::transmute(&mut slice[k + j + m/P*i])
                            }
                        }

                        for i in 0..P
                        {
                            y[i] = T::zero();
                            for j in (1..P).rev()
                            {
                                y[i] += *x[j]*wp[(j*i) % P];
                                y[i] *= w;
                            }
                            y[i] += *x[0];
                        }
                        
                        for i in 0..P
                        {
                            *x[i] = y[i]
                        }
                        
                        w *= wm;
                    }
                }
            }
            return true
        }
        // Recursive FFT
        
        let mut tempvec;
        let temp = if let Some(temp) = temp.take()
        {
            temp
        }
        else
        {
            tempvec = Some(vec![T::zero(); len]);
            tempvec.as_mut().unwrap()
        };

        partial_fft_unscaled::<_, I>(slice, temp, P);
        let x: Vec<_> = temp.chunks(len/P).collect();

        let wp: [_; P] = {
            core::array::from_fn(|i| {
                if i == 0
                {
                    One::one()
                }
                else
                {
                    Complex::cis(<T::Real as NumCast>::from(if I {TAU} else {-TAU}*i as f64/P as f64).unwrap()).into()
                }
            })
        };
        let mut y: [T; P] = [T::zero(); P];

        let wn = Complex::cis(<T::Real as NumCast>::from(if I {TAU} else {-TAU}/len as f64).unwrap()).into();
        let mut w = T::one();
        let m = len/P;
        for k in 0..m
        {
            for i in 0..P
            {
                y[i] = T::zero();
                for j in (1..P).rev()
                {
                    y[i] += x[j][k]*wp[(j*i) % P];
                    y[i] *= w;
                }
                y[i] += x[0][k];
            }
            
            for i in 0..P
            {
                slice[k + m*i] = y[i]
            }
            
            w *= wn;
        }
        return true;
    }
    false
}

pub fn fft_radix_n_sqrt_unscaled<T, const I: bool>(slice: &mut [T], temp: &mut Option<&mut [T]>) -> bool
where
    T: ComplexFloat<Real: Float> + MulAssign + AddAssign + Sum,
    Complex<T::Real>: Into<T>
{
    let len = slice.len();
    let p = {
        util::closest_mod0_of(1 << ((len.ilog2() + 1) / 2), len)
    };
    if let Some(p) = p
    {
        assert!(len % p == 0);
        let wp: Vec<_> = (0..p).map(|i| {
                if i == 0
                {
                    One::one()
                }
                else
                {
                    Complex::cis(<T::Real as NumCast>::from(if I {TAU} else {-TAU}*i as f64/p as f64).unwrap()).into()
                }
            }).collect();
        let mut y = vec![T::zero(); p];

        if is_power_of(len, p)
        {
            // In-place FFT

            slice.digit_rev_permutation(p);

            let mut x: Box<[&mut T]> = unsafe {Box::new_uninit_slice(p).assume_init()};
            
            for s in 0..len.ilog(p)
            {
                let m = p.pow(s + 1);
                let wm = Complex::cis(<T::Real as NumCast>::from(if I {TAU} else {-TAU}/m as f64).unwrap()).into();
                for k in (0..len).step_by(m)
                {
                    let mut w = T::one();
                    for j in 0..m/p
                    {
                        for i in 0..p
                        {
                            x[i] = unsafe {
                                core::mem::transmute(&mut slice[k + j + m/p*i])
                            }
                        }

                        for i in 0..p
                        {
                            y[i] = T::zero();
                            for j in (1..p).rev()
                            {
                                y[i] += *x[j]*wp[(j*i) % p];
                                y[i] *= w;
                            }
                            y[i] += *x[0];
                        }
                        
                        for i in 0..p
                        {
                            *x[i] = y[i]
                        }
                        
                        w *= wm;
                    }
                }
            }
            return true
        }
        // Recursive FFT

        let mut tempvec;
        let temp = if let Some(temp) = temp.take()
        {
            temp
        }
        else
        {
            tempvec = Some(vec![T::zero(); len]);
            tempvec.as_mut().unwrap()
        };

        partial_fft_unscaled::<_, I>(slice, temp, p);
        let x: Vec<_> = temp.chunks(len/p).collect();

        let wn = Complex::cis(<T::Real as NumCast>::from(if I {TAU} else {-TAU}/len as f64).unwrap()).into();
        let mut w = T::one();
        let m = len/p;
        for k in 0..m
        {
            for i in 0..p
            {
                y[i] = T::zero();
                for j in (1..p).rev()
                {
                    y[i] += x[j][k]*wp[(j*i) % p];
                    y[i] *= w;
                }
                y[i] += x[0][k];
            }
            
            for i in 0..p
            {
                slice[k + m*i] = y[i]
            }
            
            w *= wn;
        }
        return true;
    }
    false
}

pub fn dft_unscaled<T, const I: bool>(slice: &mut [T], temp: &mut Option<&mut [T]>)
where
    T: ComplexFloat<Real: Float> + MulAssign + AddAssign,
    Complex<T::Real>: Into<T>
{
    let len = slice.len();

    let mut tempvec;
    let temp = if let Some(temp) = temp.take()
    {
        temp
    }
    else
    {
        tempvec = Some(vec![T::zero(); len]);
        tempvec.as_mut().unwrap()
    };

    let wn = Complex::cis(<T::Real as NumCast>::from(if I {TAU} else {-TAU}/len as f64).unwrap()).into();
    let mut wnk = T::one();

    unsafe {
        std::ptr::swap_nonoverlapping(temp.as_mut_ptr(), slice.as_mut_ptr(), len);
    }
    for k in 0..len
    {
        let mut wnki = T::one();
        slice[k] = Zero::zero();
        for i in 0..len
        {
            slice[k] += temp[i]*wnki;
            wnki *= wnk;
        }

        wnk *= wn;
    }
}