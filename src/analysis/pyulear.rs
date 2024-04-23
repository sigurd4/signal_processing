use num::complex::ComplexFloat;
use option_trait::Maybe;

use crate::{Ar, ArYule, List, Lists, Psd, PsdMethod, PsdRange, System};

pub trait PYuleAr<X, P, F, O, N, FF, R, M>: Lists<X>
where
    X: ComplexFloat,
    P: Lists<X::Real>,
    F: List<X::Real>,
    O: Maybe<usize>,
    N: Maybe<usize>,
    FF: Maybe<F>,
    M: Maybe<PsdMethod>,
    R: Maybe<PsdRange>
{
    fn pyulear<FS>(self, order: O, numtaps: N, frequencies: FF, sampling_frequency: FS, range: R, method: M) -> (P, F)
    where
        FS: Maybe<X::Real>;
}

impl<X, XX, P, F, O, N, FF, R, M> PYuleAr<X, P, F, O, N, FF, R, M> for XX
where
    X: ComplexFloat,
    XX: Lists<X>,
    P: Lists<X::Real>,
    F: List<X::Real>,
    O: Maybe<usize>,
    N: Maybe<usize>,
    FF: Maybe<F>,
    M: Maybe<PsdMethod>,
    R: Maybe<PsdRange>,
    Ar<X, Vec<X>, XX::RowsMapped<(Vec<X>, X::Real)>>: ArYule<XX, O, XX::RowsMapped<Vec<X>>> + for<'a> Psd<'a, P, F, N, FF, R, M> + System<Domain = X>,
    XX::RowsMapped<Vec<X>>: Lists<X>
{
    fn pyulear<FS>(self, order: O, numtaps: N, frequencies: FF, sampling_frequency: FS, range: R, method: M) -> (P, F)
    where
        FS: Maybe<<X as ComplexFloat>::Real>
    {
        let (ar, _) = Ar::aryule(self, order);
        ar.psd(numtaps, frequencies, sampling_frequency, range, method)
    }
}

#[cfg(test)]
mod test
{
    use array_math::ArrayOps;

    use crate::{plot, PYuleAr};

    #[test]
    fn test()
    {
        let x = [1.0, 2.0, 3.0, 4.0, 5.0];

        const N: usize = 1024;
        let (psd, f): ([_; N], _) = x.pyulear(2, (), (), (), (), ());

        plot::plot_curves("P(e^jw)", "plots/p_z_pyulear.png", [&f.zip(psd)])
            .unwrap();
    }
}