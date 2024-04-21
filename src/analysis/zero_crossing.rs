use num::{traits::{float::TotalOrder, FloatConst}, Float};

use crate::{List, MaybeLenEq};

pub trait ZeroCrossing<T, YY>: List<T> + MaybeLenEq<YY, true>
where
    T: Float + FloatConst,
    YY: List<T>
{
    fn zero_crossing(self, y: YY) -> Vec<T>;
}

impl<T, XX, YY> ZeroCrossing<T, YY> for XX
where
    T: Float + FloatConst + TotalOrder,
    XX: List<T> + MaybeLenEq<YY, true>,
    YY: List<T>
{
    fn zero_crossing(self, y: YY) -> Vec<T>
    {
        let mut x = self.into_vec();
        let mut y = y.into_vec();

        let len = x.len().min(y.len());
        x.truncate(len);
        y.truncate(len);

        let zero = T::zero();
        let one = T::one();
        let two = one + one;

        let crossing_intervals: Vec<_> = y.iter()
            .zip(y.iter().skip(1))
            .enumerate()
            .filter(|(_, (&y1, &y2))| y1*y2 <= zero)
            .map(|(i, _)| i)
            .collect();
        
        let mut left_ends: Vec<_> = crossing_intervals.iter()
            .map(|&i| x[i])
            .collect();
        let mut right_ends: Vec<_> = crossing_intervals.iter()
            .map(|&i| x[i + 1])
            .collect();
        let mut left_vals: Vec<_> = crossing_intervals.iter()
            .map(|&i| y[i])
            .collect();
        let mut right_vals: Vec<_> = crossing_intervals.iter()
            .map(|&i| y[i + 1])
            .collect();

        let mut retval = {
            let mut mid_points: Vec<_> = left_ends.iter()
                .zip(right_ends.iter())
                .map(|(&l, &r)| (l + r)/two)
                .collect();
            let mut retval = vec![];
            let mut i = 0;
            while i < mid_points.len()
            {
                if left_vals[i] == right_vals[i]
                {
                    left_ends.remove(i);
                    right_ends.remove(i);
                    left_vals.remove(i);
                    right_vals.remove(i);
                    retval.push(mid_points.remove(i))
                }
                else
                {
                    i += 1
                }
            }
            retval
        };
        
        for z in left_ends.into_iter()
            .zip(right_ends)
            .zip(left_vals)
            .zip(right_vals)
            .map(|(((le, re), lv), rv)| le - (re - le)*lv/(rv - lv))
        {
            if !retval.contains(&z)
            {
                retval.push(z)
            }
        }
        retval.sort_by(TotalOrder::total_cmp);

        retval
    }
}