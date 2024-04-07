use core::ops::SubAssign;

use num::{Float, Zero};

use crate::{List, Matrix, MaybeList, Container};

pub enum FullWidthAt<T>
where
    T: Float
{
    HalfMaximum,
    Middle,
    RLevelMaximum {
        rlevel: T
    },
    RLevelMiddle {
        rlevel: T
    },
    ALevel {
        alevel: T
    }
}

pub trait FWHM<'a, T, X, const XX: bool>: Matrix<T>
where
    T: Float,
    X: MaybeList<T>
{
    fn fwhm(&'a self, x: X, at: FullWidthAt<T>) -> Self::RowsMapped<T>;
}

impl<'a, T, X, Y> FWHM<'a, T, X, true> for Y
where
    T: Float + SubAssign,
    Y: Matrix<T>,
    X: List<T, Length = Y::Width>,
    [(); X::LENGTH - Y::WIDTH]:,
    [(); Y::WIDTH - X::LENGTH]:
{
    fn fwhm(&'a self, x: X, at: FullWidthAt<T>) -> Self::RowsMapped<T>
    {
        let mut y: Vec<Vec<T>> = self.as_view_slices()
            .into_iter()
            .map(|y| y.to_vec())
            .collect();
        let mut x: Vec<T> = x.as_view_slice()
            .to_vec();

        let zero = T::zero();
        let one = T::one();
        let two = one + one;
        let half = two.recip();

        let (opt_middle, is_alevel, level) = match at
        {
            FullWidthAt::HalfMaximum => (false, false, half),
            FullWidthAt::Middle => (true, false, half),
            FullWidthAt::RLevelMaximum { rlevel } => (false, false, rlevel),
            FullWidthAt::RLevelMiddle { rlevel } => (true, false, rlevel),
            FullWidthAt::ALevel { alevel } => (false, true, alevel),
        };

        let (_nr, nc) = self.matrix_dim();
        let nx = x.len();

        let n = nc.max(nx);
        x.resize_with(n, Zero::zero);
        for y in y.iter_mut()
        {
            y.resize_with(n, Zero::zero);
        }

        let d = if is_alevel
        {
            level
        }
        else
        {
            let max = y.iter()
                .flatten()
                .map(|&y| y)
                .reduce(Float::max)
                .unwrap_or_else(Zero::zero);
            if opt_middle
            {
                let min = y.iter()
                    .flatten()
                    .map(|&y| y)
                    .reduce(Float::min)
                    .unwrap_or_else(Zero::zero);
                level*(max + min)
            }
            else
            {
                level*max
            }
        };
        for y in y.iter_mut()
        {
            for y in y.iter_mut()
            {
                *y -= d
            }
        }

        let mut y = y.into_iter();
        self.map_rows_to_owned(|_| {
            let yy = y.next().unwrap();
            let ind: Vec<_> = yy.iter()
                .zip(yy[1..].iter())
                .enumerate()
                .filter_map(|(i, (&y0, &y1))| if y0*y1 <= zero {Some(i)} else {None})
                .collect();
            if let Some((imax, _max)) = yy.iter()
                .map(|&y| y)
                .enumerate()
                .reduce(|a, b| if a.1 >= b.1 {a} else {b})
                && ind.len() >= 2 && imax >= ind[0] && imax <= ind[ind.len() - 1]
            {
                let ind1 = ind[0];
                let ind2 = ind1 + 1;
                let dy = yy[ind2] - yy[ind1];
                let xx1 = if !dy.is_zero() {x[ind1] - yy[ind1]*(x[ind2] - x[ind1])/dy} else {(x[ind2] + x[ind1])*half};
                let ind1 = ind[ind.len() - 1];
                let ind2 = ind1 + 1;
                let dy = yy[ind2] - yy[ind1];
                let xx2 = if !dy.is_zero() {x[ind1] - yy[ind1]*(x[ind2] - x[ind1])/dy} else {(x[ind2] + x[ind1])*half};
                xx2 - xx1
            }
            else
            {
                zero
            }
        })
    }
}

impl<'a, T, Y, X> FWHM<'a, T, (), false> for Y
where
    T: Float + SubAssign + 'a,
    Y: Matrix<T, RowView<'a>: List<T, Mapped<T> = X>> + FWHM<'a, T, X, true> + 'a,
    X: List<T>
{
    fn fwhm(&'a self, (): (), at: FullWidthAt<T>) -> Self::RowsMapped<T>
    {
        let mut i = 0;
        let x = self.index_view(0)
            .map_into_owned(|_| {
                let k = i;
                i += 1;
                T::from(k).unwrap()
            });
        self.fwhm(x, at)
    }
}

#[cfg(test)]
mod test
{
    use core::f64::consts::{PI, TAU};

    use array_math::ArrayOps;
    use linspace::LinspaceArray;
    use crate::{window::{Barthann, BlackmanHarris, Boxcar, Hamming, Hann, Kaiser, Triangular, WindowGen, WindowRange}, FullWidthAt, DFT, FWHM};

    #[test]
    fn test()
    {
        const N: usize = 1024;
        let w: [(_, [f64; N/2]); _] = [
            ("Barthann", Barthann.window_gen((), WindowRange::Symmetric)),
            ("BlackmanHarris", BlackmanHarris.window_gen((), WindowRange::Symmetric)),
            ("Boxcar", Boxcar.window_gen((), WindowRange::Symmetric)),
            ("Hamming", Hamming.window_gen((), WindowRange::Symmetric)),
            ("Hann", Hann.window_gen((), WindowRange::Symmetric)),
            ("Kaiser", Kaiser {alpha: 3.0}.window_gen((), WindowRange::Symmetric)),
            ("Triangular", Triangular.window_gen((), WindowRange::Symmetric)),
        ];
        
        let mut omega: [_; N] = (0.0..TAU).linspace_array();
        omega.map_assign(|omega| (omega + PI) % TAU - PI);
        omega.rotate_right(N/2);

        let fwhm = w.each_ref()
            .map(|(_, w)| {
                let mut w_f: [_; N] = [0.0; N/4].chain(*w).resize(|_| 0.0).dft().map(|w| w.norm());
                w_f.rotate_right(N/2);
                w_f
            }).fwhm(omega, FullWidthAt::<f64>::ALevel { alevel: 1.0 });

        let mut ranking = w.each_ref()
            .zip(fwhm)
            .map(|((w, _), fwhm)| (w, fwhm));
        ranking.sort_by(|a, b| a.1.total_cmp(&b.1));

        println!("{:?}", ranking);
    }
}