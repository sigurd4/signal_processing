use core::borrow::{Borrow, BorrowMut};

use array_trait::length;
use bulks::{AsBulk, Bulk, CollectNearest, DoubleEndedBulk, IntoBulk, Map};
use num_complex::ComplexFloat;

use crate::Dct;

pub trait Dct2D<T>
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
impl<B, R, T> Dct2D<T> for B
where
    for<'a> &'a mut B: IntoBulk<Item = &'a mut R>,
    for<'a> &'a mut R: IntoBulk<Item = &'a mut T>,
    for<'a> &'a R: IntoBulk<Item = &'a T, IntoBulk: DoubleEndedBulk>,
    B: ?Sized,
    T: ComplexFloat + 'static
{
    fn dct_i_2d(&mut self)
    {
        let len = self.bulk_mut()
            .map(|row| row.bulk_mut().len())
            .min();
        for mut bulk in self.bulk_mut()
        {
            bulk.dct_i();
        }
        /*for i in 0..len
        {
            let mut v = bulks.bulk_mut()
                .map(|bulk| bulk.bulk_mut().get_mut(i).ok_or(T::zero()))
                .collect::<Vec<_>>();

            crate::util::IndirectReffable(&mut v).$transform()
        }*/
        //crate::transform_2d_inplace!(self.dct_i())
    }
    fn dct_ii_2d(&mut self)
    {
        let len = self.bulk_mut()
            .map(|row| row.bulk_mut().len())
            .min();
        for mut bulk in self.bulk_mut()
        {
            bulk.dct_ii();
        }
        //crate::transform_2d_inplace!(self.dct_ii())
    }
    fn dct_iii_2d(&mut self)
    {
        let len = self.bulk_mut()
            .map(|row| row.bulk_mut().len())
            .min();
        for mut bulk in self.bulk_mut()
        {
            bulk.dct_iii();
        }
        //crate::transform_2d_inplace!(self.dct_iii())
    }
    fn dct_iv_2d(&mut self)
    {
        let len = self.bulk_mut()
            .map(|row| row.bulk_mut().len())
            .min();
        for mut bulk in self.bulk_mut()
        {
            bulk.dct_iv();
        }
        //crate::transform_2d_inplace!(self.dct_iv())
    }
}

#[cfg(test)]
mod test
{
    use bulks::{AsBulk, Bulk, IntoBulk};
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
    fn lena() -> Result<(), std::io::Error>
    {
        const M: usize = 64;
        const N: usize = 64;

        let mut img = image::ImageReader::open("images/lena.png")?.decode().unwrap();

        let n = img.width() as usize;
        let m = img.height() as usize;

        let [mut r, mut g, mut b]: [Vec<Vec<_>>; 3] = core::array::from_fn(|c| {
            (0..n).map(|j| (0..m).map(|i| {
                        let p = img.get_pixel(j as u32, i as u32);
                        p.0[0] as f64/255.0
                    }).collect()
                ).collect()
        });
        /*let [mut r, mut g, mut b] = pixels.bulk_mut()
            .map(|pixels| pixels.chunks_mut(m).collect::<Vec<_>>())
            .collect_array();*/
        
        r.dct_ii_2d();
        g.dct_ii_2d();
        b.dct_ii_2d();

        for (i, ((r, g), b)) in r.iter()
            .zip(&g)
            .zip(&b)
            .enumerate()
        {
            for (j, ((&r, &g), &b)) in r.iter()
                .zip(g)
                .zip(b)
                .enumerate()
            {
                let r = (r*255.0).max(0.0).min(255.0) as u8;
                let g = (g*255.0).max(0.0).min(255.0) as u8;
                let b = (b*255.0).max(0.0).min(255.0) as u8;
                img.put_pixel(i as u32, j as u32, Rgba([r, g, b, 255]))
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

        r.dct_iii_2d();
        g.dct_iii_2d();
        b.dct_iii_2d();

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