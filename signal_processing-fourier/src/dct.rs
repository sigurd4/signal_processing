use core::{borrow::{Borrow, BorrowMut}, ops::{Div, Mul}};

use array_trait::length;
use bulks::{AsBulk, Bulk, CollectNearest, DoubleEndedBulk, IntoBulk};
use num_complex::{Complex, ComplexFloat};
use num_traits::{Float, FloatConst, NumCast, One, Zero};
use crate::{Dft, Permute, SpectrumScaling, util::TruncateIm};

/// # Discrete cosine-transform
/// 
/// The discrete cosine-transform is the real-valued fourier transform of the even extension of a sequence.
/// 
/// There are four types of DCTs:
/// 
/// ## DCT I
/// 
/// ```txt
///                                        N - 2
///                                         ___
///                                         \
/// X[k] = 0.5*(x[0] + (-1)^k * x[N - 1]) +  \  x[n]*cos(π*n*k/(N - 1))
///                                          /
///                                         /__
///                                        n = 1
/// ```
/// 
/// The DCT IV is its inverse (assuming balanced scaling).
/// 
/// ## DCT II
/// 
/// ```txt
///       N - 1
///        ___
///        \
/// X[k] =  \  x[n]*cos(π*(n + 0.5)*k/(N - 1))
///         /
///        /__
///       n = 0
/// ```
/// 
/// The DCT III is its inverse (assuming balanced scaling).
/// 
/// ## DCT III
/// 
/// ```txt
///                  N - 1
///                   ___
///                   \
/// X[k] = 0.5*x[0] +  \  x[n]*cos(π*k*(k + 0.5)/N)
///                    /
///                   /__
///                  n = 1
/// ```
/// 
/// The DCT II is its inverse (assuming balanced scaling).
/// 
/// ## DCT IV
/// 
/// ```txt
///       N - 1
///        ___
///        \
/// X[k] =  \  x[n]*cos(π*(n + 0.5)*(k + 0.5)/N)
///         /
///        /__
///       n = 0
/// ```
/// 
/// The DCT I is its inverse (assuming balanced scaling).
pub trait Dct<T>: Permute<T>
where
    T: ComplexFloat
{
    #[doc(alias = "idct_i")]
    fn dct_i(&mut self);
    #[doc(alias = "idct_iii")]
    fn dct_ii(&mut self);
    #[doc(alias = "idct_ii")]
    fn dct_iii(&mut self);
    #[doc(alias = "idct_iv")]
    fn dct_iv(&mut self);
}
impl<B, T> Dct<T> for B
where
    for<'a> &'a mut B: IntoBulk<Item: BorrowMut<T>>,
    for<'a> &'a B: IntoBulk<Item: Borrow<T>, IntoBulk: DoubleEndedBulk>,
    B: ?Sized,
    T: ComplexFloat + 'static
{
    fn dct_i(&mut self)
    {
        let len = (*self).bulk().length();
        if length::value::le(len, [(); 1])
        {
            return
        }
        let len_m1 = length::value::saturating_sub(len, [(); 1]);
        let lenf = <<T as ComplexFloat>::Real as NumCast>::from(length::value::len(len)).unwrap();

        let one = <T as ComplexFloat>::Real::one();
        let two = one + one;
        let sqrt_2 = <T as ComplexFloat>::Real::SQRT_2();
        let frac_pi_2 = <T as ComplexFloat>::Real::FRAC_PI_2();

        let mut y: Vec<_> = (*self).bulk()
            .chain((*self).bulk().skip([(); 1]).rev().skip([(); 1]))
            .map(|x| Complex { re: x.borrow().re(), im: x.borrow().im() })
            .collect();
        for y in y.bulk_mut().step_by(len_m1)
        {
            *y = *y*sqrt_2
        }
        for y in y.bulk_mut()
        {
            *y = *y/two
        }
        y.dft();
        for y in y.bulk_mut().step_by(len_m1)
        {
            *y = *y*sqrt_2
        }
        let (y1, y2) = y.bulk_mut()
            .split_at(len);
        for (y1, y2) in y1.skip([(); 1])
            .zip(y2.rev())
        {
            *y1 = *y1 + *y2
        }

        for (y, mut x) in y.into_bulk()
            .zip(self.bulk_mut())
        {
            *x.borrow_mut() = <T as TruncateIm>::truncate_im(y)
        }
    }
    fn dct_ii(&mut self)
    {
        let len = (*self).bulk().length();
        if length::value::le(len, [(); 1])
        {
            return
        }
        let lenf = <<T as ComplexFloat>::Real as NumCast>::from(length::value::len(len)).unwrap();

        let frac_1_sqrt_2 = <T as ComplexFloat>::Real::FRAC_1_SQRT_2();
        let frac_pi_2 = <T as ComplexFloat>::Real::FRAC_PI_2();
        let one = T::Real::one();
        let two = one + one;
        let half = Float::recip(two);

        let m1 = bulks::once(One::one())
            .chain(bulks::range([(); 1], len)
                .map(|i| {
                    let i = <<T as ComplexFloat>::Real as NumCast>::from(i).unwrap();
                    Complex::from_polar(half, -i*frac_pi_2/lenf)
                })
            );
        let m2 = bulks::range([(); 1], len)
            .map(|i| {
                let i = <<T as ComplexFloat>::Real as NumCast>::from(i).unwrap();
                Complex::from_polar(half, i*frac_pi_2/lenf)
            });

        let mut y = (*self).bulk()
            .chain((*self).bulk().rev())
            .map(|x| Complex { re: x.borrow().re(), im: x.borrow().im() })
            .collect::<Vec<_>, _>();
        y.dft_scaled(SpectrumScaling::Balanced);
        if let Some(y) = y.first_mut()
        {
            *y = *y/T::Real::SQRT_2()
        }
        
        let (y1, y2) = y.into_bulk().split_at(len);

        for ((y1, y2), mut x) in y1.into_iter()
            .zip(m1)
            .map(|(y, m1)| y*m1)
            .zip(bulks::once(Zero::zero())
                .chain(y2.rev()
                    .zip(m2)
                    .map(|(y, m2)| y*m2)
                )
            ).zip(self.bulk_mut())
        {
            let y = y1 + y2;
            *x.borrow_mut() = <T as TruncateIm>::truncate_im(y)
        }
    }
    fn dct_iii(&mut self)
    {
        let len = (*self).bulk().length();
        if length::value::le(len, [(); 1])
        {
            return
        }
        let lenf = <<T as ComplexFloat>::Real as NumCast>::from(length::value::len(len)).unwrap();

        let frac_1_sqrt_2 = <T as ComplexFloat>::Real::FRAC_1_SQRT_2();
        let frac_pi_2 = <T as ComplexFloat>::Real::FRAC_PI_2();

        let m1 = bulks::once(One::one())
            .chain(bulks::range([(); 1], len)
                .map(|i| {
                    let i = <<T as ComplexFloat>::Real as NumCast>::from(i).unwrap();
                    Complex::from_polar(frac_1_sqrt_2, -i*frac_pi_2/lenf)
                })
            );
        let m2 = bulks::range([(); 1], len)
            .map(|i| {
                let i = <<T as ComplexFloat>::Real as NumCast>::from(i).unwrap();
                Complex::from_polar(frac_1_sqrt_2, i*frac_pi_2/lenf)
            })
            .rev();
        
        let mut y = (*self).bulk()
            .map(|x| Complex { re: x.borrow().re(), im: x.borrow().im() })
            .zip(m1)
            .map(|(x, m1)| m1*x)
            .chain(bulks::once(Zero::zero()))
            .chain((*self).bulk()
                .skip([(); 1])
                .rev()
                .map(|x| Complex { re: x.borrow().re(), im: x.borrow().im() })
                .zip(m2)
                .map(|(x, m2)| m2*x)
            ).collect::<Vec<_>, _>();
        y.dft_scaled(SpectrumScaling::Balanced);
        
        let ydiv = T::Real::FRAC_1_SQRT_2();
        for (mut y, mut x) in y.into_bulk()
            .zip(self.bulk_mut())
        {
            y = y/ydiv;
            *x.borrow_mut() = <_ as TruncateIm>::truncate_im(y)
        }

    }
    fn dct_iv(&mut self)
    {
        let len = (*self).bulk().length();
        if length::value::le(len, [(); 1])
        {
            return
        }
        let lenf = <<T as ComplexFloat>::Real as NumCast>::from(length::value::len(len)).unwrap();

        let sqrt_2 = <T as ComplexFloat>::Real::SQRT_2();
        let frac_1_sqrt_2 = <T as ComplexFloat>::Real::FRAC_1_SQRT_2();
        let frac_pi_2 = <T as ComplexFloat>::Real::FRAC_PI_2();

        let w1: Vec<_> = bulks::range([(); 0], len)
            .map(|i| {
                let i = <<T as ComplexFloat>::Real as NumCast>::from(i).unwrap();
                Complex::from_polar(frac_1_sqrt_2, frac_pi_2/lenf*i)
            }).collect();
        let w2: Vec<_> = bulks::range_inclusive([(); 1], len)
            .map(|i| {
                let i = <<T as ComplexFloat>::Real as NumCast>::from(i).unwrap();
                Complex::from_polar(frac_1_sqrt_2, -frac_pi_2/lenf*i)
            }).collect();

        let y1 = (*self).bulk()
            .map(|x| Complex { re: x.borrow().re(), im: x.borrow().im() })
            .zip(w1.bulk().copied())
            .map(|(x, w)| x*w);
        let y2 = (*self).bulk()
            .rev()
            .map(|x| Complex { re: x.borrow().re(), im: x.borrow().im() })
            .zip(w2.bulk().rev().copied())
            .map(|(x, w)| x*w);
        
        let mut y: Vec<_> = y1.into_bulk()
            .chain(y2)
            .collect();
        y.idft_scaled(SpectrumScaling::Balanced);
        
        let ymul = Complex::cis(T::Real::FRAC_PI_4()/lenf);
        for y in y.iter_mut()
        {
            *y = *y*ymul
        }
        
        let (y1, y2) = y.into_bulk().split_at(len);
        
        for (y, mut x) in y1.into_iter()
            .zip(w1)
            .map(|(y, m)| y*m)
            .zip(y2.rev()
                .zip(w2)
                .map(|(y, m)| y*m)
            ).map(|(y1, y2)| y1 + y2)
            .zip(self.bulk_mut())
        {
            *x.borrow_mut() = <T as TruncateIm>::truncate_im(y)
        }
    }
}

#[cfg(test)]
mod test
{
    use core::{borrow::{Borrow, BorrowMut}, f64::consts::{PI, SQRT_2, TAU}, ops::{Div, Mul}};

    use array_trait::length;
use bulks::{AsBulk, Bulk, IntoBulk};
    use linspace::Linspace;
use num_complex::ComplexFloat;
use num_traits::{Float, FloatConst, NumCast, One};

    use crate::{Dct, Dst, tests};

    #[test]
    fn plot_dct()
    {
        const N: usize = 1024;
        const T: f64 = 1.0;
        const F: f64 = 220.0;
        
        let x: [_; N] = core::array::from_fn(|i| (TAU*F*i as f64/N as f64*T).sin());

        let w = (0.0..TAU).linspace_array::<N>();
        let xf = [
            { let mut y = x; y.dct_i(); y },
            { let mut y = x; y.dct_ii(); y },
            { let mut y = x; y.dct_iii(); y },
            { let mut y = x; y.dct_iv(); y }
        ];

        ezplot::plot_curves("X(e^jw)", "plots/x_z_dct.png", xf.map(|xf| w.into_bulk().zip(xf)))
            .unwrap()
    }

    #[test]
    fn identities()
    {
        let a = [1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11]
            .into_bulk()
            .map(|x| x as f32)
            .collect_array();

        let mut b = a;
        b.dct_i();
        b.dct_i();

        assert!(tests::approx_eq(&a, &b, 1e-5));

        let mut b = a;
        b.dct_iii();
        b.dct_ii();

        assert!(tests::approx_eq(&a, &b, 1e-5));

        let mut b = a;
        b.dct_ii();
        b.dct_iii();

        assert!(tests::approx_eq(&a, &b, 1e-5));

        let mut b = a;
        b.dct_iv();
        b.dct_iv();

        assert!(tests::approx_eq(&a, &b, 1e-5));
    }

    #[test]
    fn test_dct_i()
    {
        let mut a = [1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11]
            .into_bulk()
            .map(|x| x as f64)
            .collect_array();

        let mut b = a;
        b.dct_i();

        fn dct_i_direct(x: &mut [f64])
        {
            let l = x.len();
            if let Some(x_first) = x.first().copied()
                && let Some(x_last) = x.last().copied()
            {
                let first_term = [
                    (x_first + x_last)/SQRT_2,
                    (x_first - x_last)/SQRT_2
                ];
                let mut y: Vec<_> = (0..l)
                    .map(|k| (
                        first_term[k % 2]
                        + x[..l - 1].iter()
                            .enumerate()
                            .skip(1)
                            .map(|(n, xn)| xn*(PI*k as f64*n as f64/(l - 1) as f64).cos())
                            .sum::<f64>()
                        )*(2.0/(l - 1) as f64).sqrt()
                    ).collect();
                y[0] /= SQRT_2;
                y[l - 1] /= SQRT_2;
                
                for (x, y) in x.iter_mut()
                    .zip(y)
                {
                    *x = y
                }
            }
        }
        dct_i_direct(&mut a);

        println!("{b:?}");
        println!("{a:?}");
        assert!(tests::approx_eq(&a, &b, 1e-5))
    }

    #[test]
    fn test_dct_ii()
    {
        let mut a = [1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11]
            .into_bulk()
            .map(|x| x as f64)
            .collect_array();

        let mut b = a;
        b.dct_ii();

        fn dct_ii_direct(x: &mut [f64])
        {
            let l = x.len();
            let mut y: Vec<_> = (0..l)
                .map(|k| (
                    x.iter()
                        .enumerate()
                        .map(|(n, xn)| xn*(PI*k as f64*(n as f64 + 0.5)/l as f64).cos())
                        .sum::<f64>()
                    )*(2.0/l as f64).sqrt()
                ).collect();
            y[0] /= SQRT_2;
            
            for (x, y) in x.iter_mut()
                .zip(y)
            {
                *x = y
            }
        }
        dct_ii_direct(&mut a);

        println!("{b:?}");
        println!("{a:?}");
        assert!(tests::approx_eq(&a, &b, 1e-5))
    }

    #[test]
    fn test_dct_iii()
    {
        let mut a = [1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11]
            .into_bulk()
            .map(|x| x as f64)
            .collect_array();

        let mut b = a;
        b.dct_iii();

        fn dct_iii_direct(x: &mut [f64])
        {
            let l = x.len();
            let y: Vec<_> = (0..l)
                .map(|k| (
                    x[0]/SQRT_2 + x.iter()
                        .enumerate()
                        .skip(1)
                        .map(|(n, xn)| xn*(PI*n as f64*(k as f64 + 0.5)/l as f64).cos())
                        .sum::<f64>()
                    )*(2.0/l as f64).sqrt()
                ).collect();
            
            for (x, y) in x.iter_mut()
                .zip(y)
            {
                *x = y
            }
        }
        dct_iii_direct(&mut a);

        println!("{b:?}");
        println!("{a:?}");
        assert!(tests::approx_eq(&a, &b, 1e-5))
    }

    #[test]
    fn test_dct_iv()
    {
        let mut a = [1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11]
            .into_bulk()
            .map(|x| x as f64)
            .collect_array();

        let mut b = a;
        b.dct_iv();

        fn dct_iv_direct(x: &mut [f64])
        {
            let l = x.len();
            let y: Vec<_> = (0..l)
                .map(|k| (
                    x.iter()
                        .enumerate()
                        .map(|(n, xn)| xn*(PI*(n as f64 + 0.5)*(k as f64 + 0.5)/l as f64).cos())
                        .sum::<f64>()
                    )*(2.0/l as f64).sqrt()
                ).collect();
            
            for (x, y) in x.iter_mut()
                .zip(y)
            {
                *x = y
            }
        }
        dct_iv_direct(&mut a);

        println!("{b:?}");
        println!("{a:?}");
        assert!(tests::approx_eq(&a, &b, 1e-5))
    }

    #[test]
    fn from_dst_ii()
    {
        let a = [1, 2, 3, 4, 5]
            .into_bulk()
            .map(|x| x as f32)
            .collect_array();

        let mut c = a;
        c.dct_ii();

        let mut s = a;
        s.bulk_mut()
            .skip(1)
            .step_by(2)
            .for_each(|x| *x = -*x);
        s.dst_ii();
        s.reverse();

        println!("{s:?}");
        println!("{c:?}");
        assert!(tests::approx_eq(&s, &c, 1e-5));
    }
}