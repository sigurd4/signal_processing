use num::{complex::ComplexFloat, traits::FloatConst, Complex, Float};
use option_trait::Maybe;

use crate::{gen::filter::GammatoneError, systems::Tf, System};

pub trait GammatoneIir: System + Sized
{
    fn gammatone_iir<FS>(
        frequency: <Self::Set as ComplexFloat>::Real,
        sampling_frequency: FS
    ) -> Result<Self, GammatoneError>
    where
        FS: Maybe<<Self::Set as ComplexFloat>::Real>;
}

impl<T> GammatoneIir for Tf<T, [T; 5], [T; 9]>
where
    T: Float + FloatConst
{
    fn gammatone_iir<FS>(
        frequency: <Self::Set as ComplexFloat>::Real,
        sampling_frequency: FS
    ) -> Result<Self, GammatoneError>
    where
        FS: Maybe<<Self::Set as ComplexFloat>::Real>
    {
        let zero = T::zero();
        let one = T::one();
        let two = one + one;
        let three = two + one;
        let four = two + two;
        let five = four + one;
        let eight = four + four;
        let sixteen = eight + eight;
        let six = three + three;
        let seven = six + one;
        let eighteen = six*three;

        let fs = if let Some(fs) = sampling_frequency.into_option()
        {
            if !(fs > zero)
            {
                return Err(GammatoneError::InvalidSamplingFrequency)
            }
            fs
        }
        else
        {
            two
        };

        if !(zero < frequency && frequency < fs/two)
        {
            return Err(GammatoneError::FrequencyOutOfRange)
        }

        let hz_to_erb = |hz: T| {
            const EAR_Q: f64 = 9.26449;
            const MIN_BW: f64 = 24.7;
            hz/T::from(EAR_Q).unwrap() + T::from(MIN_BW).unwrap()
        };

        let t = fs.recip();
        let bw = T::TAU()*T::from(1.019).unwrap()*hz_to_erb(frequency);
        let fr = T::TAU()*frequency*t;
        let bwt = bw*t;

        let sqrt2p3 = T::SQRT_2()*T::SQRT_2()*T::SQRT_2();
        let g1 = -Complex::cis(two*fr)*two*t;
        let g2 = Complex::new(-bwt, fr).exp()*two*t;
        let g3 = (three + sqrt2p3).sqrt()*fr.sin();
        let g4 = (three - sqrt2p3).sqrt()*fr.sin();
        let g5 = Complex::cis(two*fr);

        let gg = (g5 + one)*two/bwt.exp() - two/(two*bwt).exp() - g5*two;
        let gggg = (gg.conj()*gg).re;
        let g = ((g1 + g2*(fr.cos() - g4))
            *(g1 + g2*(fr.cos() + g4))
            *(g1 + g2*(fr.cos() - g3))
            *(g1 + g2*(fr.cos() + g3))
            /(gggg*gggg)
        ).abs();

        let tt = t*t;
        let tttt = tt*tt;
        Ok(Tf::new(
            [
                tttt/g,
                -four*tttt*fr.cos()/bwt.exp()/g,
                six*tttt*(two*fr).cos()/(two*bwt).exp()/g,
                -four*tttt*(three*fr).cos()/(three*bwt).exp()/g,
                tttt*(four*fr).cos()/(four*bwt).exp()/g
            ],
            [
                one,
                -eight*fr.cos()/bwt.exp(),
                four*(four + three*(two*fr).cos())/(two*bwt).exp(),
                -eight*(six*fr.cos() + (three*fr).cos())/(three*bwt).exp(),
                two*(eighteen + sixteen*(two*fr).cos() + (four*fr).cos())/(four*bwt).exp(),
                -eight*(six*fr.cos() + (three*fr).cos())/(five*bwt).exp(),
                four*(four + three*(two*fr).cos())/(six*bwt).exp(),
                -eight*fr.cos()/(seven*bwt).exp(),
                (-eight*bwt).exp()
            ]
        ))
    }
}

#[cfg(test)]
mod test
{
    use array_math::ArrayOps;

    use crate::{analysis::RealFreqZ, gen::filter::GammatoneIir, plot, systems::{Tf, Zpk}, transforms::system::ToZpk, Plane};

    #[test]
    fn test()
    {
        let h = Tf::gammatone_iir(440.0, 16000.0)
            .unwrap();

        println!("{:?}", h);

        const N: usize = 1024;
        let (h_f, w): ([_; N], _) = h.real_freqz(());

        plot::plot_curves("H(e^jw)", "plots/h_z_gammatone_iir.png", [&w.zip(h_f.map(|h| h.norm()))])
            .unwrap();

        let h: Zpk<_, Vec<_>, Vec<_>, _> = h.to_zpk((), ());

        plot::plot_pz("H(z)", "plots/pz_z_gammatone_iir.png", &h.p, &h.z, Plane::Z)
            .unwrap();
    }
}