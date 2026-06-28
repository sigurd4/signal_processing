use ndarray::{ArrayBase, DataMut, Ix2};
use num_complex::ComplexFloat;

use crate::Dct;

/// # 2D discrete cosine-transform
/// 
/// The discrete cosine-transform is the real-valued fourier transform of the even extension of a sequence.
pub trait Dct2D
{
    #[doc(alias = "idct_iv_2d")]
    fn dct_i_2d(&mut self);
    #[doc(alias = "idct_iii_2d")]
    fn dct_ii_2d(&mut self);
    #[doc(alias = "idct_ii_2d")]
    fn dct_iii_2d(&mut self);
    #[doc(alias = "idct_i_2d")]
    fn dct_iv_2d(&mut self);
}
impl<S, A> Dct2D for ArrayBase<S, Ix2, A>
where
    S: DataMut<Elem = A>,
    A: ComplexFloat + 'static
{
    fn dct_i_2d(&mut self)
    {
        for mut row in self.rows_mut()
        {
            row.dct_i();
        }
        for mut column in self.columns_mut()
        {
            column.dct_i();
        }
    }
    fn dct_ii_2d(&mut self)
    {
        for mut row in self.rows_mut()
        {
            row.dct_ii();
        }
        for mut column in self.columns_mut()
        {
            column.dct_ii();
        }
    }
    fn dct_iii_2d(&mut self)
    {
        for mut row in self.rows_mut()
        {
            row.dct_iii();
        }
        for mut column in self.columns_mut()
        {
            column.dct_iii();
        }
    }
    fn dct_iv_2d(&mut self)
    {
        for mut row in self.rows_mut()
        {
            row.dct_iv();
        }
        for mut column in self.columns_mut()
        {
            column.dct_iv();
        }
    }
}

#[cfg(test)]
mod test
{
    use bulks::{Bulk, IntoBulk};
    use image::{GenericImage, GenericImageView, Rgba};
    use ndarray::Array2;

    use crate::Dct2D;

    #[test]
    fn it_works()
    {
        let a = [
            [1, 2, 3],
            [4, 5, 6]
        ].into_bulk()
            .map(|r| r.into_bulk().map(|e| e as f64).collect_array())
            .collect_array();

        let mut b = ndarray::arr2(&a);
        b.dct_i_2d();
        
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
                p.0[c] as f64/255.0
            })
        });
        
        println!("DCT-II begin.");
        r.dct_ii_2d();
        println!("DCT-II done.");
        g.dct_ii_2d();
        println!("DCT-II done.");
        b.dct_ii_2d();
        println!("DCT-II done.");

        for i in 0..n
        {
            for j in 0..m
            {
                let r = (r[(i, j)]*255.0).max(0.0).min(255.0) as u8;
                let g = (g[(i, j)]*255.0).max(0.0).min(255.0) as u8;
                let b = (b[(i, j)]*255.0).max(0.0).min(255.0) as u8;
                img.put_pixel(i as u32, j as u32, Rgba([r, g, b, 255]))
            }
        }

        img.save("images/lena_dct_2d_transformed.png").unwrap();

        // Truncate
        for j in M..m
        {
            for i in 0..n
            {
                r[(i, j)] = 0.0;
                g[(i, j)] = 0.0;
                b[(i, j)] = 0.0;
            }
        }
        for i in N..n
        {
            for j in 0..M
            {
                r[(i, j)] = 0.0;
                g[(i, j)] = 0.0;
                b[(i, j)] = 0.0;
            }
        }

        println!("DCT-III begin.");
        r.dct_iii_2d();
        println!("DCT-III done.");
        g.dct_iii_2d();
        println!("DCT-III done.");
        b.dct_iii_2d();
        println!("DCT-III done.");

        for i in 0..n
        {
            for j in 0..m
            {
                let r = (r[(i, j)]*255.0).max(0.0).min(255.0) as u8;
                let g = (g[(i, j)]*255.0).max(0.0).min(255.0) as u8;
                let b = (b[(i, j)]*255.0).max(0.0).min(255.0) as u8;
                img.put_pixel(i as u32, j as u32, Rgba([r, g, b, 255]))
            }
        }

        img.save("images/lena_dct_2d.png").unwrap();

        Ok(())
    }
}