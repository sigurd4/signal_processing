use bulks::{Bulk, CollectNearest, DoubleEndedBulk, IntoBulk, Map};
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
impl<B, R, T> Dst2D for B
where
    for<'a> &'a mut B: IntoBulk<Item = &'a mut R>,
    for<'a> &'a B: IntoBulk<Item = &'a R, IntoBulk: DoubleEndedBulk>,
    for<'a> &'a mut R: IntoBulk<Item = &'a mut T>,
    for<'a> &'a R: IntoBulk<Item = &'a T, IntoBulk: DoubleEndedBulk>,
    B: ?Sized,
    T: ComplexFloat + 'static
{
    fn dst_i_2d(&mut self)
    {
        crate::transform_2d_inplace!(self.dst_i())
    }
    fn dst_ii_2d(&mut self)
    {
        crate::transform_2d_inplace!(self.dst_ii())
    }
    fn dst_iii_2d(&mut self)
    {
        crate::transform_2d_inplace!(self.dst_iii())
    }
    fn dst_iv_2d(&mut self)
    {
        crate::transform_2d_inplace!(self.dst_iv())
    }
}

#[cfg(test)]
mod test
{
    use bulks::{Bulk, IntoBulk};
    use image::{GenericImage, GenericImageView, Rgba};

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

        let mut b = a;
        b.dst_i_2d();
        
        println!("{b:?}");
    }
}