use num::{complex::ComplexFloat, traits::FloatConst, Float};
use option_trait::{Maybe, StaticMaybe};

use crate::{quantities::OwnedList, systems::Tf, System};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum CombFilterType
{
    Notch,
    Peak
}

pub trait IirComb<O>: System + Sized
where
    O: Maybe<usize>
{
    fn iir_comb(
        order: O,
        quality_factor: <Self::Set as ComplexFloat>::Real,
        filter_type: CombFilterType,
        pass_zero: bool
    ) -> Self;
}

impl<T, BA> IirComb<<BA::Length as StaticMaybe<usize>>::Opposite> for Tf<T, BA, BA>
where
    T: Float + FloatConst,
    BA: OwnedList<T>,
    <BA::Length as StaticMaybe<usize>>::Opposite: Sized,
    [(); BA::LENGTH - 1]:
{
    fn iir_comb(
        order: <BA::Length as StaticMaybe<usize>>::Opposite,
        quality_factor: <Self::Set as ComplexFloat>::Real,
        filter_type: CombFilterType,
        pass_zero: bool
    ) -> Self
    {
        let zero = T::zero();
        let one = T::one();
        let two = one + one;
        let four = two + two;

        let n = order.into_option()
            .unwrap_or(BA::LENGTH - 1);
        let nf = T::from(n).unwrap();
        let w0 = T::TAU()/nf;
        let w_delta = w0 / quality_factor;

        let (g0, g) = match filter_type
        {
            CombFilterType::Notch => (one, zero),
            CombFilterType::Peak => (zero, one),
        };
        let gb = T::FRAC_1_SQRT_2();
        
        let beta = ((gb*gb - g0*g0)/(g*g - gb*gb)).sqrt()*(nf*w_delta/four).tan();

        let ax = (one - beta)/(one + beta);
        let bx = (g0 + g*beta)/(one + beta);
        let cx = (g0 - g*beta)/(one + beta);

        let negative_coef = match filter_type
        {
            CombFilterType::Notch => !pass_zero,
            CombFilterType::Peak => pass_zero,
        };

        let mut b = vec![zero; n + 1];
        b[0] = bx;
        *b.last_mut().unwrap() = match negative_coef
        {
            true => -cx,
            false => cx
        };

        let mut a = vec![zero; n + 1];
        a[0] = one;
        *a.last_mut().unwrap() = match negative_coef
        {
            true => -ax,
            false => ax
        };

        let mut b = b.into_iter();
        let mut a = a.into_iter();

        Tf::new(
            BA::from_len_fn(StaticMaybe::maybe_from_fn(|| n + 1), |_| b.next().unwrap()),
            BA::from_len_fn(StaticMaybe::maybe_from_fn(|| n + 1), |_| a.next().unwrap())
        )
    }
}

#[cfg(test)]
mod test
{
    use array_math::ArrayOps;

    use crate::{analysis::RealFreqZ, plot, systems::{Tf, Zpk}, transforms::system::ToZpk, Plane};

    use super::{CombFilterType, IirComb};

    #[test]
    fn test()
    {
        let h: Tf::<_, Vec<_>, Vec<_>> = Tf::iir_comb(10, 30.0, CombFilterType::Notch, false);

        const N: usize = 1024;
        let (h_f, w): ([_; N], _) = h.real_freqz(());

        plot::plot_curves("H(e^jw)", "plots/h_z_iir_comb.png", [&w.zip(h_f.map(|h| h.norm()))])
            .unwrap();

        let h: Zpk<_, Vec<_>, Vec<_>, _> = h.to_zpk((), ());

        plot::plot_pz("H(z)", "plots/pz_z_iir_comb.png", &h.p, &h.z, Plane::Z)
            .unwrap();
    }
}