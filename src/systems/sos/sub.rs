use core::ops::{Mul, Sub};

use num::complex::ComplexFloat;
use option_trait::Maybe;

use crate::{MaybeContainer, MaybeList, Polynomial, Sos, Tf, ToSos, ToTf};

impl<T1, T2, T3, B1, B2, A1, A2, A3, S1, S2, S3, S4> Sub<Sos<T2, B2, A2, S2>> for Sos<T1, B1, A1, S1>
where
    T1: ComplexFloat,
    T2: ComplexFloat,
    T3: ComplexFloat,
    B1: Maybe<[T1; 3]> + MaybeList<T1> + Clone,
    B2: Maybe<[T2; 3]> + MaybeList<T2> + Clone,
    A1: Maybe<[T1; 3]> + MaybeList<T1> + Clone,
    A2: Maybe<[T2; 3]> + MaybeList<T2> + Clone,
    A3: Maybe<[T3; 3]> + MaybeList<T3>,
    S1: MaybeList<Tf<T1, B1, A1>>,
    S2: MaybeList<Tf<T2, B2, A2>>,
    S3: MaybeList<Tf<T3, (), A3>>,
    S4: MaybeList<Tf<T3, [T3; 3], A3>>,
    (): Maybe<[T1; 3]>,
    (): Maybe<[T2; 3]>,
    (): Maybe<[T3; 3]>,
    S1::MaybeMapped<()>: MaybeContainer<(), MaybeMapped<Tf<T1, (), A1>> = S1::MaybeMapped<Tf<T1, (), A1>>>,
    S1::MaybeMapped<()>: MaybeContainer<(), MaybeMapped<Tf<T1, B1, ()>> = S1::MaybeMapped<Tf<T1, B1, ()>>>,
    S2::MaybeMapped<()>: MaybeContainer<(), MaybeMapped<Tf<T2, (), A2>> = S2::MaybeMapped<Tf<T2, (), A2>>>,
    S2::MaybeMapped<()>: MaybeContainer<(), MaybeMapped<Tf<T2, B2, ()>> = S2::MaybeMapped<Tf<T2, B2, ()>>>,
    S1::MaybeMapped<Tf<T1, (), A1>>: MaybeList<Tf<T1, (), A1>>,
    S2::MaybeMapped<Tf<T2, (), A2>>: MaybeList<Tf<T2, (), A2>>,
    S1::MaybeMapped<Tf<T1, B1, ()>>: MaybeList<Tf<T1, B1, ()>>,
    S2::MaybeMapped<Tf<T2, B2, ()>>: MaybeList<Tf<T2, B2, ()>>,
    Sos<T1, (), A1, S1::MaybeMapped<Tf<T1, (), A1>>>: Mul<Sos<T2, (), A2, S2::MaybeMapped<Tf<T2, (), A2>>>, Output = Sos<T3, (), A3, S3>>,
    Sos<T1, B1, (), S1::MaybeMapped<Tf<T1, B1, ()>>>: ToTf<T1, Vec<T1>, (), (), ()>,
    Sos<T2, B2, (), S2::MaybeMapped<Tf<T2, B2, ()>>>: ToTf<T2, Vec<T2>, (), (), ()>,
    Tf<T1, Vec<T1>, ()>: Sub<Tf<T2, Vec<T2>>, Output: ToSos<T3, [T3; 3], (), Vec<Tf<T3, [T3; 3], ()>>, (), ()>>,
    Sos<T3, [T3; 3], (), Vec<Tf<T3, [T3; 3], ()>>>: Mul<Sos<T3, (), A3, S3>, Output = Sos<T3, [T3; 3], A3, S4>>
{
    type Output = Sos<T3, [T3; 3], A3, S4>;

    fn sub(self, rhs: Sos<T2, B2, A2, S2>) -> Self::Output
    {
        let mut b1 = vec![];
        let mut b2 = vec![];
        let mut a1 = vec![];
        let mut a2 = vec![];
        let sos1 = self.sos.into_inner()
            .maybe_map_into_owned(|sos| {
            let Tf {b, a} = sos;
            b1.push(b);
            a1.push(a);
        });
        let sos2 = rhs.sos.into_inner()
            .maybe_map_into_owned(|sos| {
            let Tf {b, a} = sos;
            b2.push(b);
            a2.push(a);
        });
        let mut b1 = b1.into_iter();
        let mut b2 = b2.into_iter();
        let mut a1 = a1.into_iter();
        let mut a2 = a2.into_iter();

        let d1 = Sos::new(sos1.maybe_map_to_owned(|_| Tf {b: Polynomial::new(()), a: a1.next().unwrap()}));
        let d2 = Sos::new(sos2.maybe_map_to_owned(|_| Tf {b: Polynomial::new(()), a: a2.next().unwrap()}));
        
        let n1 = Sos::new(sos1.maybe_map_to_owned(|_| Tf {b: b1.next().unwrap(), a: Polynomial::new(())}));
        let n2 = Sos::new(sos2.maybe_map_to_owned(|_| Tf {b: b2.next().unwrap(), a: Polynomial::new(())}));

        (n1.to_tf((), ()) - n2.to_tf((), ())).to_sos((), ())*(d1*d2)
    }
}