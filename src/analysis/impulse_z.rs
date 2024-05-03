use ndarray::{Array1, Array2};
use num::{complex::ComplexFloat, traits::real::Real, NumCast, Zero};
use option_trait::{Maybe, StaticMaybe};

use crate::{quantities::{Lists, MatrixOrSingle, MaybeList, MaybeLists, MaybeOwnedList, OwnedListOrSingle}, systems::{Sos, Ss, SsAMatrix, SsBMatrix, SsCMatrix, SsDMatrix, Tf, Zpk}, transforms::system::ToSs, util::{self, TwoSidedRange}, System};

pub trait ImpulseZ: System
{
    type Output: MatrixOrSingle<Vec<Self::Set>>;

    fn impulse_z<T, W>(self, t: T, w: W, sampling_frequency: <Self::Set as ComplexFloat>::Real)
        -> (Vec<<Self::Set as ComplexFloat>::Real>, Self::Output)
    where
        T: TwoSidedRange<<Self::Set as ComplexFloat>::Real>,
        W: Maybe<Vec<Self::Set>>;
}

impl<T, A, B, C, D> ImpulseZ for Ss<T, A, B, C, D>
where
    T: ComplexFloat + 'static,
    A: SsAMatrix<T, B, C, D>,
    B: SsBMatrix<T, A, C, D>,
    C: SsCMatrix<T, A, B, D>,
    D: SsDMatrix<T, A, B, C>,
    <D::Transpose as MaybeLists<T>>::RowsMapped<D::RowsMapped<Vec<T>>>: Lists<Vec<T>> + OwnedListOrSingle<D::RowsMapped<Vec<T>>, Length: StaticMaybe<usize, Opposite: Sized>>,
    D::RowsMapped<Vec<T>>: OwnedListOrSingle<Vec<T>, Length: StaticMaybe<usize, Opposite: Sized>>,
    Array2<T>: SsAMatrix<T, Array2<T>, Array2<T>, Array2<T>> + SsBMatrix<T, Array2<T>, Array2<T>, Array2<T>> + SsCMatrix<T, Array2<T>, Array2<T>, Array2<T>> + SsDMatrix<T, Array2<T>, Array2<T>, Array2<T>>
{
    type Output = <<D::Transpose as MaybeLists<T>>::RowsMapped<D::RowsMapped<Vec<T>>> as Lists<Vec<T>>>::CoercedMatrix;

    fn impulse_z<TT, W>(self, t: TT, w: W, sampling_frequency: T::Real)
        -> (Vec<T::Real>, Self::Output)
    where
        TT: TwoSidedRange<T::Real>,
        W: Maybe<Vec<T>>
    {
        let Ss {mut a, mut b, mut c, mut d, ..} = Ss::new(
            self.a.to_array2().map(|&a| a.into()),
            self.b.to_array2().map(|&b| b.into()),
            self.c.to_array2().map(|&c| c.into()),
            self.d.to_array2().map(|&d| d.into())
        );

        let (ma, na) = a.dim();
        let (mb, nb) = b.dim();
        let (mc, nc) = c.dim();
        let (md, nd) = d.dim();

        let n = ma.max(na).max(mb).max(nc);
        let p = nb.max(nd);
        let q = mc.max(md);

        fn resize<T>(m: &mut Array2<T>, dim: (usize, usize))
        where
            T: Zero + Clone
        {
            let row = Array1::from_elem(m.dim().1, T::zero());
            while m.dim().0 < dim.0
            {
                m.push_row(row.view()).unwrap();
            }
            let col = Array1::from_elem(m.dim().0, T::zero());
            while m.dim().1 < dim.1
            {
                m.push_column(col.view()).unwrap();
            }
        }

        resize(&mut a, (n, n));
        resize(&mut b, (n, p));
        resize(&mut c, (q, n));
        resize(&mut d, (q, p));
        
        let w0 = w.into_option()
            .map(|mut w| {
                w.resize(n, T::zero());
                w
            })
            .unwrap_or_else(|| vec![T::zero(); n]);

        let zero = T::Real::zero();

        let tmax = <T::Real as NumCast>::from(usize::MAX).unwrap();
        let t_range = <usize as NumCast>::from((*t.start()*sampling_frequency).floor()
                    .min(tmax)
                    .max(zero)
                ).unwrap()
            ..
            <usize as NumCast>::from((*t.end()*sampling_frequency).ceil()
                    .min(tmax)
                    .max(zero)
                ).unwrap()
                .saturating_add(t.is_end_inclusive() as usize);
        let t: Vec<_> = t_range.clone()
            .map(|t| <T::Real as NumCast>::from(t).unwrap())
            .collect();
        
        let yout = <D::Transpose as MaybeLists<T>>::RowsMapped::<D::RowsMapped::<Vec<T>>>::from_len_fn(StaticMaybe::maybe_from_fn(|| p), |i| {
            let mut xout = vec![w0.clone()];
            let mut yout = vec![];
            if *t_range.end() > 0 && *t_range.start() < *t_range.end()
            {
                let x = xout.last()
                    .map(|x| Array1::from_vec(x.clone()))
                    .unwrap_or_else(|| Array1::from_elem(n, T::zero()));
                let u = Array1::from_shape_fn(p, |p| T::from((p == i) as u8).unwrap());
                yout.push((c.dot(&x) + d.dot(&u)).to_vec());
                xout.push((a.dot(&x) + b.dot(&u)).to_vec());
            }
            for _ in 2..*t_range.end()
            {
                let x = xout.last()
                    .map(|x| Array1::from_vec(x.clone()))
                    .unwrap_or_else(|| Array1::from_elem(n, T::zero()));
                yout.push(c.dot(&x).to_vec());
                xout.push(a.dot(&x).to_vec());
            }
            if *t_range.end() > 0 && *t_range.start() < *t_range.end()
            {
                let x = xout.last()
                    .map(|x| Array1::from_vec(x.clone()))
                    .unwrap_or_else(|| Array1::from_elem(n, T::zero()));
                yout.push(c.dot(&x).to_vec());
            }
    
            let mut yout = util::transpose_vec_vec(yout.into_iter()
                    .skip(*t_range.start())
                    .collect()
                ).into_iter();
            OwnedListOrSingle::from_len_fn(StaticMaybe::maybe_from_fn(|| q), |_| {
                yout.next()
                    .unwrap()
            })
        }).coerce_into_matrix(|| panic!("Invalid matrix size"));

        (
            t.into_iter()
                .map(|t| t/sampling_frequency)
                .collect(),
            yout
        )
    }
}

impl<T, B, A> ImpulseZ for Tf<T, B, A>
where
    T: ComplexFloat,
    B: MaybeLists<T>,
    A: MaybeList<T>,
    Self: System<Set = T> + ToSs<T, Array2<T>, Array2<T>, Array2<T>, Array2<T>>,
    Array2<T>: SsAMatrix<T, Array2<T>, Array2<T>, Array2<T>> + SsBMatrix<T, Array2<T>, Array2<T>, Array2<T>> + SsCMatrix<T, Array2<T>, Array2<T>, Array2<T>> + SsDMatrix<T, Array2<T>, Array2<T>, Array2<T>>,
    Ss<T, Array2<T>, Array2<T>, Array2<T>, Array2<T>>: ImpulseZ<Output = Array2<Vec<T>>> + System<Set = T>,
    B::RowsMapped<Vec<T>>: Lists<T> + OwnedListOrSingle<Vec<T>, Length: StaticMaybe<usize, Opposite: Sized>>
{
    type Output = B::RowsMapped<Vec<T>>;

    fn impulse_z<TT, W>(self, t: TT, w: W, sampling_frequency: T::Real)
        -> (Vec<T::Real>, Self::Output)
    where
        TT: TwoSidedRange<T::Real>,
        W: Maybe<Vec<T>>
    {
        let (t, y) = self.to_ss()
            .impulse_z(t, w, sampling_frequency);

        let q = y.len();
        let mut y = y.into_iter();

        (
            t,
            OwnedListOrSingle::from_len_fn(StaticMaybe::maybe_from_fn(|| q), |_| y.next().unwrap())
        )
    }
}

impl<T, Z, P, K> ImpulseZ for Zpk<T, Z, P, K>
where
    T: ComplexFloat,
    Z: MaybeList<T>,
    P: MaybeList<T>,
    K: ComplexFloat<Real = T::Real>,
    Self: System<Set = K> + ToSs<K, Array2<K>, Array2<K>, Array2<K>, Array2<K>>,
    Array2<K>: SsAMatrix<K, Array2<K>, Array2<K>, Array2<K>> + SsBMatrix<K, Array2<K>, Array2<K>, Array2<K>> + SsCMatrix<K, Array2<K>, Array2<K>, Array2<K>> + SsDMatrix<K, Array2<K>, Array2<K>, Array2<K>>,
    Ss<K, Array2<K>, Array2<K>, Array2<K>, Array2<K>>: System<Set = K> + ImpulseZ<Output = Array2<Vec<K>>>
{
    type Output = Vec<K>;

    fn impulse_z<TT, W>(self, t: TT, w: W, sampling_frequency: T::Real)
        -> (Vec<T::Real>, Self::Output)
    where
        TT: TwoSidedRange<T::Real>,
        W: Maybe<Vec<K>>
    {
        let (t, y) = self.to_ss()
            .impulse_z(t, w, sampling_frequency);

        (
            t,
            y.into_iter().next().unwrap()
        )
    }
}

impl<T, B, A, S> ImpulseZ for Sos<T, B, A, S>
where
    T: ComplexFloat,
    B: Maybe<[T; 3]> + MaybeOwnedList<T>,
    A: Maybe<[T; 3]> + MaybeOwnedList<T>,
    S: MaybeList<Tf<T, B, A>>,
    Self: System<Set = T> + ToSs<T, Array2<T>, Array2<T>, Array2<T>, Array2<T>>,
    Array2<T>: SsAMatrix<T, Array2<T>, Array2<T>, Array2<T>> + SsBMatrix<T, Array2<T>, Array2<T>, Array2<T>> + SsCMatrix<T, Array2<T>, Array2<T>, Array2<T>> + SsDMatrix<T, Array2<T>, Array2<T>, Array2<T>>,
    Ss<T, Array2<T>, Array2<T>, Array2<T>, Array2<T>>: System<Set = T> + ImpulseZ<Output = Array2<Vec<T>>>
{
    type Output = Vec<T>;

    fn impulse_z<TT, W>(self, t: TT, w: W, sampling_frequency: T::Real)
        -> (Vec<T::Real>, Self::Output)
    where
        TT: TwoSidedRange<T::Real>,
        W: Maybe<Vec<T>>
    {
        let (t, y) = self.to_ss()
            .impulse_z(t, w, sampling_frequency);

        (
            t,
            y.into_iter().next().unwrap()
        )
    }
}

#[cfg(test)]
mod test
{
    use crate::{analysis::ImpulseZ, gen::filter::{BesselF, FilterGenPlane, FilterGenType}, plot, systems::Tf};

    #[test]
    fn test()
    {
        const T: f64 = 1.25;
        const N: usize = 200;
        const FS: f64 = N as f64/T;
        let h = Tf::besself(5, [12.0], FilterGenType::LowPass, FilterGenPlane::Z { sampling_frequency: Some(FS) })
            .unwrap();

        let (tt, y) = h.impulse_z(0.0..T, (), FS);

        plot::plot_curves("y(t)", "plots/y_t_impulse_z.png", [&tt.into_iter().zip(y).collect::<Vec<_>>()])
            .unwrap();
    }
}