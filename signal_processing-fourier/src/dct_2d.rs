use bulks::{Bulk, CollectNearest, DoubleEndedBulk, IntoBulk, Map};
use num_complex::ComplexFloat;

use crate::Dct;

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
impl<B, R, T> Dct2D for B
where
    for<'a> &'a mut B: IntoBulk<Item = &'a mut R>,
    for<'a> &'a B: IntoBulk<Item = &'a R, IntoBulk: DoubleEndedBulk>,
    for<'a> &'a mut R: IntoBulk<Item = &'a mut T>,
    for<'a> &'a R: IntoBulk<Item = &'a T, IntoBulk: DoubleEndedBulk>,
    B: ?Sized,
    T: ComplexFloat + 'static
{
    fn dct_i_2d(&mut self)
    {
        crate::transform_2d_inplace!(self.dct_i())
    }
    fn dct_ii_2d(&mut self)
    {
        crate::transform_2d_inplace!(self.dct_ii())
    }
    fn dct_iii_2d(&mut self)
    {
        crate::transform_2d_inplace!(self.dct_iii())
    }
    fn dct_iv_2d(&mut self)
    {
        crate::transform_2d_inplace!(self.dct_iv())
    }
}

#[cfg(test)]
mod test
{
    use bulks::{Bulk, IntoBulk};
    use image::{GenericImage, GenericImageView, Rgba};

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

        let mut b = a;
        b.dct_i_2d();
        
        println!("{b:?}");
    }

    #[test]
    fn test() -> Result<(), std::io::Error>
    {
        const M: usize = 64;
        const N: usize = 64;

        let mut img = image::ImageReader::open("images/lena.png")?.decode().unwrap();

        let n = img.width() as usize;
        let m = img.height() as usize;

        let mut r = bulks::range([(); 0], n).map(|j| bulks::range([(); 0], m).map(|i| {
                let p = img.get_pixel(j as u32, i as u32);
                p.0[0] as f64/255.0
            })
        ).dct_ii_2d()
            .map(Bulk::collect)
            .collect::<Vec<Vec<f64>>, _>();
        let mut g = bulks::range([(); 0], n).map(|j| bulks::range([(); 0], m).map(|i| {
                let p = img.get_pixel(j as u32, i as u32);
                p.0[1] as f64/255.0
            })
        ).dct_ii_2d()
            .map(Bulk::collect)
            .collect::<Vec<Vec<f64>>, _>();
        let mut b = bulks::range([(); 0], n).map(|j| bulks::range([(); 0], m).map(|i| {
                let p = img.get_pixel(j as u32, i as u32);
                p.0[2] as f64/255.0
            })
        ).dct_ii_2d()
            .map(Bulk::collect)
            .collect::<Vec<Vec<f64>>, _>();
        
        for i in 0..m
        {
            for j in 0..n
            {
                let r = (r[i][j]*255.0).max(0.0).min(255.0) as u8;
                let g = (g[i][j]*255.0).max(0.0).min(255.0) as u8;
                let b = (b[i][j]*255.0).max(0.0).min(255.0) as u8;
                img.put_pixel(j as u32, i as u32, Rgba([r, g, b, 255]))
            }
        }

        img.save("images/lena_dct_2d_transformed.png").unwrap();

        // Truncate
        for i in M..m
        {
            for j in 0..n
            {
                r[i][j] = 0.0;
                g[i][j] = 0.0;
                b[i][j] = 0.0;
            }
        }
        for j in N..n
        {
            for i in 0..M
            {
                r[i][j] = 0.0;
                g[i][j] = 0.0;
                b[i][j] = 0.0;
            }
        }

        r = r.into_bulk().dct_iii_2d().map(Bulk::collect).collect::<Vec<Vec<_>>, _>();
        g = g.into_bulk().dct_iii_2d().map(Bulk::collect).collect::<Vec<Vec<_>>, _>();
        b = b.into_bulk().dct_iii_2d().map(Bulk::collect).collect::<Vec<Vec<_>>, _>();

        for i in 0..m
        {
            for j in 0..n
            {
                let r = (r[i][j]*255.0).max(0.0).min(255.0) as u8;
                let g = (g[i][j]*255.0).max(0.0).min(255.0) as u8;
                let b = (b[i][j]*255.0).max(0.0).min(255.0) as u8;
                img.put_pixel(j as u32, i as u32, Rgba([r, g, b, 255]))
            }
        }

        img.save("images/lena_dct_2d.png").unwrap();

        Ok(())
    }
}