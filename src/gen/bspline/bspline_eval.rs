use core::ops::AddAssign;

use num::Float;
use option_trait::Maybe;

use crate::{IntoList, List, ListOrSingle, OwnedListOrSingle};

pub trait BSplineEval<T, X, N>: IntoList<T, X, N>
where
    T: Float,
    X: ListOrSingle<T>,
    N: Maybe<usize>
{
    fn bspline_eval<TT, C, CC>(self, numtaps: N, knots: TT, control_points: CC, degree: usize) -> (X::Mapped<C::Mapped<T>>, X)
    where
        TT: List<T>,
        C: ListOrSingle<T> + Clone,
        C::Mapped<T>: OwnedListOrSingle<T, Length = usize>,
        CC: List<C>;
}

impl<T, X, XX, N> BSplineEval<T, X, N> for XX
where
    T: Float + AddAssign,
    X: ListOrSingle<T>,
    XX: IntoList<T, X, N>,
    N: Maybe<usize>
{
    fn bspline_eval<TT, C, CC>(self, numtaps: N, knots: TT, control_points: CC, degree: usize) -> (X::Mapped<C::Mapped<T>>, X)
    where
        TT: List<T>,
        C: ListOrSingle<T> + Clone,
        C::Mapped<T>: OwnedListOrSingle<T, Length = usize>,
        CC: List<C>
    {
        let x = self.into_list(numtaps);
        let mut t = knots.into_vec();
        if let Some(&t0) = t.first() && let Some(&t1) = t.last()
        {
            let one = T::one();
            let c = control_points.into_vec();
            t = core::iter::repeat(t0)
                .take(degree)
                .chain(t)
                .chain(core::iter::repeat(t1)
                    .take(degree)
                ).collect();
            let mut kkk = 0;
            (x.map_to_owned(|&x| {
                for kk in 0..t.len().saturating_sub(2*degree + 1)
                {
                    let k = (kk + kkk) % t.len().saturating_sub(2*degree + 1);
                    let t0 = t[degree + k].min(t[degree + k + 1]);
                    let t1 = t[degree + k].max(t[degree + k + 1]);
                    if t0 <= x && x < t1
                    {
                        let mut d: Vec<_> = (0..degree + 1).map(|j| c.get(j + k)
                                .or_else(|| c.last())
                                .cloned()
                                .map(|c| c.map_into_owned(|c| c))
                                .unwrap_or_else(|| C::Mapped::<T>::from_len_fn((), |_| T::zero()))
                            ).collect(); 
                    
                        for r in 1..degree + 1
                        {
                            for j in (r..degree + 1).rev()
                            {
                                let alpha = (x - t[j + k])/(t[j + k + degree + 1 - r] - t[j + k]);
                                let djm1 = d[j - 1].to_vec();
                                for (dj, djm1) in d[j].as_mut_slice()
                                    .iter_mut()
                                    .zip(djm1)
                                {
                                    *dj = (one - alpha)*djm1 + alpha**dj
                                }
                            }
                        }
                        kkk = k;
                        return d.pop().unwrap_or_else(|| C::Mapped::<T>::from_len_fn((), |_| T::zero()));
                    }
                }
                C::Mapped::<T>::from_len_fn((), |_| T::zero())
            }), x)
        }
        else
        {
            (x.map_to_owned(|_| C::Mapped::<T>::from_len_fn((), |_| T::zero())), x)
        }
    }
}

#[cfg(test)]
mod test
{
    use crate::{plot, BSplineEval};

    #[test]
    fn test()
    {
        const N: usize = 1024;
        /*
        let (b, t): (_, [_; N]) = (0.0..6.0).bspline_eval((), [0.0, 1.0, 2.0, 3.0, 4.0, 5.0, 6.0], [-1.0, 2.0, 0.0, -1.0, 1.0, 0.0], 2);

        plot::plot_curves("B(t)", "plots/b_t_bspline_eval.png", [&t.zip(b)])
            .unwrap();*/
        
        let (b, _): (_, [_; N]) = (0.0..6.0).bspline_eval((),
            [0.0, 1.0, 2.0, 3.0, 4.0, 5.0, 6.0],
            [[0.0, 0.0], [1.0, 0.0], [0.0, 2.0], [-3.0, 0.0], [0.0, -4.0], [5.0, 0.0], [0.0, 6.0], [-7.0, 0.0], [0.0, -9.0], [10.0, 0.0]],
            2
        );

        plot::plot_curves("S(t)", "plots/s_t_bspline_eval.png", [&b.map(|b| (b[0], b[1]))])
            .unwrap();
    }
}