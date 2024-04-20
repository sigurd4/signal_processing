use core::{iter::Sum, ops::{AddAssign, DivAssign, MulAssign, RangeInclusive}};

use array_math::{SliceMath, SliceOps};
use num::{complex::ComplexFloat, traits::FloatConst, Complex, Float, Zero};
use option_trait::{Maybe, StaticMaybe};

use crate::{ComplexOp, Lists, MaybeList, TruncateIm};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum XCorrScale
{
    None,
    Biased,
    Unbiased,
    Coeff
}

pub trait XCorr<X, Y, YY, Z>: Lists<X>
where
    X: ComplexFloat + ComplexOp<Y, Output = Z>,
    Y: ComplexFloat<Real = X::Real> + Into<Z>,
    YY: MaybeList<Y, MaybeSome: StaticMaybe<YY::Some, MaybeOr<Self::RowOwned, Self::Owned> = Self::Owned>>,
    Z: ComplexFloat<Real = X::Real>
{
    fn xcorr<SC, ML>(
        self,
        y: YY,
        scale: SC,
        max_lag: ML
    ) -> (Self::RowsMapped<Self::RowsMapped<Vec<Z>>>, RangeInclusive<isize>)
    where
        SC: Maybe<XCorrScale>,
        ML: Maybe<usize>;
}

impl<T, X, XX, Y, YY, Z> XCorr<X, Y, YY, Z> for XX
where
    T: FloatConst + Float + Into<Z> + Sum + 'static,
    X: ComplexFloat<Real = T> + Into<Complex<T>> + ComplexOp<Y, Output = Z>,
    XX: Lists<X>,
    Y: ComplexFloat<Real = T> + Into<Complex<T>> + Into<Z>,
    YY: MaybeList<Y, MaybeSome: StaticMaybe<YY::Some, MaybeOr<XX::RowOwned, XX::Owned> = XX::Owned>>,
    Z: ComplexFloat<Real = T> + DivAssign + DivAssign<T> + 'static,
    Complex<T>: AddAssign + MulAssign + MulAssign<T>
{
    fn xcorr<SC, ML>(
        self,
        y: YY,
        scale: SC,
        max_lag: ML
    ) -> (XX::RowsMapped<XX::RowsMapped<Vec<Z>>>, RangeInclusive<isize>)
    where
        SC: Maybe<XCorrScale>,
        ML: Maybe<usize>
    {
        let x = self.to_vecs();

        let mut n = x.iter()
            .map(|x| x.len())
            .reduce(usize::max)
            .unwrap_or(0);
        if let Some(y) = y.as_view_slice_option()
        {
            n = n.max(y.len())
        }
        let nm1 = n - 1;

        let mut max_lag = max_lag.into_option()
            .unwrap_or(nm1);
        let scale = scale.into_option()
            .unwrap_or(XCorrScale::None);

        let pad_result = if max_lag > nm1
        {
            let pad_result = max_lag - nm1;
            max_lag = nm1;
            pad_result
        }
        else
        {
            0
        };

        let p = x.len();
        let m = (n + max_lag).next_power_of_two();
        let mut r: Vec<_> = (0..p*p).map(|_| vec![Z::zero(); 2*max_lag + 1])
            .collect();

        let y = y.into_vec_option();
        if let Some(y) = &y
        {
            let mut pre: Vec<Complex<T>> = x[0].iter()
                .map(|&x| x.into()
                ).collect();
            let mut post: Vec<Complex<T>> = y.iter()
                .map(|&x| x.into()
                ).collect();

            let rot = n + max_lag - pre.len();
            pre.resize(m, Zero::zero());
            pre.rotate_right(rot);
            pre.fft();
            
            post.resize(m, Zero::zero());
            post.fft();
            post.conj_assign_all();
            
            let mut corr: Vec<_> = post.iter()
                .zip(pre.iter())
                .map(|(&post, &pre)| post*pre)
                .collect();
            corr.ifft();
            corr.truncate(2*max_lag + 1);
            for (r, corr) in r[0].iter_mut()
                .zip(corr)
            {
                *r = corr.truncate_im()
            }
        }
        else
        {
            let mut pre: Vec<Vec<Complex<_>>> = x.iter()
                .map(|x| x.iter()
                    .map(|&x| x.into())
                    .collect()
                ).collect();
            let mut post = pre.clone();
            for x in pre.iter_mut()
            {
                let rot = n + max_lag - x.len();
                x.resize(m, Zero::zero());
                x.rotate_right(rot);
                x.fft();
            }
            for x in post.iter_mut()
            {
                x.resize(m, Zero::zero());
                x.fft();
                x.conj_assign_all();
            }

            let corr: Vec<_> = post.iter()
                .zip(pre.iter())
                .map(|(post, pre)| {
                    let mut corr: Vec<_> = post.iter()
                        .zip(pre.iter())
                        .map(|(&post, &pre)| post*pre)
                        .collect();
                    corr.ifft();
                    corr.truncate(2*max_lag+1);
                    corr
                }).collect();
            for (r, corr) in r.iter_mut()
                .step_by(p + 1)
                .zip(corr)
            {
                for (r, corr) in r.iter_mut()
                    .zip(corr)
                {
                    *r = corr.truncate_im()
                }
            }

            for i in 0..p - 1
            {
                let j = i + 1..p;
                let corr: Vec<_> = j.clone()
                    .map(|j| {
                        let mut corr: Vec<_> = post[j].iter()
                            .zip(pre[i].iter())
                            .map(|(&post, &pre)| post*pre)
                            .collect();
                        corr.ifft();
                        corr.truncate(2*max_lag+1);
                        corr
                    }).collect();
                for (j, mut corr) in j.zip(corr)
                {
                    for (r, &corr) in r[i*p + j].iter_mut()
                        .zip(corr.iter())
                    {
                        *r = corr.truncate_im()
                    }
                    corr.reverse();
                    corr.conj_assign_all();
                    for (r, corr) in r[j*p + i].iter_mut()
                        .zip(corr)
                    {
                        *r = corr.truncate_im()
                    }
                }
            }
        }

        match scale
        {
            XCorrScale::None => (),
            XCorrScale::Biased => {
                for r in r.iter_mut()
                {
                    r.div_assign_all(T::from(n).unwrap())
                }
            },
            XCorrScale::Unbiased => {
                for r in r.iter_mut()
                {
                    for (r, i) in r.iter_mut()
                        .zip((n - max_lag - 1..n)
                            .chain(n..=n - max_lag)
                        )
                    {
                        *r /= T::from(i).unwrap()
                    }
                }
            },
            XCorrScale::Coeff => {
                if let Some(y) = y
                {
                    let norm = (x[0].iter()
                            .map(|&x| (x.conj()*x).re())
                            .sum::<T>()
                        *y.iter()
                            .map(|&y| (y.conj()*y).re())
                            .sum::<T>()
                        ).sqrt();
                    for r in r.iter_mut()
                    {
                        r.div_assign_all(norm)
                    }
                }
                else if XX::IS_FLATTENED
                {
                    let norm = r[0][max_lag];
                    for r in r.iter_mut()
                    {
                        r.div_assign_all(norm)
                    }
                }
                else
                {
                    let norm: Vec<_> = x.iter()
                        .map(|x| x.iter()
                            .map(|&x| (x.conj()*x).re())
                            .sum::<T>()
                        ).collect();
                    let norm: Vec<_> = norm.iter()
                        .flat_map(|&n1| norm.iter()
                            .map(move |&n2| n1*n2)
                        ).collect();
                    for r in r.iter_mut()
                    {
                        for (r, &n) in r.iter_mut()
                            .zip(norm.iter())
                        {
                            *r /= n
                        }
                    }
                }
            },
        }

        max_lag += pad_result;

        for r in r.iter_mut()
        {
            r.resize(2*max_lag + 1, Z::zero());
            r.rotate_right(pad_result)
        }

        let mut r = r.into_iter();
        (self.map_rows_to_owned(|_| self.map_rows_to_owned(|_| r.next().unwrap())), -(max_lag as isize)..=max_lag as isize)
    }
}

#[cfg(test)]
mod test
{
    use crate::{plot, XCorr};

    #[test]
    fn test()
    {
        const N: usize = 16;
        let x: [_; N] = core::array::from_fn(|i| 0.84f64.powi(i as i32));
        let mut y = x;
        y.rotate_right(5);

        let (c, lags) = x.xcorr(y, (), ());

        plot::plot_curves("c(t)", "plots/c_t_xcorr.png", [
                &lags.map(|l| l as f64).zip(c).collect::<Vec<_>>()
            ]).unwrap()
    }
}