use core::ops::{AddAssign, DivAssign, MulAssign, SubAssign};

use array_math::SliceMath;

use num::{complex::ComplexFloat, Complex, Float, NumCast, One, Zero, traits::FloatConst};
use option_trait::Maybe;

use crate::{ComplexOp, Conv, Filter, List, ListOrSingle, Lists, MaybeList, MaybeLists, System, Tf};

pub trait ImpZ<'a, B, T, N>: System
where
    B: Lists<Self::Domain>,
    T: List<<Self::Domain as ComplexFloat>::Real>,
    N: Maybe<usize>
{
    fn impz(&'a self, numtaps: N, sampling_frequency: Option<<Self::Domain as ComplexFloat>::Real>) -> (B, T);
}

impl<'a, T, B, A> ImpZ<'a, B::RowsMapped<Vec<T>>, Vec<T::Real>, usize> for Tf<T, B, A>
where
    T: ComplexFloat + ComplexOp<T, Output = T>,
    B: MaybeLists<T>,
    A: MaybeList<T>,
    B::RowsMapped<Vec<T>>: for<'b> Lists<T, RowsMapped<Vec<T>> = B::RowsMapped<Vec<T>>, RowView<'b>: List<T, Mapped<T::Real> = Vec<T::Real>>> + 'a,
    Self: 'a + Filter<'a, T, Vec<T>, Output = B::RowsMapped<Vec<T>>> + System<Domain = T>,
    &'a Self: Into<Tf<T, B::RowsMapped<Vec<T>>, Vec<T>>>,
    Vec<T>: for<'b> Conv<T, T, &'b [T], Output = Vec<T>>,
    T::Real: Into<T>,
    (): Maybe<Vec<T>>
{
    fn impz(&'a self, n: usize, sampling_frequency: Option<<T as ComplexFloat>::Real>) -> (B::RowsMapped<Vec<T>>, Vec<T::Real>)
    {
        if n == 0
        {
            return (self.b.map_rows_to_owned(|_| vec![]), vec![])
        }

        let Tf {b, a}: Tf<T, B::RowsMapped<Vec<T>>, Vec<T>> = self.into();

        let fs = sampling_frequency.unwrap_or_else(One::one);
        let t: Vec<_> = (0..n).map(|n| <T::Real as NumCast>::from(n).unwrap()/fs)
            .collect();

        if a.len() == 0
        {
            return (self.b.map_rows_to_owned(|_| vec![T::Real::infinity().into(); n]), t)
        }
        let mut x = vec![T::zero(); n];
        x[0] = T::one();
        let y = if a.len() == 1
        {
            let b = b.as_views();
            let mut i = 0;
            self.b.map_rows_to_owned(|_| {
                let b = &b[i];
                let a = a[0];
                let bda: Vec<_> = b.as_view_slice()
                    .iter()
                    .map(|&b| b/a)
                    .collect();
                let mut y = bda.conv(&x);
                y.resize(n, T::zero());
                i += 1;
                y
            })
        }
        else
        {
            self.filter(x, ())
        };
        (y, t)
    }
}

impl<'a, T, B, A, const N: usize> ImpZ<'a, B::RowsMapped<[T; N]>, [T::Real; N], ()> for Tf<T, B, A>
where
    T: ComplexFloat,
    B: MaybeLists<T>,
    A: MaybeList<T>,
    B::RowsMapped<Vec<T>>: Lists<T, RowsMapped<[T; N]> = B::RowsMapped<[T; N]>>,
    B::RowsMapped<[T; N]>: Lists<T>,
    Self: ImpZ<'a, B::RowsMapped<Vec<T>>, Vec<T::Real>, usize> + System<Domain = T> + 'a
{
    fn impz(&'a self, (): (), sampling_frequency: Option<<T as ComplexFloat>::Real>) -> (B::RowsMapped<[T; N]>, [T::Real; N])
    {
        let (h, t) = self.impz(N, sampling_frequency);
        (
            h.map_rows_to_owned(|h| {
                    TryInto::<[T; N]>::try_into(h.as_view_slice_option()
                        .unwrap()
                        .to_vec()
                    ).map_err(|_| ())
                    .unwrap()
            }),
            t.try_into()
                .map_err(|_| ())
                .unwrap()
        )
    }
}

impl<'a, T, B, A> ImpZ<'a, B::RowsMapped<Vec<T>>, Vec<<T as ComplexFloat>::Real>, ()> for Tf<T, B, A>
where
    Complex<<T as ComplexFloat>::Real>: From<T> + AddAssign + SubAssign + MulAssign + DivAssign + DivAssign<<T as ComplexFloat>::Real>,
    T: ComplexFloat + ndarray_linalg::Lapack<Complex = Complex<<T as ComplexFloat>::Real>>,
    B: MaybeLists<T>,
    A: MaybeList<T>,
    B::RowsMapped<Vec<T>>: Lists<T>,
    Self: ImpZ<'a, B::RowsMapped<Vec<T>>, Vec<<T as ComplexFloat>::Real>, usize> + System<Domain = T>
{
    fn impz(&'a self, (): (), sampling_frequency: Option<<T as ComplexFloat>::Real>) -> (B::RowsMapped<Vec<T>>, Vec<<T as ComplexFloat>::Real>)
    {        
        let oneref = &[T::one()];
        let b: Vec<&[T]> = self.b.as_view_slices_option()
            .unwrap_or(vec![oneref]);
        let a = self.a.as_view_slice_option()
            .unwrap_or(oneref);
        
        let mut n = vec![0; b.len()];

        if a.is_empty()
        {
            for n in n.iter_mut()
            {
                *n = 1;
            }
        }
        else if a.len() > 1
        {
            for (n, b) in n.iter_mut()
                .zip(b)
            {
                let precision = <<T as ComplexFloat>::Real as NumCast>::from(1e-6).unwrap();
                let r: Vec<Complex<<T as ComplexFloat>::Real>> = a.polynomial_roots();
                let maxpole = r.iter()
                    .map(|&r| r.abs())
                    .reduce(Float::max)
                    .unwrap();

                let zero = <T as ComplexFloat>::Real::zero();
                let one = <T as ComplexFloat>::Real::one();
                let six = <<T as ComplexFloat>::Real as NumCast>::from(7u8).unwrap();

                if maxpole > one + precision
                {
                    *n = NumCast::from(Float::ceil(six*Float::log10(maxpole))).unwrap()
                }
                else if maxpole < one - precision
                {
                    *n = NumCast::from(Float::ceil(-six*Float::log10(maxpole))).unwrap()
                }
                else
                {
                    let three = <<T as ComplexFloat>::Real as NumCast>::from(3u8).unwrap();
                    let ten = <<T as ComplexFloat>::Real as NumCast>::from(10u8).unwrap();

                    *n = 30;

                    let r_periodic: Vec<_> = r.iter()
                        .filter(|&r| r.abs() >= one - precision && Float::abs(r.arg()) > zero)
                        .collect();
                    if !r_periodic.is_empty()
                    {
                        let r_periodic_min_arg = r_periodic.iter()
                            .map(|&r| Float::abs(r.arg()))
                            .reduce(Float::min)
                            .unwrap();
                        let n_periodic = <usize as NumCast>::from(
                            Float::ceil(ten*<T as ComplexFloat>::Real::PI()/r_periodic_min_arg)
                        ).unwrap();
                        *n = (*n).max(n_periodic);
                    }
                    
                    let r_damped: Vec<_> = r.iter()
                        .filter(|&r| r.abs() < one - precision)
                        .collect();
                    if !r_damped.is_empty()
                    {
                        let r_damped_max = r_periodic.iter()
                            .map(|&r| r.abs())
                            .reduce(Float::max)
                            .unwrap();
                        let n_damped = <usize as NumCast>::from(
                            Float::ceil(-three/Float::log10(r_damped_max))
                        ).unwrap();
                        *n = (*n).max(n_damped);
                    }
                }
                *n += b.len()
            }
        }
        else
        {
            for (n, b) in n.iter_mut()
                .zip(b)
            {
                *n = b.len()
            }
        }

        let n = n.into_iter()
            .max()
            .unwrap_or(0);
        self.impz(n, sampling_frequency)
    }
}

#[cfg(test)]
mod test
{
    use array_math::ArrayOps;
    use num::complex::ComplexFloat;

    use crate::{plot, Butter, FilterGenPlane, FilterGenType, FreqZ, ImpZ, Polynomial, Tf};

    #[test]
    fn test()
    {
        let fs = 1000.0;
        
        let h = Tf::butter(15, [220.0], FilterGenType::LowPass, FilterGenPlane::Z { sampling_frequency: Some(fs) })
            .unwrap();

        let (h_i, t): (Vec<_>, _) = h.impz((), Some(fs));

        let th_i = t.into_iter()
            .zip(h_i.iter().copied())
            .collect::<Vec<_>>();

        plot::plot_curves("h(t)", "plots/h_t_impz.png", [&th_i])
            .unwrap();

        let h2 = Tf {
            b: Polynomial::new(h_i),
            a: Polynomial::new(())
        };

        const N: usize = 1024;

        let (h1_z, w): ([_; N], _) = h.freqz(());
        let (h2_z, _) = h2.freqz(());

        plot::plot_curves("H(e^jw)", "plots/h_z_impz.png", [&w.zip(h1_z.map(|h| h.abs())), &w.zip(h2_z.map(|h| h.abs()))])
            .unwrap()
    }
}