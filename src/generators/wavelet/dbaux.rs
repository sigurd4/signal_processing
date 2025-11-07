use core::ops::{DivAssign, MulAssign};

use ndarray_linalg::Lapack;
use num::{traits::{float::TotalOrder, FloatConst}, Complex, Float, One};
use array_math::SliceMath;
use option_trait::Maybe;

moddef::moddef!(
    mod {
        lut
    }
);

pub fn dbaux<T, S>(order: usize, scale: S) -> Vec<T>
where
    T: Float + FloatConst + MulAssign + DivAssign + TotalOrder + Lapack<Complex = Complex<T>>,
    S: Maybe<T>
{
    let one = T::one();

    let scale = scale.into_option()
        .unwrap_or(one);

    let mut psi = if let Some(c) = lut::DBAUX_LUT.get(order)
    {
        c.iter()
            .map(|&c| T::from(c).unwrap())
            .collect()
    }
    else
    {
        let zero = T::zero();
        let two = one + one;
        let half = two.recip();
        let cone = Complex::one();

        let sup = (1 - order as isize)..order as isize;
        let a: Vec<_> = (1..=order).map(|n| {
            let nf = T::from(n).unwrap();
            let mut a = one;
            for non in sup.clone()
                .take(n + order - 1)
                .chain(sup.clone()
                    .skip(n + order - 1 + 1)
                )
            {
                let non = T::from(non).unwrap();
                a *= half - non;
                a /= nf - non
            }
            a
        }).collect();

        let p: Vec<_> = a.iter()
            .flat_map(|&a| core::iter::once(zero).chain(core::iter::once(a)))
            .skip(1)
            .collect();
        let p: Vec<_> = p.clone()
            .into_iter()
            .chain(core::iter::once(one))
            .chain(p.into_iter()
                .rev()
            ).collect();
        let mut r: Vec<_> = p.rpolynomial_roots();
        r.retain(|r| r.re > zero && r.norm_sqr() < one);
        r.sort_by(|a, b| b.norm_sqr().total_cmp(&a.norm_sqr()));
        let c: Vec<_> = core::iter::repeat(-cone)
            .take(order)
            .chain(r)
            .map(|r| vec![cone, -r])
            .reduce(|a, b| a.convolve_direct(&b))
            .map(|c| c.into_iter()
                .map(|c| c.re)
                .collect()
            ).unwrap_or_else(|| vec![one]);

        c
    };

    let s = psi.iter()
        .copied()
        .sum::<T>();
    if !s.is_zero()
    {
        for c in psi.iter_mut()
        {
            *c /= s;
            *c *= scale
        }
    }

    psi
}

#[cfg(test)]
mod test
{
    use linspace::Linspace;

    use crate::plot;

    #[test]
    fn test()
    {
        let phi = crate::generators::wavelet::dbaux(38, 1.0);

        plot::plot_curves("ϕ[n]", "plots/phi_n_dbaux.png", [
                &(0.0..phi.len() as f64).linspace(phi.len())
                    .into_iter()
                    .zip(phi)
                    .collect::<Vec<_>>()
            ]).unwrap()
    }
}