use ndarray::{ArrayBase, DataMut, Ix2};
use num_complex::{Complex, ComplexFloat};

use crate::{Dft2D, util::{IntoComplex, MulAssignSpec, TruncateIm}};

pub trait Hilbert2D
{
    fn hilbert_2d(&mut self);
}
impl<S, A> Hilbert2D for ArrayBase<S, Ix2, A>
where
    S: DataMut<Elem = A>,
    A: ComplexFloat + 'static
{
    fn hilbert_2d(&mut self)
    {
        let (n, m) = self.dim();
        let mut image = self.map(|v| v.into_complex());

        image.dft_2d();

        let mut rows = image.rows_mut()
            .into_iter()
            .skip(1);

        for r in [false, true]
        {
            for row in rows.by_ref().take((n/2).saturating_sub(1))
            {
                let mut cells = row.into_iter().skip(1);
                for c in [false, true]
                {
                    let coeff = if r ^ c
                    {
                        Complex::i()
                    }
                    else
                    {
                        -Complex::i()
                    };

                    for cell in cells.by_ref().take((m/2).saturating_sub(1))
                    {
                        cell._mul_assign(coeff);
                    }
                }
            }
        }

        image.idft_2d();

        self.iter_mut()
            .zip(image)
            .for_each(|(x, y)| *x = A::truncate_im(y))
    }
}

#[cfg(test)]
mod test
{
    use super::Hilbert2D;

    #[test]
    fn test()
    {
        let x = [
            [1.0, 2.0, 3.0, 4.0],
            [5.0, 6.0, 0.0, 8.0],
            [9.0, 10.0, 11.0, 12.0],
            [13.0, 14.0, 15.0, 16.0]
        ];
        let mut y = ndarray::arr2(&x);
        y.hilbert_2d();

        println!("{:?}", y)
    }
}