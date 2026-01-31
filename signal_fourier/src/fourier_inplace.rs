use bulks::{Bulk, RandomAccessBulk};
use num_complex::Complex;
use num_traits::Float;

use crate::{fft, permute::Permute, util::DivAssignSpec};

pub const trait FourierInplace: RandomAccessBulk<ItemPointee = Complex<Self::ItemReal>>
{
    type ItemReal: Float;

    fn dft_inplace(&mut self);
    fn idft_inplace(&mut self);
}
impl<T, F> FourierInplace for T
where
    T: Permute + RandomAccessBulk<ItemPointee = Complex<F>>,
    F: Float
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