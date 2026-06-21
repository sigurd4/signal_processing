use core::borrow::{Borrow, BorrowMut};

use array_trait::length;
use bulks::{AsBulk, Bulk, DoubleEndedBulk, IntoBulk};
use num_complex::{Complex, ComplexFloat};
use num_traits::{Float, FloatConst, NumCast, One, Zero};
use crate::{Dft, Permute, util::TruncateIm};

pub trait Dct<T>: Permute<T>
where
    T: ComplexFloat
{
    #[doc(alias = "idct_iv")]
    fn dct_i(&mut self);
    #[doc(alias = "idct_iii")]
    fn dct_ii(&mut self);
    #[doc(alias = "idct_ii")]
    fn dct_iii(&mut self);
    #[doc(alias = "idct_i")]
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
        let lenf = <<T as ComplexFloat>::Real as NumCast>::from(length::value::len(len)).unwrap();
        let sqrt_len = Float::sqrt(lenf);

        let one = <T as ComplexFloat>::Real::one();
        let two = one + one;
        let frac_1_sqrt_2 = <T as ComplexFloat>::Real::FRAC_1_SQRT_2();
        let frac_pi_2 = <T as ComplexFloat>::Real::FRAC_PI_2();

        let w1 = bulks::once(From::from(Float::recip(sqrt_len)))
            .chain(bulks::range([(); 1], len)
                .map(|i| {
                    let i = <<T as ComplexFloat>::Real as NumCast>::from(i).unwrap();
                    Complex::from_polar(frac_1_sqrt_2/sqrt_len, -frac_pi_2/lenf*i)
                })
            );
        let w2 = bulks::range([(); 1], len)
            .map(|i| {
                let i = <<T as ComplexFloat>::Real as NumCast>::from(i).unwrap();
                Complex::from_polar(frac_1_sqrt_2/sqrt_len, frac_pi_2/lenf*i)
            });

        let mut y = (*self).bulk()
            .chain((*self).bulk().rev())
            .map(|x| Complex { re: x.borrow().re(), im: x.borrow().im() })
            .collect::<Vec<_>, _>();
        y.dft();

        let (y1, y2) = y.into_bulk().split_at(len);

        for ((y1, y2), mut x) in y1.into_iter()
            .zip(w1)
            .map(|(y, w)| y*w)
            .zip(bulks::once(Zero::zero())
                .chain(
                    y2.rev()
                        .zip(w2)
                        .map(|(y, w)| y*w)
                )
            )
            .zip(self.bulk_mut())
        {
            let y = (y1 + y2)/two;
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
            });

        let mut y = (*self).bulk()
            .chain((*self).bulk().rev())
            .map(|x| Complex { re: x.borrow().re(), im: x.borrow().im() })
            .collect::<Vec<_>, _>();
        y.dft();
        let (y1, y2) = y.into_bulk().split_at(len);

        let one = <T as ComplexFloat>::Real::one();
        let two = one + one;
        let ydiv = Float::sqrt(lenf)*two;

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
            let y = (y1 + y2)/ydiv;
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
            .chain(
                (*self).bulk()
                    .skip([(); 1])
                    .rev()
                    .map(|x| Complex { re: x.borrow().re(), im: x.borrow().im() })
                    .zip(m2)
                    .map(|(x, m2)| m2*x)
            ).collect::<Vec<_>, _>();
        y.dft();
        
        let ydiv = Float::sqrt(lenf);
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
        let sqrt_len = Float::sqrt(lenf);

        let one = <T as ComplexFloat>::Real::one();
        let two = one + one;
        let sqrt_2 = <T as ComplexFloat>::Real::SQRT_2();
        let frac_pi_2 = <T as ComplexFloat>::Real::FRAC_PI_2();
        let pi = <T as ComplexFloat>::Real::PI();

        let w1 = bulks::once(From::from(two*sqrt_len))
            .chain(bulks::range([(); 1], len)
                .map(|i| {
                    let i = <<T as ComplexFloat>::Real as NumCast>::from(i).unwrap();
                    Complex::from_polar(sqrt_2*sqrt_len, frac_pi_2/lenf*i)
                })
            );

        let y1: Vec<_> = (*self).bulk()
            .map(|x| Complex { re: x.borrow().re(), im: x.borrow().im() })
            .zip(w1)
            .map(|(x, w)| x*w)
            .collect();

        let w2 = bulks::range([(); 1], len)
            .map(|i| {
                let i = <<T as ComplexFloat>::Real as NumCast>::from(i).unwrap();
                Complex::cis(-pi/lenf*i)
            }).rev();

        let y2: Vec<_> = y1.bulk()
            .copied()
            .rev()
            .zip(w2)
            .map(|(x, w)| x*w)
            .collect();
        
        let mut y: Vec<_> = y1.into_bulk()
            .chain(bulks::once(Zero::zero()))
            .chain(y2.into_bulk())
            .collect();
        y.idft();
        
        for (y, mut x) in y.into_bulk()
            .zip(self.bulk_mut())
        {
            *x.borrow_mut() = <_ as TruncateIm>::truncate_im(y)
        }
    }
}

#[cfg(test)]
mod test
{
    use core::f64::consts::TAU;

    use bulks::{AsBulk, Bulk, IntoBulk};
    use linspace::Linspace;

    use crate::{Dct, Dst, tests};

    #[test]
    fn it_works()
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
        b.dct_iv();

        assert!(tests::approx_eq(&a, &b, 1e-5));

        let mut b = a;
        b.dct_ii();
        b.dct_iii();

        assert!(tests::approx_eq(&a, &b, 1e-5))
    }

    #[test]
    fn test_dct_i()
    {
        let a = [1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11]
            .into_bulk()
            .map(|x| x as f32)
            .collect_array();

        let mut b = a;
        b.dct_i();

        println!("{b:?}")
    }

    #[test]
    fn test_dct_ii()
    {
        let a = [1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11]
            .into_bulk()
            .map(|x| x as f32)
            .collect_array();

        let mut b = a;
        b.dct_ii();

        println!("{b:?}")
    }

    #[test]
    fn test_dct_iii()
    {
        let a = [1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11]
            .into_bulk()
            .map(|x| x as f32)
            .collect_array();

        let mut b = a;
        b.dct_iii();

        println!("{b:?}")
    }

    #[test]
    fn test_dct_iv()
    {
        let a = [1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11]
            .into_bulk()
            .map(|x| x as f32)
            .collect_array();

        let mut b = a;
        b.dct_iv();

        println!("{b:?}")
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