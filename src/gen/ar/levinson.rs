use core::{iter::Sum, ops::{AddAssign, Div, Mul, MulAssign}};

use num::{complex::ComplexFloat, One};
use option_trait::Maybe;

use crate::{Ar, ContainerOrSingle, List, ListOrSingle, Lists, System};

pub trait Levinson<C, O, K>: System + Sized
where
    C: Lists<Self::Domain>,
    O: Maybe<usize>,
    K: Lists<Self::Domain>
{
    fn levinson(c: C, order: O) -> (Self, K);
}

impl<T, X, const N: usize> Levinson<X, (), X::RowsMapped<[T; N - 1]>> for Ar<T, [T; N], X::RowsMapped<([T; N], T::Real)>>
where
    T: ComplexFloat,
    X: Lists<T>,
    X::RowsMapped<[T; N - 1]>: Lists<T>,
    X::RowsMapped<(Vec<T>, T::Real)>: ListOrSingle<(Vec<T>, T::Real), Mapped<([T; N], T::Real)> = X::RowsMapped<([T; N], T::Real)>>,
    X::RowsMapped<Vec<T>>: ListOrSingle<Vec<T>, Mapped<[T; N - 1]> = X::RowsMapped<[T; N - 1]>> + Lists<T>,
    Ar<T, Vec<T>, X::RowsMapped<(Vec<T>, T::Real)>>: Levinson<X, usize, X::RowsMapped<Vec<T>>> + System<Domain = T>,
    [(); N - 1]:
{
    fn levinson(x: X, (): ()) -> (Self, X::RowsMapped<[T; N - 1]>)
    {
        let order = N - 1;
        let (ar, k) = Ar::levinson(x, order);
        (
            Ar::new(ar.av.map_into_owned(|(mut a, v)| {
                a.resize(N, T::zero());
                (a.try_into().ok().unwrap(), v)
            })),
            k.map_into_owned(|mut k: Vec<T>| {
                k.resize(N - 1, T::zero());
                TryInto::<[T; N - 1]>::try_into(k).ok().unwrap()
            })
        )
    }
}

impl<T, C> Levinson<C, usize, C::RowsMapped<Vec<T>>> for Ar<T, Vec<T>, C::RowsMapped<(Vec<T>, T::Real)>>
where
    T: ComplexFloat<Real: MulAssign> + Mul<T::Real, Output = T> + Div<T::Real, Output = T> + AddAssign + Sum,
    C: Lists<T, RowOwned: List<T>>,
    C::RowsMapped<(Vec<T>, T::Real, Vec<T>)>: ListOrSingle<(Vec<T>, T::Real, Vec<T>), Mapped<(Vec<T>, T::Real)> = C::RowsMapped<(Vec<T>, T::Real)>>,
    C::RowsMapped<(Vec<T>, T::Real)>: ListOrSingle<(Vec<T>, T::Real), Mapped<Vec<T>> = C::RowsMapped<Vec<T>>>,
    C::RowsMapped<Vec<T>>: Lists<T>
{
    fn levinson(c: C, order: usize) -> (Self, C::RowsMapped<Vec<T>>)
    {
        let one = T::Real::one();

        let avk = c.map_rows_into_owned(|c| {
            let mut c = c.into_vec();
            let mut p = order;
            if p >= c.len()
            {
                p = c.len().saturating_sub(1)
            }
            if c.len() <= 1
            {
                c.resize(2, T::zero());
            }

            let mut k = vec![T::zero(); p];
            let mut g = -c[1]/c[0];
            let mut a = vec![g];
            let mut v = (c[0]*(one - (g.conj()*g).re())).re();
            if let Some(k) = k.get_mut(0)
            {
                *k = g
            }
            for t in 2..=p
            {
                g = -(c[t] + a.iter()
                    .zip(c[1..t].iter()
                        .rev()
                    ).map(|(&a, &c)| a*c)
                    .sum::<T>())/v;
                let ar: Vec<_> = a.iter()
                    .rev()
                    .copied()
                    .collect();
                for (a, ar) in a.iter_mut()
                    .zip(ar)
                {
                    *a += g*ar.conj()
                }
                a.push(g);
                v *= one - (g*g.conj()).re();
                k[t - 1] = g;
            }
            a.insert(0, T::one());

            (a, v, k)
        });

        let mut kvec = vec![];
        let av = avk.map_into_owned(|(a, v, k)| {
            kvec.push(k);
            (a, v)
        });
        let mut k = kvec.into_iter();
        let k = av.map_to_owned(|_| k.next().unwrap());

        (
            Ar::new(av),
            k
        )
    }
}