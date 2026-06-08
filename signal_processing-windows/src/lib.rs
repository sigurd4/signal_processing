#![feature(impl_trait_in_assoc_type)]
#![feature(const_trait_impl)]
#![feature(const_destruct)]
#![feature(const_precise_live_drops)]
#![feature(unboxed_closures)]
#![feature(const_ops)]
#![feature(fn_traits)]
#![feature(const_try)]
#![feature(const_clone)]
#![feature(derive_const)]
#![feature(try_trait_v2)]
#![feature(generic_const_exprs)]

use core::{marker::{Destruct, PhantomData}, ops::{Mul, Try}};

use array_trait::length::{self, Length, LengthValue};
use bulks::{Bulk, DoubleEndedBulk, SplitBulk};

moddef::moddef!(
    pub mod {
        windows
    },
    pub(crate) mod {
        util
    }
);

const fn identity<T>(x: T) -> T
{
    x
}

#[derive(Clone, Copy, Debug)]
pub enum Shape
{
    Symmetric,
    Periodic
}
impl Shape
{
    const fn window_len<L>(self, len: L) -> usize
    where
        L: LengthValue
    {
        let len = length::value::len(len);
        match self
        {
            Shape::Symmetric => len - 1,
            Shape::Periodic => len,
        }
    }
}

pub use Shape::*;

pub const trait WindowFn<L>: Copy
where
    L: Length + ?Sized
{
    type Functor: Fn(usize) -> L::Elem;

    fn window_fn(self, len: L::Value, range: Shape) -> Self::Functor;
}

pub const trait Window<W>: Bulk
{
    fn window(self, window: W, range: Shape) -> Windowed<Self, W>
    where
        W: WindowFn<<Self::Length as Length>::Mapped<Self::Item>>,
        Self::Item: Mul,
        Self: Sized
    {
        self.window_as(window, range)
    }

    fn window_as<T>(self, window: W, range: Shape) -> Windowed<Self, W, T>
    where
        W: WindowFn<<Self::Length as Length>::Mapped<T>>,
        Self::Item: Mul<T>,
        Self: Sized;
}

impl<B, W> const Window<W> for B
where
    B: Bulk + ?Sized
{
    fn window_as<T>(self, window: W, range: Shape) -> Windowed<Self, W, T>
    where
        W: WindowFn<<Self::Length as Length>::Mapped<T>>,
        Self::Item: Mul<T>,
        Self: Sized
    {
        Windowed::new(self, window, range)
    }
}

pub struct Windowed<I, W, T = <I as IntoIterator>::Item>
where
    I: Bulk<Item: Mul<T>>,
    W: WindowFn<<I::Length as Length>::Mapped<T>>,
{
    bulk: I,
    window: W,
    range: Shape,
    marker: PhantomData<T>
}
impl<I, W, T> Windowed<I, W, T>
where
    I: Bulk<Item: Mul<T>>,
    W: WindowFn<<I::Length as Length>::Mapped<T>>,
{
    pub const fn new(bulk: I, window: W, range: Shape) -> Self
    {
        Self {
            bulk,
            window,
            range,
            marker: PhantomData
        }
    }

    const fn functor<F>(self, f: F) -> (I, Functor<W::Functor, F>)
    where
        I: ~const Bulk<Item: Mul<T>>,
        W: ~const WindowFn<<I::Length as Length>::Mapped<T>>,
    {
        let Self { bulk, window, range, marker: PhantomData } = self;
        let functor = Functor {
            window_function: window.window_fn(bulk.length(), range),
            f
        };
        (bulk, functor)
    }
}
impl<I, W, T, U> IntoIterator for Windowed<I, W, T>
where
    I: Bulk<Item: Mul<T, Output = U>>,
    W: WindowFn<<I::Length as Length>::Mapped<T>>,
{
    type Item = U;
    type IntoIter = core::iter::Map<core::iter::Enumerate<I::IntoIter>, impl Fn((usize, I::Item)) -> U>;

    fn into_iter(self) -> Self::IntoIter
    {
        let (bulk, functor) = self.functor(identity);
        bulk.into_iter()
            .enumerate()
            .map(functor)
    }
}
impl<I, W, T, U> const Bulk for Windowed<I, W, T>
where
    I: ~const Bulk<Item: ~const Mul<T, Output = U> + ~const Destruct>,
    W: ~const WindowFn<<I::Length as Length>::Mapped<T>, Functor: ~const FnMut(usize) -> T + ~const Destruct>
{
    type MinLength = I::MinLength;
    type MaxLength = I::MaxLength;

    fn len(&self) -> usize
    {
        self.bulk.len()
    }
    fn is_empty(&self) -> bool
    {
        self.bulk.is_empty()
    }
    fn for_each<F>(self, f: F)
    where
        Self: Sized,
        F: ~const FnMut(Self::Item) + ~const Destruct
    {
        let (bulk, functor) = self.functor(f);
        bulk.enumerate()
            .for_each(functor);
    }
    fn try_for_each<F, R>(self, f: F) -> R
    where
        Self: Sized,
        Self::Item: ~const Destruct,
        F: ~const FnMut(Self::Item) -> R + ~const Destruct,
        R: ~const Try<Output = (), Residual: ~const Destruct>
    {
        let (bulk, functor) = self.functor(f);
        bulk.enumerate()
            .try_for_each(functor)
    }
}
impl<I, W, T, U> const DoubleEndedBulk for Windowed<I, W, T>
where
    I: ~const Bulk<Item: ~const Mul<T, Output = U> + ~const Destruct> + ~const DoubleEndedBulk,
    W: ~const WindowFn<<I::Length as Length>::Mapped<T>, Functor: ~const FnMut(usize) -> T + ~const Destruct>
{
    fn rev_for_each<F>(self, f: F)
    where
        Self: Sized,
        F: ~const FnMut(Self::Item) + ~const Destruct
    {
        let (bulk, functor) = self.functor(f);
        bulk.enumerate()
            .rev_for_each(functor);
    }
    fn try_rev_for_each<F, R>(self, f: F) -> R
    where
        Self: Sized,
        Self::Item: ~const Destruct,
        F: ~const FnMut(Self::Item) -> R + ~const Destruct,
        R: ~const Try<Output = (), Residual: ~const Destruct>
    {
        let (bulk, functor) = self.functor(f);
        bulk.enumerate()
            .try_rev_for_each(functor)
    }
}
impl<I, W, T, U, L> const SplitBulk<L> for Windowed<I, W, T>
where
    I: ~const Bulk<Item: ~const Mul<T, Output = U> + ~const Destruct>,
    bulks::Enumerate<I>: ~const SplitBulk<L, Item = (usize, I::Item), Left: ~const Bulk, Right: ~const Bulk>,
    W: ~const WindowFn<<I::Length as Length>::Mapped<T>, Functor: ~const FnMut(usize) -> T + ~const Clone + ~const Destruct>,
    L: length::LengthValue
{
    type Left = bulks::Map<<bulks::Enumerate<I> as SplitBulk<L>>::Left, impl ~const Fn((usize, I::Item)) -> U>;
    type Right = bulks::Map<<bulks::Enumerate<I> as SplitBulk<L>>::Right, impl ~const Fn((usize, I::Item)) -> U>;

    fn split_at(bulk: Self, n: L) -> (Self::Left, Self::Right)
    where
        Self: Sized
    {
        let (bulk, functor) = bulk.functor(identity);
        let (left, right) = bulk.enumerate().split_at(n);
        (
            left.map(functor.clone()),
            right.map(functor)
        )
    }
}

#[derive(Copy)]
#[derive_const(Clone)]
struct Functor<W, F>
{
    window_function: W,
    f: F
}
impl<W, F, T, X, U, Y> const FnOnce<((usize, X),)> for Functor<W, F>
where
    W: ~const FnOnce(usize) -> T,
    X: ~const Mul<T, Output = U>,
    F: ~const FnOnce(U) -> Y
{
    type Output = Y;

    extern "rust-call" fn call_once(self, ((i, x),): ((usize, X),)) -> Self::Output
    {
        let Self { window_function, f } = self;
        f(x*window_function(i))
    }
}
impl<W, F, T, X, U, Y> const FnMut<((usize, X),)> for Functor<W, F>
where
    W: ~const FnMut(usize) -> T,
    X: ~const Mul<T, Output = U>,
    F: ~const FnMut(U) -> Y
{
    extern "rust-call" fn call_mut(&mut self, ((i, x),): ((usize, X),)) -> Self::Output
    {
        let Self { window_function, f } = self;
        f(x*window_function(i))
    }
}
impl<W, F, T, X, U, Y> const Fn<((usize, X),)> for Functor<W, F>
where
    W: ~const Fn(usize) -> T,
    X: ~const Mul<T, Output = U>,
    F: ~const Fn(U) -> Y
{
    extern "rust-call" fn call(&self, ((i, x),): ((usize, X),)) -> Self::Output
    {
        let Self { window_function, f } = self;
        f(x*window_function(i))
    }
}

#[cfg(test)]
mod tests
{
    use core::f64::consts::PI;

    use bulks::*;
    use linspace::Linspace;
    use signal_processing_fourier::Dft;
use to_snake_case::ToSnakeCase;

    use crate::{Shape, Window, WindowFn};

    const N: usize = 1024;

    pub fn plot_window<W>(w: W)
    where
        W: WindowFn<[f64]> + WindowFn<[f64; N/2]>
    {
        let type_name = core::any::type_name::<W>().split("::").last().unwrap().split("<").next().unwrap().to_snake_case();

        let w = bulks::repeat_n(1.0, [(); N/2]).window(w, Shape::Symmetric)
            .collect_array();

        let data = (0.0..1.0).linspace(w.len()).zip(w);

        ezplot::plot_curves("g(n/N)", &format!("plots/windows/g_n_{type_name}.png"), [data]).unwrap();

        let mut w_f = w.into_bulk().chain([0.0; N/2]).dft().collect_array();
        w_f.rotate_right(N/4);
        let data = (-PI..PI).linspace(w.len()).zip(w_f.into_bulk().map(|w| 20.0*w.norm().log10()));
        
        ezplot::plot_curves("G(e^jw)", &format!("plots/windows/g_f_{type_name}.png"), [data]).unwrap();
    }
}