use core::{iter::Sum, ops::DivAssign};

use num::{complex::ComplexFloat, traits::FloatConst, Float};
use option_trait::{Maybe, StaticMaybe};

use crate::{ComplexOp, Container, Conv2d, Matrix, MaybeMatrix, OwnedMatrix, XCorrScale, OwnedLists, Lists};

pub trait XCorr2d<X, Y, YY, Z>: Matrix<X>
where
    X: ComplexFloat + ComplexOp<Y, Output = Z>,
    Y: ComplexFloat<Real = X::Real> + Into<Z>,
    YY: MaybeMatrix<Y>,
    Z: ComplexFloat<Real = X::Real>,
    <YY::MaybeSome as StaticMaybe<YY::Some>>::MaybeOr<<YY::Some as Container<Y>>::Mapped<Z>, Self::Mapped<Z>>: Matrix<Z> + Sized,
    Self::Mapped<Z>: Conv2d<Z, Z, <YY::MaybeSome as StaticMaybe<YY::Some>>::MaybeOr<<YY::Some as Container<Y>>::Mapped<Z>, Self::Mapped<Z>>>
{
    fn xcorr_2d<SC>(
        self,
        y: YY,
        scale: SC
    ) -> <Self::Mapped<Z> as Conv2d<Z, Z, <YY::MaybeSome as StaticMaybe<YY::Some>>::MaybeOr<<YY::Some as Container<Y>>::Mapped<Z>, Self::Mapped<Z>>>>::Output
    where
        SC: Maybe<XCorrScale>,;
}

impl<T, X, XX, Y, YY, YYY, Z, ZZ> XCorr2d<X, Y, YY, Z> for XX
where
    T: Float + FloatConst + Sum,
    X: ComplexFloat<Real = T> + ComplexOp<Y, Output = Z>,
    Y: ComplexFloat<Real = T> + Into<Z>,
    XX: Matrix<X>,
    YY: MaybeMatrix<Y>,
    Z: ComplexFloat<Real = T> + DivAssign<T>,
    XX::Mapped<T>: OwnedMatrix<T> + Conv2d<T, T, YYY::Mapped<T>, Output: OwnedMatrix<T>>,
    YY::Some: Sized,
    YY::MaybeSome: Sized + StaticMaybe<YY::Some, MaybeOr<<YY::Some as Container<Y>>::Mapped<Z>, Self::Mapped<Z>> = YYY>,
    YYY: OwnedMatrix<Z, Transpose: OwnedMatrix<Y, Transpose: Into<YYY>>> + Sized,
    YYY::Mapped<T>: OwnedMatrix<T>,
    Self::Mapped<Z>: Conv2d<Z, Z, YYY, Output = ZZ>,
    ZZ: OwnedMatrix<Z>
{
    fn xcorr_2d<SC>(
        self,
        y: YY,
        scale: SC
    ) -> ZZ
    where
        SC: Maybe<XCorrScale>
    {
        let (ma, na) = self.matrix_dim();
        
        let mut y: YYY = YY::MaybeSome::maybe_or_from_fn(
            || y.into_maybe_some()
                .into_option()
                .unwrap()
                .map_into_owned(|y| y.into()),
            || self.map_to_owned(|&x| x.into())
        );
        
        let (mb, nb) = y.matrix_dim();

        let scale = scale.into_option()
            .unwrap_or_default();

        let coeff_scale = if scale == XCorrScale::Coeff
        {
            let x_sqr = self.map_to_owned(|&x| (x.conj()*x).re());
            let one = y.map_to_owned(|_| T::one());

            let b = x_sqr.to_vecs()
                .into_iter()
                .flatten()
                .sum::<T>();
            let a = x_sqr.conv_2d(one);

            Some((a, b))
        }
        else
        {
            None
        };

        for y in y.as_mut_slices()
        {
            y.reverse()
        }
        let mut y = y.matrix_transpose();
        for y in y.as_mut_slices()
        {
            y.reverse();
        }
        let y = y.matrix_transpose();

        let x = self.map_into_owned(|x| x.into());

        let mut z = x.conv_2d(y.into());

        match scale
        {
            XCorrScale::None => (),
            XCorrScale::Biased => {
                let scale = T::from(ma.min(mb)*na.min(nb)).unwrap();
                for z in z.as_mut_slices()
                {
                    for z in z.iter_mut()
                    {
                        *z /= scale
                    }
                }
            },
            XCorrScale::Unbiased => {
                let lo = (ma.min(mb), na.min(nb));
                let hi = (ma.max(mb), na.max(nb));

                let row: Vec<_> = (1..lo.1).chain(core::iter::repeat(lo.1)
                        .take(hi.1 - lo.1 + 1)
                    ).chain((1..lo.1)
                        .rev()
                    ).collect();
                    
                let col: Vec<_> = (1..lo.0).chain(core::iter::repeat(lo.0)
                        .take(hi.0 - lo.0 + 1)
                    ).chain((1..lo.0)
                        .rev()
                    ).collect();

                for (z, c) in z.as_mut_slices()
                    .into_iter()
                    .zip(col)
                {
                    for (z, r) in z.iter_mut()
                        .zip(row.iter())
                    {
                        let scale = T::from(r*c).unwrap();
                        *z /= scale
                    }
                }
            },
            XCorrScale::Coeff => {
                let (mut a, b) = coeff_scale.unwrap();
                for (z, a) in z.as_mut_slices()
                    .into_iter()
                    .zip(a.as_mut_slices())
                {
                    for (z, &mut a) in z.iter_mut()
                        .zip(a.into_iter())
                    {
                        let scale = Float::sqrt(a*b);
                        *z /= scale
                    }
                }
            }
        }

        z
    }
}

#[cfg(test)]
mod test
{
    use image::{GenericImageView, Rgb};
    use ndarray::Array2;

    use crate::{XCorr2d, XCorrScale};

    #[test]
    fn test() -> Result<(), Box<dyn std::error::Error>>
    {
        let ximg = image::io::Reader::open("images/durer.png")?.decode()?;

        let xn = ximg.width() as usize;
        let xm = ximg.height() as usize;

        let mut x = Array2::from_shape_fn((xm, xn), |(i, j)| {
            let p = ximg.get_pixel(j as u32, i as u32);
            (p.0[0] as f64 + p.0[1] as f64 + p.0[2] as f64)/255.0/3.0
        });

        let mean = x.mean().unwrap();

        for x in x.iter_mut()
        {
            *x -= mean
        }

        let yimg = image::io::Reader::open("images/durer_dog.png")?.decode()?;

        let yn = yimg.width() as usize;
        let ym = yimg.height() as usize;

        let mut y = Array2::from_shape_fn((ym, yn), |(i, j)| {
            let p = yimg.get_pixel(j as u32, i as u32);
            (p.0[0] as f64 + p.0[1] as f64 + p.0[2] as f64)/255.0/3.0
        });
        
        for y in y.iter_mut()
        {
            *y -= mean
        }

        let mut c = x.view().xcorr_2d(y.view(), XCorrScale::Biased);

        let max = c.iter()
            .map(|&c| c)
            .reduce(f64::max)
            .unwrap();
        for c in c.iter_mut()
        {
            *c /= max
        }

        let (m, n) = c.dim();

        let cimg = image::RgbImage::from_fn(n as u32, m as u32, |j, i| {
            let c = (c[(i as usize, j as usize)]*255.0).max(0.0).min(255.0).round() as u8;
            Rgb([c, c, c])
        });

        cimg.save("images/durer_xcorr_2d_biased.png")?;
        
        let mut c = x.view().xcorr_2d(y.view(), XCorrScale::Unbiased);

        let max = c.iter()
            .map(|&c| c)
            .reduce(f64::max)
            .unwrap();
        for c in c.iter_mut()
        {
            *c /= max
        }

        let (m, n) = c.dim();

        let cimg = image::RgbImage::from_fn(n as u32, m as u32, |j, i| {
            let c = (c[(i as usize, j as usize)]*255.0).max(0.0).min(255.0).round() as u8;
            Rgb([c, c, c])
        });

        cimg.save("images/durer_xcorr_2d_unbiased.png")?;
        
        let mut c = x.xcorr_2d(y, XCorrScale::Coeff);

        let max = c.iter()
            .map(|&c| c)
            .reduce(f64::max)
            .unwrap();
        for c in c.iter_mut()
        {
            *c /= max
        }

        let (m, n) = c.dim();

        let cimg = image::RgbImage::from_fn(n as u32, m as u32, |j, i| {
            let c = (c[(i as usize, j as usize)]*255.0).max(0.0).min(255.0).round() as u8;
            Rgb([c, c, c])
        });

        cimg.save("images/durer_xcorr_2d_coeff.png")?;

        Ok(())
    }
}