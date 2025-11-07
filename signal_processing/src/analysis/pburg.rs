use num::complex::ComplexFloat;
use option_trait::Maybe;

use crate::{systems::Ar, identification::ar::{ArBurg, ArBurgCriterion}, quantities::{List, Lists}, analysis::{Psd, PsdMethod, PsdRange}, System};


pub trait PBurg<X, O, P, F, N, FF, R, M, C>: Lists<X>
where
    X: ComplexFloat,
    P: Lists<X::Real>,
    F: List<X::Real>,
    O: Maybe<usize>,
    N: Maybe<usize>,
    FF: Maybe<F>,
    M: Maybe<PsdMethod>,
    R: Maybe<PsdRange>,
    C: Maybe<ArBurgCriterion>,
{
    fn pburg<FS>(self, order: O, numtaps: N, frequencies: FF, sampling_frequency: FS, range: R, method: M, criterion: C) -> (P, F)
    where
        FS: Maybe<X::Real>;
}

impl<X, XX, O, P, F, N, FF, R, M, C> PBurg<X, O, P, F, N, FF, R, M, C> for XX
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
    C: Maybe<ArBurgCriterion>,
    Ar<X, Vec<X>, XX::RowsMapped<(Vec<X>, X::Real)>>: ArBurg<XX, O, C, XX::RowsMapped<Vec<X>>> + for<'a> Psd<'a, P, F, N, FF, R, M> + System<Set = X>,
    XX::RowsMapped<Vec<X>>: Lists<X>
{
    fn pburg<FS>(self, order: O, numtaps: N, frequencies: FF, sampling_frequency: FS, range: R, method: M, criterion: C) -> (P, F)
    where
        FS: Maybe<X::Real>
    {
        let (ar, _) = Ar::arburg(self, order, criterion);
        ar.psd(numtaps, frequencies, sampling_frequency, range, method)
    }
}

#[cfg(test)]
mod test
{
    use array_math::ArrayOps;

    use crate::{plot, analysis::PBurg};

    #[test]
    fn test()
    {
        let x = [1.0, 2.0, 3.0, 4.0, 5.0];

        const N: usize = 1024;
        let (psd, f): ([_; N], _) = x.pburg(2, (), (), (), (), (), ());

        plot::plot_curves("P(e^jw)", "plots/p_z_pburg.png", [&f.zip(psd)])
            .unwrap();
    }
}