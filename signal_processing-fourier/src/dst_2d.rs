use ndarray::{ArrayBase, DataMut, Ix2};
use num_complex::ComplexFloat;

use crate::Dst;

pub trait Dst2D
{
    #[doc(alias = "idst_i_2d")]
    fn dst_i_2d(&mut self);
    #[doc(alias = "idst_iii_2d")]
    fn dst_ii_2d(&mut self);
    #[doc(alias = "idst_ii_2d")]
    fn dst_iii_2d(&mut self);
    #[doc(alias = "idst_iv_2d")]
    fn dst_iv_2d(&mut self);
}
impl<S, A> Dst2D for ArrayBase<S, Ix2, A>
where
    S: DataMut<Elem = A>,
    A: ComplexFloat + 'static
{
    fn dst_i_2d(&mut self)
    {
        for mut row in self.rows_mut()
        {
            row.dst_i();
        }
        for mut column in self.columns_mut()
        {
            column.dst_i();
        }
    }
    fn dst_ii_2d(&mut self)
    {
        for mut row in self.rows_mut()
        {
            row.dst_ii();
        }
        for mut column in self.columns_mut()
        {
            column.dst_ii();
        }
    }
    fn dst_iii_2d(&mut self)
    {
        for mut row in self.rows_mut()
        {
            row.dst_iii();
        }
        for mut column in self.columns_mut()
        {
            column.dst_iii();
        }
    }
    fn dst_iv_2d(&mut self)
    {
        for mut row in self.rows_mut()
        {
            row.dst_iv();
        }
        for mut column in self.columns_mut()
        {
            column.dst_iv();
        }
    }
}

#[cfg(test)]
mod test
{
    use bulks::{Bulk, IntoBulk};
use image::{GenericImage, GenericImageView, Rgba};
use ndarray::Array2;

    use crate::Dst2D;

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
        b.dst_i_2d();
        
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
        
        r.dst_ii_2d();
        g.dst_ii_2d();
        b.dst_ii_2d();

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

        img.save("images/lena_dst_2d_transformed.png").unwrap();

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

        r.dst_iii_2d();
        g.dst_iii_2d();
        b.dst_iii_2d();

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

        img.save("images/lena_dst_2d.png").unwrap();

        Ok(())
    }
}