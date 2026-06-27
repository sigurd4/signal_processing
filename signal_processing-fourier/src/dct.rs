use core::borrow::{Borrow, BorrowMut};

use array_trait::length;
use bulks::{AsBulk, Bulk, DoubleEndedBulk, IntoBulk};
use num_complex::{Complex, ComplexFloat};
use num_traits::{Float, FloatConst, Inv, NumCast, One, Zero};
use crate::{Dft, Permute, SpectrumScaling, util::{RealDiv, RealMul, TruncateIm, fct_i, fct_ii, fct_iii, fct_iv}};

/// # Discrete cosine-transform
/// 
/// The discrete cosine-transform is the real-valued fourier transform of the even extension of a sequence.
/// 
/// While there is only one continuous cosine-transform, due to the nature of how quantized signals can be mirrored there are four types of DCTs in total.
/// 
/// ## DCT I
/// 
/// For input `[a, b, c]`, it's equivalent to the DFT of `[a, b, c, b]`.
/// 
/// The DCT I is orthogonal, i.e. it's its own inverse (assuming balanced scaling).
/// 
/// ## DCT II
/// 
/// The DCT III is its inverse (assuming balanced scaling).
/// 
/// ## DCT III
/// 
/// The DCT II is its inverse (assuming balanced scaling).
/// 
/// ## DCT IV
/// 
/// The DCT IV is orthogonal, i.e. it's its own inverse (assuming balanced scaling).
pub trait Dct<T>: Permute<T>
where
    T: ComplexFloat
{
    /// The type I discrete cosine-transform.
    /// 
    /// The DCT I is its own inverse (assuming balanced scaling).
    #[doc(alias = "idct_i")]
    fn dct_i(&mut self)
    {
        self.dct_i_scaled(SpectrumScaling::Balanced);
    }
    /// The type II discrete cosine-transform.
    /// 
    /// The DCT III is its inverse (assuming balanced scaling).
    #[doc(alias = "idct_iii")]
    fn dct_ii(&mut self)
    {
        self.dct_ii_scaled(SpectrumScaling::Balanced);
    }
    /// The type III discrete cosine-transform.
    /// 
    /// The DCT II is its inverse (assuming balanced scaling).
    #[doc(alias = "idct_ii")]
    fn dct_iii(&mut self)
    {
        self.dct_iii_scaled(SpectrumScaling::Balanced);
    }
    /// The type IV discrete cosine-transform.
    /// 
    /// The DCT IV is its own inverse (assuming balanced scaling).
    #[doc(alias = "idct_iv")]
    fn dct_iv(&mut self)
    {
        self.dct_iv_scaled(SpectrumScaling::Balanced);
    }

    fn dct_i_scaled(&mut self, scaling: SpectrumScaling);
    fn dct_ii_scaled(&mut self, scaling: SpectrumScaling);
    fn dct_iii_scaled(&mut self, scaling: SpectrumScaling);
    fn dct_iv_scaled(&mut self, scaling: SpectrumScaling);
}
impl<B, T> Dct<T> for B
where
    for<'a> &'a mut B: IntoBulk<Item: BorrowMut<T>>,
    for<'a> &'a B: IntoBulk<Item: Borrow<T>, IntoBulk: DoubleEndedBulk>,
    B: ?Sized,
    T: ComplexFloat + 'static
{
    fn dct_i_scaled(&mut self, scaling: SpectrumScaling)
    {
        let len = (*self).bulk().length();
        let len_m1 = length::value::saturating_sub(len, [(); 1]);

        let sqrt_2 = T::Real::SQRT_2();
        let one = T::Real::one();
        let two = one + one;

        if matches!(scaling, SpectrumScaling::Balanced)
        {
            self.bulk_mut()
                .step_by(len_m1)
                .map(|mut x| (*x.borrow_mut(), x))
                .for_each(|(x, mut y)| *y.borrow_mut() = x._real_mul(sqrt_2));
        }

        fct_i::fct_i_unscaled(self, None);

        if matches!(scaling, SpectrumScaling::Balanced)
        {
            self.bulk_mut()
                .step_by(len_m1)
                .map(|mut x| (*x.borrow_mut(), x))
                .for_each(|(x, mut y)| *y.borrow_mut() = x._real_div(sqrt_2));
        }
        if let Some(scale) = match scaling
        {
            SpectrumScaling::Summed => None,
            SpectrumScaling::Balanced => Some(Float::sqrt(two/<T::Real as NumCast>::from(length::value::len(len_m1)).unwrap())),
            SpectrumScaling::Averaged => Some(two/<T::Real as NumCast>::from(length::value::len(len_m1)).unwrap())
        }
        {
            self.bulk_mut()
                .map(|mut x| (*x.borrow_mut(), x))
                .for_each(|(x, mut y)| *y.borrow_mut() = x._real_mul(scale));
        }
    }
    fn dct_ii_scaled(&mut self, scaling: SpectrumScaling)
    {
        let len = (*self).bulk().length();

        let sqrt_2 = T::Real::SQRT_2();
        let one = T::Real::one();
        let two = one + one;

        fct_ii::fct_ii_unscaled(self, None);

        if matches!(scaling, SpectrumScaling::Balanced)
        {
            self.bulk_mut()
                .first()
                .map(|mut x| (*x.borrow_mut(), x))
                .into_bulk()
                .for_each(|(x, mut y)| *y.borrow_mut() = x._real_div(sqrt_2));
        }
        if let Some(scale) = match scaling
        {
            SpectrumScaling::Summed => None,
            SpectrumScaling::Balanced => Some(Float::sqrt(two/<T::Real as NumCast>::from(length::value::len(len)).unwrap())),
            SpectrumScaling::Averaged => Some(two/<T::Real as NumCast>::from(length::value::len(len)).unwrap())
        }
        {
            self.bulk_mut()
                .map(|mut x| (*x.borrow_mut(), x))
                .for_each(|(x, mut y)| *y.borrow_mut() = x._real_mul(scale));
        }
    }
    fn dct_iii_scaled(&mut self, scaling: SpectrumScaling)
    {
        let len = (*self).bulk().length();

        let sqrt_2 = T::Real::SQRT_2();
        let one = T::Real::one();
        let two = one + one;

        if matches!(scaling, SpectrumScaling::Balanced)
        {
            self.bulk_mut()
                .first()
                .map(|mut x| (*x.borrow_mut(), x))
                .into_bulk()
                .for_each(|(x, mut y)| *y.borrow_mut() = x._real_mul(sqrt_2));
        }

        fct_iii::fct_iii_unscaled(self, None);
        
        if let Some(scale) = match scaling
        {
            SpectrumScaling::Summed => None,
            SpectrumScaling::Balanced => Some(Float::sqrt(two/<T::Real as NumCast>::from(length::value::len(len)).unwrap())),
            SpectrumScaling::Averaged => Some(two/<T::Real as NumCast>::from(length::value::len(len)).unwrap())
        }
        {
            self.bulk_mut()
                .map(|mut x| (*x.borrow_mut(), x))
                .for_each(|(x, mut y)| *y.borrow_mut() = x._real_mul(scale));
        }
    }
    fn dct_iv_scaled(&mut self, scaling: SpectrumScaling)
    {
        let len = (*self).bulk().length();

        let one = T::Real::one();
        let two = one + one;

        fct_iv::fct_iv_unscaled(self, None);
        
        if let Some(scale) = match scaling
        {
            SpectrumScaling::Summed => None,
            SpectrumScaling::Balanced => Some(Float::sqrt(two/<T::Real as NumCast>::from(length::value::len(len)).unwrap())),
            SpectrumScaling::Averaged => Some(two/<T::Real as NumCast>::from(length::value::len(len)).unwrap())
        }
        {
            self.bulk_mut()
                .map(|mut x| (*x.borrow_mut(), x))
                .for_each(|(x, mut y)| *y.borrow_mut() = x._real_mul(scale));
        }
    }
}

#[cfg(test)]
mod test
{
    use core::f64::consts::{PI, SQRT_2, TAU};

    use bulks::{AsBulk, Bulk, IntoBulk};
    use linspace::Linspace;

    use crate::{Dct, Dst, SpectrumScaling, tests, util::{fct_iii, fct_iv}};

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

        println!("{b:?}");
        assert!(tests::approx_eq(&a, &b, 1e-5));

        let mut b = a;
        b.dct_ii();
        b.dct_iii();

        println!("{b:?}");
        assert!(tests::approx_eq(&a, &b, 1e-5));

        let mut b = a;
        b.dct_iii();
        b.dct_ii();

        println!("{b:?}");
        assert!(tests::approx_eq(&a, &b, 1e-5));

        let mut b = a;
        b.dct_iv();
        b.dct_iv();

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
        b.dct_i_scaled(SpectrumScaling::Summed);
        b.dct_i_scaled(SpectrumScaling::Averaged);

        println!("{b:?}");
        assert!(tests::approx_eq(&a, &b, 1e-5));

        let mut b = a;
        b.dct_ii_scaled(SpectrumScaling::Summed);
        b.dct_iii_scaled(SpectrumScaling::Averaged);

        println!("{b:?}");
        assert!(tests::approx_eq(&a, &b, 1e-5));

        let mut b = a;
        b.dct_iii_scaled(SpectrumScaling::Summed);
        b.dct_ii_scaled(SpectrumScaling::Averaged);

        println!("{b:?}");
        assert!(tests::approx_eq(&a, &b, 1e-5));

        let mut b = a;
        b.dct_iv_scaled(SpectrumScaling::Summed);
        b.dct_iv_scaled(SpectrumScaling::Averaged);

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
        b.dct_i_scaled(SpectrumScaling::Averaged);
        b.dct_i_scaled(SpectrumScaling::Summed);

        println!("{b:?}");
        assert!(tests::approx_eq(&a, &b, 1e-5));

        let mut b = a;
        b.dct_ii_scaled(SpectrumScaling::Averaged);
        b.dct_iii_scaled(SpectrumScaling::Summed);

        println!("{b:?}");
        assert!(tests::approx_eq(&a, &b, 1e-5));

        let mut b = a;
        b.dct_iii_scaled(SpectrumScaling::Averaged);
        b.dct_ii_scaled(SpectrumScaling::Summed);

        println!("{b:?}");
        assert!(tests::approx_eq(&a, &b, 1e-5));

        let mut b = a;
        b.dct_iv_scaled(SpectrumScaling::Averaged);
        b.dct_iv_scaled(SpectrumScaling::Summed);

        println!("{b:?}");
        assert!(tests::approx_eq(&a, &b, 1e-5));
    }

    #[test]
    fn test_dct_i()
    {
        let a = [1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11]
            .into_bulk()
            .map(|x| x as f64)
            .collect_array();

        fn dct_i_direct_unscaled(x: &mut [f64])
        {
            let l = x.len();
            if let Some(x_first) = x.first().copied()
                && let Some(x_last) = x.last().copied()
            {
                let first_term = [
                    x_first.midpoint(x_last),
                    x_first.midpoint(-x_last)
                ];
                let y: Vec<_> = (0..l)
                    .map(|k| first_term[k % 2]
                        + x[..l - 1].iter()
                            .enumerate()
                            .skip(1)
                            .map(|(n, xn)| xn*(PI*k as f64*n as f64/(l - 1) as f64).cos())
                            .sum::<f64>()
                    ).collect();
                
                for (x, y) in x.iter_mut()
                    .zip(y)
                {
                    *x = y
                }
            }
        }
        fn dct_i_direct(x: &mut [f64])
        {
            let l = x.len();
            x[0] *= SQRT_2;
            x[l - 1] *= SQRT_2;
            dct_i_direct_unscaled(x);
            for x in x.iter_mut()
            {
                *x *= (2.0/(l - 1) as f64).sqrt()
            }
            x[0] /= SQRT_2;
            x[l - 1] /= SQRT_2;
        }

        let mut b = a;
        let mut c = a;
        b.dct_i_scaled(SpectrumScaling::Summed);
        dct_i_direct_unscaled(&mut c);

        println!("{b:?}");
        println!("{c:?}");
        assert!(tests::approx_eq(&b, &c, 1e-5));

        let mut b = a;
        let mut c = a;
        b.dct_i();
        dct_i_direct(&mut c);

        println!("{b:?}");
        println!("{c:?}");
        assert!(tests::approx_eq(&b, &c, 1e-5));
    }

    #[test]
    fn test_dct_ii()
    {
        let a = [1, 2, 3, 4, 5, 6, 7, 8]
            .into_bulk()
            .map(|x| x as f64)
            .collect_array();

        fn dct_ii_direct_unscaled(x: &mut [f64])
        {
            let l = x.len();
            let y: Vec<_> = (0..l)
                .map(|k| x.iter()
                        .enumerate()
                        .map(|(n, xn)| xn*(PI*k as f64*(n as f64 + 0.5)/l as f64).cos())
                        .sum::<f64>()
                ).collect();
            
            for (x, y) in x.iter_mut()
                .zip(y)
            {
                *x = y
            }
        }
        fn dct_ii_direct(x: &mut [f64])
        {
            let l = x.len();
            dct_ii_direct_unscaled(x);
            for x in x.iter_mut()
            {
                *x *= (2.0/l as f64).sqrt()
            }
            x[0] /= SQRT_2;
        }
        
        let mut b = a;
        let mut c = a;
        b.dct_ii_scaled(SpectrumScaling::Summed);
        dct_ii_direct_unscaled(&mut c);

        println!("{b:?}");
        println!("{c:?}");
        assert!(tests::approx_eq(&b, &c, 1e-5));
        
        let mut b = a;
        let mut c = a;
        b.dct_ii();
        dct_ii_direct(&mut c);

        println!("{b:?}");
        println!("{c:?}");
        assert!(tests::approx_eq(&b, &c, 1e-5));
    }

    #[test]
    fn test_dct_iii()
    {
        let a = [1, 2, 3, 4, 5, 6, 7, 8]
            .into_bulk()
            .map(|x| x as f64)
            .collect_array();

        fn dct_iii_direct_unscaled(x: &mut [f64])
        {
            let l = x.len();
            let y: Vec<_> = (0..l)
                .map(|k| x[0]/2.0
                    + x.iter()
                        .enumerate()
                        .skip(1)
                        .map(|(n, xn)| xn*(PI*n as f64*(k as f64 + 0.5)/l as f64).cos())
                        .sum::<f64>()
                ).collect();
            
            for (x, y) in x.iter_mut()
                .zip(y)
            {
                *x = y
            }
        }
        fn dct_iii_direct(x: &mut [f64])
        {
            let l = x.len();
            x[0] *= SQRT_2;
            dct_iii_direct_unscaled(x);
            for x in x
            {
                *x *= (2.0/l as f64).sqrt()
            }
        }
        
        let mut b = a;
        let mut c = a;
        b.dct_iii_scaled(SpectrumScaling::Summed);
        dct_iii_direct_unscaled(&mut c);

        println!("{b:?}");
        println!("{c:?}");
        assert!(tests::approx_eq(&b, &c, 1e-5));
        
        let mut b = a;
        let mut c = a;
        b.dct_iii();
        dct_iii_direct(&mut c);

        println!("{b:?}");
        println!("{c:?}");
        assert!(tests::approx_eq(&b, &c, 1e-5));
    }

    #[test]
    fn test_dct_iv()
    {
        let a = [1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11]
            .into_bulk()
            .map(|x| x as f64)
            .collect_array();

        fn dct_iv_direct_unscaled(x: &mut [f64])
        {
            let l = x.len();
            let y: Vec<_> = (0..l)
                .map(|k| x.iter()
                        .enumerate()
                        .map(|(n, xn)| xn*(PI*(n as f64 + 0.5)*(k as f64 + 0.5)/l as f64).cos())
                        .sum::<f64>()
                ).collect();
            
            for (x, y) in x.iter_mut()
                .zip(y)
            {
                *x = y
            }
        }
        fn dct_iv_direct(x: &mut [f64])
        {
            let l = x.len();
            dct_iv_direct_unscaled(x);
            for x in x
            {
                *x *= (2.0/l as f64).sqrt()
            }
        }

        let mut b = a;
        let mut c = a;
        b.dct_iv_scaled(SpectrumScaling::Summed);
        dct_iv_direct_unscaled(&mut c);

        println!("{b:?}");
        println!("{c:?}");
        assert!(tests::approx_eq(&b, &c, 1e-5));

        let mut b = a;
        let mut c = a;
        b.dct_iv();
        dct_iv_direct(&mut c);

        println!("{b:?}");
        println!("{c:?}");
        assert!(tests::approx_eq(&b, &c, 1e-5));
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