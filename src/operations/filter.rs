use num::complex::ComplexFloat;
use option_trait::Maybe;

use crate::{ComplexOp, FilterMut, ListOrSingle, Lists, Rtf, RtfOrSystem, System};

pub trait Filter<'a, X, XX>: System
where
    Self::Domain: ComplexOp<X>,
    X: Into<<Self::Domain as ComplexOp<X>>::Output> + ComplexFloat<Real = <Self::Domain as ComplexFloat>::Real>,
    XX: Lists<X>
{
    type Output: ListOrSingle<XX::Mapped<<Self::Domain as ComplexOp<X>>::Output>>;

    fn filter<W: Maybe<Vec<<Self::Domain as ComplexOp<X>>::Output>>>(&'a self, x: XX, w: W) -> Self::Output;
}

impl<'a, W, S, X, XX, O> Filter<'a, X, XX> for S
where
    S: System,
    S::Domain: ComplexOp<X, Output = W>,
    X: Into<W> + ComplexFloat<Real = W::Real>,
    XX: Lists<X>,
    W: ComplexOp<X, Output = W> + ComplexFloat<Real = <S::Domain as ComplexFloat>::Real> + 'a,
    S: 'a,
    Rtf<'a, W, S>: FilterMut<X, XX, Output = O> + RtfOrSystem<Domain = W>,
    O: ListOrSingle<XX::Mapped<<W as ComplexOp<X>>::Output>>
{
    type Output = O;

    fn filter<WW: Maybe<Vec<W>>>(&'a self, x: XX, w: WW) -> Self::Output
    {
        Rtf::new(self, w)
            .filter_mut(x)
    }
}

#[cfg(test)]
mod test
{
    use array_math::ArrayOps;
    use linspace::LinspaceArray;
    use rand::distributions::uniform::SampleRange;

    use crate::{plot, Butter, Filter, FilterGenPlane, FilterGenType, Tf};

    #[test]
    fn test()
    {
        let h = Tf::butter(10, [220.0], FilterGenType::LowPass, FilterGenPlane::Z { sampling_frequency: Some(1000.0) })
            .unwrap();

        const N: usize = 64;
        let mut rng = rand::thread_rng();
        let x: [_; N] = ArrayOps::fill(|_| (-1.0..1.0).sample_single(&mut rng));

        let y = h.filter(x, ());

        let t: [_; N] = (0.0..N as f64).linspace_array();

        plot::plot_curves("h(t)", "plots/h_t_filter.png", [&t.zip(x), &t.zip(y)])
            .unwrap()
    }
}