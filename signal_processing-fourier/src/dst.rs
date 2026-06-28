use core::borrow::BorrowMut;

use array_trait::length;
use bulks::{AsBulk, Bulk, IntoBulk};
use num_complex::{ComplexFloat};
use num_traits::{Float, FloatConst, NumCast, One};
use crate::{Permute, SpectrumScaling, util::{RealDiv, RealMul, fst_i, fst_ii, fst_iii, fst_iv}};

/// # Discrete sine-transform
/// 
/// The discrete sine-transform is the real-valued fourier transform of the odd extension of a sequence.
/// 
/// While there is only one continuous sine-transform, due to the nature of how quantized signals can be mirrored there are four types of DSTs in total.
/// 
/// ## DST I
/// 
/// For input `[a, b, c]`, it's equivalent to the DFT of `[0, a, b, c, 0, -c, -b, -a]`.
/// 
/// The DST I is orthogonal, i.e. it's its own inverse (assuming balanced scaling).
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
/// The DST IV is orthogonal, i.e. it's its own inverse (assuming balanced scaling).
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
    B: ?Sized,
    T: ComplexFloat + 'static
{
    fn dst_i_scaled(&mut self, scaling: SpectrumScaling)
    {
        let len = self.bulk_mut().length();
        let len_p1 = length::value::add(len, [(); 1]);

        let one = T::Real::one();
        let two = one + one;

        fst_i::fst_i_unscaled(self, None);
        
        if let Some(scale) = match scaling
        {
            SpectrumScaling::Summed => None,
            SpectrumScaling::Balanced => Some(Float::sqrt(two/<T::Real as NumCast>::from(length::value::len(len_p1)).unwrap())),
            SpectrumScaling::Averaged => Some(two/<T::Real as NumCast>::from(length::value::len(len_p1)).unwrap())
        }
        {
            self.bulk_mut()
                .map(|mut x| (*x.borrow_mut(), x))
                .for_each(|(x, mut y)| *y.borrow_mut() = x._real_mul(scale));
        }
    }
    fn dst_ii_scaled(&mut self, scaling: SpectrumScaling)
    {
        let len = self.bulk_mut().length();

        let one = T::Real::one();
        let two = one + one;
        let sqrt_2 = T::Real::SQRT_2();

        fst_ii::fst_ii_unscaled(self, None);
        if matches!(scaling, SpectrumScaling::Balanced)
        {
            self.bulk_mut()
                .last()
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
    fn dst_iii_scaled(&mut self, scaling: SpectrumScaling)
    {
        let len = self.bulk_mut().length();

        let one = T::Real::one();
        let two = one + one;
        let sqrt_2 = T::Real::SQRT_2();

        if matches!(scaling, SpectrumScaling::Balanced)
        {
            self.bulk_mut()
                .last()
                .map(|mut x| (*x.borrow_mut(), x))
                .into_bulk()
                .for_each(|(x, mut y)| *y.borrow_mut() = x._real_mul(sqrt_2));
        }

        fst_iii::fst_iii_unscaled(self, None);
        
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
    fn dst_iv_scaled(&mut self, scaling: SpectrumScaling)
    {
        let len = self.bulk_mut().length();

        let one = T::Real::one();
        let two = one + one;

        fst_iv::fst_iv_unscaled(self, None);
        
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
        b.dst_i_scaled(SpectrumScaling::Summed);
        dst_i_direct_unscaled(&mut c);

        println!("{b:?}");
        println!("{c:?}");
        assert!(tests::approx_eq(&b, &c, 1e-5));

        let mut b = a;
        let mut c = a;
        b.dst_i();
        dst_i_direct(&mut c);

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
        b.dst_ii_scaled(SpectrumScaling::Summed);
        dst_ii_direct_unscaled(&mut c);

        println!("{b:?}");
        println!("{c:?}");
        assert!(tests::approx_eq(&b, &c, 1e-5));

        let mut b = a;
        let mut c = a;
        b.dst_ii();
        dst_ii_direct(&mut c);

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
        b.dst_iii_scaled(SpectrumScaling::Summed);
        dst_iii_direct_unscaled(&mut c);

        println!("{b:?}");
        println!("{c:?}");
        assert!(tests::approx_eq(&b, &c, 1e-5));

        let mut b = a;
        let mut c = a;
        b.dst_iii();
        dst_iii_direct(&mut c);

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
        b.dst_iv_scaled(SpectrumScaling::Summed);
        dst_iv_direct_unscaled(&mut c);

        println!("{b:?}");
        println!("{c:?}");
        assert!(tests::approx_eq(&b, &c, 1e-5));

        let mut b = a;
        let mut c = a;
        b.dst_iv();
        dst_iv_direct(&mut c);

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