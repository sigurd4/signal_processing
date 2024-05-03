use core::ops::Mul;

use ndarray::{Array1, Array2};
use ndarray_linalg::Lapack;
use num::{complex::ComplexFloat, traits::real::Real, NumCast, One, Zero};
use option_trait::{Maybe, StaticMaybe};

use crate::{quantities::{List, ListOrSingle, Lists, Matrix, MaybeList, MaybeLists, MaybeOwnedList, OwnedListOrSingle}, systems::{Sos, Ss, SsAMatrix, SsBMatrix, SsCMatrix, SsDMatrix, Tf, Zpk}, transforms::system::ToSs, util::{self, ComplexOp, MaybeLenEq, TwoSidedRange}, System};

pub trait SimZ<X, XX>: System
where
    Self::Set: ComplexOp<X>,
    X: ComplexFloat<Real = <Self::Set as ComplexFloat>::Real> + Into<<Self::Set as ComplexOp<X>>::Output>,
    XX: Matrix<X>,
{
    type Output: Lists<<Self::Set as ComplexOp<X>>::Output> + ListOrSingle<Vec<<Self::Set as ComplexOp<X>>::Output>>;

    fn sim_z<T, W>(self, t: T, x: XX, w: W, sampling_frequency: <Self::Set as ComplexFloat>::Real)
        -> (Vec<<Self::Set as ComplexFloat>::Real>, Self::Output, Vec<Vec<<Self::Set as ComplexOp<X>>::Output>>)
    where
        T: TwoSidedRange<<Self::Set as ComplexFloat>::Real>,
        W: Maybe<Vec<<Self::Set as ComplexOp<X>>::Output>>;
}

impl<T, A, B, C, D, Y, X, XX> SimZ<X, XX> for Ss<T, A, B, C, D>
where
    T: ComplexOp<X, Output = Y> + ComplexFloat<Real: Into<Y>>,
    X: ComplexFloat<Real = T::Real> + Into<Y>,
    Y: ComplexFloat<Real = T::Real> + Mul<T::Real, Output = Y> + Lapack,
    A: SsAMatrix<T, B, C, D>,
    B: SsBMatrix<T, A, C, D>,
    C: SsCMatrix<T, A, B, D>,
    D: SsDMatrix<T, A, B, C>,
    XX: Matrix<X, Transpose: Matrix<X, RowOwned: MaybeLenEq<D::RowOwned, true> + MaybeLenEq<B::RowOwned, true>>> + Clone,
    Array2<Y>: SsAMatrix<Y, Array2<Y>, Array2<Y>, Array2<Y>> + SsBMatrix<Y, Array2<Y>, Array2<Y>, Array2<Y>> + SsCMatrix<Y, Array2<Y>, Array2<Y>, Array2<Y>> + SsDMatrix<Y, Array2<Y>, Array2<Y>, Array2<Y>>,
    D::RowsMapped<Vec<Y>>: Lists<Y> + OwnedListOrSingle<Vec<Y>, Length: StaticMaybe<usize, Opposite: Sized>>
{
    type Output = D::RowsMapped<Vec<Y>>;

    fn sim_z<TT, W>(self, t: TT, x: XX, w: W, sampling_frequency: T::Real)
        -> (Vec<T::Real>, Self::Output, Vec<Vec<Y>>)
    where
        TT: TwoSidedRange<T::Real>,
        W: Maybe<Vec<Y>>
    {
        (|| {
        let Ss {mut a, mut b, mut c, mut d, ..} = Ss::new(
            self.a.to_array2().map(|&a| a.into()),
            self.b.to_array2().map(|&b| b.into()),
            self.c.to_array2().map(|&c| c.into()),
            self.d.to_array2().map(|&d| d.into())
        );

        let (mu, nu) = x.matrix_dim();
        let (ma, na) = a.dim();
        let (mb, nb) = b.dim();
        let (mc, nc) = c.dim();
        let (md, nd) = d.dim();

        let n = ma.max(na).max(mb).max(nc);
        let p = nb.max(nd).max(mu);
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
                w.resize(n, Y::zero());
                w
            })
            .unwrap_or_else(|| vec![Y::zero(); n]);

        let mut xout = vec![w0];

        let mut u = x.to_array2()
            .map(|&u| u.into());

        let zero = T::Real::zero();
        let one = T::Real::one();

        let nuf = <T::Real as NumCast>::from(nu).unwrap();
        let numaybem1 = if t.is_end_inclusive()
        {
            nuf - one
        }
        else
        {
            nuf
        };

        let dtt = (*t.end() - *t.start())*sampling_frequency/numaybem1;

        let tt: Vec<_> = (0..nu).map(|i| {
            let i = <T::Real as NumCast>::from(i).unwrap();
            *t.start()*sampling_frequency + i*dtt
        }).collect();

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
        
        resize(&mut u, (p, nu));

        let mut j = 0;
        let u: Vec<Vec<_>> = t.iter()
            .map(|&i| {
                if nu == 0
                {
                    return vec![Y::zero(); p]
                }
                while j < nu && i > tt[j]
                {
                    j += 1;
                }
                if j <= 0
                {
                    u.column(0).to_vec()
                }
                else if j >= nu
                {
                    u.column(nu - 1).to_vec()
                }
                else
                {
                    let p = (i - tt[j - 1])/(tt[j] - tt[j - 1]);
                    let q = one - p;

                    u.column(j - 1).into_iter()
                        .zip(u.column(j))
                        .map(|(&u0, &u1)| u0*q + u1*p)
                        .collect()
                }
            }).collect();
        
        let mut yout = vec![];
        for _ in 0..*t_range.start()
        {
            let x = xout.last()
                .map(|x| Array1::from_vec(x.clone()))
                .unwrap_or_else(|| Array1::from_elem(n, Y::zero()));
            yout.push(c.dot(&x).to_vec());
            xout.push(a.dot(&x).to_vec());
        }
        let mut u = u.into_iter();
        for _ in (*t_range.start() + 1)..*t_range.end()
        {
            let x = xout.last()
                .map(|x| Array1::from_vec(x.clone()))
                .unwrap_or_else(|| Array1::from_elem(n, Y::zero()));
            let u = Array1::from_vec(u.next().unwrap());
            yout.push((c.dot(&x) + d.dot(&u)).to_vec());
            xout.push((a.dot(&x) + b.dot(&u)).to_vec());
        }
        if *t_range.start() < *t_range.end()
        {
            let x = xout.last()
                .map(|x| Array1::from_vec(x.clone()))
                .unwrap_or_else(|| Array1::from_elem(n, Y::zero()));
            let u = Array1::from_vec(u.next().unwrap());
            yout.push((c.dot(&x) + d.dot(&u)).to_vec());
        }

        let xout = xout.into_iter()
            .skip(*t_range.start())
            .collect();
        let mut yout = util::transpose_vec_vec(yout.into_iter()
                .skip(*t_range.start())
                .collect()
            ).into_iter();
        (
            t.into_iter()
                .map(|t| t/sampling_frequency)
                .collect(),
            OwnedListOrSingle::from_len_fn(StaticMaybe::maybe_from_fn(|| q), |_| {
                yout.next()
                    .unwrap()
            }),
            xout
        )
        })()
    }
}

impl<T, B, A, X, XX, Y> SimZ<X, XX> for Tf<T, B, A>
where
    T: ComplexFloat + ComplexOp<X, Output = Y>,
    B: MaybeLists<T>,
    A: MaybeList<T>,
    X: ComplexFloat<Real = T::Real> + Into<Y>,
    XX: List<X, Transpose: MaybeLists<X>>,
    Self: System<Set = T> + ToSs<T, Array2<T>, Array2<T>, Array2<T>, Array2<T>>,
    Array2<T>: SsAMatrix<T, Array2<T>, Array2<T>, Array2<T>> + SsBMatrix<T, Array2<T>, Array2<T>, Array2<T>> + SsCMatrix<T, Array2<T>, Array2<T>, Array2<T>> + SsDMatrix<T, Array2<T>, Array2<T>, Array2<T>>,
    Ss<T, Array2<T>, Array2<T>, Array2<T>, Array2<T>>: SimZ<X, XX, Output = Vec<Vec<Y>>> + System<Set = T>,
    B::RowsMapped<Vec<Y>>: Lists<Y> + OwnedListOrSingle<Vec<Y>, Length: StaticMaybe<usize, Opposite: Sized>>
{
    type Output = B::RowsMapped<Vec<Y>>;

    fn sim_z<TT, W>(self, t: TT, x: XX, w: W, sampling_frequency: T::Real)
        -> (Vec<T::Real>, Self::Output, Vec<Vec<Y>>)
    where
        TT: TwoSidedRange<<Self::Set as ComplexFloat>::Real>,
        W: Maybe<Vec<Y>>
    {
        let (t, y, x) = self.to_ss()
            .sim_z(t, x, w, sampling_frequency);

        let q = y.len();
        let mut y = y.into_iter();

        (
            t,
            OwnedListOrSingle::from_len_fn(StaticMaybe::maybe_from_fn(|| q), |_| y.next().unwrap()),
            x
        )
    }
}

impl<T, Z, P, K, X, XX, Y> SimZ<X, XX> for Zpk<T, Z, P, K>
where
    T: ComplexFloat,
    Z: MaybeList<T>,
    P: MaybeList<T>,
    K: ComplexFloat<Real = T::Real> + ComplexOp<X, Output = Y>,
    X: ComplexFloat<Real = T::Real> + Into<Y>,
    XX: List<X, Transpose: Lists<X>>,
    Self: System<Set = K> + ToSs<K, Array2<K>, Array2<K>, Array2<K>, Array2<K>>,
    Array2<K>: SsAMatrix<K, Array2<K>, Array2<K>, Array2<K>> + SsBMatrix<K, Array2<K>, Array2<K>, Array2<K>> + SsCMatrix<K, Array2<K>, Array2<K>, Array2<K>> + SsDMatrix<K, Array2<K>, Array2<K>, Array2<K>>,
    Ss<K, Array2<K>, Array2<K>, Array2<K>, Array2<K>>: System<Set = K> + SimZ<X, XX, Output = Vec<Vec<Y>>>
{
    type Output = Vec<Y>;

    fn sim_z<TT, W>(self, t: TT, x: XX, w: W, sampling_frequency: T::Real)
        -> (Vec<T::Real>, Self::Output, Vec<Vec<Y>>)
    where
        TT: TwoSidedRange<T::Real>,
        W: Maybe<Vec<Y>>
    {
        let (t, y, x) = self.to_ss()
            .sim_z(t, x, w, sampling_frequency);

        (
            t,
            y.into_iter().next().unwrap(),
            x
        )
    }
}

impl<T, B, A, S, X, XX, Y> SimZ<X, XX> for Sos<T, B, A, S>
where
    T: ComplexFloat + ComplexOp<X, Output = Y>,
    B: Maybe<[T; 3]> + MaybeOwnedList<T>,
    A: Maybe<[T; 3]> + MaybeOwnedList<T>,
    S: MaybeList<Tf<T, B, A>>,
    X: ComplexFloat<Real = T::Real> + Into<Y>,
    Y: ComplexFloat,
    XX: List<X, Transpose: Lists<X>>,
    Self: System<Set = T> + ToSs<T, Array2<T>, Array2<T>, Array2<T>, Array2<T>>,
    Array2<T>: SsAMatrix<T, Array2<T>, Array2<T>, Array2<T>> + SsBMatrix<T, Array2<T>, Array2<T>, Array2<T>> + SsCMatrix<T, Array2<T>, Array2<T>, Array2<T>> + SsDMatrix<T, Array2<T>, Array2<T>, Array2<T>>,
    Ss<T, Array2<T>, Array2<T>, Array2<T>, Array2<T>>: System<Set = T> + SimZ<X, XX, Output = Vec<Vec<Y>>>
{
    type Output = Vec<Y>;

    fn sim_z<TT, W>(self, t: TT, x: XX, w: W, sampling_frequency: T::Real)
        -> (Vec<T::Real>, Self::Output, Vec<Vec<Y>>)
    where
        TT: TwoSidedRange<T::Real>,
        W: Maybe<Vec<Y>>
    {
        let (t, y, x) = self.to_ss()
            .sim_z(t, x, w, sampling_frequency);

        (
            t,
            y.into_iter().next().unwrap(),
            x
        )
    }
}

#[cfg(test)]
mod test
{
    use core::f64::consts::TAU;

    use array_math::ArrayOps;
    use linspace::LinspaceArray;

    use crate::{analysis::SimZ, gen::filter::{BesselF, FilterGenPlane, FilterGenType}, plot, systems::Tf};

    #[test]
    fn test()
    {
        const T: f64 = 1.25;
        const N: usize = 200;
        const FS: f64 = N as f64/T;
        let h = Tf::besself(5, [12.0], FilterGenType::LowPass, FilterGenPlane::Z { sampling_frequency: Some(FS) })
            .unwrap();

        let t: [_; N] = (0.0..T).linspace_array();
        let u = t.map(|t| (TAU*4.0*t).cos() + 0.6*(TAU*40.0*t).sin() + 0.5*(TAU*80.0*t).cos());

        let (tt, y, _) = h.sim_z(0.0..T, u, (), FS);

        plot::plot_curves("u(t), y(t)", "plots/uy_t_sim_z.png", [&t.zip(u), &tt.into_iter().zip(y).collect::<Vec<_>>()])
            .unwrap();
    }
}