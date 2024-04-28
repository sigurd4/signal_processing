use core::ops::{AddAssign, DivAssign, Mul, MulAssign};

use array_math::SliceMath;
use num::{complex::ComplexFloat, Complex};

use crate::{Matrix, MaybeMatrix, OwnedLists, OwnedMatrix};

pub trait Dct2d<T>: Matrix<T>
where
    T: ComplexFloat,
    Self::Owned: Matrix<T>
{
    fn dct_i_2d(self) -> Self::Owned;
    fn dct_ii_2d(self) -> Self::Owned;
    fn dct_iii_2d(self) -> Self::Owned;
    fn dct_iv_2d(self) -> Self::Owned;
}

impl<T, M> Dct2d<T> for M
where
    M: Matrix<T>,
    M::Owned: OwnedMatrix<T>,
    T: ComplexFloat<Real: Into<T>> + Into<Complex<T::Real>> + DivAssign<T::Real> + 'static,
    Complex<T::Real>: AddAssign + MulAssign + Mul<T, Output = Complex<T::Real>> + Mul<T::Real, Output = Complex<T::Real>> + DivAssign<T::Real>,
    <Self::Owned as MaybeMatrix<T>>::Transpose: OwnedMatrix<T, Transpose: Into<M::Owned>>,
{
    fn dct_i_2d(self) -> Self::Owned
    {
        let mut h = self.into_owned();
        for h in h.as_mut_slices()
        {
            h.dct_i();
        }
        let mut ht = h.matrix_transpose();
        for ht in ht.as_mut_slices()
        {
            ht.dct_i();
        }
        ht.matrix_transpose()
            .into()
    }
    fn dct_ii_2d(self) -> Self::Owned
    {
        let mut h = self.into_owned();
        for h in h.as_mut_slices()
        {
            h.dct_ii();
        }
        let mut ht = h.matrix_transpose();
        for ht in ht.as_mut_slices()
        {
            ht.dct_ii();
        }
        ht.matrix_transpose()
            .into()
    }
    fn dct_iii_2d(self) -> Self::Owned
    {
        let mut h = self.into_owned();
        for h in h.as_mut_slices()
        {
            h.dct_iii();
        }
        let mut ht = h.matrix_transpose();
        for ht in ht.as_mut_slices()
        {
            ht.dct_iii();
        }
        ht.matrix_transpose()
            .into()
    }
    fn dct_iv_2d(self) -> Self::Owned
    {
        let mut h = self.into_owned();
        for h in h.as_mut_slices()
        {
            h.dct_iv();
        }
        let mut ht = h.matrix_transpose();
        for ht in ht.as_mut_slices()
        {
            ht.dct_iv();
        }
        ht.matrix_transpose()
            .into()
    }
}

#[cfg(test)]
mod test
{
    use image::{GenericImage, GenericImageView, Rgba};
    use ndarray::Array2;

    use crate::Dct2d;

    #[test]
    fn test() -> Result<(), std::io::Error>
    {
        const M: usize = 64;
        const N: usize = 64;

        let mut img = image::io::Reader::open("images/lena.png")?.decode().unwrap();

        let n = img.width() as usize;
        let m = img.height() as usize;

        let r = Array2::from_shape_fn((m, n), |(i, j)| {
            let p = img.get_pixel(j as u32, i as u32);
            p.0[0] as f64/255.0
        });
        let g = Array2::from_shape_fn((m, n), |(i, j)| {
            let p = img.get_pixel(j as u32, i as u32);
            p.0[1] as f64/255.0
        });
        let b = Array2::from_shape_fn((m, n), |(i, j)| {
            let p = img.get_pixel(j as u32, i as u32);
            p.0[2] as f64/255.0
        });

        let mut r = r.dct_ii_2d();
        let mut g = g.dct_ii_2d();
        let mut b = b.dct_ii_2d();
        
        for i in 0..m
        {
            for j in 0..n
            {
                let r = (r[(i, j)]*255.0).max(0.0).min(255.0) as u8;
                let g = (g[(i, j)]*255.0).max(0.0).min(255.0) as u8;
                let b = (b[(i, j)]*255.0).max(0.0).min(255.0) as u8;
                img.put_pixel(j as u32, i as u32, Rgba([r, g, b, 255]))
            }
        }

        img.save("images/lena_dct_2d_transformed.png").unwrap();

        // Truncate
        for i in M..m
        {
            for j in 0..n
            {
                r[(i, j)] = 0.0;
                g[(i, j)] = 0.0;
                b[(i, j)] = 0.0;
            }
        }
        for j in N..n
        {
            for i in 0..M
            {
                r[(i, j)] = 0.0;
                g[(i, j)] = 0.0;
                b[(i, j)] = 0.0;
            }
        }

        r = r.dct_iii_2d();
        g = g.dct_iii_2d();
        b = b.dct_iii_2d();

        for i in 0..m
        {
            for j in 0..n
            {
                let r = (r[(i, j)]*255.0).max(0.0).min(255.0) as u8;
                let g = (g[(i, j)]*255.0).max(0.0).min(255.0) as u8;
                let b = (b[(i, j)]*255.0).max(0.0).min(255.0) as u8;
                img.put_pixel(j as u32, i as u32, Rgba([r, g, b, 255]))
            }
        }

        img.save("images/lena_dct_2d.png").unwrap();

        Ok(())
    }
}