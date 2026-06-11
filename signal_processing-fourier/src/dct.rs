use bulks::{Bulk, CollectNearest, IntoBulk, Map};
use num_complex::{Complex, ComplexFloat};
use crate::{DctInplace, DftInplace, util::IntoComplex};

pub trait Dct: Bulk<Item: ComplexFloat>
{
    type Output: DctInplace<Item = Self::Item>;

    #[doc(alias = "idct_iv")]
    fn dct_i(self) -> Self::Output;
    #[doc(alias = "idct_iii")]
    fn dct_ii(self) -> Self::Output;
    #[doc(alias = "idct_ii")]
    fn dct_iii(self) -> Self::Output;
    #[doc(alias = "idct_i")]
    fn dct_iv(self) -> Self::Output;
}
impl<B, T, N> Dct for B
where
    B: Bulk<Item = T> + CollectNearest<Nearest = N>,
    T: ComplexFloat + 'static,
    N: IntoBulk<IntoBulk: DctInplace<Item = Self::Item>>
{
    type Output = N::IntoBulk;

    fn dct_i(self) -> Self::Output
    {
        let mut bulk = self.collect_nearest()
            .into_bulk();
        bulk.dct_i_inplace();
        bulk
    }
    fn dct_ii(self) -> Self::Output
    {
        let mut bulk = self.collect_nearest()
            .into_bulk();
        bulk.dct_ii_inplace();
        bulk
    }
    fn dct_iii(self) -> Self::Output
    {
        let mut bulk = self.collect_nearest()
            .into_bulk();
        bulk.dct_iii_inplace();
        bulk
    }
    fn dct_iv(self) -> Self::Output
    {
        let mut bulk = self.collect_nearest()
            .into_bulk();
        bulk.dct_iv_inplace();
        bulk
    }
}

#[cfg(test)]
mod test
{
    use core::f64::consts::TAU;

    use bulks::{Bulk, CollectNearest, IntoBulk};
    use linspace::Linspace;

    use crate::Dct;

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
                .dct_i()
                .collect_nearest(),
            x.into_bulk()
                .dct_ii()
                .collect_nearest(),
            x.into_bulk()
                .dct_iii()
                .collect_nearest(),
            x.into_bulk()
                .dct_iv()
                .collect_nearest()
        ];

        ezplot::plot_curves("X(e^jw)", "plots/x_z_dct.png", xf.map(|xf| w.into_bulk().zip(xf)))
            .unwrap()
    }
}