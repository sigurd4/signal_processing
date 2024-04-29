use num::Float;

moddef::moddef!(
    mod {
        lut
    }
);

pub fn coifwavf<T>(k: usize) -> Option<Vec<T>>
where
    T: Float
{
    if k == 0
    {
        return None
    }
    if let Some(h) = lut::COIF_LUT.get(k - 1)
    {
        let h = h.iter()
            .map(|&h| T::from(h).unwrap())
            .collect();
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
        let psi = crate::gen::wavelet::coifwavf(17)
            .unwrap();

        plot::plot_curves("ψ[n]", "plots/psi_n_coifwavf.png", [
                &(0.0..psi.len() as f64).linspace(psi.len())
                    .into_iter()
                    .zip(psi)
                    .collect::<Vec<_>>()
            ]).unwrap()
    }
}