use core::borrow::{Borrow, BorrowMut};

use array_trait::length;
use bulks::{AsBulk, Bulk, DoubleEndedBulk, IntoBulk};
use num_complex::{Complex, ComplexFloat};
use num_traits::{Float, FloatConst, Inv, NumCast, One, Zero};
use crate::{Dft, Permute, SpectrumScaling, util::TruncateIm};

/// # Discrete sine-transform
/// 
/// The discrete sine-transform is the real-valued fourier transform of the odd extension of a sequence.
/// 
/// There are four types of DSTs:
/// 
/// ## DST I
/// 
/// The DST I is its own inverse (assuming balanced scaling).
/// 
/// ## DST II
/// 
/// The DST III is its inverse (assuming balanced scaling).
/// 
/// ## DST III
/// 
/// The DST II is its inverse (assuming balanced scaling).
/// 
/// ## DST IV
/// 
/// The DST IV is its own inverse (assuming balanced scaling).
pub trait Dst<T>: Permute<T>
{
    /// The type I discrete sine-transform.
    /// 
    /// The DST I is its own inverse (assuming balanced scaling).
    #[doc(alias = "idst_i")]
    fn dst_i(&mut self)
    {
        self.dst_i_scaled(SpectrumScaling::Balanced)
    }
    /// The type II discrete sine-transform.
    /// 
    /// The DST III is its inverse (assuming balanced scaling).
    #[doc(alias = "idst_iii")]
    fn dst_ii(&mut self)
    {
        self.dst_ii_scaled(SpectrumScaling::Balanced)
    }
    /// The type III discrete sine-transform.
    /// 
    /// The DST II is its inverse (assuming balanced scaling).
    #[doc(alias = "idst_ii")]
    fn dst_iii(&mut self)
    {
        self.dst_iii_scaled(SpectrumScaling::Balanced)
    }
    /// The type IV discrete sine-transform.
    /// 
    /// The DST IV is its own inverse (assuming balanced scaling).
    #[doc(alias = "idst_iv")]
    fn dst_iv(&mut self)
    {
        self.dst_iv_scaled(SpectrumScaling::Balanced)
    }

    fn dst_i_scaled(&mut self, scaling: SpectrumScaling);
    fn dst_ii_scaled(&mut self, scaling: SpectrumScaling);
    fn dst_iii_scaled(&mut self, scaling: SpectrumScaling);
    fn dst_iv_scaled(&mut self, scaling: SpectrumScaling);
}
impl<B, T> Dst<T> for B
where
    for<'a> &'a mut B: IntoBulk<Item: BorrowMut<T>>,
    for<'a> &'a B: IntoBulk<Item: Borrow<T>, IntoBulk: DoubleEndedBulk>,
    B: ?Sized,
    T: ComplexFloat + 'static
{
    fn dst_i_scaled(&mut self, scaling: SpectrumScaling)
    {
        let len = (*self).bulk().length();
        if length::value::le(len, [(); 1])
        {
            return
        }
        let one = <T as ComplexFloat>::Real::one();
        let two = one + one;

        let mut y: Vec<_> = bulks::once(Zero::zero())
            .chain(
                (*self).bulk()
                    .map(|x| Complex { re: x.borrow().re(), im: x.borrow().im() })
            )
            .chain(bulks::once(Zero::zero()))
            .chain(
                (*self).bulk()
                    .rev()
                    .map(|x| -Complex { re: x.borrow().re(), im: x.borrow().im() })
            )
            .collect();
        y.dft_scaled(scaling);

        let (y1, y2) = y.into_bulk().split_at([(); 1]).1.split_at(len);

        let y_div = match scaling
        {
            SpectrumScaling::Summed => two,
            SpectrumScaling::Balanced => one,
            SpectrumScaling::Averaged => Float::recip(two)
        };
        let y_div = Complex::new(Zero::zero(), -y_div);
        for (y, mut x) in y1.into_iter()
            .zip(y2.rev())
            .map(|(y1, y2)| (y1 - y2)/two)
            .map(|y| y/y_div)
            .zip(self.bulk_mut())
        {
            *x.borrow_mut() = <T as TruncateIm>::truncate_im(y)
        }
    }
    fn dst_ii_scaled(&mut self, scaling: SpectrumScaling)
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
        let zero = <T as ComplexFloat>::Real::zero();

        let mut y: Vec<_> = (*self).bulk()
            .map(|x| Complex { re: x.borrow().re(), im: x.borrow().im() })
            .chain(
                (*self).bulk()
                    .rev()
                    .map(|x| -Complex { re: x.borrow().re(), im: x.borrow().im() })
            ).collect();
        y.dft_scaled(scaling);
    
        let mul = Complex::new(zero, match scaling
        {
            SpectrumScaling::Summed => half,
            SpectrumScaling::Balanced => one,
            SpectrumScaling::Averaged => two
        });
        let (y1, y2) = y.into_bulk()
            .split_at([(); 1]).1
            .map(|y| y*mul)
            .split_at(len);
    
        let m1 = bulks::range([(); 1], len)
            .map(|i| {
                let i = <<T as ComplexFloat>::Real as NumCast>::from(i).unwrap();
                Complex::from_polar(half, -i*frac_pi_2/lenf)
            }).chain(bulks::once(Complex::new(zero, -if matches!(scaling, SpectrumScaling::Balanced) { frac_1_sqrt_2 } else { one })));
    
        let m2 = bulks::range([(); 1], len)
            .map(|i| {
                let i = <<T as ComplexFloat>::Real as NumCast>::from(i).unwrap();
                Complex::from_polar(-half, i*frac_pi_2/lenf)
            });
    
        for (y, mut x) in y1.into_iter()
            .zip(m1)
            .map(|(y, m1)| y*m1)
            .zip(y2.into_iter()
                .rev()
                .zip(m2)
                .map(|(y, m2)| y*m2)
                .chain(core::iter::once(Zero::zero()))
            ).map(|(y1, y2)| y1 + y2)
            .zip(self.bulk_mut())
        {
            *x.borrow_mut() = TruncateIm::truncate_im(y);
        }
    }
    fn dst_iii_scaled(&mut self, scaling: SpectrumScaling)
    {
        let len = (*self).bulk().length();
        if length::value::le(len, [(); 1])
        {
            return
        }
        let lenf = <<T as ComplexFloat>::Real as NumCast>::from(length::value::len(len)).unwrap();

        let frac_pi_2 = <T as ComplexFloat>::Real::FRAC_PI_2();
        let one = T::Real::one();
        let two = one + one;

        let m1 = bulks::range([(); 1], len)
            .map(|i| {
                let i = <<T as ComplexFloat>::Real as NumCast>::from(i).unwrap();
                Complex::cis(i*frac_pi_2/lenf)
            }).chain(bulks::once(if matches!(scaling, SpectrumScaling::Balanced) { Complex::i()*T::Real::SQRT_2() } else { Complex::i() }));
        let m2 = bulks::range([(); 1], len)
            .map(|i| {
                let i = <<T as ComplexFloat>::Real as NumCast>::from(i).unwrap();
                -Complex::cis(-i*frac_pi_2/lenf)
            })
            .rev();
        
        let mut y = bulks::once(Zero::zero())
            .chain(
                (*self).bulk()
                    .map(|x| Complex { re: x.borrow().re(), im: x.borrow().im() })
                    .zip(m1)
                    .map(|(x, m1)| m1*x)
            )
            .chain(
                (*self).bulk()
                    .rev()
                    .skip([(); 1])
                    .map(|x| Complex { re: x.borrow().re(), im: x.borrow().im() })
                    .zip(m2)
                    .map(|(x, m2)| m2*x)
            ).collect::<Vec<_>, _>();
        y.idft_scaled(scaling.inv());
        
        let ymul = -Complex::i()*match scaling
        {
            SpectrumScaling::Summed => Float::recip(two),
            SpectrumScaling::Balanced => one,
            SpectrumScaling::Averaged => two
        };
        let (y1, y2) = y.bulk_mut()
            .split_at(len);
        for (y1, y2) in y1.zip(y2.rev())
        {
            *y1 = (*y1 - *y2)/two
        }
        
        for (mut y, mut x) in y.into_bulk()
            .zip(self.bulk_mut())
        {
            y = y*ymul;
            *x.borrow_mut() = <_ as TruncateIm>::truncate_im(y)
        }

    }
    fn dst_iv_scaled(&mut self, scaling: SpectrumScaling)
    {
        let len = (*self).bulk().length();
        if length::value::le(len, [(); 1])
        {
            return
        }
        let len_p1 = length::value::add(len, [(); 1]);
        let lenf = <<T as ComplexFloat>::Real as NumCast>::from(length::value::len(len)).unwrap();

        let frac_1_sqrt_2 = <T as ComplexFloat>::Real::FRAC_1_SQRT_2();
        let frac_pi_2 = <T as ComplexFloat>::Real::FRAC_PI_2();
        let frac_pi_4 = <T as ComplexFloat>::Real::FRAC_PI_4();
        let one = T::Real::one();
        let two = one + one;
        let half = Float::recip(two);

        let m1: Vec<_> = bulks::range([(); 0], len)
            .map(|i| {
                let i = <<T as ComplexFloat>::Real as NumCast>::from(i).unwrap();
                Complex::from_polar(frac_1_sqrt_2, -i*frac_pi_2/lenf)
            }).chain(core::iter::once(Complex::i()))
            .collect();
        let m2: Vec<_> = bulks::range([(); 1], len_p1)
            .map(|i| {
                let i = <<T as ComplexFloat>::Real as NumCast>::from(i).unwrap();
                Complex::from_polar(-frac_1_sqrt_2, i*frac_pi_2/lenf)
            }).collect();

        let mut y = (*self).bulk()
            .map(|x| Complex { re: x.borrow().re(), im: x.borrow().im() })
            .zip(m1.bulk())
            .map(|(x, &m1)| m1*x)
            .chain(
                (*self).bulk()
                    .rev()
                    .map(|x| Complex { re: x.borrow().re(), im: x.borrow().im() })
                    .zip(m2.bulk()
                        .rev()
                    ).map(|(x, &m2)| m2*x)
            ).collect::<Vec<_>, _>();
        y.dft_scaled(scaling);

        let scale = match scaling
        {
            SpectrumScaling::Summed => half,
            SpectrumScaling::Balanced => one,
            SpectrumScaling::Averaged => two
        };
        let ymul = Complex::<T::Real>::i()*Complex::from_polar(scale, -frac_pi_4/lenf);
        let (y1, y2) = y.into_bulk()
            .map(|y| y*ymul)
            .split_at(len);
        
        for (y, mut x) in y1.into_iter()
            .zip(m1)
            .map(|(y, m1)| y*m1)
            .zip(y2.rev()
                .zip(m2)
                .map(|(y, m2)| y*m2)
            ).map(|(y1, y2)| y1 + y2)
            .zip(self.bulk_mut())
        {
            *x.borrow_mut() = TruncateIm::truncate_im(y);
        }
    }
}

#[cfg(test)]
mod test
{
    use core::f64::consts::{PI, SQRT_2, TAU};

    use bulks::{AsBulk, Bulk, IntoBulk};
    use linspace::Linspace;

    use crate::{Dct, Dst, SpectrumScaling, tests};

    #[test]
    fn plot_dst()
    {
        const N: usize = 1024;
        const T: f64 = 1.0;
        const F: f64 = 220.0;
        
        let x: [_; N] = core::array::from_fn(|i| (TAU*F*i as f64/N as f64*T).sin());

        let w = (0.0..TAU).linspace_array::<N>();
        let xf = [
            { let mut y = x; y.dst_i(); y },
            { let mut y = x; y.dst_ii(); y },
            { let mut y = x; y.dst_iii(); y },
            { let mut y = x; y.dst_iv(); y }
        ];

        ezplot::plot_curves("X(e^jw)", "plots/x_z_dst.png", xf.map(|xf| w.into_bulk().zip(xf)))
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
        b.dst_i();
        b.dst_i();

        println!("{b:?}");
        assert!(tests::approx_eq(&a, &b, 1e-5));

        let mut b = a;
        b.dst_ii();
        b.dst_iii();

        println!("{b:?}");
        assert!(tests::approx_eq(&a, &b, 1e-5));

        let mut b = a;
        b.dst_iii();
        b.dst_ii();

        println!("{b:?}");
        assert!(tests::approx_eq(&a, &b, 1e-5));

        let mut b = a;
        b.dst_iv();
        b.dst_iv();

        println!("{b:?}");
        assert!(tests::approx_eq(&a, &b, 1e-5));
    }

    #[test]
    fn identities_summed()
    {
        let a = [1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11]
            .into_bulk()
            .map(|x| x as f32)
            .collect_array();

        let mut b = a;
        b.dst_i_scaled(SpectrumScaling::Summed);
        b.dst_i_scaled(SpectrumScaling::Averaged);

        println!("{b:?}");
        assert!(tests::approx_eq(&a, &b, 1e-5));

        let mut b = a;
        b.dst_ii_scaled(SpectrumScaling::Summed);
        b.dst_iii_scaled(SpectrumScaling::Averaged);

        println!("{b:?}");
        assert!(tests::approx_eq(&a, &b, 1e-5));

        let mut b = a;
        b.dst_iii_scaled(SpectrumScaling::Summed);
        b.dst_ii_scaled(SpectrumScaling::Averaged);

        println!("{b:?}");
        assert!(tests::approx_eq(&a, &b, 1e-5));

        let mut b = a;
        b.dst_iv_scaled(SpectrumScaling::Summed);
        b.dst_iv_scaled(SpectrumScaling::Averaged);

        println!("{b:?}");
        assert!(tests::approx_eq(&a, &b, 1e-5));
    }

    #[test]
    fn identities_averaged()
    {
        let a = [1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11]
            .into_bulk()
            .map(|x| x as f32)
            .collect_array();

        let mut b = a;
        b.dst_i_scaled(SpectrumScaling::Averaged);
        b.dst_i_scaled(SpectrumScaling::Summed);

        println!("{b:?}");
        assert!(tests::approx_eq(&a, &b, 1e-5));

        let mut b = a;
        b.dst_ii_scaled(SpectrumScaling::Averaged);
        b.dst_iii_scaled(SpectrumScaling::Summed);

        println!("{b:?}");
        assert!(tests::approx_eq(&a, &b, 1e-5));

        let mut b = a;
        b.dst_iii_scaled(SpectrumScaling::Averaged);
        b.dst_ii_scaled(SpectrumScaling::Summed);

        println!("{b:?}");
        assert!(tests::approx_eq(&a, &b, 1e-5));

        let mut b = a;
        b.dst_iv_scaled(SpectrumScaling::Averaged);
        b.dst_iv_scaled(SpectrumScaling::Summed);

        println!("{b:?}");
        assert!(tests::approx_eq(&a, &b, 1e-5));
    }

    #[test]
    fn test_dst_i()
    {
        let a = [1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11]
            .into_bulk()
            .map(|x| x as f64)
            .collect_array();

        fn dst_i_direct_unscaled(x: &mut [f64])
        {
            let l = x.len();
            let y: Vec<_> = (0..l)
                .map(|k| x.iter()
                        .enumerate()
                        .map(|(n, xn)| xn*(PI*(n + 1) as f64*(k + 1) as f64/(l + 1) as f64).sin())
                        .sum::<f64>()
                ).collect();
            
            for (x, y) in x.iter_mut()
                .zip(y)
            {
                *x = y
            }
        }
        fn dst_i_direct(x: &mut [f64])
        {
            let l = x.len();
            dst_i_direct_unscaled(x);
            for x in x
            {
                *x *= (2.0/(l + 1) as f64).sqrt()
            }
        }

        let mut b = a;
        let mut c = a;
        b.dst_i();
        dst_i_direct(&mut c);

        println!("{b:?}");
        println!("{c:?}");
        assert!(tests::approx_eq(&b, &c, 1e-5));

        let mut b = a;
        let mut c = a;
        b.dst_i_scaled(SpectrumScaling::Summed);
        dst_i_direct_unscaled(&mut c);

        println!("{b:?}");
        println!("{c:?}");
        assert!(tests::approx_eq(&b, &c, 1e-5));
    }

    #[test]
    fn test_dst_ii()
    {
        let a = [1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11]
            .into_bulk()
            .map(|x| x as f64)
            .collect_array();

        fn dst_ii_direct_unscaled(x: &mut [f64])
        {
            let l = x.len();
            let y: Vec<_> = (0..l)
                .map(|k| x.iter()
                        .enumerate()
                        .map(|(n, xn)| xn*(PI*(n as f64 + 0.5)*(k + 1) as f64/l as f64).sin())
                        .sum::<f64>()
                ).collect();
            
            for (x, y) in x.iter_mut()
                .zip(y)
            {
                *x = y
            }
        }
        fn dst_ii_direct(x: &mut [f64])
        {
            let l = x.len();
            dst_ii_direct_unscaled(x);
            x[l - 1] /= SQRT_2;
            for x in x
            {
                *x *= (2.0/l as f64).sqrt()
            }
        }

        let mut b = a;
        let mut c = a;
        b.dst_ii();
        dst_ii_direct(&mut c);

        println!("{b:?}");
        println!("{c:?}");
        assert!(tests::approx_eq(&b, &c, 1e-5));

        let mut b = a;
        let mut c = a;
        b.dst_ii_scaled(SpectrumScaling::Summed);
        dst_ii_direct_unscaled(&mut c);

        println!("{b:?}");
        println!("{c:?}");
        assert!(tests::approx_eq(&b, &c, 1e-5));
    }

    #[test]
    fn test_dst_iii()
    {
        let a = [1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11]
            .into_bulk()
            .map(|x| x as f64)
            .collect_array();

        fn dst_iii_direct_unscaled(x: &mut [f64])
        {
            let l = x.len();
            let first_term = [
                0.5*x[l - 1],
                -0.5*x[l - 1]
            ];
            let y: Vec<_> = (0..l)
                .map(|k| first_term[k % 2]
                    + x.iter()
                        .take(l - 1)
                        .enumerate()
                        .map(|(n, xn)| xn*(PI*(n + 1) as f64*(k as f64 + 0.5)/l as f64).sin())
                        .sum::<f64>()
                ).collect();
            
            for (x, y) in x.iter_mut()
                .zip(y)
            {
                *x = y
            }
        }
        fn dst_iii_direct(x: &mut [f64])
        {
            let l = x.len();
            x[l - 1] *= SQRT_2;
            dst_iii_direct_unscaled(x);
            for x in x
            {
                *x *= (2.0/l as f64).sqrt()
            }
        }

        let mut b = a;
        let mut c = a;
        b.dst_iii();
        dst_iii_direct(&mut c);

        println!("{b:?}");
        println!("{c:?}");
        assert!(tests::approx_eq(&b, &c, 1e-5));

        let mut b = a;
        let mut c = a;
        b.dst_iii_scaled(SpectrumScaling::Summed);
        dst_iii_direct_unscaled(&mut c);

        println!("{b:?}");
        println!("{c:?}");
        assert!(tests::approx_eq(&b, &c, 1e-5));
    }

    #[test]
    fn test_dst_iv()
    {
        let a = [1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11]
            .into_bulk()
            .map(|x| x as f64)
            .collect_array();

        fn dst_iv_direct_unscaled(x: &mut [f64])
        {
            let l = x.len();
            let y: Vec<_> = (0..l)
                .map(|k| x.iter()
                        .enumerate()
                        .map(|(n, xn)| xn*(PI*(n as f64 + 0.5)*(k as f64 + 0.5)/l as f64).sin())
                        .sum::<f64>()
                ).collect();
            
            for (x, y) in x.iter_mut()
                .zip(y)
            {
                *x = y
            }
        }
        fn dst_iv_direct(x: &mut [f64])
        {
            let l = x.len();
            dst_iv_direct_unscaled(x);
            for x in x
            {
                *x *= (2.0/l as f64).sqrt()
            }
        }

        let mut b = a;
        let mut c = a;
        b.dst_iv();
        dst_iv_direct(&mut c);

        println!("{b:?}");
        println!("{c:?}");
        assert!(tests::approx_eq(&b, &c, 1e-5));

        let mut b = a;
        let mut c = a;
        b.dst_iv_scaled(SpectrumScaling::Summed);
        dst_iv_direct_unscaled(&mut c);

        println!("{b:?}");
        println!("{c:?}");
        assert!(tests::approx_eq(&b, &c, 1e-5));
    }

    #[test]
    fn from_dct_iii()
    {
        let a = [1, 2, 3, 4, 5]
            .into_bulk()
            .map(|x| x as f32)
            .collect_array();

        let mut s = a;
        s.dst_iii();

        let mut c = a.into_bulk()
            .rev()
            .collect_array();
        c.dct_iii();
        c.bulk_mut()
            .skip(1)
            .step_by(2)
            .for_each(|x| *x = -*x);

        println!("{s:?}");
        println!("{c:?}");
        assert!(tests::approx_eq(&s, &c, 1e-5));
    }

    #[test]
    fn from_dct_iv()
    {
        let a = [1, 2, 3, 4, 5]
            .into_bulk()
            .map(|x| x as f32)
            .collect_array();

        let mut s = a;
        s.dst_iv();

        let mut c = a.into_bulk()
            .rev()
            .collect_array();
        c.dct_iv();
        c.bulk_mut()
            .skip(1)
            .step_by(2)
            .for_each(|x| *x = -*x);

        println!("{s:?}");
        println!("{c:?}");
        assert!(tests::approx_eq(&s, &c, 1e-5));
    }
}