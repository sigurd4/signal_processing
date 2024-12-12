use core::iter::Sum;

use num::{complex::ComplexFloat, Complex, Float, Zero};

use crate::{
    analysis::{FreqZ, ImpZ},
    quantities::{List, ListOrSingle, Lists, MaybeList, MaybeLists},
    System,
    systems::Tf
};

/// A trait for computing the `Lp`-norm or infinity-norm of a digital filter.
pub trait FilterNorm<'a>: System
{
    type Output: ListOrSingle<<Self::Set as ComplexFloat>::Real>;

    /// Computes the `Lp`-norm or infinity-norm of a digital filter.
    /// 
    /// # Arguments
    /// 
    /// * `pnorm` - The norm power. If infinite, the infinity-norm of the filter is returned.
    /// 
    /// # Returns
    /// 
    /// * `norm` - The `Lp`-norm or infinity-norm of the digital filter.
    fn filternorm(&'a self, pnorm: <Self::Set as ComplexFloat>::Real) -> Self::Output;
}

const FILTER_INF_NORM_RES: usize = 1024;

impl<'a, T, B, A> FilterNorm<'a> for Tf<T, B, A>
where
    T: ComplexFloat<Real: Sum>,
    B: MaybeLists<T>,
    A: MaybeList<T>,
    B::RowsMapped<Vec<T>>: for<'b> Lists<T, RowView<'b>: List<T>, RowsMapped<T::Real> = B::RowsMapped<T::Real>>,
    B::RowsMapped<[Complex<T::Real>; FILTER_INF_NORM_RES]>: for<'b> Lists<Complex<T::Real>, RowView<'b>: List<Complex<T::Real>>, RowsMapped<T::Real> = B::RowsMapped<T::Real>>, 
    Self: ImpZ<'a, B::RowsMapped<Vec<T>>, Vec<T::Real>, ()> + FreqZ<'a, B::RowsMapped<[Complex<T::Real>; FILTER_INF_NORM_RES]>, [T::Real; FILTER_INF_NORM_RES], ()> + System<Set = T>
{
    type Output = B::RowsMapped<T::Real>;

    fn filternorm(&'a self, pnorm: T::Real) -> Self::Output
    {
        if Float::abs(pnorm) > Float::sqrt(<T::Real as Float>::max_value())
        {
            let (h, _): (_, [_; FILTER_INF_NORM_RES]) = self.freqz((), false);
            h.map_rows_to_owned(|h| h.as_view_slice()
                .iter()
                .map(|&h| h.abs())
                .reduce(if pnorm.is_sign_positive() {Float::max} else {Float::min})
                .unwrap_or_else(Zero::zero)
            )
        }
        else
        {
            let (h, _) = self.impz((), None);
            let mut inf = false;
            let n = h.map_rows_to_owned(|h| {
                let n = Float::powf(h.as_view_slice()
                    .iter()
                    .map(|&h| Float::powf(h.abs(), pnorm))
                    .sum::<T::Real>(),
                    Float::recip(pnorm)
                );
                if Float::is_infinite(n)
                {
                    inf = true;
                }
                n
            });
            if inf
            {
                return self.filternorm(Float::copysign(Float::infinity(), pnorm))
            }
            n
        }
    }
}

#[cfg(test)]
mod test
{
    use crate::{
        gen::filter::{Butter, FilterGenPlane, FilterGenType},
        analysis::FilterNorm,
        systems::Tf
    };

    #[test]
    fn test()
    {
        let fs = 1000.0;
        
        let h: Tf::<f64, _, _> = Tf::butter(15, [220.0], FilterGenType::LowPass, FilterGenPlane::Z { sampling_frequency: Some(fs) })
            .unwrap();
        
        let n2 = h.filternorm(2.0);

        println!("n2 = {}", n2);
        
        let n_inf = h.filternorm(f64::INFINITY);

        println!("n_inf = {}", n_inf);
    }
}