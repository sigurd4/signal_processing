

use ndarray::{Array2};
use num::{complex::ComplexFloat, traits::FloatConst, Complex, Float};
use option_trait::Maybe;


use crate::{Bilinear, Cheb2AP, FilterGenError, FilterGenPlane, FilterGenType, SfTrans, Sos, Ss, System, Tf, ToSos, ToSs, ToTf, Zpk};

pub trait Cheby2<O>: System + Sized
where
    Self::Domain: Float,
    O: Maybe<usize>
{
    fn cheby2<const F: usize>(
        order: O,
        ripple: Self::Domain,
        frequencies: [Self::Domain; F],
        filter_type: FilterGenType,
        plane: FilterGenPlane<Self::Domain>
    ) -> Result<Self, FilterGenError>
    where
        [(); F - 1]:,
        [(); 2 - F]:;
}

impl<T> Cheby2<usize> for Zpk<Complex<T>, Vec<Complex<T>>, Vec<Complex<T>>, T>
where
    T: Float + FloatConst,
    Complex<T>: ComplexFloat<Real = T>,
    Zpk<Complex<T>, Vec<Complex<T>>, Vec<Complex<T>>, T>: Cheb2AP<usize> + for<'a> SfTrans<'a, Output = Self> + System<Domain = T>,
    Self: for<'a> Bilinear<'a, Output = Self> + System<Domain = T>
{
    fn cheby2<const F: usize>(
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
        let two = T::from(2.0).unwrap();
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
    
        let zpk = Zpk::cheb2ap(order, ripple);
    
        let zpk = if !band && F == 2
        {
            zpk.sftrans::<1>([frequencies[!stop as usize]], stop).unwrap()
        }
        else if band && F == 1
        {
            zpk.sftrans::<2>([frequencies[0], frequencies[0]], stop).unwrap()
        }
        else
        {
            zpk.sftrans(frequencies, stop).unwrap()
        };

        if let Some(t) = t
        {
            Ok(zpk.bilinear(t).unwrap())
        }
        else
        {
            Ok(zpk)
        }
    }
}

impl<T> Cheby2<usize> for Tf<T, Vec<T>, Vec<T>>
where
    T: Float + FloatConst,
    Zpk<Complex<T>, Vec<Complex<T>>, Vec<Complex<T>>, T>: Cheby2<usize> + for<'a> ToTf<'a, T, Vec<T>, Vec<T>, (), ()> + System<Domain = T>
{
    fn cheby2<const F: usize>(
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
        let zpk = Zpk::cheby2(order, ripple, frequencies, filter_type, plane)?;
    
        Ok(zpk.to_tf((), ()))
    }
}

impl<T, const N: usize> Cheby2<()> for Tf<T, [T; N], [T; N]>
where
    [(); N - 2]:,
    T: Float + FloatConst,
    Tf<T, Vec<T>, Vec<T>>: Cheby2<usize> + System<Domain = T>
{
    fn cheby2<const F: usize>(
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
        let tf = Tf::cheby2(N - 1, ripple, frequencies, filter_type, plane)?;

        Ok(tf.truncate())
    }
}

impl<T> Cheby2<usize> for Sos<T, [T; 3], [T; 3], Vec<Tf<T, [T; 3], [T; 3]>>>
where
    T: Float + FloatConst,
    Zpk<Complex<T>, Vec<Complex<T>>, Vec<Complex<T>>, T>: Cheby2<usize> + for<'a> ToSos<'a, T, [T; 3], [T; 3], Vec<Tf<T, [T; 3], [T; 3]>>, (), ()> + System<Domain = T>
{
    fn cheby2<const F: usize>(
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
        let zpk = Zpk::cheby2(order, ripple, frequencies, filter_type, plane)?;
    
        Ok(zpk.to_sos((), ()))
    }
}

impl<T, const N: usize> Cheby2<()> for Sos<T, [T; 3], [T; 3], [Tf<T, [T; 3], [T; 3]>; N]>
where
    T: Float + FloatConst,
    Sos<T, [T; 3], [T; 3], Vec<Tf<T, [T; 3], [T; 3]>>>: Cheby2<usize> + System<Domain = T>
{
    fn cheby2<const F: usize>(
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
        let sos = Sos::cheby2(N*2, ripple, frequencies, filter_type, plane)?;

        Ok(Sos {
            sos: sos.sos.try_into().map_err(|_| ()).unwrap()
        })
    }
}

impl<T> Cheby2<usize> for Ss<T, Array2<T>, Array2<T>, Array2<T>, Array2<T>>
where
    T: Float + FloatConst,
    Zpk<Complex<T>, Vec<Complex<T>>, Vec<Complex<T>>, T>: Cheby2<usize> + for<'a> ToSs<'a, T, Array2<T>, Array2<T>, Array2<T>, Array2<T>> + System<Domain = T>
{
    fn cheby2<const F: usize>(
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
        let zpk = Zpk::cheby2(order, ripple, frequencies, filter_type, plane)?;
    
        Ok(zpk.to_ss().unwrap())
    }
}

#[cfg(test)]
mod test
{
    use array_math::{ArrayOps};
    
    

    use crate::{plot, Cheby2, FilterGenPlane, RealFreqZ, Tf};

    #[test]
    fn test()
    {
        let fs = 1000.0;

        let (n, wp, _ws, rs, t) = crate::cheb2ord(
            [40.0],
            [150.0],
            3.0,
            60.0,
            FilterGenPlane::Z { sampling_frequency: Some(fs) }
        ).unwrap();

        let h = Tf::cheby2(n, rs, wp, t, FilterGenPlane::Z { sampling_frequency: None })
            .unwrap();

        const N: usize = 1024;
        let (h_f, w): ([_; N], _) = h.real_freqz(());

        plot::plot_curves("H(e^jw)", "plots/h_z_cheby2.png", [&w.zip(h_f.map(|h| h.norm())), &w.zip(h_f.map(|h| h.arg()))]).unwrap();
    }
}