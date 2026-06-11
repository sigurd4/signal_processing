use core::{fmt::Debug, ops::Div};

use array_trait::length;
use bulks::{AsBulk, Bulk, CollectNearest, IntoBulk};
use num_complex::{Complex, ComplexFloat};
use num_traits::{Float, FloatConst, NumCast, One, Zero};

use crate::{DftInplace, permute::Permute, util::{DivAssignSpec, TruncateIm, fft}};

pub const trait DctInplace: ~const Permute<ItemPointee: ComplexFloat<Real: Into<Self::ItemPointee>>>
{
    #[doc(alias = "idct_iv_inplace")]
    fn dct_i_inplace(&mut self);
    #[doc(alias = "idct_iii_inplace")]
    fn dct_ii_inplace(&mut self);
    #[doc(alias = "idct_ii_inplace")]
    fn dct_iii_inplace(&mut self);
    #[doc(alias = "idct_i_inplace")]
    fn dct_iv_inplace(&mut self);
}
impl<T> DctInplace for T
where
    T: Permute<ItemPointee: ComplexFloat<Real: Into<T::ItemPointee>> + Div<<T::ItemPointee as ComplexFloat>::Real, Output = T::ItemPointee>>
{
    fn dct_i_inplace(&mut self)
    {
        let len = self.length();
        if length::value::le(len, [(); 1])
        {
            return
        }
        let lenf = <<T::ItemPointee as ComplexFloat>::Real as NumCast>::from(length::value::len(len)).unwrap();
        let sqrt_len = Float::sqrt(lenf);

        let one = <T::ItemPointee as ComplexFloat>::Real::one();
        let two = one + one;
        let half = Float::recip(two);
        let frac_1_sqrt_2 = <T::ItemPointee as ComplexFloat>::Real::FRAC_1_SQRT_2();
        let frac_pi_2 = <T::ItemPointee as ComplexFloat>::Real::FRAC_PI_2();

        let w1 = bulks::once(From::from(Float::recip(sqrt_len)))
            .chain(bulks::range([(); 1], len)
                .map(|i| {
                    let i = <<T::ItemPointee as ComplexFloat>::Real as NumCast>::from(i).unwrap();
                    Complex::from_polar(frac_1_sqrt_2/sqrt_len, -frac_pi_2/lenf*i)
                })
            );
        let w2 = bulks::range([(); 1], len)
            .map(|i| {
                let i = <<T::ItemPointee as ComplexFloat>::Real as NumCast>::from(i).unwrap();
                Complex::from_polar(frac_1_sqrt_2/sqrt_len, frac_pi_2/lenf*i)
            });

        let mut y = bulks::range([(); 0], len)
            .chain(bulks::range([(); 0], len).rev())
            .map(|i| *self.get(i).unwrap())
            .map(|x| Complex { re: x.re(), im: x.im() })
            .collect::<Vec<_>, _>()
            .into_bulk();
        y.dft_inplace();

        let (y1, y2) = y.split_at(len);

        for (i, (y1, y2)) in y1.zip(w1)
            .map(|(y, w)| y*w)
            .zip(bulks::once(Zero::zero())
                .chain(
                    y2.rev()
                        .zip(w2)
                        .map(|(y, w)| y*w)
                )
            )
            .enumerate()
        {
            let y = (y1 + y2)/two;
            let x = self.get_mut(i).unwrap();
            *x = <T::ItemPointee as TruncateIm>::truncate_im(y)
        }
    }
    fn dct_ii_inplace(&mut self)
    {
        let len = self.length();
        if length::value::le(len, [(); 1])
        {
            return
        }
        let lenf = <<T::ItemPointee as ComplexFloat>::Real as NumCast>::from(length::value::len(len)).unwrap();

        let frac_1_sqrt_2 = <T::ItemPointee as ComplexFloat>::Real::FRAC_1_SQRT_2();
        let frac_pi_2 = <T::ItemPointee as ComplexFloat>::Real::FRAC_PI_2();

        let m1 = bulks::once(One::one())
            .chain(bulks::range([(); 1], len)
                .map(|i| {
                    let i = <<T::ItemPointee as ComplexFloat>::Real as NumCast>::from(i).unwrap();
                    Complex::from_polar(frac_1_sqrt_2, -i*frac_pi_2/lenf)
                })
            );
        let m2 = bulks::range([(); 1], len)
            .map(|i| {
                let i = <<T::ItemPointee as ComplexFloat>::Real as NumCast>::from(i).unwrap();
                Complex::from_polar(frac_1_sqrt_2, i*frac_pi_2/lenf)
            });

        let mut y = bulks::range([(); 0], len)
            .chain(bulks::range([(); 0], len).rev())
            .map(|i| *self.get(i).unwrap())
            .map(|x| Complex { re: x.re(), im: x.im() })
            .collect::<Vec<_>, _>()
            .into_bulk();
        y.dft_inplace();
        let (y1, y2) = y.split_at(len);

        let one = <T::ItemPointee as ComplexFloat>::Real::one();
        let two = one + one;
        let ydiv = Float::sqrt(lenf)*two;

        for (i, (y1, y2)) in y1
            .zip(m1)
            .map(|(y, m1)| y*m1)
            .zip(bulks::once(Zero::zero())
                .chain(y2.rev()
                    .zip(m2)
                    .map(|(y, m2)| y*m2)
                )
            ).enumerate()
        {
            let y = (y1 + y2)/ydiv;
            let x = self.get_mut(i).unwrap();
            *x = <_ as TruncateIm>::truncate_im(y)
        }
    }
    fn dct_iii_inplace(&mut self)
    {
        let len = self.length();
        if length::value::le(len, [(); 1])
        {
            return
        }
        let lenf = <<T::ItemPointee as ComplexFloat>::Real as NumCast>::from(length::value::len(len)).unwrap();

        let frac_1_sqrt_2 = <T::ItemPointee as ComplexFloat>::Real::FRAC_1_SQRT_2();
        let frac_pi_2 = <T::ItemPointee as ComplexFloat>::Real::FRAC_PI_2();

        let m1 = bulks::once(One::one())
            .chain(bulks::range([(); 1], len)
                .map(|i| {
                    let i = <<T::ItemPointee as ComplexFloat>::Real as NumCast>::from(i).unwrap();
                    Complex::from_polar(frac_1_sqrt_2, -i*frac_pi_2/lenf)
                })
            );
        let m2 = bulks::range([(); 1], len)
            .map(|i| {
                let i = <<T::ItemPointee as ComplexFloat>::Real as NumCast>::from(i).unwrap();
                Complex::from_polar(frac_1_sqrt_2, i*frac_pi_2/lenf)
            })
            .rev();
        
        let mut y = bulks::range([(); 0], len)
            .map(|i| *self.get(i).unwrap())
            .map(|x| Complex { re: x.re(), im: x.im() })
            .zip(m1)
            .map(|(x, m1)| m1*x)
            .chain(bulks::once(Zero::zero()))
            .chain(
                bulks::range([(); 1], len)
                    .rev()
                    .map(|i| *self.get(i).unwrap())
                    .map(|x| Complex { re: x.re(), im: x.im() })
                    .zip(m2)
                    .map(|(x, m2)| m2*x)
            ).collect::<Vec<_>, _>()
            .into_bulk();
        y.dft_inplace();
        
        let ydiv = Float::sqrt(lenf);
        for (mut y, x) in y.zip(self.each_mut())
        {
            y = y/ydiv;
            *x = <_ as TruncateIm>::truncate_im(y)
        }

    }
    fn dct_iv_inplace(&mut self)
    {
        let len = self.length();
        if length::value::le(len, [(); 1])
        {
            return
        }
        let lenf = <<T::ItemPointee as ComplexFloat>::Real as NumCast>::from(length::value::len(len)).unwrap();
        let sqrt_len = Float::sqrt(lenf);

        let one = <T::ItemPointee as ComplexFloat>::Real::one();
        let two = one + one;
        let sqrt_2 = <T::ItemPointee as ComplexFloat>::Real::SQRT_2();
        let frac_pi_2 = <T::ItemPointee as ComplexFloat>::Real::FRAC_PI_2();
        let pi = <T::ItemPointee as ComplexFloat>::Real::PI();

        let w1 = bulks::once(From::from(two*sqrt_len))
            .chain(bulks::range([(); 1], len)
                .map(|i| {
                    let i = <<T::ItemPointee as ComplexFloat>::Real as NumCast>::from(i).unwrap();
                    Complex::from_polar(sqrt_2*sqrt_len, frac_pi_2/lenf*i)
                })
            );

        let y1: Vec<_> = self.each_ref()
            .map(|x| Complex { re: x.re(), im: x.im() })
            .zip(w1)
            .map(|(x, w)| x*w)
            .collect();

        let w2 = bulks::range([(); 1], len)
            .map(|i| {
                let i = <<T::ItemPointee as ComplexFloat>::Real as NumCast>::from(i).unwrap();
                Complex::cis(-pi/lenf*i)
            }).rev();

        let y2: Vec<_> = y1.bulk()
            .copied()
            .rev()
            .zip(w2)
            .map(|(x, w)| x*w)
            .collect();
        
        let mut y = y1.into_bulk()
            .chain(bulks::once(Zero::zero()))
            .chain(y2.into_bulk())
            .collect::<Vec<_>, _>()
            .into_bulk();
        y.idft_inplace();
        
        for (y, x) in y.zip(self.each_mut())
        {
            *x = <_ as TruncateIm>::truncate_im(y)
        }
    }
}

#[cfg(test)]
mod test
{
    use bulks::{Bulk, IntoBulk};

    use crate::{DctInplace, tests};

    #[test]
    fn it_works()
    {
        let a = [1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11]
            .into_bulk()
            .map(|x| x as f32)
            .collect_array();

        let mut bulk = a.into_bulk();

        bulk.dct_i_inplace();
        bulk.dct_iv_inplace();

        let b = bulk.collect_array();

        assert!(tests::approx_eq(&a, &b, 1e-5));

        let mut bulk = a.into_bulk();

        bulk.dct_ii_inplace();
        bulk.dct_iii_inplace();

        let b = bulk.collect_array();

        assert!(tests::approx_eq(&a, &b, 1e-5))
    }

    #[test]
    fn test_dct_i()
    {
        let a = [1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11]
            .into_bulk()
            .map(|x| x as f32)
            .collect_array();

        let mut bulk = a.into_bulk();

        bulk.dct_i_inplace();

        let b = bulk.collect_array();

        println!("{b:?}")
    }

    #[test]
    fn test_dct_ii()
    {
        let a = [1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11]
            .into_bulk()
            .map(|x| x as f32)
            .collect_array();

        let mut bulk = a.into_bulk();

        bulk.dct_ii_inplace();

        let b = bulk.collect_array();

        println!("{b:?}")
    }

    #[test]
    fn test_dct_iii()
    {
        let a = [1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11]
            .into_bulk()
            .map(|x| x as f32)
            .collect_array();

        let mut bulk = a.into_bulk();

        bulk.dct_iii_inplace();

        let b = bulk.collect_array();

        println!("{b:?}")
    }

    #[test]
    fn test_dct_iv()
    {
        let a = [1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11]
            .into_bulk()
            .map(|x| x as f32)
            .collect_array();

        let mut bulk = a.into_bulk();

        bulk.dct_iv_inplace();

        let b = bulk.collect_array();

        println!("{b:?}")
    }
}