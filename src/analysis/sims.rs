use core::ops::Mul;

use ndarray::{Array1, Array2};
use ndarray_linalg::Lapack;
use num::{complex::ComplexFloat, NumCast, One, Zero};
use option_trait::{Maybe, StaticMaybe};

use crate::{quantities::{ListOrSingle, Lists, Matrix, MaybeLists, OwnedList, OwnedListOrSingle, OwnedLists}, systems::{Ss, SsAMatrix, SsBMatrix, SsCMatrix, SsDMatrix}, util::{self, ComplexOp, MaybeLenEq, TwoSidedRange}, System};

pub trait SimS<X, XX>: System
where
    Self::Set: ComplexOp<X>,
    X: ComplexFloat<Real = <Self::Set as ComplexFloat>::Real> + Into<<Self::Set as ComplexOp<X>>::Output>,
    XX: Matrix<X>,
{
    type Output: Lists<<Self::Set as ComplexOp<X>>::Output> + ListOrSingle<<XX::Transpose as MaybeLists<X>>::RowsMapped<<Self::Set as ComplexOp<X>>::Output>>;

    fn sims<T, W>(self, t: T, x: XX, w: W, interpolate: bool)
        -> (<XX::Transpose as MaybeLists<X>>::RowsMapped<<Self::Set as ComplexFloat>::Real>, Self::Output, Self::Output)
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
    C: SsCMatrix<T, A, B, D, RowsMapped<YY> = YYY>,
    D: SsDMatrix<T, A, B, C>,
    XX: Matrix<X, Transpose: Matrix<X, RowOwned: MaybeLenEq<B::RowOwned, true>>> + Clone,
    Array2<Y>: SsAMatrix<Y, Array2<Y>, Array2<Y>, Array2<Y>> + SsBMatrix<Y, Array2<Y>, Array2<Y>, Array2<Y>> + SsCMatrix<Y, Array2<Y>, Array2<Y>, Array2<Y>> + SsDMatrix<Y, Array2<Y>, Array2<Y>, Array2<Y>>,
    YYY: OwnedLists<Y> + OwnedListOrSingle<YY>,
    XX::Transpose: MaybeLists<X, RowsMapped<Y> = YY>,
    YY: OwnedList<Y> + ListOrSingle<Y>,
    <XX::Transpose as MaybeLists<X>>::RowsMapped<T::Real>: OwnedList<T::Real>,
    <<<XX::Transpose as MaybeLists<X>>::RowsMapped<T::Real> as ListOrSingle<T::Real>>::Length as StaticMaybe<usize>>::Opposite: Sized,
    <YY::Length as StaticMaybe<usize>>::Opposite: Sized,
    <<YYY as ListOrSingle<YY>>::Length as StaticMaybe<usize>>::Opposite: Sized
{
    type Output = C::RowsMapped<YY>;

    fn sims<TT, W>(self, t: TT, x: XX, w: W, interpolate: bool)
        -> (<XX::Transpose as MaybeLists<X>>::RowsMapped<T::Real>, Self::Output, Self::Output)
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
                let mut xout = util::transpose_vec_vec(xout).into_iter();
                let mut yout = util::transpose_vec_vec(yout).into_iter();
                return (
                    tt,
                    OwnedListOrSingle::from_len_fn(StaticMaybe::maybe_from_fn(|| q), |_| {
                        let mut yout = yout.next()
                            .unwrap()
                            .into_iter();
                        OwnedListOrSingle::from_len_fn(StaticMaybe::maybe_from_fn(|| nu), |_| yout.next().unwrap())
                    }),
                    OwnedListOrSingle::from_len_fn(StaticMaybe::maybe_from_fn(|| n), |_| {
                        let mut xout = xout.next()
                            .unwrap()
                            .into_iter();
                        OwnedListOrSingle::from_len_fn(StaticMaybe::maybe_from_fn(|| nu), |_| xout.next().unwrap())
                    })
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

            let mut xout = util::transpose_vec_vec(xout).into_iter();
            let mut yout = util::transpose_vec_vec(yout).into_iter();
            (
                tt,
                OwnedListOrSingle::from_len_fn(StaticMaybe::maybe_from_fn(|| q), |_| {
                    let mut yout = yout.next()
                        .unwrap()
                        .into_iter();
                    OwnedListOrSingle::from_len_fn(StaticMaybe::maybe_from_fn(|| nu), |_| yout.next().unwrap())
                }),
                OwnedListOrSingle::from_len_fn(StaticMaybe::maybe_from_fn(|| n), |_| {
                    let mut xout = xout.next()
                        .unwrap()
                        .into_iter();
                    OwnedListOrSingle::from_len_fn(StaticMaybe::maybe_from_fn(|| nu), |_| xout.next().unwrap())
                })
            )
        })()
    }
}

#[cfg(test)]
mod test
{
    use core::f64::consts::TAU;

    use array_math::ArrayOps;
    use linspace::LinspaceArray;

    use crate::{analysis::SimS, gen::filter::{BesselF, FilterGenPlane, FilterGenType}, plot, systems::Ss};

    #[test]
    fn test()
    {
        let h = Ss::besself(5, [TAU*12.0], FilterGenType::LowPass, FilterGenPlane::S)
            .unwrap();

        const T: f64 = 1.25;
        const N: usize = 200;
        let t: [_; N] = (0.0..T).linspace_array();
        let u = t.map(|t| (TAU*4.0*t).cos() + 0.6*(TAU*40.0*t).sin() + 0.5*(TAU*80.0*t).cos());

        let (t, y, _) = h.sims(0.0..T, u, (), false);
        let y = y.into_iter().next().unwrap();

        plot::plot_curves("u(t), y(t)", "plots/uy_t_sims.png", [&t.zip(u), &t.zip(y)])
            .unwrap();
    }
}