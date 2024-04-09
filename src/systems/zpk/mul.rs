use std::ops::Mul;

use num::complex::ComplexFloat;

use crate::{ComplexOp, MaybeList, ProductSequence, Zpk};

impl<T1, T2, T3, Z1, Z2, Z3, P1, P2, P3, K1, K2, K3> Mul<Zpk<T2, Z2, P2, K2>> for Zpk<T1, Z1, P1, K1>
where
    T1: ComplexFloat + ComplexOp<T2, Output = T3>,
    T2: ComplexFloat + Into<T3>,
    T3: ComplexFloat,
    K1: ComplexFloat<Real = T1::Real> + ComplexOp<K2, Output = K3>,
    K2: ComplexFloat<Real = T2::Real> + Into<K3>,
    K3: ComplexFloat<Real = T3::Real> + Mul<Output = K3>,
    Z1: MaybeList<T1>,
    Z2: MaybeList<T2>,
    Z3: MaybeList<T3>,
    P1: MaybeList<T1>,
    P2: MaybeList<T2>,
    P3: MaybeList<T3>,
    Z1::MaybeMapped<T3>: MaybeList<T3>,
    Z2::MaybeMapped<T3>: MaybeList<T3>,
    P1::MaybeMapped<T3>: MaybeList<T3>,
    P2::MaybeMapped<T3>: MaybeList<T3>,
    ProductSequence<T3, Z1::MaybeMapped<T3>>: Mul<ProductSequence<T3, Z2::MaybeMapped<T3>>, Output = ProductSequence<T3, Z3>>,
    ProductSequence<T3, P1::MaybeMapped<T3>>: Mul<ProductSequence<T3, P2::MaybeMapped<T3>>, Output = ProductSequence<T3, P3>>
{
    type Output = Zpk<T3, Z3, P3, K3>;

    fn mul(self, rhs: Zpk<T2, Z2, P2, K2>) -> Self::Output
    {
        Zpk {
            z: ProductSequence::new(self.z.into_inner().maybe_map_into_owned(|z| z.into()))
                *ProductSequence::new(rhs.z.into_inner().maybe_map_into_owned(|z| z.into())),
            p: ProductSequence::new(self.p.into_inner().maybe_map_into_owned(|p| p.into()))
                *ProductSequence::new(rhs.p.into_inner().maybe_map_into_owned(|p| p.into())),
            k: self.k.into()*rhs.k.into()
        }
    }
}