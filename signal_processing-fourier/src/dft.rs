use bulks::{Bulk, CollectNearest, IntoBulk, Map};
use num_complex::{Complex, ComplexFloat};
use crate::{DftInplace, util::IntoComplex};

pub trait Dft: Bulk<Item: ComplexFloat>
{
    type Output: DftInplace<Item = Complex<<Self::Item as ComplexFloat>::Real>>;

    fn dft(self) -> Self::Output;
}
impl<B, T, N> Dft for B
where
    B: Bulk<Item = T>,
    T: ComplexFloat + 'static,
    Map<B, fn(T) -> Complex<T::Real>>: CollectNearest<Nearest = N>,
    N: IntoBulk<IntoBulk: DftInplace<Item = Complex<<Self::Item as ComplexFloat>::Real>>>
{
    type Output = N::IntoBulk;

    fn dft(self) -> Self::Output
    {
        let mut bulk = self.map(IntoComplex::into_complex as fn(_) -> _)
            .collect_nearest()
            .into_bulk();
        bulk.dft_inplace();
        bulk
    }
}

#[cfg(test)]
mod test
{
    use core::f64::consts::TAU;

    use bulks::{Bulk, CollectNearest, IntoBulk};
use linspace::Linspace;

    use crate::Dft;

    #[test]
    fn test()
    {
        const N: usize = 1024;
        const T: f64 = 1.0;
        const F: f64 = 220.0;
        
        let x: [_; N] = core::array::from_fn(|i| (TAU*F*i as f64/N as f64*T).sin());

        let w = (0.0..TAU).linspace_array::<N>();
        let xf = x.into_bulk()
            .dft()
            .collect_nearest();

        ezplot::plot_curves("X(e^jw)", "plots/x_z_dft.png", [&w.into_bulk().zip(xf.map(|xf| xf.norm())).collect_array()])
            .unwrap()
    }
}