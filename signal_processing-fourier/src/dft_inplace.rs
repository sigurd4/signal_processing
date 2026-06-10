use bulks::Bulk;
use num_complex::Complex;
use num_traits::{Float, FloatConst};

use crate::{permute::Permute, util::{fft, DivAssignSpec}};

pub const trait DftInplace: ~const Permute<ItemPointee = Complex<Self::ItemReal>>
{
    type ItemReal: Float + FloatConst;

    fn dft_inplace(&mut self);
    fn idft_inplace(&mut self);
}
impl<T, F> DftInplace for T
where
    T: Permute<ItemPointee = Complex<F>>,
    F: Float + FloatConst
{
    type ItemReal = F;

    fn dft_inplace(&mut self)
    {
        fft::fft_unscaled::<_, _, false>(self, None);
    }

    fn idft_inplace(&mut self)
    {
        let norm = F::from(self.len()).unwrap();
        fft::fft_unscaled::<_, _, true>(self, None);
        self.each_mut()
            .for_each(|x| x._div_assign(norm))
    }
}

#[cfg(test)]
mod test
{
    use bulks::{Bulk, IntoBulk};
    use num_complex::Complex;

    use crate::{DftInplace, tests};

    #[test]
    fn test_dft()
    {
        let a = [1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11]
            .into_bulk()
            .map(|x| Complex::from(x as f32))
            .collect_array();

        let mut bulk = a.into_bulk();

        bulk.dft_inplace();
        bulk.idft_inplace();

        let b = bulk.collect_array();

        assert!(tests::approx_eq(&a, &b, 1e-5))
    }
}