use core::{fmt::Debug, ops::Div};

use array_trait::length;
use bulks::{AsBulk, Bulk, CollectNearest, IntoBulk};
use num_complex::{Complex, ComplexFloat};
use num_traits::{Float, FloatConst, NumCast, One, Zero};

use crate::{DftInplace, permute::Permute, util::{DivAssignSpec, TruncateIm, fft}};

pub const trait DstInplace: ~const Permute<ItemPointee: ComplexFloat<Real: Into<Self::ItemPointee>>>
{
    #[doc(alias = "idst_iv_inplace")]
    fn dst_i_inplace(&mut self);
    #[doc(alias = "idst_iii_inplace")]
    fn dst_ii_inplace(&mut self);
    #[doc(alias = "idst_ii_inplace")]
    fn dst_iii_inplace(&mut self);
    #[doc(alias = "idst_i_inplace")]
    fn dst_iv_inplace(&mut self);
}
impl<T> DstInplace for T
where
    T: Permute<ItemPointee: ComplexFloat<Real: Into<T::ItemPointee>> + Div<<T::ItemPointee as ComplexFloat>::Real, Output = T::ItemPointee>>
{
    fn dst_i_inplace(&mut self)
    {
        let len = self.length();
        if length::value::le(len, [(); 1])
        {
            return
        }
        let len_p1 = length::value::add(len, [(); 1]);
        let lenf_p1 = <<T::ItemPointee as ComplexFloat>::Real as NumCast>::from(length::value::len(len_p1)).unwrap();

        let one = <T::ItemPointee as ComplexFloat>::Real::one();
        let two = one + one;
        let half = Float::recip(two);

        let mut y = bulks::once(Zero::zero())
            .chain(
                self.each_ref()
                    .map(|x| Complex { re: x.re(), im: x.im() })
            )
            .chain(bulks::once(Zero::zero()))
            .chain(
                bulks::range([(); 0], len)
                    .rev()
                    .map(|i| *self.get(i).unwrap())
                    .map(|x| -Complex { re: x.re(), im: x.im() })
            )
            .collect::<Vec<_>, _>()
            .into_bulk();
        y.dft_inplace();

        let (y1, y2) = y.split_at([(); 1]).1.split_at(len);

        let y_div = Complex::new(Zero::zero(), -Float::sqrt(two*lenf_p1));
        for (i, y) in y1.zip(
                y2.rev()
            ).map(|(y1, y2)| (y1 - y2)/two)
            .map(|y| y/y_div)
            .enumerate()
        {
            let x = self.get_mut(i).unwrap();
            *x = <T::ItemPointee as TruncateIm>::truncate_im(y)
        }
    }
    fn dst_ii_inplace(&mut self)
    {
        let len = self.length();
        if length::value::le(len, [(); 1])
        {
            return
        }
        let len_p1 = length::value::add(len, [(); 1]);
        let lenf = <<T::ItemPointee as ComplexFloat>::Real as NumCast>::from(length::value::len(len)).unwrap();

        let frac_1_sqrt_2 = <T::ItemPointee as ComplexFloat>::Real::FRAC_1_SQRT_2();
        let frac_pi_2 = <T::ItemPointee as ComplexFloat>::Real::FRAC_PI_2();

        let mut y = self.each_ref()
            .map(|x| Complex { re: x.re(), im: x.im() })
            .chain(
                bulks::range([(); 0], len)
                    .rev()
                    .map(|i| *self.get(i).unwrap())
                    .map(|x| -Complex { re: x.re(), im: x.im() })
            ).collect::<Vec<_>, _>()
            .into_bulk();
        y.dft_inplace();
    
        let zero = <T::ItemPointee as ComplexFloat>::Real::zero();
        let one = <T::ItemPointee as ComplexFloat>::Real::one();
        let two = one + one;
    
        let mul = Complex::new(zero, Float::recip(Float::sqrt(lenf))/two);
        let (y1, y2) = y.split_at([(); 1]).1
            .map(|y| y*mul)
            .split_at(len);
    
        let m1 = bulks::range([(); 1], len)
            .map(|i| {
                let i = <<T::ItemPointee as ComplexFloat>::Real as NumCast>::from(i).unwrap();
                Complex::from_polar(frac_1_sqrt_2, -i*frac_pi_2/lenf)
            }).chain(core::iter::once(Complex::new(zero, -one)));
    
        let m2 = bulks::range([(); 1], len)
            .map(|i| {
                let i = <<T::ItemPointee as ComplexFloat>::Real as NumCast>::from(i).unwrap();
                Complex::from_polar(-frac_1_sqrt_2, i*frac_pi_2/lenf)
            });
    
        for (i, y) in y1.zip(m1)
            .map(|(y, m1)| y*m1)
            .zip(y2.rev()
                .zip(m2)
                .map(|(y, m2)| y*m2)
                .chain(core::iter::once(Zero::zero()))
            ).map(|(y1, y2)| y1 + y2)
            .enumerate()
        {
            let x = self.get_mut(i).unwrap();
            *x = TruncateIm::truncate_im(y);
        }
    }
    fn dst_iii_inplace(&mut self)
    {
        let len = self.length();
        if length::value::le(len, [(); 1])
        {
            return
        }
        let len_m1 = length::value::saturating_sub(len, [(); 1]);
        let lenf = <<T::ItemPointee as ComplexFloat>::Real as NumCast>::from(length::value::len(len)).unwrap();

        let frac_1_sqrt_2 = <T::ItemPointee as ComplexFloat>::Real::FRAC_1_SQRT_2();
        let frac_pi_2 = <T::ItemPointee as ComplexFloat>::Real::FRAC_PI_2();

        let m1 = bulks::range([(); 1], len)
            .map(|i| {
                let i = <<T::ItemPointee as ComplexFloat>::Real as NumCast>::from(i).unwrap();
                Complex::from_polar(frac_1_sqrt_2, i*frac_pi_2/lenf)
            }).chain(bulks::once(Complex::i()));
        let m2 = bulks::range([(); 1], len)
            .map(|i| {
                let i = <<T::ItemPointee as ComplexFloat>::Real as NumCast>::from(i).unwrap();
                Complex::from_polar(-frac_1_sqrt_2, -i*frac_pi_2/lenf)
            })
            .rev();
        
        let mut y = bulks::once(Zero::zero())
            .chain(
                self.each_ref()
                    .map(|x| Complex { re: x.re(), im: x.im() })
                    .zip(m1)
                    .map(|(x, m1)| m1*x)
            )
            .chain(
                bulks::range([(); 0], len_m1)
                    .rev()
                    .map(|i| *self.get(i).unwrap())
                    .map(|x| Complex { re: x.re(), im: x.im() })
                    .zip(m2)
                    .map(|(x, m2)| m2*x)
            ).collect::<Vec<_>, _>()
            .into_bulk();
        y.idft_inplace();
        
        let one = <T::ItemPointee as ComplexFloat>::Real::one();
        let two = one + one;
        let ymul = Complex::new(Zero::zero(), -Float::sqrt(lenf)*two);
        for (mut y, x) in y.zip(self.each_mut())
        {
            y = y*ymul;
            *x = <_ as TruncateIm>::truncate_im(y)
        }

    }
    fn dst_iv_inplace(&mut self)
    {
        let len = self.length();
        if length::value::le(len, [(); 1])
        {
            return
        }
        let len_m1 = length::value::saturating_sub(len, [(); 1]);
        let len_p1 = length::value::add(len, [(); 1]);
        let lenf = <<T::ItemPointee as ComplexFloat>::Real as NumCast>::from(length::value::len(len)).unwrap();

        let frac_1_sqrt_2 = <T::ItemPointee as ComplexFloat>::Real::FRAC_1_SQRT_2();
        let frac_pi_2 = <T::ItemPointee as ComplexFloat>::Real::FRAC_PI_2();
        let frac_pi_4 = <T::ItemPointee as ComplexFloat>::Real::FRAC_PI_4();

        let m1: Vec<_> = bulks::range([(); 0], len)
            .map(|i| {
                let i = <<T::ItemPointee as ComplexFloat>::Real as NumCast>::from(i).unwrap();
                Complex::from_polar(frac_1_sqrt_2, -i*frac_pi_2/lenf)
            }).chain(core::iter::once(Complex::i()))
            .collect();
        let m2: Vec<_> = bulks::range([(); 1], len_p1)
            .map(|i| {
                let i = <<T::ItemPointee as ComplexFloat>::Real as NumCast>::from(i).unwrap();
                Complex::from_polar(-frac_1_sqrt_2, i*frac_pi_2/lenf)
            }).collect();

        let mut y = self.each_ref()
            .map(|x| Complex { re: x.re(), im: x.im() })
            .zip(m1.bulk())
            .map(|(x, &m1)| m1*x)
            .chain(
                bulks::range([(); 0], len)
                    .rev()
                    .map(|i| *self.get(i).unwrap())
                    .map(|x| Complex { re: x.re(), im: x.im() })
                    .zip(m2.bulk()
                        .rev()
                    ).map(|(x, &m2)| m2*x)
            ).collect::<Vec<_>, _>()
            .into_bulk();
        y.dft_inplace();

        let zero = <T::ItemPointee as ComplexFloat>::Real::zero();

        let ymul = Complex::new(zero, frac_1_sqrt_2/Float::sqrt(lenf))*Complex::cis(-frac_pi_4/lenf);
        let (y1, y2) = y.map(|y| y*ymul)
            .split_at(len);
        
        for (i, y) in y1.zip(m1.into_iter())
            .map(|(y, m1)| y*m1)
            .zip(y2.rev()
                .zip(m2)
                .map(|(y, m2)| y*m2)
            ).map(|(y1, y2)| y1 + y2)
            .enumerate()
        {
            let x = self.get_mut(i).unwrap();
            *x = TruncateIm::truncate_im(y);
        }
    }
}

#[cfg(test)]
mod test
{
    use bulks::{Bulk, IntoBulk};

    use crate::{DstInplace, tests};

    #[test]
    fn it_works()
    {
        let a = [1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11]
            .into_bulk()
            .map(|x| x as f32)
            .collect_array();

        let mut bulk = a.into_bulk();

        bulk.dst_i_inplace();
        bulk.dst_i_inplace();

        let b = bulk.collect_array();

        assert!(tests::approx_eq(&a, &b, 1e-5));

        let mut bulk = a.into_bulk();

        bulk.dst_ii_inplace();
        bulk.dst_iii_inplace();

        let b = bulk.collect_array();

        assert!(tests::approx_eq(&a, &b, 1e-5));

        let mut bulk = a.into_bulk();

        bulk.dst_iv_inplace();
        bulk.dst_iv_inplace();

        let b = bulk.collect_array();

        assert!(tests::approx_eq(&a, &b, 1e-5));
    }

    #[test]
    fn test_dst_i()
    {
        let a = [1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11]
            .into_bulk()
            .map(|x| x as f32)
            .collect_array();

        let mut bulk = a.into_bulk();

        bulk.dst_i_inplace();

        let b = bulk.collect_array();

        println!("{b:?}")
    }

    #[test]
    fn test_dst_ii()
    {
        let a = [1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11]
            .into_bulk()
            .map(|x| x as f32)
            .collect_array();

        let mut bulk = a.into_bulk();

        bulk.dst_ii_inplace();

        let b = bulk.collect_array();

        println!("{b:?}")
    }

    #[test]
    fn test_dst_iii()
    {
        let a = [1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11]
            .into_bulk()
            .map(|x| x as f32)
            .collect_array();

        let mut bulk = a.into_bulk();

        bulk.dst_iii_inplace();

        let b = bulk.collect_array();

        println!("{b:?}")
    }

    #[test]
    fn test_dst_iv()
    {
        let a = [1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11]
            .into_bulk()
            .map(|x| x as f32)
            .collect_array();

        let mut bulk = a.into_bulk();

        bulk.dst_iv_inplace();

        let b = bulk.collect_array();

        println!("{b:?}")
    }
}