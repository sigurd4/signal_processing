use core::{ops::{Div, Mul, MulAssign, Sub}};

use array_math::{ArrayOps, SliceMath};
use ndarray::{Array2};
use ndarray_linalg::{solve::Solve, Lapack, least_squares::LeastSquaresSvd};
use num::{complex::ComplexFloat, traits::FloatConst, Float, NumCast, One, Zero};
use option_trait::Maybe;

use crate::{FilterGenError, Polynomial, System, Tf};



pub trait FirLS<O>: System + Sized
where
    O: Maybe<usize>
{
    fn firls<const B2: usize, const M: usize, FS, W>(
        order: O,
        frequencies: [<Self::Domain as ComplexFloat>::Real; B2],
        magnitudes: [Self::Domain; M],
        weights: W,
        sampling_frequency: FS
    ) -> Result<Self, FilterGenError>
    where
        FS: Maybe<<Self::Domain as ComplexFloat>::Real>,
        W: Maybe<[Self::Domain; B2/2]>,
        [(); 0 - B2 % 2]:,
        [(); B2/2 - 1]:,
        [(); B2 - M]:,
        [(); 0 - M % (B2/2)]:,
        [(); B2/2*2]:;
}

impl<T, const N: usize> FirLS<()> for Tf<T, [T; N], ()>
where
    T: ComplexFloat,
    Tf<T, Vec<T>, ()>: FirLS<usize> + System<Domain = T>
{
    fn firls<const B2: usize, const M: usize, FS, W>(
        (): (),
        frequencies: [<Self::Domain as ComplexFloat>::Real; B2],
        magnitudes: [Self::Domain; M],
        weights: W,
        sampling_frequency: FS
    ) -> Result<Self, FilterGenError>
    where
        FS: Maybe<<Self::Domain as ComplexFloat>::Real>,
        W: Maybe<[Self::Domain; B2/2]>,
        [(); 0 - B2 % 2]:,
        [(); B2/2 - 1]:,
        [(); B2 - M]:,
        [(); 0 - M % (B2/2)]:,
        [(); B2/2*2]:
    {
        let mut h = Tf::firls(N.saturating_sub(1), frequencies, magnitudes, weights, sampling_frequency)?;
        h.b.resize(N, T::zero());
        Ok(Tf {
            b: Polynomial::new(h.b.into_inner()
                .try_into()
                .ok()
                .unwrap()
            ),
            a: Polynomial::new(())
        })
    }
}

impl<T> FirLS<usize> for Tf<T, Vec<T>, ()>
where
    T: ComplexFloat + Lapack + Sub<<T as ComplexFloat>::Real, Output = T> + Mul<<T as ComplexFloat>::Real, Output = T> + Div<<T as ComplexFloat>::Real, Output = T>,
    <T as ComplexFloat>::Real: MulAssign + Into<T>
{
    fn firls<const B2: usize, const M: usize, FS, W>(
        order: usize,
        mut frequencies: [<Self::Domain as ComplexFloat>::Real; B2],
        magnitudes: [Self::Domain; M],
        weights: W,
        sampling_frequency: FS
    ) -> Result<Self, FilterGenError>
    where
        FS: Maybe<<T as ComplexFloat>::Real>,
        W: Maybe<[T; B2/2]>,
        [(); 0 - B2 % 2]:,
        [(); B2/2 - 1]:,
        [(); B2 - M]:,
        [(); 0 - M % (B2/2)]:,
        [(); B2/2*2]:,
    {
        let one = <T as ComplexFloat>::Real::one();
        let zero = Zero::zero();
        let two = one + one;
        
        if let Some(fs) = sampling_frequency.into_option()
        {
            for wc in frequencies.iter_mut()
            {
                if *wc > fs/two
                {
                    return Err(FilterGenError::FrequenciesOutOfRange)
                }
                *wc *= two/fs;
            }
        }
        if frequencies.iter()
            .any(|f| *f < zero || *f > one)
        {
            return Err(FilterGenError::FrequenciesOutOfRange)
        }
        if !frequencies.is_sorted()
        {
            return Err(FilterGenError::FrequenciesNotNondecreasing)
        }
        
        let weight: [_; B2/2] = weights.into_option()
            .unwrap_or_else(|| [one.into(); B2/2]);

        let numtaps = order + 1 - order % 2;
        let m = (numtaps - 1)/2;
        let i1: [_; B2/2] = ArrayOps::fill(|i| i*2);
        let i2: [_; B2/2] = ArrayOps::fill(|i| i*2 + 1);
        let s = Array2::from_shape_fn((numtaps, B2/2), |(n, i)| {
            let n: <T as ComplexFloat>::Real = NumCast::from(n).unwrap();
            let sinc = [frequencies[i1[i]], frequencies[i2[i]]]
                .map(|f| {
                    if n.is_zero()
                    {
                        f
                    }
                    else
                    {
                        let npi = n*FloatConst::PI();
                        Float::sin(f*npi)/npi
                    }
                });

            sinc[1] - sinc[0]
        });
        let w = Array2::from_shape_fn((B2/2, 1), |(i, _)| weight[i]);
        let q = s.map(|&s| T::from(s).unwrap()).dot(&w).column(0).to_vec();
        let q1 = &q[0..=m];
        let q2 = &q[m..];
        let q = q1.toeplitz_matrix() + q1.hankel_matrix(q2);

        let mm: [_; B2/2] = i2.zip(i1)
            .map(|(i2, i1)| (magnitudes[i2] - magnitudes[i1])/(frequencies[i2] - frequencies[i1]));
        let c = i1.zip(mm)
            .map(|(i, mm)| magnitudes[i] - mm*frequencies[i]);
        let bbb = Array2::from_shape_fn((m + 1, B2/2), |(m, i)| {
            let m: <T as ComplexFloat>::Real = NumCast::from(m).unwrap();
            let sinc = [frequencies[i1[i]], frequencies[i2[i]]]
                .map(|f| {
                    let mfpi = m*f*FloatConst::PI();
                    let mut b = (mm[i]*f + c[i])*f*if mfpi.is_zero()
                    {
                        one
                    }
                    else
                    {
                        Float::sin(mfpi)/mfpi
                    };
                    if m.is_zero()
                    {
                        b -= mm[i]*f*f/two
                    }
                    else
                    {
                        let mpi = m*<T as ComplexFloat>::Real::PI();
                        b += mm[i]*Float::cos(mpi*f)/(mpi*mpi)
                    }
                    b
                });

            sinc[1] - sinc[0]
        });
        let b = bbb.dot(&Array2::from_shape_fn((B2/2, 1), |(i, _)| weight[i]));

        let a: Vec<T> = q.solve(&b.column(0))
            .map(|a| a.to_vec())
            .unwrap_or_else(|_| {
                q.least_squares(&b.column(0))
                    .map(|a| a.solution.to_vec())
                    .unwrap()
            });

        let mut coef: Vec<_> = a[1..].iter()
            .rev().copied()
            .chain(core::iter::once(a[0]*two))
            .chain(a[1..].iter().copied()
            ).collect();
        coef.resize(order + 1, zero.into());

        Ok(Tf {
            b: Polynomial::new(coef),
            a: Polynomial::new(())
        })
    }
}

#[cfg(test)]
mod test
{
    use array_math::ArrayOps;

    use crate::{plot, FirLS, RealFreqZ, Tf};

    #[test]
    fn test()
    {
        let h: Tf<_, [_; 254]> = Tf::firls((), [0.0, 0.25, 0.3, 1.0], [1.0, 1.0, 0.0, 0.0], (), ())
            .unwrap();

        println!("l = {}", h.b.len() - 1);
        
        const M: usize = 1024;
        let (h_f, w): ([_; M], _) = h.real_freqz(());

        plot::plot_curves("H(e^jw)", "plots/h_z_firls.png", [&w.zip(h_f.map(|h_f| h_f.norm()))])
            .unwrap()
    }
}