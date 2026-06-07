use core::ops::AddAssign;

use num::{Float, NumCast};
use option_trait::Maybe;

use crate::{quantities::{IntoList, List, ListOrSingle, OwnedListOrSingle}, transforms::bspline::BSplineEval};

pub trait CSpline1dEval<T, X, N, C>: IntoList<T, X, N>
where
    T: Float,
    X: ListOrSingle<T>,
    N: Maybe<usize>,
    C: List<T>
{
    fn cspline_1d_eval(self, numtaps: N, c: C) -> (X::Mapped<T>, X);
}

impl<T, C, X, XX, N> CSpline1dEval<T, X, N, C> for XX
where
    T: Float + AddAssign,
    XX: IntoList<T, X, N>,
    X: ListOrSingle<T>,
    X::Mapped<T>: OwnedListOrSingle<T>,
    N: Maybe<usize>,
    C: List<T>
{
    fn cspline_1d_eval(self, numtaps: N, c: C) -> (X::Mapped<T>, X)
    {
        let x = self.into_list(numtaps);
        let mut res = x.map_to_owned(|_| T::zero());
        if res.length() == 0
        {
            return (res, x)
        }
        let rr = res.as_mut_slice();

        let xx = x.as_view_slice();

        let c = c.into_vec();
        let n = c.len();
        let nf = T::from(n).unwrap();

        let zero = T::zero();
        let one = T::one();
        let two = one + one;
        let nm1 = nf - one;

        let cond1: Vec<_> = x.as_view_slice()
            .iter()
            .enumerate()
            .filter(|(_, x)| *x < &zero)
            .map(|(i, _)| i)
            .collect();
        let cond2: Vec<_> = x.as_view_slice()
            .iter()
            .enumerate()
            .filter(|(_, x)| *x > &nm1)
            .map(|(i, _)| i)
            .collect();
        let cond3: Vec<_> = x.as_view_slice()
            .iter()
            .enumerate()
            .filter(|(_, x)| !(*x < &zero || *x > &nm1))
            .map(|(i, _)| i)
            .collect();

        if cond1.len() > 0
        {
            let x: Vec<_> = cond1.iter()
                .map(|&i| -xx[i])
                .collect();
            let (y, _) = x.cspline_1d_eval((), c.as_slice());
            for (i, y) in cond1.into_iter()
                .zip(y)
            {
                rr[i] = y;
            }
        }
        if cond2.len() > 0
        {
            let x: Vec<_> = cond2.iter()
                .map(|&i| two*nm1 - xx[i])
                .collect();
            let (y, _) = x.cspline_1d_eval((), c.as_slice());
            for (i, y) in cond2.into_iter()
                .zip(y)
            {
                rr[i] = y;
            }
        }
        if cond3.len() > 0
        {
            let cubic = |x: T| {
                if x.abs() > two
                {
                    return zero
                }
                const TT: [f64; 5] = [-2.0, -1.0, 0.0, 1.0, 2.0];
                const CC: [f64; TT.len()*3 - 4] = const {
                    let mut c = [0.0; TT.len()*3 - 4];
                    c[TT.len() - 2] = 1.0;
                    c
                };
                x.bspline_eval((), TT.map(|t| T::from(t).unwrap()), CC.map(|c| T::from(c).unwrap()), TT.len() - 2).0
            };

            let x: Vec<_> = cond3.iter()
                .map(|&i| xx[i])
                .collect();
            let jlower: Vec<_> = x.iter()
                .map(|&x| <isize as NumCast>::from((x - two).floor() + one).unwrap())
                .collect();
            for i in 0..4
            {
                for ((r, &x), &j) in cond3.iter()
                    .copied()
                    .zip(x.iter())
                    .zip(jlower.iter())
                {
                    let thisj: isize = j + i;
                    let indj = (thisj.max(0) as usize).min(n - 1);
                    rr[r] += c[indj]*cubic(x - T::from(thisj).unwrap())
                }
            }
        }

        (res, x)
    }
}