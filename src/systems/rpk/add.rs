use core::ops::{Add, AddAssign};

use num::{complex::ComplexFloat, traits::float::TotalOrder, Zero};

use crate::{ComplexOp, MaybeList, Polynomial, Rpk, SumSequence};

impl<T1, T2, T3, R1, R2, R3, P1, P2, P3, RP1, RP2, K1, K2, K3> Add<Rpk<T2, R2, P2, RP2, K2>> for Rpk<T1, R1, P1, RP1, K1>
where
    T1: ComplexOp<T2, Output = T3>,
    T2: ComplexFloat + Into<T3>,
    T3: ComplexFloat<Real: TotalOrder>,
    R1: ComplexFloat<Real = T1::Real> + ComplexOp<R2, Output = R3>,
    R2: ComplexFloat<Real = T2::Real> + Into<R3>,
    R3: ComplexFloat<Real = T3::Real> + AddAssign,
    P1: ComplexFloat<Real = T1::Real> + ComplexOp<P2, Output = P3>,
    P2: ComplexFloat<Real = T2::Real> + Into<P3>,
    P3: ComplexFloat<Real = T3::Real>,
    RP1: MaybeList<(R1, P1)>,
    RP2: MaybeList<(R2, P2)>,
    RP1::MaybeMapped<(R3, P3)>: MaybeList<(R3, P3)>,
    RP2::MaybeMapped<(R3, P3)>: MaybeList<(R3, P3)>,
    K1: MaybeList<T1>,
    K2: MaybeList<T2>,
    K3: MaybeList<T3>,
    K1::MaybeMapped<T3>: MaybeList<T3>,
    K2::MaybeMapped<T3>: MaybeList<T3>,
    SumSequence<(R3, P3), RP1::MaybeMapped<(R3, P3)>>: Into<SumSequence<(R3, P3), Vec<(R3, P3)>>>,
    SumSequence<(R3, P3), RP2::MaybeMapped<(R3, P3)>>: Into<SumSequence<(R3, P3), Vec<(R3, P3)>>>,
    Polynomial<T3, K1::MaybeMapped<T3>>: Add<Polynomial<T3, K2::MaybeMapped<T3>>, Output = Polynomial<T3, K3>>
{
    type Output = Rpk<T3, R3, P3, Vec<(R3, P3)>, K3>;

    fn add(self, rhs: Rpk<T2, R2, P2, RP2, K2>) -> Self::Output
    {
        let mut rp1 = SumSequence::new(self.rp.into_inner().maybe_map_into_owned(|(r, p)| (r.into(), p.into()))).into();
        let mut rp2 = SumSequence::new(rhs.rp.into_inner().maybe_map_into_owned(|(r, p)| (r.into(), p.into()))).into();
        rp1.sort_by(|a, b| a.1.abs().total_cmp(&b.1.abs()));
        rp2.sort_by(|a, b| a.1.abs().total_cmp(&b.1.abs()));

        let mut rp1 = rp1.into_inner()
            .into_iter();
        let mut rp1_next = rp1.next();
        let mut rp2 = rp2.into_inner()
            .into_iter();
        let mut rp2_next = rp2.next();
        let mut next = |prev| match (&rp1_next, &rp2_next)
        {
            (Some(a), Some(b)) => if !(a.1.abs() > b.1.abs()) && ((a.1.abs() != b.1.abs()) || Some(b.1) != prev)
            {
                let n = rp1_next;
                rp1_next = rp1.next();
                n.map(|n| (n, false))
            }
            else
            {
                let n = rp2_next;
                rp2_next = rp2.next();
                n.map(|n| (n, true))
            },
            (Some(_), None) => {
                let n = rp1_next;
                rp1_next = rp1.next();
                n.map(|n| (n, false))
            },
            (None, Some(_)) => {
                let n = rp2_next;
                rp2_next = rp2.next();
                n.map(|n| (n, true))
            },
            (None, None) => None
        };
        let mut rp = vec![];
        let mut prev_in = [None, None];
        let mut mult_in = [1, 1];
        let mut prev = None;
        let mut i = 0;
        while let Some(((r, p), which)) = next(prev)
        {
            if prev_in[which as usize] == Some(p)
            {
                mult_in[which as usize] += 1
            }
            else if prev != Some(p)
            {
                mult_in = [1, 1];
                i = rp.len();
            }
            if i + mult_in[which as usize] > rp.len()
            {
                while i + mult_in[which as usize] > rp.len() + 1
                {
                    rp.push((Zero::zero(), p))
                }
                rp.push((r, p))
            }
            else
            {
                rp[i + mult_in[which as usize] - 1].0 += r
            }

            prev_in[which as usize] = Some(p);
            prev = Some(p);
        }
        Rpk {
            rp: SumSequence::new(rp),
            k: Polynomial::new(self.k.into_inner().maybe_map_into_owned(Into::into))
                + Polynomial::new(rhs.k.into_inner().maybe_map_into_owned(Into::into))
        }
    }
}