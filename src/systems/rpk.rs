use num::complex::ComplexFloat;

use crate::{MaybeList, Polynomial, SumSequence};

moddef::moddef!(
    mod {
        add,
        neg,
        sub
    }
);

#[derive(Debug, Clone, Copy)]
pub struct Rpk<T, R, P, RP, K>
where
    T: ComplexFloat,
    R: ComplexFloat<Real = T::Real>,
    P: ComplexFloat<Real = T::Real>,
    RP: MaybeList<(R, P)>,
    K: MaybeList<T>
{
    pub rp: SumSequence<(R, P), RP>,
    pub k: Polynomial<T, K>
}

impl<T, R, P, RP, K> Rpk<T, R, P, RP, K>
where
    T: ComplexFloat,
    R: ComplexFloat<Real = T::Real>,
    P: ComplexFloat<Real = T::Real>,
    RP: MaybeList<(R, P)>,
    K: MaybeList<T>
{
    pub fn new(rp: RP, k: K) -> Self
    {
        Self {
            rp: SumSequence::new(rp),
            k: Polynomial::new(k)
        }
    }

    pub type Owned = Rpk<T, R, P, RP::Owned, K::Owned>
    where
        RP::Owned: MaybeList<(R, P)>,
        K::Owned: MaybeList<T>;
    pub type View<'a> = Rpk<T, R, P, RP::View<'a>, K::View<'a>>
    where
        Self: 'a,
        RP::View<'a>: MaybeList<(R, P)>,
        K::View<'a>: MaybeList<T>;

    pub fn as_view<'a>(&'a self) -> Rpk<T, R, P, RP::View<'a>, K::View<'a>>
    where
        Self: 'a,
        RP::View<'a>: MaybeList<(R, P)>,
        K::View<'a>: MaybeList<T>
    {
        Rpk {
            rp: self.rp.as_view(),
            k: self.k.as_view()
        }
    }
    pub fn to_owned(&self) -> Rpk<T, R, P, RP::Owned, K::Owned>
    where
        RP::Owned: MaybeList<(R, P)>,
        K::Owned: MaybeList<T>
    {
        Rpk {
            rp: self.rp.to_owned(),
            k: self.k.to_owned()
        }
    }
    pub fn into_owned(self) -> Rpk<T, R, P, RP::Owned, K::Owned>
    where
        RP::Owned: MaybeList<(R, P)>,
        K::Owned: MaybeList<T>
    {
        Rpk {
            rp: self.rp.into_owned(),
            k: self.k.into_owned()
        }
    }
}