use array_math::SliceMath;
use num::complex::ComplexFloat;
use option_trait::Maybe;

use crate::{ListOrSingle, MaybeList, MaybeLists, Sos, System, Tf, Zpk};

pub trait FiltOrd: System
{
    type Output: ListOrSingle<usize>;

    fn filtord(&self) -> Self::Output;
}

impl<T, B, A> FiltOrd for Tf<T, B, A>
where
    T: ComplexFloat,
    B: MaybeLists<T>,
    A: MaybeList<T>
{
    type Output = B::RowsMapped<usize>;

    fn filtord(&self) -> Self::Output
    {
        self.b.map_rows_to_owned(|b| {
            let nb = b.as_view_slice_option()
                .map(|b: &[T]| b.trim_zeros_front().len())
                .unwrap_or(1);
            let na = self.a.as_view_slice_option()
                .map(|a: &[T]| a.trim_zeros_front().len())
                .unwrap_or(1);
            nb.max(na).saturating_sub(1)
        })
    }
}

impl<T, B, A, S> FiltOrd for Sos<T, B, A, S>
where
    T: ComplexFloat,
    B: Maybe<[T; 3]> + MaybeList<T>,
    A: Maybe<[T; 3]> + MaybeList<T>,
    S: MaybeList<Tf<T, B, A>>
{
    type Output = usize;

    fn filtord(&self) -> Self::Output
    {
        self.sos.as_view_slice_option()
            .map(|sos: &[Tf<T, B, A>]| {
                let nb = sos.iter()
                    .map(|tf| tf.b.as_view_slice_option()
                        .map(|b: &[T]| b.trim_zeros_front().len())
                        .unwrap_or(1)
                    ).reduce(|a, b| if a != 0 && b != 0 {a + b - 1} else {0})
                    .unwrap_or(1);
                let na = sos.iter()
                    .map(|tf| tf.a.as_view_slice_option()
                        .map(|a: &[T]| a.trim_zeros_front().len())
                        .unwrap_or(1)
                    ).reduce(|a, b| if a != 0 && b != 0 {a + b - 1} else {0})
                    .unwrap_or(1);
                nb.max(na).saturating_sub(1)
            })
            .unwrap_or(0)
    }
}

impl<T, Z, P, K> FiltOrd for Zpk<T, Z, P, K>
where
    T: ComplexFloat,
    Z: MaybeList<T>,
    P: MaybeList<T>,
    K: ComplexFloat<Real = T::Real>
{
    type Output = usize;

    fn filtord(&self) -> Self::Output
    {
        let nz = self.z.as_view_slice_option()
            .map(|z| z.len())
            .unwrap_or(0);
        let np = self.p.as_view_slice_option()
            .map(|p| p.len())
            .unwrap_or(0);
        nz.max(np)
    }
}
