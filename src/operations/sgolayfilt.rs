use std::ops::Deref;

use ndarray::{Array1, Array2};
use num::complex::ComplexFloat;
use thiserror::Error;

use crate::{ComplexOp, OwnedListOrSingle, ContainerOrSingle, Filter, List, ListOrSingle, Lists, MaybeContainer, MaybeLenEq, OwnedList, System, Tf};

//FIXME: Discontinuous at len - k.

#[derive(Debug, Clone, Copy, PartialEq, Error)]
pub enum SGolayFiltError
{
    #[error("Not a Savitzsky-Golay filter set. Filter count should equal filter length.")]
    NotSGolay,
    #[error("Insufficient data for filter. Data sequence length must be larger or equal to filter length.")]
    InsufficientData
}

pub trait SGolayFilt<T, B, X, XX>: List<Tf<T, B>>
where
    X: ComplexFloat + Into<<T as ComplexOp<X>>::Output>,
    T: ComplexFloat<Real = X::Real> + ComplexOp<X>,
    B: List<T> + MaybeLenEq<Self, true>,
    XX: Lists<X>
{
    fn sgolayfilt(self, x: XX) -> Result<XX::Mapped<<T as ComplexOp<X>>::Output>, SGolayFiltError>;
}

impl<T, B, L, X, XX, Y> SGolayFilt<T, B, X, XX> for L
where
    L: List<Tf<T, B>>,
    X: ComplexFloat + Into<Y>,
    T: ComplexFloat<Real = X::Real> + ComplexOp<X, Output = Y>,
    B: List<T, Owned: OwnedList<T>> + MaybeLenEq<Self, true> + Clone,
    XX: Lists<X, RowOwned: OwnedList<X>>,
    <XX::RowOwned as ContainerOrSingle<X>>::Mapped<Y>: OwnedList<Y>,
    Y: ComplexFloat + Clone + 'static,
    XX::RowsMapped<<XX::RowOwned as ContainerOrSingle<X>>::Mapped<Y>>: Into<XX::Mapped<Y>>,
    for<'a> <B::Owned as MaybeContainer<T>>::View<'a>: List<T>,
    for<'a> Tf<T, <B::Owned as MaybeContainer<T>>::View<'a>>: Filter<X, XX::RowOwned, Output = <XX::RowOwned as ContainerOrSingle<X>>::Mapped<Y>> + System<Domain = T>
{
    fn sgolayfilt(self, x: XX) -> Result<XX::Mapped<Y>, SGolayFiltError>
    {
        let mut h: Vec<_> = self.into_vec()
            .into_iter()
            .map(|tf| tf.into_owned())
            .collect();
        let n = h.len();

        for tf in h.iter()
        {
            if tf.b.as_view_slice().len() > n
            {
                return Err(SGolayFiltError::NotSGolay)
            }
        }

        let k = n/2;
        h[k].b.as_mut_slice()
            .reverse();

        Ok(x.try_map_rows_into_owned(|x| {
            let len = x.as_view_slice()
                .len();
            if len < n
            {
                return Err(SGolayFiltError::InsufficientData)
            }

            let x1 = Array1::<Y>::from_shape_fn(n, |i| x.as_view_slice()[i].into());
            let x2 = Array1::<Y>::from_shape_fn(n, |i| x.as_view_slice()[len + i - n].into());

            let mut y = h[k].as_view()
                .filter(x, ());

            y.as_mut_slice()
                .rotate_left(n - k);

            let h1 = Array2::<Y>::from_shape_fn((k, n), |(i, j)| h[i].b.deref().as_view_slice()[j].into());
            let h2 = Array2::<Y>::from_shape_fn((n - k - 1, n), |(i, j)| h[i + k + 1].b.deref().as_view_slice()[j].into());

            let y1 = h1.dot(&x1);
            let y2 = h2.dot(&x2);

            for (y, y1) in y.as_mut_slice()[..k]
                .iter_mut()
                .zip(y1.into_iter())
            {
                *y = y1
            }
            for (y, y2) in y.as_mut_slice()[len + 1 + k - n..]
                .iter_mut()
                .zip(y2.into_iter())
            {
                *y = y2
            }
            Ok(y)
        })?.into())
    }
}

#[cfg(test)]
mod test
{
    use core::f64::consts::TAU;

    use array_math::ArrayOps;
    use rand::distributions::uniform::SampleRange;

    use crate::{plot, SGolay, SGolayFilt, Tf};

    #[test]
    fn test()
    {
        const N: usize = 21;
        let h: [Tf::<_, [f64; N]>; N] = Tf::sgolay(4, (), (), ())
            .unwrap();

        const M: usize = 128;
        const FS: f64 = 100.0;
        const F: f64 = 4.0;
        let t: [_; M] = core::array::from_fn(|i| i as f64/FS);

        let mut rng = rand::thread_rng();
        let x = t.map(|t| (TAU*F*t).cos() + (-1.0..1.0).sample_single(&mut rng));

        let y = SGolayFilt::sgolayfilt(h, x)
            .unwrap();

        plot::plot_curves("x(t), y(t)", "plots/xy_t_sgolayfilt.png", [&t.zip(x), &t.zip(y)])
            .unwrap();
    }
}