use core::ops::{Add, AddAssign, Mul, MulAssign};

use num::{complex::ComplexFloat, Float, NumCast, Zero, One};
use option_trait::Maybe;
use thiserror::Error;
use array_math::SliceMath;

use crate::{util::{self, ComplexOp}, quantities::{MaybeList, MaybeLists, NotPolynomial, Polynomial, SumSequence}, decompositions::Residue, systems::{Rpk, Tf}, System};

#[derive(Debug, Clone, Copy, PartialEq, Error)]
pub enum ImpInvarError
{
    #[error("Non causal transfer function, i.e. it contains one or more poles at infinity.")]
    NonCausal
}

pub trait ImpInvar: System
{
    type Output: Sized;

    fn impinvar<FS, TOL>(
        self,
        sampling_frequency: FS,
        tolerance: TOL
    ) -> Result<Self::Output, ImpInvarError>
    where
        FS: Maybe<<Self::Domain as ComplexFloat>::Real>,
        TOL: Maybe<<Self::Domain as ComplexFloat>::Real>;
}

impl<T, T2, B, B2, A, A2, R, R2, P, RP, K> ImpInvar for Tf<T, B, A>
where
    T: ComplexFloat<Real: Into<T> + NotPolynomial> + 'static,
    B: MaybeLists<T>,
    A: MaybeList<T>,
    Self: Residue<Output = Rpk<T, R, P, RP, K>> + System<Domain = T>,
    Rpk<R, R2, P, Vec<(R2, P)>, [R; 1]>: Residue<Output = Tf<T2, B2, A2>> + System<Domain: ComplexFloat<Real = T::Real>>,
    T2: ComplexFloat<Real = T::Real> + 'static,
    B2: MaybeLists<T2>,
    A2: MaybeList<T2>,
    B2::MaybeMapped<T>: MaybeLists<T, RowsMapped<Vec<T>>: MaybeLists<T>, RowOwned: MaybeList<T>>,
    Polynomial<T, <B2::MaybeMapped<T> as MaybeLists<T>>::RowOwned>: Into<Polynomial<T, Vec<T>>>,
    A2::MaybeMapped<T>: MaybeList<T>,
    R: ComplexFloat<Real = T::Real> + Mul<T::Real, Output = R> + ComplexOp<P, Output = R2> + AddAssign,
    P: ComplexFloat<Real = T::Real> + Mul<T::Real, Output = P> + Into<R2> + MulAssign + Add<T::Real, Output = P> + Mul<T::Real, Output = P>,
    R2: ComplexFloat<Real = T::Real> + NotPolynomial,
    RP: MaybeList<(R, P)>,
    K: MaybeList<T>,
    SumSequence<(R, P), RP>: Into<SumSequence<(R, P), Vec<(R, P)>>>
{
    type Output = Tf<T, <B2::MaybeMapped<T> as MaybeLists<T>>::RowsMapped<Vec<T>>, A2::MaybeMapped<T>>;

    fn impinvar<FS, TOL>(self, sampling_frequency: FS, tol: TOL) -> Result<Self::Output, ImpInvarError>
    where
        FS: Maybe<T::Real>,
        TOL: Maybe<T::Real>
    {
        let tol = tol.into_option()
            .map(|tol| Float::abs(tol))
            .unwrap_or_else(|| <T::Real as NumCast>::from(1e-3).unwrap());

        let ts = sampling_frequency.into_option()
            .map(|fs| Float::recip(fs))
            .unwrap_or_else(One::one);

        let rpk = self.residue(tol);
        
        let n = rpk.rp.as_view_slice_option()
            .map(<[_]>::len)
            .unwrap_or(0);

        if !rpk.k.is_zero()
        {
            return Err(ImpInvarError::NonCausal)
        }

        let rp_in: SumSequence<(R, P), Vec<(R, P)>> = rpk.rp.into();
        let mut rp_out: SumSequence<(R2, P), Vec<(R2, P)>> = SumSequence::zero();
        let mut k_out = R::zero();

        let mut i = 0;
        while i < n
        {
            let mut m = 1;
            let first_pole = rp_in[i].1;
            while i + 1 < n && (first_pole - rp_in[i + 1].1).abs() < tol
            {
                i += 1;
                m += 1;
            }

            let rp_in = &rp_in[i + 1 - m..=i];
            let n = rp_in.len();

            let p_out = (first_pole*ts).exp();
            let kn_out = rp_in[0].0*ts;
            let mut r_out: Polynomial<R2, _> = Polynomial::new(vec![kn_out.into()*p_out.into()]);

            for j in 1..n
            {
                let h1 = h1_z_deriv(j, p_out, ts);
                r_out = r_out + h1.map_into_owned(Into::into)*rp_in[j].0.into()
            }

            let mut rpn_out = r_out.into_inner()
                .into_iter()
                .rev()
                .map(|r| (r, p_out))
                .collect();

            k_out += kn_out;
            rp_out.append(&mut rpn_out);
            
            i += 1;
        }

        let rpk = Rpk {
            rp: rp_out,
            k: Polynomial::new([k_out])
        };

        let tf = rpk.residue(tol);

        let b = tf.b.truncate_im()
            .into_inner()
            .map_rows_into_owned(|b| {
                let mut b: Vec<_> = Polynomial::new(b).into().into_inner();
                b.pop();
                b
            });

        Ok(Tf {
            b: Polynomial::new(b),
            a: tf.a.truncate_im()
        })
    }
}

pub(crate) fn h1_z_deriv<T>(n: usize, p: T, ts: T::Real)
    -> Polynomial<T, Vec<T>>
where
    T: ComplexFloat<Real: NotPolynomial> + MulAssign + Mul<T::Real, Output = T> + Add<T::Real, Output = T>
{
    let mut d = Polynomial::new(vec![<T::Real as NumCast>::from(1 - (n % 2) as i8*2).unwrap()]);
    for _ in 1..n
    {
        d.push(Zero::zero());
        d = d.clone() + Polynomial::<_, Vec<_>>::new(d.clone().derivate_rpolynomial())
    }

    let mut b = Polynomial::new(vec![]);
    for i in 1..n
    {
        b = b + h1_deriv::<T::Real>(n + 1 - i)*d[i]
    }
    let mul = Float::powi(ts, n as i32 + 1)/util::factorial(n);
    for (i, b) in b.iter_mut()
        .enumerate()
    {
        *b *= p.powi((n + 1 - i) as i32)*mul
    }
    b
}

fn h1_deriv<T>(n: usize)
    -> Polynomial<T, Vec<T>>
where
    T: Float
{
    let f: T = util::factorial(n);
    let s = <T as NumCast>::from(1 - (n % 2) as i8*2).unwrap();
    Polynomial::new(
        (0..=n).map(|i| f*s*util::bincoeff(n, i))
            .collect()
    )
}

#[cfg(test)]
mod test
{
    use crate::{systems::Tf, transforms::domain::ImpInvar};

    #[test]
    fn test()
    {
        let h = Tf::new(
            [4.0, 5.0],
            [1.0, 2.0, 3.0]
        );

        let hz = h.impinvar((), ())
            .unwrap();

        println!("{:?}", hz);
    }
}