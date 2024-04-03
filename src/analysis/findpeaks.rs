use core::{iter::Sum, ops::{RangeInclusive, SubAssign}};

use array_math::{ArrayMath, ArrayOps, SliceOps, SliceMath};

use ndarray_linalg::{Lapack};
use num::{Float, NumCast};
use option_trait::{Maybe, MaybeCell};

use crate::List;

pub trait FindPeaks<T>: List<T>
where
    T: Float
{
    fn find_peaks<const EXTRA: bool, MPH, MPD, MPW, MMPW>(
        &self,
        min_peak_height: MPH,
        min_peak_distance: MPD,
        min_peak_width: MPW,
        max_peak_width: MMPW,
        double_sided: bool
    ) -> Vec<(usize, T, MaybeCell<((RangeInclusive<usize>, [T; 3]), [T; 2], T, T), EXTRA>)>
    where
        MPH: Maybe<T>,
        MPD: Maybe<usize>,
        MPW: Maybe<usize>,
        MMPW: Maybe<usize>,
        [(); EXTRA as usize]:;
}

impl<T, L> FindPeaks<T> for L
where
    T: Float + Sum + SubAssign + Lapack,
    L: List<T>
{
    fn find_peaks<const EXTRA: bool, MPH, MPD, MPW, MMPW>(
        &self,
        min_peak_height: MPH,
        min_peak_distance: MPD,
        min_peak_width: MPW,
        max_peak_width: MMPW,
        double_sided: bool
    ) -> Vec<(usize, T, MaybeCell<((RangeInclusive<usize>, [T; 3]), [T; 2], T, T), EXTRA>)>
    where
        MPH: Maybe<T>,
        MPD: Maybe<usize>,
        MPW: Maybe<usize>,
        MMPW: Maybe<usize>,
        [(); EXTRA as usize]:
    {
        let minh = Float::abs(min_peak_height.into_option()
                .unwrap_or_else(T::epsilon)
            );
        let mind = min_peak_distance.into_option()
            .unwrap_or(1)
            .max(1);
        let minw = min_peak_width.into_option()
            .unwrap_or(1)
            .max(1);
        let maxw = max_peak_width.into_option();

        let zero = T::zero();

        let data: Vec<T> = self.as_view_slice()
            .to_vec();
        let n = data.len();
        let sub = if double_sided
        {
            data.iter().copied()
                .sum::<T>()
                /NumCast::from(n).unwrap()
        }
        else
        {
            data.iter().copied()
                .reduce(Float::min)
                .unwrap_or(zero)
        };
        let data: Vec<_> = data.into_iter()
            .map(|x| Float::abs(x - sub))
            .collect();
        let mut df1 = data.clone();
        df1.differentiate();
        let mut df2 = df1.clone();
        df2.differentiate();

        let mut idx: Vec<_> = data.iter()
            .zip(df1.iter().zip(df1[2..].iter()).zip(df2[2..].iter()))
            .enumerate()
            .filter(|(_, (&x, ((&df1_pre, &df1), &df2)))| x > minh && df1_pre*df1 <= zero && df2 < zero)
            .map(|(i, _)| i + 1)
            .collect();
        idx.sort_by(|&i, &j| data[i].partial_cmp(&data[j]).unwrap());

        let mut d: Vec<Vec<_>> = idx.iter()
            .map(|i| idx.iter()
                .map(|j| Some(if j >= i {j - i} else {i - j}))
                .collect()
            ).collect();
        for (i, d) in d.iter_mut()
            .enumerate()
        {
            d[i] = None
        }
        if d.iter()
            .flatten()
            .any(|d| d.is_some_and(|d| d < mind))
        {
            let mut node2visit: Vec<_> = (0..idx.len()).rev().collect();
            let mut visited = vec![];

            while let Some(i) = node2visit.pop()
            {
                let d = &d[i];
                visited.push(i);
                let mut neighs: Vec<_> = d.iter()
                    .enumerate()
                    .filter(|(i, d)| d.is_some_and(|d| d < mind) && !visited.contains(i))
                    .map(|(i, _)| i)
                    .collect();
                if !neighs.is_empty()
                {
                    idx.extract_if(|i| neighs.contains(i))
                        .for_each(core::mem::drop);
                    node2visit.extract_if(|i| neighs.contains(i))
                        .for_each(core::mem::drop);
                    visited.append(&mut neighs);
                }
            }
        }

        let np = data.len();
        let mut h = zero;

        let one = T::one();
        let two = one + one;

        let pks = idx.into_iter()
            .filter_map(|idx| {
                let ind: Vec<_> = ((idx as f32 - mind as f32/2.0).max(1.0).floor() as usize
                    ..=(idx as f32 + mind as f32/2.0).min((np - 1) as f32).ceil() as usize)
                        .collect();
                let pp;
                let xm;
                if data[idx - 1] == data[idx]
                {
                    pp = [one; 3];
                    xm = zero;
                }
                else if ind.iter()
                    .any(|&ind| data[ind] > data[idx])
                {
                    let data: Vec<_> = ind.iter()
                        .map(|&ind| data[ind])
                        .collect();
                    let ind: Vec<_> = ind.iter()
                        .map(|&ind| T::from(ind).unwrap())
                        .collect();
                    pp = ind.rpolyfit(&data, 2)
                        .try_into()
                        .unwrap();
                    xm = -pp[1]/(two*pp[0]);
                    h = pp.rpolynomial(xm);
                }
                else
                {
                    h = data[idx];
                    xm = T::from(idx).unwrap();
                    let pp0 = {
                        let d = T::from(ind[0]).unwrap() - xm;
                        d*d/(data[ind[0]] - h)
                    };
                    pp = [
                        pp0,
                        -two*pp0*xm,
                        h + pp0*xm*xm
                    ]
                }
    
                let width = Float::sqrt(Float::abs(pp[0].recip())) + xm;
    
                if !(maxw.is_some_and(|maxw| width > T::from(maxw).unwrap())
                    || width < T::from(minw).unwrap()
                    || pp[0] > zero || h < minh
                    || data[idx] < T::from(0.99).unwrap()*h
                    || Float::abs(T::from(idx).unwrap() - xm) > T::from(mind).unwrap()/two)
                {
                    Some((
                        idx,
                        self.as_view_slice()[idx],
                        MaybeCell::from_fn(|| (
                            (ind[0]..=*ind.last().unwrap(), pp),
                            [-width, width].div_all(two).add_all(xm),
                            h,
                            (h + minh)/two
                        ))
                    ))
                }
                else
                {
                    None
                }
            }).collect();

        pks
    }
}

#[cfg(test)]
mod test
{
    use array_math::ArrayOps;
    use rand::distributions::uniform::SampleRange;

    use crate::{plot, Butter, FiltFilt, FilterGenPlane, FilterGenType, FindPeaks, Tf};

    #[test]
    fn test()
    {
        const N: usize = 1024;

        let mut rng = rand::thread_rng();
        let mut x: [_; N] = ArrayOps::fill(|_| (-1.0..1.0).sample_single(&mut rng));

        let h = Tf::butter(2, [10.0], FilterGenType::LowPass, FilterGenPlane::Z { sampling_frequency: Some(1000.0) })
            .unwrap();
        x = h.filtfilt(x);

        let pks: Vec<_> = x.find_peaks::<false, _, _, _, _>((), (), (), (), false)
            .into_iter()
            .map(|(i, p, _)| (i as f64, p))
            .collect();

        plot::plot_curves("x[n]", "plots/x_n_findpeaks.png", [&x.enumerate().map(|(i, x)| (i as f64, x)), &pks])
            .unwrap()
    }
}