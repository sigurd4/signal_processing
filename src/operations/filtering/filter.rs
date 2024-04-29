use num::complex::ComplexFloat;
use option_trait::Maybe;

use crate::{util::ComplexOp, operations::filtering::FilterMut, quantities::Lists, systems::Rtf, RtfOrSystem, System};

pub trait Filter<X, XX>: System
where
    Self::Domain: ComplexOp<X>,
    X: Into<<Self::Domain as ComplexOp<X>>::Output> + ComplexFloat<Real = <Self::Domain as ComplexFloat>::Real>,
    XX: Lists<X>
{
    type Output: Lists<<Self::Domain as ComplexOp<X>>::Output>;

    fn filter<W: Maybe<Vec<<Self::Domain as ComplexOp<X>>::Output>>>(self, x: XX, w: W) -> Self::Output;
}

impl<W, S, X, XX, O> Filter<X, XX> for S
where
    S: System,
    S::Domain: ComplexOp<X, Output = W>,
    X: Into<W> + ComplexFloat<Real = W::Real>,
    XX: Lists<X>,
    W: ComplexOp<X, Output = W> + ComplexFloat<Real = <S::Domain as ComplexFloat>::Real>,
    Rtf<W, S>: FilterMut<X, XX, Output = O> + RtfOrSystem<Domain = W>,
    O: Lists<W>
{
    type Output = O;

    fn filter<WW: Maybe<Vec<W>>>(self, x: XX, w: WW) -> Self::Output
    {
        Rtf::<W, S>::new(self, w)
            .filter_mut(x)
    }
}

#[cfg(test)]
mod test
{
    use array_math::ArrayOps;
    use linspace::LinspaceArray;
    use rand::distributions::uniform::SampleRange;

    use crate::{plot, gen::filter::{Butter, FilterGenPlane, FilterGenType}, operations::filtering::Filter, systems::Ss};

    #[test]
    fn test()
    {
        let h = Ss::butter(4, [0.2], FilterGenType::LowPass, FilterGenPlane::Z { sampling_frequency: None })
            .unwrap();

        const N: usize = 64;
        let mut rng = rand::thread_rng();
        let x: [f64; N] = ArrayOps::fill(|_| (-1.0..1.0).sample_single(&mut rng));

        let y = Filter::<f64, [_; _]>::filter(h, x, ());

        let t: [_; N] = (0.0..N as f64).linspace_array();

        plot::plot_curves("x(t), y(t)", "plots/xy_t_filter.png", [&t.zip(x), &t.zip(y[0])])
            .unwrap()
    }
}