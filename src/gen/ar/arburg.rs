use core::{iter::Sum, ops::{AddAssign, Div, Mul}};

use num::{complex::ComplexFloat, NumCast, One};
use option_trait::{Maybe, StaticMaybe};

use crate::{Ar, ContainerOrSingle, List, ListOrSingle, Lists, System};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ArBurgCriterion
{
    AIC,
    AICc,
    KIC,
    AKICc,
    PFE
}

pub trait ArBurg<X, O, C, K>: System + Sized
where
    X: Lists<Self::Domain>,
    O: StaticMaybe<usize>,
    C: Maybe<ArBurgCriterion>,
    K: Lists<Self::Domain>
{
    fn arburg(x: X, order: O, criterion: C) -> (Self, K);
}

impl<T, X, const N: usize> ArBurg<X, (), (), X::RowsMapped<[T; N - 1]>> for Ar<T, [T; N], X::RowsMapped<([T; N], T::Real)>>
where
    T: ComplexFloat,
    X: Lists<T>,
    X::RowsMapped<[T; N - 1]>: Lists<T>,
    X::RowsMapped<(Vec<T>, T::Real)>: ListOrSingle<(Vec<T>, T::Real), Mapped<([T; N], T::Real)> = X::RowsMapped<([T; N], T::Real)>>,
    X::RowsMapped<Vec<T>>: ListOrSingle<Vec<T>, Mapped<[T; N - 1]> = X::RowsMapped<[T; N - 1]>> + Lists<T>,
    Ar<T, Vec<T>, X::RowsMapped<(Vec<T>, T::Real)>>: ArBurg<X, usize, (), X::RowsMapped<Vec<T>>> + System<Domain = T>,
    [(); N - 1]:
{
    fn arburg(x: X, (): (), (): ()) -> (Self, X::RowsMapped<[T; N - 1]>)
    {
        let order = N - 1;
        let (ar, k) = Ar::arburg(x, order, ());
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

impl<T, X, C> ArBurg<X, usize, C, X::RowsMapped<Vec<T>>> for Ar<T, Vec<T>, X::RowsMapped<(Vec<T>, T::Real)>>
where
    T: ComplexFloat<Real: Sum> + Sum + Mul<T::Real, Output = T> + Div<T::Real, Output = T> + AddAssign,
    X: Lists<T, RowOwned: List<T>>,
    C: Maybe<ArBurgCriterion>,
    X::RowsMapped<(Vec<T>, T::Real, Vec<T>)>: ListOrSingle<(Vec<T>, T::Real, Vec<T>), Mapped<(Vec<T>, T::Real)> = X::RowsMapped<(Vec<T>, T::Real)>>,
    X::RowsMapped<(Vec<T>, T::Real)>: ListOrSingle<(Vec<T>, T::Real), Mapped<Vec<T>> = X::RowsMapped<Vec<T>>>,
    X::RowsMapped<Vec<T>>: Lists<T>
{
    fn arburg(x: X, order: usize, criterion: C) -> (Self, X::RowsMapped<Vec<T>>)
    {
        let one = T::Real::one();
        let two = one + one;
        let three = two + one;

        let criterion = criterion.into_option();

        let avk = x.map_rows_into_owned(|x| {
            let mut x = x.into_vec();
            
            let n = x.len()
                .max(order.saturating_sub(1))
                .max(3);

            if x.len() < n
            {
                x.resize(n, T::zero());
            }

            let nf = <T::Real as NumCast>::from(n).unwrap();

            let mut f = x[1..].to_vec();
            let mut b = x[..n - 1].to_vec();
            let mut v = x.iter()
                .map(|&x| (x.conj()*x).re()/nf)
                .sum::<T::Real>();

            let mut new_crit = v.abs();
            let mut old_crit;

            let mut k = vec![];
            let mut a = vec![];
            for p in 1..=order
            {
                let pf = <T::Real as NumCast>::from(p).unwrap();
                let last_k = -b.iter()
                        .zip(f.iter())
                        .map(|(&b, &f)| b.conj()*f)
                        .sum::<T>()
                    *two/(b.iter()
                            .map(|&b| (b.conj()*b).re())
                            .sum::<T::Real>()
                        + f.iter()
                            .map(|&f| (f.conj()*f).re())
                            .sum::<T::Real>()
                    );
                let new_v = v*(one - (last_k.conj()*last_k).re());

                if p > 1
                {
                    match criterion
                    {
                        Some(criterion) => {
                            old_crit = new_crit;
                            new_crit = match criterion
                            {
                                ArBurgCriterion::AKICc => {
                                    new_v.ln() + pf/nf/(nf - pf) + (three - (pf + two)/nf)*(pf + one)/(nf - pf - two)
                                },
                                ArBurgCriterion::KIC => {
                                    new_v.ln() + three*(pf + one)/nf
                                },
                                ArBurgCriterion::AICc => {
                                    new_v.ln() + two*(pf + one)/(nf - pf - two)
                                },
                                ArBurgCriterion::AIC => {
                                    new_v.ln() + two*(pf + one)/nf
                                },
                                ArBurgCriterion::PFE => {
                                    new_v*(nf + pf + one)/(nf - pf - one)
                                }
                            };
                            if new_crit > old_crit
                            {
                                break
                            }
                        },
                        None => ()
                    }
                    let ar = a.iter()
                        .rev()
                        .copied()
                        .collect::<Vec<T>>();
                    for (a, ar) in a.iter_mut()
                        .zip(ar)
                    {
                        *a = *a + last_k*ar.conj()
                    }
                }
                a.push(last_k);
                k.push(last_k);
                v = new_v;
                if p < order
                {
                    let nn = n - p;

                    let new_f = f[1..nn].iter()
                        .zip(b[1..nn].iter())
                        .map(|(&f, &b)| f + last_k*b)
                        .collect();
                    b.truncate(nn - 1);
                    for (b, &f) in b.iter_mut()
                        .zip(f[..nn - 1].iter())
                    {
                        *b += last_k.conj()*f;
                    }

                    f = new_f;
                }
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

#[cfg(test)]
mod test
{
    use crate::{Ar, ArBurg};

    #[test]
    fn test()
    {
        let x = [1.0, 2.0, 3.0, 4.0, 5.0];

        let (ar, k) = Ar::arburg(x, 2, ());

        println!("{:?}", ar);
        println!("{:?}", k);
    }
}