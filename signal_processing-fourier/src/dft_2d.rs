use ndarray::{ArrayBase, DataMut, Ix2};
use num_complex::Complex;
use num_traits::{Float, FloatConst};

use crate::Dft;

pub trait Dft2D
{
    #[doc(alias = "fft_2d")]
    fn dft_2d(&mut self);
    #[doc(alias = "ifft_2d")]
    fn idft_2d(&mut self);
}
impl<S, T> Dft2D for ArrayBase<S, Ix2>
where
    S: DataMut<Elem = Complex<T>>,
    T: Float + FloatConst + 'static
{
    fn dft_2d(&mut self)
    {
        for mut row in self.rows_mut()
        {
            row.dft();
        }
        for mut column in self.columns_mut()
        {
            column.dft();
        }
    }
    fn idft_2d(&mut self)
    {
        for mut row in self.rows_mut()
        {
            row.idft();
        }
        for mut column in self.columns_mut()
        {
            column.idft();
        }
    }
}

#[cfg(test)]
mod test
{
    use bulks::{Bulk, IntoBulk};
use num_complex::Complex;

    use crate::Dft2D;

    #[test]
    fn it_works()
    {
        let a = [
            [1, 2, 3],
            [4, 5, 6]
        ].into_bulk()
            .map(|r| r.into_bulk().map(|e| Complex::from(e as f64)).collect_array())
            .collect_array();

        let mut b = ndarray::arr2(&a);
        b.dft_2d();
        
        println!("{b:?}");
    }
}