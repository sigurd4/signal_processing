use core::{iter::Sum, ops::{DivAssign, MulAssign}};

use ndarray::Array2;
use num::{complex::ComplexFloat, NumCast, One};
use option_trait::{Maybe, StaticMaybe};

use crate::{quantities::{ContainerOrSingle, List, ListOrSingle, Lists, Matrix, MatrixOrSingle, MaybeList, MaybeLists, MaybeOwnedList, OwnedList, OwnedListOrSingle}, systems::{Sos, Ss, SsAMatrix, SsBMatrix, SsCMatrix, SsDMatrix, Tf, Zpk}, transforms::system::ToSs, util::TwoSidedRange, System};

use super::SimS;

pub trait ImpulseS<L>: System
where
    L: List<<Self::Set as ComplexFloat>::Real>,
    <L::Length as StaticMaybe<usize>>::Opposite: Sized
{
    type OutputI: Lists<Self::Set> + ListOrSingle<L::Mapped<Self::Set>>;
    type Output: MatrixOrSingle<L::Mapped<Self::Set>>;

    fn impulse_s<T, W>(self, t: T, numtaps: <L::Length as StaticMaybe<usize>>::Opposite, w: W)
        -> (L, Self::Output)
    where
        T: TwoSidedRange<<Self::Set as ComplexFloat>::Real> + Clone,
        W: Maybe<Vec<Self::Set>>;
}

impl<T, A, B, C, D, L, YY> ImpulseS<L> for Ss<T, A, B, C, D>
where
    T: ComplexFloat + MulAssign<T::Real>,
    A: SsAMatrix<T, B, C, D>,
    B: for<'a> SsBMatrix<T, A, C, D, RowView<'a>: List<T>>,
    C: SsCMatrix<T, A, B, D>,
    D: SsDMatrix<T, A, B, C>,
    L: OwnedList<T::Real, Mapped<T> = YY>,
    <L::Length as StaticMaybe<usize>>::Opposite: Sized + Clone,
    D::RowsMapped<YY>: Lists<T>,
    YY: OwnedList<T, Length = L::Length> + Clone,
    L::Mapped<Vec<T>>: OwnedList<Vec<T>, Length: StaticMaybe<usize, Opposite: Sized>>,
    for<'a> Ss<T, A::View<'a>, B::View<'a>, C::View<'a>, D::View<'a>>: System<Set = T> + SimS<T, Array2<T>, Output = D::RowsMapped<Vec<T>>>,
    for<'a> A::View<'a>: SsAMatrix<T, B::View<'a>, C::View<'a>, D::View<'a>>,
    for<'a> B::View<'a>: SsBMatrix<T, A::View<'a>, C::View<'a>, D::View<'a>>,
    for<'a> C::View<'a>: SsCMatrix<T, A::View<'a>, B::View<'a>, D::View<'a>>,
    for<'a> D::View<'a>: SsDMatrix<T, A::View<'a>, B::View<'a>, C::View<'a>>,
    D::RowsMapped<Vec<T>>: ListOrSingle<Vec<T>, Mapped<YY>: Into<D::RowsMapped<YY>>>,
    <D::Transpose as MaybeLists<T>>::RowsMapped<D::RowsMapped<YY>>: OwnedListOrSingle<D::RowsMapped<YY>, Length: StaticMaybe<usize, Opposite: Sized>> + Lists<YY>,
    <<D::Transpose as MaybeLists<T>>::RowsMapped<D::RowsMapped<YY>> as Lists<YY>>::CoercedMatrix: Matrix<YY>
{
    type OutputI = D::RowsMapped<YY>;
    type Output = <<D::Transpose as MaybeLists<T>>::RowsMapped<Self::OutputI> as Lists<YY>>::CoercedMatrix;

    fn impulse_s<TT, W>(self, t: TT, numtaps: <L::Length as StaticMaybe<usize>>::Opposite, w: W)
        -> (L, Self::Output)
    where
        TT: TwoSidedRange<T::Real> + Clone,
        W: Maybe<Vec<T>>
    {
        let w = w.into_option();
        
        let b: Array2<T> = self.b.to_array2();

        let n = numtaps.into_option()
            .unwrap_or(L::LENGTH);
        let nuf = <T::Real as NumCast>::from(n).unwrap();
        let numaybem1 = if t.is_end_inclusive()
        {
            nuf - T::Real::one()
        }
        else
        {
            nuf
        };

        let dt = (*t.end() - *t.start())/numaybem1;

        let tt = OwnedListOrSingle::from_len_fn(StaticMaybe::maybe_from_fn(|| n), |i| {
            let i = <T::Real as NumCast>::from(i).unwrap();
            *t.start() + i*dt
        });

        let dn = self.d.matrix_dim().1.max(self.b.matrix_dim().1);
        let y = <D::Transpose as MaybeLists<T>>::RowsMapped::<Self::OutputI>::from_len_fn(StaticMaybe::maybe_from_fn(|| dn), |i| {
            let mut b = b.column(i)
                .to_vec();
            for b in b.iter_mut()
            {
                *b *= dt
            }
    
            let w = match w.clone()
            {
                Some(w) => {
                    let wn = w.len();
                    w.into_iter()
                        .chain(core::iter::repeat(T::zero())
                            .take(b.len().saturating_sub(wn))
                        ).zip(b)
                        .map(|(w, b)| w + b)
                        .collect()
                },
                None => b
            };
    
            let (_, y, _) = self.as_view()
                .sim_s(
                    t.clone(),
                    Array2::from_elem((dn, n), T::zero()),
                    w,
                    false
                );
    
            let y: Self::OutputI = y.map_into_owned(|y| {
                    let ny = y.len();
                    let mut y = y.into_iter();
                    OwnedListOrSingle::from_len_fn(StaticMaybe::maybe_from_fn(|| ny), |_| y.next().unwrap())
                }).into();
            y
        })
        .coerce_into_matrix(|| panic!("Matrix is not correctly shaped"));

        (tt, y)
    }
}

impl<T, B, A, L> ImpulseS<L> for Tf<T, B, A>
where
    T: ComplexFloat,
    B: MaybeLists<T>,
    A: MaybeList<T>,
    L: List<T::Real>,
    <L::Length as StaticMaybe<usize>>::Opposite: Sized,
    B::RowsMapped<L::Mapped<T>>: Lists<T> + OwnedListOrSingle<L::Mapped<T>, Length: StaticMaybe<usize, Opposite: Sized>>,
    Self: System<Set = T> + ToSs<T, Array2<T>, Array2<T>, Array2<T>, Array2<T>>,
    Array2<T>: SsAMatrix<T, Array2<T>, Array2<T>, Array2<T>> + SsBMatrix<T, Array2<T>, Array2<T>, Array2<T>> + SsCMatrix<T, Array2<T>, Array2<T>, Array2<T>> + SsDMatrix<T, Array2<T>, Array2<T>, Array2<T>>,
    Ss<T, Array2<T>, Array2<T>, Array2<T>, Array2<T>>: ImpulseS<L, Output = Array2<L::Mapped<T>>> + System<Set = T>
{
    type OutputI = B::RowsMapped<L::Mapped<T>>;
    type Output = B::RowsMapped<L::Mapped<T>>;

    fn impulse_s<TT, W>(self, t: TT, numtaps: <L::Length as StaticMaybe<usize>>::Opposite, w: W)
        -> (L, Self::Output)
    where
        TT: TwoSidedRange<T::Real> + Clone,
        W: Maybe<Vec<T>>
    {
        let (t, y) = self.to_ss()
            .impulse_s(t, numtaps, w);

        let q = y.len();
        let mut y = y.into_iter();

        (
            t,
            OwnedListOrSingle::from_len_fn(StaticMaybe::maybe_from_fn(|| q), |_| y.next().unwrap())
        )
    }
}

impl<T, Z, P, K, L> ImpulseS<L> for Zpk<T, Z, P, K>
where
    T: ComplexFloat,
    Z: MaybeList<T>,
    P: MaybeList<T>,
    K: ComplexFloat<Real = T::Real>,
    L: List<T::Real>,
    <L::Length as StaticMaybe<usize>>::Opposite: Sized,
    L::Mapped<K>: List<K> + ListOrSingle<L::Mapped<K>>,
    Self: System<Set = K> + ToSs<K, Array2<K>, Array2<K>, Array2<K>, Array2<K>>,
    Array2<K>: SsAMatrix<K, Array2<K>, Array2<K>, Array2<K>> + SsBMatrix<K, Array2<K>, Array2<K>, Array2<K>> + SsCMatrix<K, Array2<K>, Array2<K>, Array2<K>> + SsDMatrix<K, Array2<K>, Array2<K>, Array2<K>>,
    Ss<K, Array2<K>, Array2<K>, Array2<K>, Array2<K>>: System<Set = K> + ImpulseS<L, Output = Array2<L::Mapped<K>>>
{
    type OutputI = L::Mapped<K>;
    type Output = L::Mapped<K>;

    fn impulse_s<TT, W>(self, t: TT, numtaps: <L::Length as StaticMaybe<usize>>::Opposite, w: W)
        -> (L, Self::Output)
    where
        TT: TwoSidedRange<T::Real> + Clone,
        W: Maybe<Vec<K>>
    {
        let (t, y) = self.to_ss()
            .impulse_s(t, numtaps, w);

        (
            t,
            y.into_iter()
                .next()
                .unwrap()
        )
    }
}

impl<T, B, A, S, L> ImpulseS<L> for Sos<T, B, A, S>
where
    T: ComplexFloat,
    B: Maybe<[T; 3]> + MaybeOwnedList<T>,
    A: Maybe<[T; 3]> + MaybeOwnedList<T>,
    S: MaybeList<Tf<T, B, A>>,
    L: List<T::Real>,
    <L::Length as StaticMaybe<usize>>::Opposite: Sized,
    L::Mapped<T>: List<T> + ListOrSingle<L::Mapped<T>>,
    Self: System<Set = T> + ToSs<T, Array2<T>, Array2<T>, Array2<T>, Array2<T>>,
    Array2<T>: SsAMatrix<T, Array2<T>, Array2<T>, Array2<T>> + SsBMatrix<T, Array2<T>, Array2<T>, Array2<T>> + SsCMatrix<T, Array2<T>, Array2<T>, Array2<T>> + SsDMatrix<T, Array2<T>, Array2<T>, Array2<T>>,
    Ss<T, Array2<T>, Array2<T>, Array2<T>, Array2<T>>: System<Set = T> + ImpulseS<L, Output = Array2<L::Mapped<T>>>
{
    type OutputI = L::Mapped<T>;
    type Output = L::Mapped<T>;

    fn impulse_s<TT, W>(self, t: TT, numtaps: <L::Length as StaticMaybe<usize>>::Opposite, w: W)
        -> (L, Self::Output)
    where
        TT: TwoSidedRange<T::Real> + Clone,
        W: Maybe<Vec<T>>
    {
        let (t, y) = self.to_ss()
            .impulse_s(t, numtaps, w);

        (
            t,
            y.into_iter()
                .next()
                .unwrap()
        )
    }
}

#[cfg(test)]
mod test
{
    use core::f64::consts::TAU;

    use array_math::ArrayOps;

    use crate::{analysis::ImpulseS, gen::filter::{BesselF, FilterGenPlane, FilterGenType}, plot, systems::Tf};

    #[test]
    fn test()
    {
        let h = Tf::besself(5, [TAU*12.0], FilterGenType::LowPass, FilterGenPlane::S)
            .unwrap();

        const T: f64 = 1.25;
        const N: usize = 200;
        let (t, y): ([_; N], _) = h.impulse_s(0.0..T, (), ());

        plot::plot_curves("y(t)", "plots/y_t_impulse_s.png", [&t.zip(y)])
            .unwrap();
    }
}