use ndarray::{ArrayBase, DataMut, Ix2};
use num_complex::Complex;
use num_traits::{Float, FloatConst};

use crate::Dft;

pub trait Dft2D
{
    #[doc(alias = "fft_2d")]
    fn dft_2d(&mut self);
    #[doc(alias = "ifft_2d")]
    fn idft_2d(&mut self);
}
impl<S, T> Dft2D for ArrayBase<S, Ix2>
where
    S: DataMut<Elem = Complex<T>>,
    T: Float + FloatConst + 'static
{
    fn dft_2d(&mut self)
    {
        for mut row in self.rows_mut()
        {
            row.dft();
        }
        for mut column in self.columns_mut()
        {
            column.dft();
        }
    }
    fn idft_2d(&mut self)
    {
        for mut row in self.rows_mut()
        {
            row.idft();
        }
        for mut column in self.columns_mut()
        {
            column.idft();
        }
    }
}

#[cfg(test)]
mod test
{
    use bulks::{Bulk, IntoBulk};
use image::{GenericImage, GenericImageView, Rgba};
use ndarray::Array2;
use num_complex::Complex;
use num_traits::Zero;

    use crate::Dft2D;

    #[test]
    fn it_works()
    {
        let a = [
            [1, 2, 3],
            [4, 5, 6]
        ].into_bulk()
            .map(|r| r.into_bulk().map(|e| Complex::from(e as f64)).collect_array())
            .collect_array();

        let mut b = ndarray::arr2(&a);
        b.dft_2d();
        
        println!("{b:?}");
    }

    #[test]
    fn lena() -> Result<(), std::io::Error>
    {
        const M: usize = 64;
        const N: usize = 64;

        let mut img = image::ImageReader::open("images/lena.png")?.decode().unwrap();

        let n = img.width() as usize;
        let m = img.height() as usize;

        let [mut r, mut g, mut b]: [_; 3] = core::array::from_fn(|c| {
            Array2::from_shape_fn((n as usize, m as usize), |(i, j)| {
                let p = img.get_pixel(i as u32, j as u32);
                Complex::from(p.0[c] as f64/255.0)
            })
        });
        
        println!("DFT begin.");
        r.dft_2d();
        println!("DFT done.");
        g.dft_2d();
        println!("DFT done.");
        b.dft_2d();
        println!("DFT done.");

        for i in 0..n
        {
            for j in 0..m
            {
                let r = (r[(i, j)].re*255.0).max(0.0).min(255.0) as u8;
                let g = (g[(i, j)].re*255.0).max(0.0).min(255.0) as u8;
                let b = (b[(i, j)].re*255.0).max(0.0).min(255.0) as u8;
                img.put_pixel(i as u32, j as u32, Rgba([r, g, b, 255]))
            }
        }

        img.save("images/lena_dft_2d_transformed_re.png").unwrap();

        for i in 0..n
        {
            for j in 0..m
            {
                let r = (r[(i, j)].im*255.0).max(0.0).min(255.0) as u8;
                let g = (g[(i, j)].im*255.0).max(0.0).min(255.0) as u8;
                let b = (b[(i, j)].im*255.0).max(0.0).min(255.0) as u8;
                img.put_pixel(i as u32, j as u32, Rgba([r, g, b, 255]))
            }
        }

        img.save("images/lena_dft_2d_transformed_im.png").unwrap();

        // Truncate
        for j in M/2..(m - M/2)
        {
            for i in 0..n
            {
                r[(i, j)] = Complex::zero();
                g[(i, j)] = Complex::zero();
                b[(i, j)] = Complex::zero();
            }
        }
        for i in N/2..(n - N/2)
        {
            for j in (0..M/2).chain((m - M/2)..m)
            {
                r[(i, j)] = Complex::zero();
                g[(i, j)] = Complex::zero();
                b[(i, j)] = Complex::zero();
            }
        }

        println!("IDFT begin.");
        r.idft_2d();
        println!("IDFT done.");
        g.idft_2d();
        println!("IDFT done.");
        b.idft_2d();
        println!("IDFT done.");

        for i in 0..n
        {
            for j in 0..m
            {
                let r = (r[(i, j)].re*255.0).max(0.0).min(255.0) as u8;
                let g = (g[(i, j)].re*255.0).max(0.0).min(255.0) as u8;
                let b = (b[(i, j)].re*255.0).max(0.0).min(255.0) as u8;
                img.put_pixel(i as u32, j as u32, Rgba([r, g, b, 255]))
            }
        }

        img.save("images/lena_dft_2d.png").unwrap();

        Ok(())
    }
}