use core::ops::{Mul, Range, RangeInclusive};

use array_math::ArrayOps;
use num::{traits::FloatConst, Float, Zero};
use option_trait::{Maybe, StaticMaybe};

use crate::{List, ListOrSingle, NotRange};

pub trait PulseTrain<T, L, N>
where
    T: Float,
    L: List<T>,
    N: Maybe<usize>
{
    fn pulse_train<D, G, GG, P, R, O>(self, numtaps: N, train: D, pulse: P, fold: R) -> (D::Mapped<L::Mapped<O>>, L::Mapped<O>, L)
    where
        D: List<(T, GG)>,
        GG: StaticMaybe<G, MaybeOr<G, T> = G> + Clone,
        G: Clone,
        D::Mapped<L::Mapped<O>>: List<L::Mapped<O>>,
        L::Mapped<O>: List<O>,
        P: FnMut<(T,)>,
        R: Fn(O, O) -> O,
        O: Zero + Clone,
        P::Output: Clone + Mul<G, Output = O>;
}

impl<T, L> PulseTrain<T, L, ()> for L
where
    T: Float,
    L: List<T>,
{
    fn pulse_train<D, G, GG, P, R, O>(self, (): (), train: D, mut pulse: P, fold: R) -> (D::Mapped<L::Mapped<O>>, L::Mapped<O>, L)
    where
        D: List<(T, GG)>,
        GG: StaticMaybe<G, MaybeOr<G, T> = G> + Clone,
        G: Clone,
        D::Mapped<L::Mapped<O>>: List<L::Mapped<O>>,
        L::Mapped<O>: List<O>,
        P: FnMut<(T,)>,
        R: Fn(O, O) -> O,
        O: Zero + Clone,
        P::Output: Clone + Mul<G, Output = O>
    {
        let s = train.map_into_owned(|(d, g)| {
            let g: G = g.into_option()
                .unwrap_or_else(|| unsafe {core::mem::transmute_copy(&T::one())});
            self.map_to_owned(|&t| {
                pulse(t - d)*g.clone()
            })
        });

        let ss = s.as_view_slice();

        let mut i = 0;
        let y = self.map_to_owned(|_| {
            let y = ss.iter()
                .map(|s| s.as_view_slice()[i].clone())
                .reduce(&fold)
                .unwrap_or_else(O::zero);
            i += 1;
            y
        });

        (s, y, self)
    }
}

impl<T, const N: usize> PulseTrain<T, [T; N], ()> for Range<T>
where
    T: Float + FloatConst,
    [T; N]: NotRange
{
    fn pulse_train<D, G, GG, P, R, O>(self, (): (), train: D, pulse: P, fold: R) -> (D::Mapped<[O; N]>, [O; N], [T; N])
    where
        D: List<(T, GG)>,
        GG: StaticMaybe<G, MaybeOr<G, T> = G> + Clone,
        G: Clone,
        D::Mapped<[O; N]>: List<[O; N]>,
        P: FnMut<(T,)>,
        R: Fn(O, O) -> O,
        O: Zero + Clone,
        P::Output: Clone + Mul<G, Output = O>
    {
        let x: [_; N] = ArrayOps::fill(|i| {
            let p = T::from(i).unwrap()/T::from(N).unwrap();
            self.start + (self.end - self.start)*p
        });
        
        x.pulse_train((), train, pulse, fold)
    }
}

impl<T> PulseTrain<T, Vec<T>, usize> for Range<T>
where
    T: Float + FloatConst,
    Vec<T>: NotRange
{
    fn pulse_train<D, G, GG, P, R, O>(self, n: usize, train: D, pulse: P, fold: R) -> (D::Mapped<Vec<O>>, Vec<O>, Vec<T>)
    where
        D: List<(T, GG)>,
        GG: StaticMaybe<G, MaybeOr<G, T> = G> + Clone,
        G: Clone,
        D::Mapped<Vec<O>>: List<Vec<O>>,
        P: FnMut<(T,)>,
        R: Fn(O, O) -> O,
        O: Zero + Clone,
        P::Output: Clone + Mul<G, Output = O>
    {
        let x: Vec<_> = (0..n).map(|i| {
                let p = T::from(i).unwrap()/T::from(n).unwrap();
                self.start + (self.end - self.start)*p
            }).collect();

        x.pulse_train((), train, pulse, fold)
    }
}

impl<T, const N: usize> PulseTrain<T, [T; N], ()> for RangeInclusive<T>
where
    T: Float + FloatConst,
    [T; N]: NotRange
{
    fn pulse_train<D, G, GG, P, R, O>(self, (): (), train: D, pulse: P, fold: R) -> (D::Mapped<[O; N]>, [O; N], [T; N])
    where
        D: List<(T, GG)>,
        GG: StaticMaybe<G, MaybeOr<G, T> = G> + Clone,
        G: Clone,
        D::Mapped<[O; N]>: List<[O; N]>,
        P: FnMut<(T,)>,
        R: Fn(O, O) -> O,
        O: Zero + Clone,
        P::Output: Clone + Mul<G, Output = O>
    {
        let x: [_; N] = ArrayOps::fill(|i| {
            let p = T::from(i).unwrap()/T::from(N - 1).unwrap();
            *self.start() + (*self.end() - *self.start())*p
        });
        
        x.pulse_train((), train, pulse, fold)
    }
}

impl<T> PulseTrain<T, Vec<T>, usize> for RangeInclusive<T>
where
    T: Float + FloatConst,
    Vec<T>: NotRange
{
    fn pulse_train<D, G, GG, P, R, O>(self, n: usize, train: D, pulse: P, fold: R) -> (D::Mapped<Vec<O>>, Vec<O>, Vec<T>)
    where
        D: List<(T, GG)>,
        GG: StaticMaybe<G, MaybeOr<G, T> = G> + Clone,
        G: Clone,
        D::Mapped<Vec<O>>: List<Vec<O>>,
        P: FnMut<(T,)>,
        R: Fn(O, O) -> O,
        O: Zero + Clone,
        P::Output: Clone + Mul<G, Output = O>
    {
        let x: Vec<_> = (0..n).map(|i| {
                let p = T::from(i).unwrap()/T::from(n - 1).unwrap();
                *self.start() + (*self.end() - *self.start())*p
            }).collect();

        x.pulse_train((), train, pulse, fold)
    }
}

#[cfg(test)]
mod test
{
    use core::ops::Add;

    use array_math::ArrayOps;

    use crate::{plot, GausPuls, PulseTrain};

    #[test]
    fn test()
    {
        const N: usize = 1024;
        let (_s, y, t): (_, [_; N], _) = (0.0..=8.0).pulse_train((), [
            (1.0, 1.0),
            (3.0, 0.66),
            (6.0, 0.33)
        ], |x| [x].gauspuls((), 5.0, 0.5).0[0], Add::add);

        plot::plot_curves("y(t)", "plots/y_t_pulse_train.png", [&t.zip(y)])
            .unwrap()
    }
}