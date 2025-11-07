#![feature(trait_alias)]
#![feature(unsize)]
#![feature(unboxed_closures)]
#![feature(associated_type_defaults)]
#![feature(auto_traits)]
#![feature(negative_impls)]
#![feature(associated_const_equality)]
#![feature(split_array)]
#![feature(iterator_try_collect)]
#![feature(float_gamma)]
#![feature(fn_traits)]
#![feature(coerce_unsized)]
#![feature(decl_macro)]
#![feature(array_try_map)]
#![feature(impl_trait_in_assoc_type)]
#![feature(const_trait_impl)]

#![allow(internal_features)]
#![allow(incomplete_features)]

#![feature(adt_const_params)]
#![feature(core_intrinsics)]
#![feature(inherent_associated_types)]
#![feature(generic_const_exprs)]
#![feature(specialization)]

moddef::moddef!(
    flat(pub) mod {
        maybe_rtf_or_system,
        maybe_system,
        rtf_or_system,
        system,
        validate_filter_bands
    },
    pub mod {
        //analysis,
        //decompositions,
        generators,
        //identification,
        //operations,
        //quantities,
        //systems,
        //transforms,
        util,
        windows,
    },
    mod {
        plot for cfg(test)
    }
);

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum Plane
{
    S,
    Z
}

#[cfg(test)]
mod tests
{
    use linspace::Linspace;
    use num::Complex;

    use crate::{plot, generators::filter::{BesselAP, BesselF, ButtAP, Butter, Cheb1AP, Cheb2AP, EllipAP, FilterGenPlane, FilterGenType}, analysis::{FreqS, FreqZ, RealFreqZ}, systems::{Tf, Zpk}};

    #[test]
    fn testt()
    {
        let fs = 1e3f64;
        let f_p = 50.0;
        let f_s = 10.0;
        let dp = 3.0;
        let ds = 80.0;

        let (n, wn, _, t) = crate::generators::filter::buttord([f_p], [f_s], dp, ds, FilterGenPlane::Z { sampling_frequency: Some(fs) })
            .unwrap();
        let ba: Tf::<f64, _, _> = Tf::butter(n, wn, t, FilterGenPlane::Z { sampling_frequency: None })
            .unwrap();

        const N: usize = 1024;
        let (h, w): ([_; N], _) = ba.real_freqz(());

        plot::plot_curves("H(e^jw)", "temp/butter_hf.png", [&w.zip(h.map(|h| 10.0*h.norm_sqr().log10()))])
            .unwrap()
    }

    #[test]
    fn test_ap()
    {
        let fs = 2.0f64;
        let o = 6;
        let h_bessel = Zpk::besselap(o);
        let h_butter = Zpk::buttap(o);
        let h_cheb1 = Zpk::cheb1ap(o, 5.0);
        let h_cheb2 = Zpk::cheb2ap(o, 5.0);
        let h_ellip = Zpk::ellipap(o, 5.0, 5.0);
        
        const N: usize = 1024;
        let w: [_; N] = (0.0..fs).linspace_array();
        let h_f_bessel = h_bessel.freqs(w.map(|w| Complex::new(0.0, w)));
        let h_f_butter = h_butter.freqs(w.map(|w| Complex::new(0.0, w)));
        let h_f_cheb1 = h_cheb1.freqs(w.map(|w| Complex::new(0.0, w)));
        let h_f_cheb2 = h_cheb2.freqs(w.map(|w| Complex::new(0.0, w)));
        let h_f_ellip = h_ellip.freqs(w.map(|w| Complex::new(0.0, w)));

        plot::plot_curves("H(jw)", "plots/h_s_ap.png",
            [
                &w.zip(h_f_bessel.map(|h| 10.0*h.norm_sqr().log10())),
                &w.zip(h_f_butter.map(|h| 10.0*h.norm_sqr().log10())),
                &w.zip(h_f_cheb1.map(|h| 10.0*h.norm_sqr().log10())),
                &w.zip(h_f_cheb2.map(|h| 10.0*h.norm_sqr().log10())),
                &w.zip(h_f_ellip.map(|h| 10.0*h.norm_sqr().log10())),
            ]
        ).unwrap();
    }

    #[test]
    fn test()
    {
        let fs: f64 = 44100.0;
        let h1 = Tf::<f64, [_; 2], [_; 2]>::besself((), [800.0], FilterGenType::HighPass, FilterGenPlane::Z { sampling_frequency: Some(fs) }).unwrap();
        let h2 = Tf::<f64, [_; 5], [_; 5]>::butter((), [8000.0], FilterGenType::LowPass, FilterGenPlane::Z { sampling_frequency: Some(fs) }).unwrap();
        let h3 = Tf::<f64, [_; 3], [_; 3]>::butter((), [20000.0], FilterGenType::HighPass, FilterGenPlane::Z { sampling_frequency: Some(fs) }).unwrap();

        let h = h1*&h2 + h3;

        const N: usize = 1024;

        //let h: Sos<f64, _> = h.to_sos((), ());
        let (h_f, w): ([_; N], [_; N]) = h.freqz((), false);

        plot::plot_curves("H(e^jw)", "plots/h_z.png", [&w.zip(h_f.map(|h| h.norm())), &w.zip(h_f.map(|h| h.arg()))]).unwrap();

        const M: usize = 256;

        let z_r = Complex::new(1.0, 1.0);
        let z = (0..M).map(|i| (0..M).map(|j| Complex::new((2.0*i as f64/(M - 1) as f64 - 1.0)*z_r.re, (2.0*j as f64/(M - 1) as f64 - 1.0)*z_r.im)).collect()).collect();
        let h_z: Vec<Vec<Complex<f64>>> = FreqS::<Complex<f64>, Vec<Vec<Complex<f64>>>>::freqs(&h, z);

        let o = <[_; M]>::fill(|j| (2.0*j as f64/(M - 1) as f64 - 1.0)*z_r.re);
        let w = <[_; M]>::fill(|i| (2.0*i as f64/(M - 1) as f64 - 1.0)*z_r.im);

        let i: [f64; M] = (0..M).linspace_array().map(|i| i as f64);

        plot::plot_parametric_curve_2d("H(z)", "plots/h_z.svg",
            i,
            i,
            |i, j| [w[i as usize], o[j as usize], (h_z[i as usize][j as usize].norm().log10()*20.0).min(10.0).max(-10.0)]
        ).unwrap();
    }
}
