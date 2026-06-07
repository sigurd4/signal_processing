use bulks::{Bulk, CollectNearest, IntoBulk, Map};
use num_complex::{Complex, ComplexFloat};
use crate::DftInplace;

pub trait Idft: Bulk<Item: ComplexFloat + Into<Complex<<Self::Item as ComplexFloat>::Real>>>
{
    type Output: DftInplace<Item = Complex<<Self::Item as ComplexFloat>::Real>>;

    fn idft(self) -> Self::Output;
}
impl<B, T, N> Idft for B
where
    B: Bulk<Item = T>,
    T: ComplexFloat + Into<Complex<T::Real>> + 'static,
    Map<B, fn(T) -> Complex<T::Real>>: CollectNearest<Nearest = N>,
    N: IntoBulk<IntoBulk: DftInplace<Item = Complex<<Self::Item as ComplexFloat>::Real>>>
{
    type Output = N::IntoBulk;

    fn idft(self) -> Self::Output
    {
        let mut bulk = self.map(Into::into as fn(_) -> _)
            .collect_nearest()
            .into_bulk();
        bulk.idft_inplace();
        bulk
    }
}

#[cfg(test)]
mod test
{
    use core::f64::consts::TAU;

    use bulks::{Bulk, CollectNearest, IntoBulk};
use linspace::Linspace;

    use crate::{Dft, Idft};

    #[test]
    fn test()
    {
        const N: usize = 1024;
        const T: f64 = 0.1;
        const F: f64 = 220.0;
        
        let x: [_; N] = core::array::from_fn(|i| (TAU*F*i as f64/N as f64*T).sin());

        let t: [_; N] = (0.0..T).linspace_array::<N>();
        let y = x.into_bulk()
            .dft()
            .idft()
            .collect_nearest();

        ezplot::plot_curves("x(t)", "plots/x_t_idft.png", [&t.into_bulk().zip(y.map(|y| y.re)).collect_array(), &t.into_bulk().zip(x).collect_array()])
            .unwrap()
    }
}