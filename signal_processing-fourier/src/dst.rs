use bulks::{Bulk, CollectNearest, IntoBulk, Map};
use num_complex::{Complex, ComplexFloat};
use crate::{DstInplace, DftInplace, util::IntoComplex};

pub trait Dst: Bulk<Item: ComplexFloat>
{
    type Output: DstInplace<Item = Self::Item>;

    #[doc(alias = "idst_iv")]
    fn dst_i(self) -> Self::Output;
    #[doc(alias = "idst_iii")]
    fn dst_ii(self) -> Self::Output;
    #[doc(alias = "idst_ii")]
    fn dst_iii(self) -> Self::Output;
    #[doc(alias = "idst_i")]
    fn dst_iv(self) -> Self::Output;
}
impl<B, T, N> Dst for B
where
    B: Bulk<Item = T> + CollectNearest<Nearest = N>,
    T: ComplexFloat + 'static,
    N: IntoBulk<IntoBulk: DstInplace<Item = Self::Item>>
{
    type Output = N::IntoBulk;

    fn dst_i(self) -> Self::Output
    {
        let mut bulk = self.collect_nearest()
            .into_bulk();
        bulk.dst_i_inplace();
        bulk
    }
    fn dst_ii(self) -> Self::Output
    {
        let mut bulk = self.collect_nearest()
            .into_bulk();
        bulk.dst_ii_inplace();
        bulk
    }
    fn dst_iii(self) -> Self::Output
    {
        let mut bulk = self.collect_nearest()
            .into_bulk();
        bulk.dst_iii_inplace();
        bulk
    }
    fn dst_iv(self) -> Self::Output
    {
        let mut bulk = self.collect_nearest()
            .into_bulk();
        bulk.dst_iv_inplace();
        bulk
    }
}

#[cfg(test)]
mod test
{
    use core::f64::consts::TAU;

    use bulks::{Bulk, CollectNearest, IntoBulk};
    use linspace::Linspace;

    use crate::Dst;

    #[test]
    fn it_works()
    {
        const N: usize = 1024;
        const T: f64 = 1.0;
        const F: f64 = 220.0;
        
        let x: [_; N] = core::array::from_fn(|i| (TAU*F*i as f64/N as f64*T).sin());

        let w = (0.0..TAU).linspace_array::<N>();
        let xf = [
            x.into_bulk()
                .dst_i()
                .collect_nearest(),
            x.into_bulk()
                .dst_ii()
                .collect_nearest(),
            x.into_bulk()
                .dst_iii()
                .collect_nearest(),
            x.into_bulk()
                .dst_iv()
                .collect_nearest()
        ];

        ezplot::plot_curves("X(e^jw)", "plots/x_z_dst.png", xf.map(|xf| w.into_bulk().zip(xf)))
            .unwrap()
    }
}