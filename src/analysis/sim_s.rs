use core::ops::Mul;

use ndarray::{Array1, Array2};
use ndarray_linalg::Lapack;
use num::{complex::ComplexFloat, NumCast, One, Zero};
use option_trait::{Maybe, StaticMaybe};

use crate::{quantities::{List, ListOrSingle, Lists, Matrix, MaybeList, MaybeLists, MaybeOwnedList, OwnedList, OwnedListOrSingle, OwnedLists}, systems::{Sos, Ss, SsAMatrix, SsBMatrix, SsCMatrix, SsDMatrix, Tf, Zpk}, transforms::system::ToSs, util::{self, ComplexOp, MaybeLenEq, TwoSidedRange}, System};

pub trait SimS<X, XX>: System
where
    Self::Set: ComplexOp<X>,
    X: ComplexFloat<Real = <Self::Set as ComplexFloat>::Real> + Into<<Self::Set as ComplexOp<X>>::Output>,
    XX: Matrix<X>,
{
    type Output: Lists<<Self::Set as ComplexOp<X>>::Output> + ListOrSingle<<XX::Transpose as MaybeLists<X>>::RowsMapped<<Self::Set as ComplexOp<X>>::Output>>;

    fn sim_s<T, W>(self, t: T, x: XX, w: W, interpolate: bool)
        -> (<XX::Transpose as MaybeLists<X>>::RowsMapped<<Self::Set as ComplexFloat>::Real>, Self::Output, <XX::Transpose as MaybeLists<X>>::RowsMapped<Vec<<Self::Set as ComplexOp<X>>::Output>>)
    where
        T: TwoSidedRange<<Self::Set as ComplexFloat>::Real>,
        W: Maybe<Vec<<Self::Set as ComplexOp<X>>::Output>>;
}

impl<T, A, B, C, D, Y, YY, X, XX, YYY> SimS<X, XX> for Ss<T, A, B, C, D>
where
    T: ComplexOp<X, Output = Y> + ComplexFloat<Real: Into<Y>>,
    X: ComplexFloat<Real = T::Real> + Into<Y>,
    Y: ComplexFloat<Real = T::Real> + Mul<T::Real, Output = Y> + Lapack,
    A: SsAMatrix<T, B, C, D>,
    B: SsBMatrix<T, A, C, D>,
    C: SsCMatrix<T, A, B, D>,
    D: SsDMatrix<T, A, B, C, RowsMapped<YY> = YYY>,
    XX: Matrix<X, Transpose: Matrix<X, RowOwned: MaybeLenEq<B::RowOwned, true>>> + Clone,
    Array2<Y>: SsAMatrix<Y, Array2<Y>, Array2<Y>, Array2<Y>> + SsBMatrix<Y, Array2<Y>, Array2<Y>, Array2<Y>> + SsCMatrix<Y, Array2<Y>, Array2<Y>, Array2<Y>> + SsDMatrix<Y, Array2<Y>, Array2<Y>, Array2<Y>>,
    YYY: OwnedLists<Y> + OwnedListOrSingle<YY>,
    XX::Transpose: MaybeLists<X, RowsMapped<Y> = YY>,
    YY: OwnedList<Y> + ListOrSingle<Y>,
    <XX::Transpose as MaybeLists<X>>::RowsMapped<T::Real>: OwnedList<T::Real>,
    <<<XX::Transpose as MaybeLists<X>>::RowsMapped<T::Real> as ListOrSingle<T::Real>>::Length as StaticMaybe<usize>>::Opposite: Sized,
    <YY::Length as StaticMaybe<usize>>::Opposite: Sized,
    <<YYY as ListOrSingle<YY>>::Length as StaticMaybe<usize>>::Opposite: Sized,
    <XX::Transpose as MaybeLists<X>>::RowsMapped<Vec<Y>>: OwnedList<Vec<Y>>,
    <<<XX::Transpose as MaybeLists<X>>::RowsMapped<Vec<Y>> as ListOrSingle<Vec<Y>>>::Length as StaticMaybe<usize>>::Opposite: Sized
{
    type Output = D::RowsMapped<YY>;

    fn sim_s<TT, W>(self, t: TT, x: XX, w: W, interpolate: bool)
        -> (<XX::Transpose as MaybeLists<X>>::RowsMapped<T::Real>, Self::Output, <XX::Transpose as MaybeLists<X>>::RowsMapped<Vec<Y>>)
    where
        TT: TwoSidedRange<T::Real>,
        W: Maybe<Vec<Y>>
    {
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

        let t0 = *t.start();
        let mut xout = if !t0.is_zero()
        {
            match util::expm(a.t().map(|&a| a*t0))
            {
                Ok(at_expm) => {
                    let w0 = Array1::from_vec(w0).dot(&at_expm);
                    vec![w0.into_vec()]
                },
                Err(_) => {
                    vec![vec![Y::zero(); n]]
                }
            }
        }
        else
        {
            vec![w0]
        };

        let mut u = x.to_array2()
            .map(|&u| u.into());
        let no_input = u.iter()
            .all(|u: &Y| u.is_zero());

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

        let dt = (*t.end() - *t.start())/numaybem1;

        let tt = OwnedListOrSingle::from_len_fn(StaticMaybe::maybe_from_fn(|| nu), |i| {
            let i = <T::Real as NumCast>::from(i).unwrap();
            *t.start() + i*dt
        });

        if no_input
        {
            let expat_dt = match util::expm(a.reversed_axes().map(|&a| a*dt))
            {
                Ok(e) => e,
                Err(_) => Array2::from_elem((n, n), Y::zero())
            };
            for _ in 1..nu
            {
                xout.push(Array1::from_vec(xout.last().unwrap().clone()).dot(&expat_dt).into_vec())
            }
            let c_t = c.t();
            let yout: Vec<_> = xout.iter()
                .map(|xout| Array1::from_vec(xout.clone()).dot(&c_t).into_vec())
                .collect();
            let mut yout = util::transpose_vec_vec(yout).into_iter();
            let mut xout = xout.into_iter();
            return (
                tt,
                OwnedListOrSingle::from_len_fn(StaticMaybe::maybe_from_fn(|| q), |_| {
                    let mut yout = yout.next()
                        .unwrap()
                        .into_iter();
                    OwnedListOrSingle::from_len_fn(StaticMaybe::maybe_from_fn(|| nu), |_| yout.next().unwrap())
                }),
                OwnedListOrSingle::from_len_fn(StaticMaybe::maybe_from_fn(|| nu), |_| xout.next().unwrap())
            )
        }

        resize(&mut u, (p, nu));

        if !interpolate
        {
            let m = Array2::from_shape_fn((n + p, n + p), |(i, j)| {
                if i < n
                {
                    (if j < n
                    {
                        a[(i, j)]
                    }
                    else
                    {
                        b[(i, j - n)]
                    })*dt
                }
                else
                {
                    Y::zero()
                }
            });
            let expmt = match util::expm(m.reversed_axes())
            {
                Ok(e) => e,
                Err(_) => Array2::from_elem((n + p, n + p), Y::zero())
            };
            let ad = expmt.slice(ndarray::s![..n, ..n]);
            let bd = expmt.slice(ndarray::s![n.., ..n]);
            for i in 1..nu
            {
                xout.push((Array1::from_vec(xout.last().unwrap().clone()).dot(&ad)
                    + u.column(i - 1).dot(&bd)
                ).into_vec())
            }
        }
        else
        {
            let m = Array2::from_shape_fn((n + 2*p, n + 2*p), |(i, j)| {
                if i < n
                {
                    if j < n + p
                    {
                        (if j < n
                        {
                            a[(i, j)]
                        }
                        else
                        {
                            b[(i, j - n)]
                        })*dt
                    }
                    else
                    {
                        Y::zero()
                    }
                }
                else
                {
                    if j < n + p || i - n != j - n - p
                    {
                        Y::zero()
                    }
                    else
                    {
                        Y::one()
                    }
                }
            });
            let expmt = match util::expm(m.reversed_axes())
            {
                Ok(e) => e,
                Err(_) => Array2::from_elem((n + 2*p, n + 2*p), Y::zero())
            };
            let ad = expmt.slice(ndarray::s![..n, ..n]);
            let bd1 = expmt.slice(ndarray::s![n + p.., ..n]);
            let bd0 = expmt.slice(ndarray::s![n..n + p, ..n]).to_owned() - bd1;
            for i in 1..nu
            {
                xout.push((Array1::from_vec(xout.last().unwrap().clone()).dot(&ad)
                    + u.column(i - 1).dot(&bd0)
                    + u.column(i).dot(&bd1)
                ).into_vec())
            }
        }

        let c_t = c.t();
        let d_t = d.t();
        let yout: Vec<Vec<Y>> = xout.iter()
            .zip(u.columns())
            .map(|(xout, u)| (Array1::from_vec(xout.clone()).dot(&c_t)
                    + u.dot(&d_t)
                ).into_vec()
            ).collect();

        let mut xout = xout.into_iter();
        let mut yout = util::transpose_vec_vec(yout).into_iter();
        (
            tt,
            OwnedListOrSingle::from_len_fn(StaticMaybe::maybe_from_fn(|| q), |_| {
                let mut yout = yout.next()
                    .unwrap()
                    .into_iter();
                OwnedListOrSingle::from_len_fn(StaticMaybe::maybe_from_fn(|| nu), |_| yout.next().unwrap())
            }),
            OwnedListOrSingle::from_len_fn(StaticMaybe::maybe_from_fn(|| nu), |_| xout.next().unwrap())
        )
    }
}

impl<T, B, A, X, XX, Y, YY, YYY> SimS<X, XX> for Tf<T, B, A>
where
    T: ComplexFloat + ComplexOp<X, Output = Y>,
    B: MaybeLists<T, RowsMapped<YY> = YYY>,
    A: MaybeList<T>,
    X: ComplexFloat<Real = T::Real> + Into<Y>,
    XX: List<X, Transpose: MaybeLists<X, RowsMapped<Y> = YY>>,
    YYY: Lists<Y> + OwnedListOrSingle<YY>,
    Self: System<Set = T> + ToSs<T, Array2<T>, Array2<T>, Array2<T>, Array2<T>>,
    Array2<T>: SsAMatrix<T, Array2<T>, Array2<T>, Array2<T>> + SsBMatrix<T, Array2<T>, Array2<T>, Array2<T>> + SsCMatrix<T, Array2<T>, Array2<T>, Array2<T>> + SsDMatrix<T, Array2<T>, Array2<T>, Array2<T>>,
    Ss<T, Array2<T>, Array2<T>, Array2<T>, Array2<T>>: SimS<X, XX, Output = Vec<YY>> + System<Set = T>,
    <<YYY as ListOrSingle<YY>>::Length as StaticMaybe<usize>>::Opposite: Sized
{
    type Output = YYY;

    fn sim_s<TT, W>(self, t: TT, x: XX, w: W, interpolate: bool)
        -> (<XX::Transpose as MaybeLists<X>>::RowsMapped<T::Real>, Self::Output, <XX::Transpose as MaybeLists<X>>::RowsMapped<Vec<Y>>)
    where
        TT: TwoSidedRange<<Self::Set as ComplexFloat>::Real>,
        W: Maybe<Vec<Y>>
    {
        let (t, y, x) = self.to_ss()
            .sim_s(t, x, w, interpolate);

        let q = y.len();
        let mut y = y.into_iter();

        (
            t,
            OwnedListOrSingle::from_len_fn(StaticMaybe::maybe_from_fn(|| q), |_| y.next().unwrap()),
            x
        )
    }
}

impl<T, Z, P, K, X, XX, Y, YY> SimS<X, XX> for Zpk<T, Z, P, K>
where
    T: ComplexFloat,
    Z: MaybeList<T>,
    P: MaybeList<T>,
    K: ComplexFloat<Real = T::Real> + ComplexOp<X, Output = Y>,
    X: ComplexFloat<Real = T::Real> + Into<Y>,
    XX: List<X, Transpose: Lists<X, RowsMapped<Y> = YY>>,
    YY: List<Y>,
    Self: System<Set = K> + ToSs<K, Array2<K>, Array2<K>, Array2<K>, Array2<K>>,
    Array2<K>: SsAMatrix<K, Array2<K>, Array2<K>, Array2<K>> + SsBMatrix<K, Array2<K>, Array2<K>, Array2<K>> + SsCMatrix<K, Array2<K>, Array2<K>, Array2<K>> + SsDMatrix<K, Array2<K>, Array2<K>, Array2<K>>,
    Ss<K, Array2<K>, Array2<K>, Array2<K>, Array2<K>>: System<Set = K> + SimS<X, XX, Output = Vec<YY>>
{
    type Output = YY;

    fn sim_s<TT, W>(self, t: TT, x: XX, w: W, interpolate: bool)
        -> (<XX::Transpose as MaybeLists<X>>::RowsMapped<T::Real>, Self::Output, <XX::Transpose as MaybeLists<X>>::RowsMapped<Vec<Y>>)
    where
        TT: TwoSidedRange<T::Real>,
        W: Maybe<Vec<Y>>
    {
        let (t, y, x) = self.to_ss()
            .sim_s(t, x, w, interpolate);

        (
            t,
            y.into_iter().next().unwrap(),
            x
        )
    }
}

impl<T, B, A, S, X, XX, Y, YY> SimS<X, XX> for Sos<T, B, A, S>
where
    T: ComplexFloat + ComplexOp<X, Output = Y>,
    B: Maybe<[T; 3]> + MaybeOwnedList<T>,
    A: Maybe<[T; 3]> + MaybeOwnedList<T>,
    S: MaybeList<Tf<T, B, A>>,
    X: ComplexFloat<Real = T::Real> + Into<Y>,
    Y: ComplexFloat,
    XX: List<X, Transpose: Lists<X, RowsMapped<Y> = YY>>,
    YY: List<Y>,
    Self: System<Set = T> + ToSs<T, Array2<T>, Array2<T>, Array2<T>, Array2<T>>,
    Array2<T>: SsAMatrix<T, Array2<T>, Array2<T>, Array2<T>> + SsBMatrix<T, Array2<T>, Array2<T>, Array2<T>> + SsCMatrix<T, Array2<T>, Array2<T>, Array2<T>> + SsDMatrix<T, Array2<T>, Array2<T>, Array2<T>>,
    Ss<T, Array2<T>, Array2<T>, Array2<T>, Array2<T>>: System<Set = T> + SimS<X, XX, Output = Vec<YY>>
{
    type Output = YY;

    fn sim_s<TT, W>(self, t: TT, x: XX, w: W, interpolate: bool)
        -> (<XX::Transpose as MaybeLists<X>>::RowsMapped<T::Real>, Self::Output, <XX::Transpose as MaybeLists<X>>::RowsMapped<Vec<Y>>)
    where
        TT: TwoSidedRange<T::Real>,
        W: Maybe<Vec<Y>>
    {
        let (t, y, x) = self.to_ss()
            .sim_s(t, x, w, interpolate);

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

    use crate::{analysis::SimS, gen::filter::{BesselF, FilterGenPlane, FilterGenType}, plot, systems::Tf};

    #[test]
    fn test()
    {
        let h = Tf::besself(5, [TAU*12.0], FilterGenType::LowPass, FilterGenPlane::S)
            .unwrap();

        const T: f64 = 1.25;
        const N: usize = 200;
        let t: [_; N] = (0.0..T).linspace_array();
        let u = t.map(|t| (TAU*4.0*t).cos() + 0.6*(TAU*40.0*t).sin() + 0.5*(TAU*80.0*t).cos());

        let (t, y, _) = h.sim_s(0.0..T, u, (), true);

        plot::plot_curves("u(t), y(t)", "plots/uy_t_sim_s.png", [&t.zip(u), &t.zip(y)])
            .unwrap();
    }
}