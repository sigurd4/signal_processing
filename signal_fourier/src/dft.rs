use bulks::{Bulk, CollectNearest, InplaceBulk, IntoBulk, IntoInplaceBulk};
use num_complex::{Complex, ComplexFloat};
use crate::{DftInplace, util::IntoComplex};

pub trait Dft: Bulk<Item: ComplexFloat>
{
    type Output: Bulk<Item = Complex<<Self::Item as ComplexFloat>::Real>>;

    fn dft(self) -> Self::Output;
}
impl<B, T, O> Dft for B
where
    B: Bulk<Item = T>,
    T: ComplexFloat,
    bulks::Map<B, fn(T) -> Complex<T::Real>>: IntoInplaceBulk<IntoInplaceBulk = O>,
    O: InplaceBulk
{
    type Output = impl Bulk<Item = Complex<T::Real>>;

    fn dft(self) -> Self::Output
    {
        let mut bulk = self.map(IntoComplex::into_complex)
            .into_inplace_bulk();
        bulk.dft_inplace();
        bulk
    }
}