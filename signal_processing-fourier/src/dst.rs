use array_trait::length;
use bulks::{AsBulk, Bulk, DoubleEndedBulk, IntoBulk};
use num_complex::{Complex, ComplexFloat};
use num_traits::{Float, One, FloatConst, NumCast, Zero};
use crate::{Dft, Permute, util::TruncateIm};

pub trait Dst: Permute
{
    #[doc(alias = "idst_i")]
    fn dst_i(&mut self);
    #[doc(alias = "idst_iii")]
    fn dst_ii(&mut self);
    #[doc(alias = "idst_ii")]
    fn dst_iii(&mut self);
    #[doc(alias = "idst_iv")]
    fn dst_iv(&mut self);
}
impl<B, T> Dst for B
where
    for<'a> &'a mut B: IntoBulk<Item = &'a mut T>,
    for<'a> &'a B: IntoBulk<Item = &'a T, IntoBulk: DoubleEndedBulk>,
    B: ?Sized,
    T: ComplexFloat + 'static
{
    fn dst_i(&mut self)
    {
        let len = (*self).bulk().length();
        if length::value::le(len, [(); 1])
        {
            return
        }
        let len_p1 = length::value::add(len, [(); 1]);
        let lenf_p1 = <<T as ComplexFloat>::Real as NumCast>::from(length::value::len(len_p1)).unwrap();

        let one = <T as ComplexFloat>::Real::one();
        let two = one + one;

        let mut y = bulks::once(Zero::zero())
            .chain(
                (*self).bulk()
                    .map(|x| Complex { re: x.re(), im: x.im() })
            )
            .chain(bulks::once(Zero::zero()))
            .chain(
                (*self).bulk()
                    .rev()
                    .map(|x| -Complex { re: x.re(), im: x.im() })
            )
            .collect::<Vec<_>, _>();
        y.dft();

        let (y1, y2) = y.into_bulk().split_at([(); 1]).1.split_at(len);

        let y_div = Complex::new(Zero::zero(), -Float::sqrt(two*lenf_p1));
        for (y, x) in y1.into_iter()
            .zip(y2.rev())
            .map(|(y1, y2)| (y1 - y2)/two)
            .map(|y| y/y_div)
            .zip(self.bulk_mut())
        {
            *x = <T as TruncateIm>::truncate_im(y)
        }
    }
    fn dst_ii(&mut self)
    {
        let len = (*self).bulk().length();
        if length::value::le(len, [(); 1])
        {
            return
        }
        let lenf = <<T as ComplexFloat>::Real as NumCast>::from(length::value::len(len)).unwrap();

        let frac_1_sqrt_2 = <T as ComplexFloat>::Real::FRAC_1_SQRT_2();
        let frac_pi_2 = <T as ComplexFloat>::Real::FRAC_PI_2();

        let mut y = (*self).bulk()
            .map(|x| Complex { re: x.re(), im: x.im() })
            .chain(
                (*self).bulk()
                    .rev()
                    .map(|x| -Complex { re: x.re(), im: x.im() })
            ).collect::<Vec<_>, _>();
        y.dft();
    
        let zero = <T as ComplexFloat>::Real::zero();
        let one = <T as ComplexFloat>::Real::one();
        let two = one + one;
    
        let mul = Complex::new(zero, Float::recip(Float::sqrt(lenf))/two);
        let (y1, y2) = y.into_bulk()
            .split_at([(); 1]).1
            .map(|y| y*mul)
            .split_at(len);
    
        let m1 = bulks::range([(); 1], len)
            .map(|i| {
                let i = <<T as ComplexFloat>::Real as NumCast>::from(i).unwrap();
                Complex::from_polar(frac_1_sqrt_2, -i*frac_pi_2/lenf)
            }).chain(core::iter::once(Complex::new(zero, -one)));
    
        let m2 = bulks::range([(); 1], len)
            .map(|i| {
                let i = <<T as ComplexFloat>::Real as NumCast>::from(i).unwrap();
                Complex::from_polar(-frac_1_sqrt_2, i*frac_pi_2/lenf)
            });
    
        for (y, x) in y1.into_iter()
            .zip(m1)
            .map(|(y, m1)| y*m1)
            .zip(y2.rev()
                .zip(m2)
                .map(|(y, m2)| y*m2)
                .chain(core::iter::once(Zero::zero()))
            ).map(|(y1, y2)| y1 + y2)
            .zip(self.bulk_mut())
        {
            *x = TruncateIm::truncate_im(y);
        }
    }
    fn dst_iii(&mut self)
    {
        let len = (*self).bulk().length();
        if length::value::le(len, [(); 1])
        {
            return
        }
        let lenf = <<T as ComplexFloat>::Real as NumCast>::from(length::value::len(len)).unwrap();

        let frac_1_sqrt_2 = <T as ComplexFloat>::Real::FRAC_1_SQRT_2();
        let frac_pi_2 = <T as ComplexFloat>::Real::FRAC_PI_2();

        let m1 = bulks::range([(); 1], len)
            .map(|i| {
                let i = <<T as ComplexFloat>::Real as NumCast>::from(i).unwrap();
                Complex::from_polar(frac_1_sqrt_2, i*frac_pi_2/lenf)
            }).chain(bulks::once(Complex::i()));
        let m2 = bulks::range([(); 1], len)
            .map(|i| {
                let i = <<T as ComplexFloat>::Real as NumCast>::from(i).unwrap();
                Complex::from_polar(-frac_1_sqrt_2, -i*frac_pi_2/lenf)
            })
            .rev();
        
        let mut y = bulks::once(Zero::zero())
            .chain(
                (*self).bulk()
                    .map(|x| Complex { re: x.re(), im: x.im() })
                    .zip(m1)
                    .map(|(x, m1)| m1*x)
            )
            .chain(
                (*self).bulk()
                    .rev()
                    .map(|x| Complex { re: x.re(), im: x.im() })
                    .zip(m2)
                    .map(|(x, m2)| m2*x)
            ).collect::<Vec<_>, _>();
        y.idft();
        
        let one = <T as ComplexFloat>::Real::one();
        let two = one + one;
        let ymul = Complex::new(Zero::zero(), -Float::sqrt(lenf)*two);
        for (mut y, x) in y.into_bulk()
            .zip(self.bulk_mut())
        {
            y = y*ymul;
            *x = <_ as TruncateIm>::truncate_im(y)
        }

    }
    fn dst_iv(&mut self)
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
            .map(|x| Complex { re: x.re(), im: x.im() })
            .zip(m1.bulk())
            .map(|(x, &m1)| m1*x)
            .chain(
                (*self).bulk()
                    .rev()
                    .map(|x| Complex { re: x.re(), im: x.im() })
                    .zip(m2.bulk()
                        .rev()
                    ).map(|(x, &m2)| m2*x)
            ).collect::<Vec<_>, _>();
        y.dft();

        let zero = <T as ComplexFloat>::Real::zero();

        let ymul = Complex::new(zero, frac_1_sqrt_2/Float::sqrt(lenf))*Complex::cis(-frac_pi_4/lenf);
        let (y1, y2) = y.into_bulk()
            .map(|y| y*ymul)
            .split_at(len);
        
        for (y, x) in y1.into_iter()
            .zip(m1)
            .map(|(y, m1)| y*m1)
            .zip(y2.rev()
                .zip(m2)
                .map(|(y, m2)| y*m2)
            ).map(|(y1, y2)| y1 + y2)
            .zip(self.bulk_mut())
        {
            *x = TruncateIm::truncate_im(y);
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
            { let mut y = x; y.dst_i(); y },
            { let mut y = x; y.dst_ii(); y },
            { let mut y = x; y.dst_iii(); y },
            { let mut y = x; y.dst_iv(); y }
        ];

        ezplot::plot_curves("X(e^jw)", "plots/x_z_dst.png", xf.map(|xf| w.into_bulk().zip(xf)))
            .unwrap()
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
}