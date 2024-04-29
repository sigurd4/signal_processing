use core::{iter::Sum, ops::{DivAssign, MulAssign}};

use num::Float;
use option_trait::Maybe;

pub fn coifaux<T, S>(order: usize, scale: S) -> Option<Vec<T>>
where
    T: Float + Sum + MulAssign + DivAssign,
    S: Maybe<T>
{
    if let Some(mut h) = crate::gen::wavelet::coifwavf::<T>(order)
    {
        let one = T::one();
    
        let scale = scale.into_option()
            .unwrap_or(one);

        h.reverse();
        for h in h.iter_mut()
            .skip(1)
            .step_by(2)
        {
            *h = -*h
        }
        
        let s = h.iter()
            .copied()
            .sum::<T>();
        if !s.is_zero()
        {
            for c in h.iter_mut()
            {
                *c /= s;
                *c *= scale
            }
        }

        return Some(h)
    }
    None
}

#[cfg(test)]
mod test
{
    use linspace::Linspace;

    use crate::plot;

    #[test]
    fn test()
    {
        let phi = crate::gen::wavelet::coifaux(17, ())
            .unwrap();

        plot::plot_curves("ϕ[n]", "plots/phi_n_coifaux.png", [
                &(0.0..phi.len() as f64).linspace(phi.len())
                    .into_iter()
                    .zip(phi)
                    .collect::<Vec<_>>()
            ]).unwrap()
    }
}