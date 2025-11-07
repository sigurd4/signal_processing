use core::{iter::Sum, ops::{AddAssign, Mul, MulAssign, SubAssign}};

use num::{complex::ComplexFloat, traits::float::FloatConst, Complex, NumCast, One, Zero};
use option_trait::{Maybe, StaticMaybe};
use array_math::SliceMath;

use crate::{analysis::CepsError, quantities::{List, ListOrSingle, Lists, MaybeLists}, util::TruncateIm};

pub trait RCeps<'a, T, C, N>: Lists<T>
where
    T: ComplexFloat,
    N: Maybe<usize>,
    C: List<T>
{
    fn rceps<YM>(&'a self, numtaps: N) -> Result<(Self::RowsMapped<C>, YM), CepsError>
    where
        YM: StaticMaybe<Self::RowsMapped<C>>;
}

impl<'a, T, C, L> RCeps<'a, T, C, <C::Length as StaticMaybe<usize>>::Opposite> for L
where
    T: ComplexFloat + AddAssign + SubAssign + Into<Complex<T::Real>> + Mul<T::Real, Output = T> + 'static,
    Complex<T::Real>: MulAssign + AddAssign + MulAssign<T::Real>,
    T::Real: AddAssign + SubAssign + Sum + Into<Complex<T::Real>> + Into<T>,
    L: Lists<T> + 'a + ?Sized,
    C: List<T>,
    Vec<T>: TryInto<C>,
    <C::Length as StaticMaybe<usize>>::Opposite: Sized,
    L::RowsMapped<C>: Lists<T, RowOwned: List<T>, RowsMapped<C>: Into<L::RowsMapped<C>>> + Clone,
    L::RowView<'a>: List<T>
{
    fn rceps<YM>(&'a self, n: <C::Length as StaticMaybe<usize>>::Opposite) -> Result<(Self::RowsMapped<C>, YM), CepsError>
    where
        YM: StaticMaybe<Self::RowsMapped<C>>
    {
        let n = n.into_option()
            .unwrap_or(C::LENGTH);

        let y = self.try_map_rows_to_owned(|x| {
            let x = x.as_view_slice();

            let mut f: Vec<Complex<T::Real>> = x.iter()
                .map(|&x| x.into())
                .collect();

            f.resize(n, Zero::zero());

            let zero = T::Real::zero();
            let half = n/2;
            if 2*half == n && f.dtft(T::Real::TAU()*<T::Real as NumCast>::from(half + 1).unwrap()/<T::Real as NumCast>::from(n).unwrap()).re < zero {
                f.pop();
            }

            f.fft();
            if f.iter().any(|f| f.is_zero())
            {
                return Err(CepsError::ZeroInFourier)
            }

            for f in f.iter_mut()
            {
                *f = f.abs().ln().into();
            }
            f.ifft();

            let zero = T::zero();
            let mut y: Vec<_> = f.into_iter()
                .take(n)
                .map(|y| y.truncate_im())
                .collect();
            y.resize(n, zero);
            Ok(y.try_into().ok().unwrap())
        })?;

        let one = T::Real::one();
        let two = one + one;

        let ym = YM::maybe_from_fn(|| y.clone().map_rows_into_owned(|y| {
            let y = y.as_view_slice();

            let mut ym: Vec<_> = if y.len() % 2 == 1
            {
                core::iter::once(y[0])
                    .chain(y[1..y.len()/2 + 1].iter()
                        .map(|&y| y*two)
                    ).chain(vec![T::zero(); y.len()/2])
                    .map(|y| y.into())
                    .collect()
            }
            else
            {
                core::iter::once(y[0])
                    .chain(y[1..y.len()/2].iter()
                        .map(|&y| y*two)
                    ).chain(core::iter::once(y[y.len()/2]))
                    .chain(vec![T::zero(); y.len()/2 - 1])
                    .map(|y| y.into())
                    .collect()
            };

            ym.fft();

            for y in ym.iter_mut()
            {
                *y = (*y).exp()
            }

            ym.ifft();

            let zero = T::zero();
            let mut ym: Vec<_> = ym.into_iter()
                .take(n)
                .map(|y| y.truncate_im())
                .collect();
            ym.resize(n, zero);
            ym.try_into().ok().unwrap()
        }).into());

        Ok((y, ym))
    }
}

#[cfg(test)]
mod test
{
    use core::f64::consts::TAU;

    use array_math::ArrayOps;
    use linspace::LinspaceArray;

    use crate::{plot, analysis::RCeps};

    #[test]
    fn test()
    {
        const T: f64 = 1.27;
        const N: usize = (T/0.01) as usize;
        let t: [_; N] = (0.0..T).linspace_array();

        let d = (N as f64*0.3/T) as usize;
        let s1 = t.map(|t| (TAU*45.0*t).sin());
        let s2 = s1.add_each(ArrayOps::fill(|i| if i >= d {0.5*s1[i - d]} else {0.0}));

        let (c, ym): ([_; _], _) = s2.rceps(()).unwrap();

        plot::plot_curves("x̂(t), y_m(t)", "plots/x_hat_rceps.png", [&t.zip(c), &t.zip(ym)]).unwrap()
    }
}