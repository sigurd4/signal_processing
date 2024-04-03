use num::{complex::ComplexFloat};

use crate::{MaybeList, ProductSequence, Zpk};


impl<'a, T1, T2, K1, K2, Z1, Z2, P1, P2> From<&'a Zpk<T1, Z1, P1, K1>> for Zpk<T2, Z2, P2, K2>
where
    T1: ComplexFloat,
    T2: ComplexFloat,
    K1: ComplexFloat<Real = T1::Real> + Into<K2>,
    K2: ComplexFloat<Real = T2::Real>,
    Z1: MaybeList<T1>,
    P1: MaybeList<T1>,
    Z2: MaybeList<T2>,
    P2: MaybeList<T2>,
    Z1::View<'a>: MaybeList<T1>,
    P1::View<'a>: MaybeList<T1>,
    ProductSequence<T1, Z1::View<'a>>: Into<ProductSequence<T2, Z2>>,
    ProductSequence<T1, P1::View<'a>>: Into<ProductSequence<T2, P2>>
{
    fn from(zpk: &'a Zpk<T1, Z1, P1, K1>) -> Self
    {
        Zpk {
            z: zpk.z.as_view().into(),
            p: zpk.p.as_view().into(),
            k: zpk.k.into()
        }
    }
}