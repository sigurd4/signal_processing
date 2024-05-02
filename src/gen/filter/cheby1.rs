

use ndarray::Array2;
use num::{complex::ComplexFloat, traits::FloatConst, Complex, Float};
use option_trait::Maybe;

use crate::{gen::filter::{Cheb1AP, FilterGenError, FilterGenPlane, FilterGenType}, transforms::{domain::Bilinear, filter::SfTrans, system::{ToSos, ToSs, ToTf}}, systems::{Sos, Ss, SsAMatrix, SsBMatrix, SsCMatrix, SsDMatrix, Tf, Zpk}, System};

pub trait Cheby1<O>: System + Sized
where
    O: Maybe<usize>
{
    fn cheby1<const F: usize>(
        order: O,
        ripple: <Self::Set as ComplexFloat>::Real,
        frequencies: [<Self::Set as ComplexFloat>::Real; F],
        filter_type: FilterGenType,
        plane: FilterGenPlane<<Self::Set as ComplexFloat>::Real>
    ) -> Result<Self, FilterGenError>
    where
        [(); F - 1]:,
        [(); 2 - F]:;
}

impl<T> Cheby1<usize> for Zpk<Complex<T>, Vec<Complex<T>>, Vec<Complex<T>>, T>
where
    T: Float + FloatConst,
    Complex<T>: ComplexFloat<Real = T>,
    Zpk<Complex<T>, (), Vec<Complex<T>>, T>: Cheb1AP<usize> + SfTrans<1, Output = Self> + SfTrans<2, Output = Self> + System<Set = T>,
    Self: Bilinear<Output = Self> + System<Set = T>
{
    fn cheby1<const F: usize>(
        order: usize,
        ripple: T,
        mut frequencies: [T; F],
        filter_type: FilterGenType,
        plane: FilterGenPlane<T>
    ) -> Result<Self, FilterGenError>
    where
        [(); F - 1]:,
        [(); 2 - F]:
    {
        if order < 1
        {
            return Err(FilterGenError::ZeroOrder)
        }
        if !frequencies.is_sorted()
        {
            return Err(FilterGenError::FrequenciesNotNondecreasing)
        }
        let stop = match filter_type
        {
            FilterGenType::LowPass => false,
            FilterGenType::HighPass => true,
            FilterGenType::BandPass => false,
            FilterGenType::BandStop => true,
        };
        let band = match filter_type
        {
            FilterGenType::LowPass => false,
            FilterGenType::HighPass => false,
            FilterGenType::BandPass => true,
            FilterGenType::BandStop => true
        };
        let one = T::one();
        let two = one + one;
        let t = if let FilterGenPlane::Z { sampling_frequency } = plane
        {
            let t = sampling_frequency.unwrap_or(two);
            for wc in frequencies.iter_mut()
            {
                if *wc > t/two
                {
                    return Err(FilterGenError::FrequenciesOutOfRange)
                }
                *wc = two/t*(T::PI()**wc/t).tan()
            }
            Some(t)
        }
        else
        {
            None
        };
    
        let zpk = Zpk::cheb1ap(order, ripple);
    
        let zpk = if !band && F == 2
        {
            SfTrans::<1>::sftrans(zpk, one, [frequencies[!stop as usize]], stop).unwrap()
        }
        else if band && F == 1
        {
            SfTrans::<2>::sftrans(zpk, one, [frequencies[0], frequencies[0]], stop).unwrap()
        }
        else if F == 1
        {
            SfTrans::<1>::sftrans(zpk, one, [frequencies[0]], stop).unwrap()
        }
        else
        {
            SfTrans::<2>::sftrans(zpk, one, [frequencies[0], frequencies[1]], stop).unwrap()
        };

        if let Some(t) = t
        {
            Ok(zpk.bilinear(t.recip()).unwrap())
        }
        else
        {
            Ok(zpk)
        }
    }
}

impl<T> Cheby1<usize> for Tf<T, Vec<T>, Vec<T>>
where
    T: Float + FloatConst,
    Zpk<Complex<T>, Vec<Complex<T>>, Vec<Complex<T>>, T>: Cheby1<usize> + ToTf<T, Vec<T>, Vec<T>, (), ()> + System<Set = T>
{
    fn cheby1<const F: usize>(
        order: usize,
        ripple: T,
        frequencies: [T; F],
        filter_type: FilterGenType,
        plane: FilterGenPlane<T>
    ) -> Result<Self, FilterGenError>
    where
        [(); F - 1]:,
        [(); 2 - F]:
    {
        let zpk = Zpk::cheby1(order, ripple, frequencies, filter_type, plane)?;
    
        Ok(zpk.to_tf((), ()))
    }
}

impl<T, const N: usize> Cheby1<()> for Tf<T, [T; N], [T; N]>
where
    [(); N - 2]:,
    T: Float + FloatConst,
    Tf<T, Vec<T>, Vec<T>>: Cheby1<usize> + System<Set = T>
{
    fn cheby1<const F: usize>(
        (): (),
        ripple: T,
        frequencies: [T; F],
        filter_type: FilterGenType,
        plane: FilterGenPlane<T>
    ) -> Result<Self, FilterGenError>
    where
        [(); F - 1]:,
        [(); 2 - F]:
    {
        let tf = Tf::cheby1(N - 1, ripple, frequencies, filter_type, plane)?;

        Ok(tf.truncate())
    }
}

impl<T> Cheby1<usize> for Sos<T, [T; 3], [T; 3], Vec<Tf<T, [T; 3], [T; 3]>>>
where
    T: Float + FloatConst,
    Zpk<Complex<T>, Vec<Complex<T>>, Vec<Complex<T>>, T>: Cheby1<usize> + ToSos<T, [T; 3], [T; 3], Vec<Tf<T, [T; 3], [T; 3]>>, (), ()> + System<Set = T>
{
    fn cheby1<const F: usize>(
        order: usize,
        ripple: T,
        frequencies: [T; F],
        filter_type: FilterGenType,
        plane: FilterGenPlane<T>
    ) -> Result<Self, FilterGenError>
    where
        [(); F - 1]:,
        [(); 2 - F]:
    {
        let zpk = Zpk::cheby1(order, ripple, frequencies, filter_type, plane)?;
    
        Ok(zpk.to_sos((), ()))
    }
}

impl<T, const N: usize> Cheby1<()> for Sos<T, [T; 3], [T; 3], [Tf<T, [T; 3], [T; 3]>; N]>
where
    T: Float + FloatConst,
    Sos<T, [T; 3], [T; 3], Vec<Tf<T, [T; 3], [T; 3]>>>: Cheby1<usize> + System<Set = T>
{
    fn cheby1<const F: usize>(
        (): (),
        ripple: T,
        frequencies: [T; F],
        filter_type: FilterGenType,
        plane: FilterGenPlane<T>
    ) -> Result<Self, FilterGenError>
    where
        [(); F - 1]:,
        [(); 2 - F]:
    {
        let sos = Sos::cheby1(N*2, ripple, frequencies, filter_type, plane)?;

        Ok(Sos {
            sos: sos.sos.try_into().map_err(|_| ()).unwrap()
        })
    }
}

impl<T> Cheby1<usize> for Ss<T, Array2<T>, Array2<T>, Array2<T>, Array2<T>>
where
    T: Float + FloatConst,
    Zpk<Complex<T>, Vec<Complex<T>>, Vec<Complex<T>>, T>: Cheby1<usize> + ToSs<T, Array2<T>, Array2<T>, Array2<T>, Array2<T>> + System<Set = T>,
    Array2<T>: SsAMatrix<T, Array2<T>, Array2<T>, Array2<T>> + SsBMatrix<T, Array2<T>, Array2<T>, Array2<T>> + SsCMatrix<T, Array2<T>, Array2<T>, Array2<T>>+ SsDMatrix<T, Array2<T>, Array2<T>, Array2<T>>
{
    fn cheby1<const F: usize>(
        order: usize,
        ripple: T,
        frequencies: [T; F],
        filter_type: FilterGenType,
        plane: FilterGenPlane<T>
    ) -> Result<Self, FilterGenError>
    where
        [(); F - 1]:,
        [(); 2 - F]:
    {
        let zpk = Zpk::cheby1(order, ripple, frequencies, filter_type, plane)?;
    
        Ok(zpk.to_ss())
    }
}

#[cfg(test)]
mod test
{
    use array_math::ArrayOps;
    
    use crate::{plot, gen::filter::{Cheby1, FilterGenPlane}, Plane, analysis::RealFreqZ, systems::Tf, transforms::system::ToZpk, systems::Zpk};

    #[test]
    fn test()
    {
        let fs = 1000.0;

        let (n, wp, _ws, rp, t) = crate::gen::filter::cheb1ord(
            [40.0],
            [150.0],
            3.0,
            60.0,
            FilterGenPlane::Z { sampling_frequency: Some(fs) }
        ).unwrap();

        let h = Tf::cheby1(n, rp, wp, t, FilterGenPlane::Z { sampling_frequency: None })
            .unwrap();

        const N: usize = 1024;
        let (h_f, w): ([_; N], _) = h.real_freqz(());

        plot::plot_curves("H(e^jw)", "plots/h_z_cheby1.png", [&w.zip(h_f.map(|h| h.norm())), &w.zip(h_f.map(|h| h.arg()))])
            .unwrap();

        let h: Zpk<_, Vec<_>, Vec<_>, _> = h.to_zpk((), ());

        plot::plot_pz("H(z)", "plots/pz_z_cheby1.png", &h.p, &h.z, Plane::Z)
            .unwrap();
    }
}