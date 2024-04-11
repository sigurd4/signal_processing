use num::{complex::ComplexFloat, One};
use option_trait::{Maybe, StaticMaybe};

use crate::{MaybeContainer, MaybeList, MaybeLists, Polynomial, ProductSequence, Sos, System, Tf, Zpk};

pub trait SplitNumerDenom: System
{
    type OutputNum: System<Domain: ComplexFloat<Real = <Self::Domain as ComplexFloat>::Real>>;
    type OutputDen: System<Domain: ComplexFloat<Real = <Self::Domain as ComplexFloat>::Real>>;

    fn split_numer_denom(self) -> (Self::OutputNum, Self::OutputDen);
}

impl<T, B, A> SplitNumerDenom for Tf<T, B, A>
where
    T: ComplexFloat,
    B: MaybeLists<T>,
    A: MaybeList<T>
{
    type OutputNum = Tf<T, B, ()>;
    type OutputDen = Tf<T, (), A>;

    fn split_numer_denom(self) -> (Self::OutputNum, Self::OutputDen)
    {
        let Tf {b, a} = self;
        (
            Tf {
                b,
                a: Polynomial::new(())
            },
            Tf {
                b: Polynomial::new(()),
                a
            }
        )
    }
}

impl<T, Z, P, K> SplitNumerDenom for Zpk<T, Z, P, K>
where
    T: ComplexFloat,
    Z: MaybeList<T>,
    P: MaybeList<T>,
    K: ComplexFloat<Real = T::Real>
{
    type OutputNum = Zpk<T, Z, (), K>;
    type OutputDen = Zpk<T, (), P, K::Real>;

    fn split_numer_denom(self) -> (Self::OutputNum, Self::OutputDen)
    {
        let Zpk {z, p, k} = self;
        (
            Zpk {
                z,
                p: ProductSequence::new(()),
                k
            },
            Zpk {
                z: ProductSequence::new(()),
                p,
                k: One::one()
            }
        )
    }
}

impl<T, B, A, S> SplitNumerDenom for Sos<T, B, A, S>
where
    T: ComplexFloat,
    B: Maybe<[T; 3]> + MaybeList<T>,
    A: Maybe<[T; 3]> + MaybeList<T>,
    S: MaybeList<Tf<T, B, A>>,
    Tf<T, B, A>: Clone,
    S::MaybeMapped<Tf<T, B, ()>>: MaybeList<Tf<T, B, ()>, Some: Sized, MaybeSome: Sized>,
    S::MaybeMapped<Tf<T, (), A>>: MaybeList<Tf<T, (), A>, Some: Sized, MaybeSome: Sized>,
    <B::MaybeSome as StaticMaybe<B::Some>>::Maybe<<S::MaybeMapped<Tf<T, B, ()>> as MaybeContainer<Tf<T, B, ()>>>::Some>: MaybeList<Tf<T, B, ()>> + Sized,
    <A::MaybeSome as StaticMaybe<A::Some>>::Maybe<<S::MaybeMapped<Tf<T, (), A>> as MaybeContainer<Tf<T, (), A>>>::Some>: MaybeList<Tf<T, (), A>> + Sized,
    S::MaybeMapped<()>: MaybeList<(), MaybeMapped<Tf<T, B, ()>> = S::MaybeMapped<Tf<T, B, ()>>> + MaybeList<(), MaybeMapped<Tf<T, (), A>> = S::MaybeMapped<Tf<T, (), A>>>
{
    type OutputNum = Sos<T, B, (), <B::MaybeSome as StaticMaybe<B::Some>>::Maybe<<S::MaybeMapped<Tf<T, B, ()>> as MaybeContainer<Tf<T, B, ()>>>::Some>>;
    type OutputDen = Sos<T, (), A, <A::MaybeSome as StaticMaybe<A::Some>>::Maybe<<S::MaybeMapped<Tf<T, (), A>> as MaybeContainer<Tf<T, (), A>>>::Some>>;

    fn split_numer_denom(self) -> (Self::OutputNum, Self::OutputDen)
    {
        let mut b = vec![];
        let mut a = vec![];
        let sos = self.sos.into_inner()
            .maybe_map_into_owned(|sos| {
                let Tf {b: b_next, a: a_next} = sos;
                b.push(b_next);
                a.push(a_next);
            });
        
        let mut b = b.into_iter();
        let mut a = a.into_iter();

        let n = Sos::new(sos.maybe_map_to_owned(|_| Tf {b: b.next().unwrap(), a: Polynomial::new(())}));
        let d = Sos::new(sos.maybe_map_to_owned(|_| Tf {b: Polynomial::new(()), a: a.next().unwrap()}));

        (
            Sos::new(StaticMaybe::maybe_from_fn(|| n.sos.into_inner().into_maybe_some().into_option().unwrap())),
            Sos::new(StaticMaybe::maybe_from_fn(|| d.sos.into_inner().into_maybe_some().into_option().unwrap()))
        )
    }
}