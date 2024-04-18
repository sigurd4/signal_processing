use num::{complex::ComplexFloat, traits::FloatConst, Float};
use option_trait::Maybe;

use crate::{ComplexOp, Conv, Filter, MaybeList, MaybeOwnedList, Polynomial, ResidueZ, Rpk, System, Tf};

pub trait ResidueD: System
{
    type Output: System<Domain: ComplexFloat<Real = <Self::Domain as ComplexFloat>::Real>>;
    
    fn residued<TOL>(self, tol: TOL) -> Self::Output
    where
        TOL: Maybe<<Self::Domain as ComplexFloat>::Real>;
}

impl<T, TR, B, A, R, P, RP, K> ResidueD for Tf<T, B, A>
where
    T: ComplexFloat<Real = TR> + ComplexOp<TR, Output = T>,
    TR: Float + FloatConst + Into<T>,
    B: MaybeList<T, Owned: MaybeOwnedList<T>>,
    A: MaybeList<T> + for<'a> Conv<T, T, &'a [T], Output = Vec<T>, OutputT = T> + Clone,
    Tf<T, B::Owned, A>: ResidueZ<Output = Rpk<T, R, P, RP, K>> + System<Domain = T>,
    for<'b> Tf<T, Vec<T>, A>: Filter<'b, TR, Vec<TR>, Output = Vec<T>> + System<Domain = T>,
    R: ComplexFloat<Real = TR>,
    P: ComplexFloat<Real = TR>,
    RP: MaybeList<(R, P)>,
    K: MaybeList<T>
{
    type Output = Rpk<T, R, P, RP, Vec<T>>;

    fn residued<TOL>(self, tol: TOL) -> Self::Output
    where
        TOL: Maybe<<Self::Domain as ComplexFloat>::Real>
    {
        (|| {
            let mut tf = Tf {
                b: self.b.to_owned(),
                a: self.a
            };

            let na = tf.a.as_view_slice_option().map(|a| a.len()).unwrap_or(1);

            let k = if let Some(b) = tf.b.as_mut_slice_option()
            {
                let nb = b.len();
                if nb >= na
                {
                    let mut delta = vec![TR::zero(); nb - na + 1];
                    delta[0] = TR::one();
                    let ba = Tf::new(b.to_vec(), tf.a.clone().into_inner());
                    let f = ba.filter(delta, ());
                    let fa = tf.a.clone().into_inner().conv(&f);
                    for i in 0..nb
                    {
                        b[i] = b.get(i + nb - na + 1).map(|&b| b).unwrap_or_else(T::zero)
                            - fa.get(i + nb - na + 1).map(|&f| f).unwrap_or_else(T::zero)
                    }

                    f
                }
                else
                {
                    vec![]
                }
            }
            else
            {
                vec![]
            };

            let rpk = tf.residuez(tol);

            Rpk {
                rp: rpk.rp,
                k: Polynomial::new(k)
            }
        })()
    }
}

#[cfg(test)]
mod test
{
    use crate::{ResidueD, Tf};

    #[test]
    fn test()
    {
        let h = Tf::new(
            [4.0, 5.0, 6.0],
            [1.0, 2.0, 3.0]
        );

        let rpk = h.residued(());
        println!("{:?}", rpk);
    }
}