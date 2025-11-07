use core::{iter::Sum, ops::DivAssign};

use num::{complex::ComplexFloat, Zero};

use crate::{util::ComplexOp, MaybeSystem, quantities::{ContainerOrSingle, OwnedListOrSingle, ListOrSingle, List, Lists, MaybeList, OwnedList}, operations::{resampling::Downsample, filtering::FftFilt}, transforms::filter::Qmf, System, systems::Tf};

pub trait Dwt<T, W, WW, L, H>: Lists<T, RowOwned: List<T>>
where
    T: ComplexFloat + Into<<W as ComplexOp<T>>::Output>,
    W: ComplexFloat<Real = T::Real> + ComplexOp<T>,
    WW: MaybeList<W>,
    L: MaybeSystem<W>,
    H: MaybeSystem<W>,
    <Self::RowOwned as ContainerOrSingle<T>>::Mapped<<W as ComplexOp<T>>::Output>: List<<W as ComplexOp<T>>::Output>,
{
    fn dwt(
        self,
        wavelet: WW,
        low_pass: L,
        high_pass: H
    ) -> (
        Self::RowsMapped<<<Self::RowOwned as ContainerOrSingle<T>>::Mapped<<W as ComplexOp<T>>::Output> as List<<W as ComplexOp<T>>::Output>>::ResizedList<{<Self::RowOwned as ListOrSingle<T>>::LENGTH/2}>>,
        Self::RowsMapped<<<Self::RowOwned as ContainerOrSingle<T>>::Mapped<<W as ComplexOp<T>>::Output> as List<<W as ComplexOp<T>>::Output>>::ResizedList<{<Self::RowOwned as ListOrSingle<T>>::LENGTH/2}>>
    );
}

impl<T, W, WW, X> Dwt<T, W, WW, (), ()> for X
where
    X: Lists<T, RowOwned: List<T>> + Dwt<T, W, (), Tf<W, WW::Owned, ()>, ()>,
    T: ComplexFloat + Into<<W as ComplexOp<T>>::Output>,
    W: ComplexFloat<Real = T::Real> + ComplexOp<T> + DivAssign<T::Real>,
    WW: List<W, Owned: OwnedList<W>>,
    <X::RowOwned as ContainerOrSingle<T>>::Mapped<<W as ComplexOp<T>>::Output>: List<<W as ComplexOp<T>>::Output>,
    T::Real: Sum
{
    fn dwt(
        self,
        wavelet: WW,
        (): (),
        (): ()
    ) -> (
        Self::RowsMapped<<<Self::RowOwned as ContainerOrSingle<T>>::Mapped<<W as ComplexOp<T>>::Output> as List<<W as ComplexOp<T>>::Output>>::ResizedList<{<Self::RowOwned as ListOrSingle<T>>::LENGTH/2}>>,
        Self::RowsMapped<<<Self::RowOwned as ContainerOrSingle<T>>::Mapped<<W as ComplexOp<T>>::Output> as List<<W as ComplexOp<T>>::Output>>::ResizedList<{<Self::RowOwned as ListOrSingle<T>>::LENGTH/2}>>
    )
    {
        let mut w = wavelet.to_owned();
        w.as_mut_slice()
            .reverse();
        let wsqrsum = w.as_view_slice()
            .iter()
            .map(|&w| (w.conj()*w).re())
            .sum::<T::Real>();
        if !wsqrsum.is_zero()
        {
            for w in w.as_mut_slice()
                .iter_mut()
            {
                *w /= wsqrsum
            }
        }
        let low_pass = Tf::new(w, ());

        self.dwt((), low_pass, ())
    }
}

impl<T, W, L, X> Dwt<T, W, (), L, ()> for X
where
    X: Lists<T, RowOwned: List<T>> + Dwt<T, W, (), L, <L as Qmf>::Output>,
    T: ComplexFloat + Into<<W as ComplexOp<T>>::Output>,
    W: ComplexFloat<Real = T::Real> + ComplexOp<T>,
    L: System<Set = W> + Clone + Qmf,
    <X::RowOwned as ContainerOrSingle<T>>::Mapped<<W as ComplexOp<T>>::Output>: List<<W as ComplexOp<T>>::Output>,
{
    fn dwt(
        self,
        (): (),
        low_pass: L,
        (): ()
    ) -> (
        Self::RowsMapped<<<Self::RowOwned as ContainerOrSingle<T>>::Mapped<<W as ComplexOp<T>>::Output> as List<<W as ComplexOp<T>>::Output>>::ResizedList<{<Self::RowOwned as ListOrSingle<T>>::LENGTH/2}>>,
        Self::RowsMapped<<<Self::RowOwned as ContainerOrSingle<T>>::Mapped<<W as ComplexOp<T>>::Output> as List<<W as ComplexOp<T>>::Output>>::ResizedList<{<Self::RowOwned as ListOrSingle<T>>::LENGTH/2}>>
    )
    {
        let high_pass = low_pass.clone()
            .qmf();

        self.dwt((), low_pass, high_pass)
    }
}

impl<T, W, Y, L, H, X, YY> Dwt<T, W, (), L, H> for X
where
    T: ComplexFloat + Into<Y>,
    W: ComplexFloat<Real = T::Real> + ComplexOp<T, Output = Y>,
    L: System<Set = W>,
    H: System<Set = W>,
    X: Lists<T, RowOwned: List<T, Mapped<Y> = YY> + List<T, Mapped<()>: List<(), ResizedList<{<Self::RowOwned as ListOrSingle<T>>::LENGTH/2}>: List<(), Mapped<Y>: Into<<<Self::RowOwned as ContainerOrSingle<T>>::Mapped<Y> as List<Y>>::ResizedList<{<Self::RowOwned as ListOrSingle<T>>::LENGTH/2}>>>>>> + Clone,
    YY: List<Y, RowsMapped<Vec<Y>> = Vec<Y>> + Downsample<Y, usize, Vec<Y>>,
    L: for<'a> FftFilt<'a, T, X::RowOwned, Output = YY>,
    H: for<'a> FftFilt<'a, T, X::RowOwned, Output = YY>
{
    fn dwt(
        self,
        (): (),
        low_pass: L,
        high_pass: H
    ) -> (
        Self::RowsMapped<<<Self::RowOwned as ContainerOrSingle<T>>::Mapped<Y> as List<Y>>::ResizedList<{<Self::RowOwned as ListOrSingle<T>>::LENGTH/2}>>,
        Self::RowsMapped<<<Self::RowOwned as ContainerOrSingle<T>>::Mapped<Y> as List<Y>>::ResizedList<{<Self::RowOwned as ListOrSingle<T>>::LENGTH/2}>>
    )
    {
        (
            self.clone().map_rows_into_owned(|x| {
                let x_void = x.map_to_owned(|_| ())
                    .static_resize_list::<{<Self::RowOwned as ListOrSingle<T>>::LENGTH/2}>(x.length()/2, || ());
                let y = low_pass.fftfilt(x, ());
                let mut y = y.downsample(2, 0)
                    .into_iter();

                x_void.map_into_owned(|()| y.next().unwrap())
                    .into()
            }),
            self.map_rows_into_owned(|x| {
                let x_void = x.map_to_owned(|_| ())
                    .static_resize_list::<{<Self::RowOwned as ListOrSingle<T>>::LENGTH/2}>(x.length()/2, || ());
                let y = high_pass.fftfilt(x, ());
                let mut y = y.downsample(2, 0)
                    .into_iter();

                x_void.map_into_owned(|()| y.next().unwrap())
                    .into()
            })
        )
    }
}