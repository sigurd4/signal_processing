use num::{complex::ComplexFloat, Complex, Float, NumCast, One, Zero};
use option_trait::{Maybe, StaticMaybe};

use crate::{quantities::{ListOrSingle, MaybeList, MaybeLists, MaybeOwnedList, OwnedList, OwnedLists}, systems::{Sos, Tf, Zpk}, transforms::system::ToZpk, System};

pub trait FindFreqS<F, FF>: System
where
    F: OwnedList<<Self::Set as ComplexFloat>::Real>,
    <F::Length as StaticMaybe<usize>>::Opposite: Sized,
    FF: ListOrSingle<F> + OwnedLists<<Self::Set as ComplexFloat>::Real>
{
    fn find_freqs(self, numtaps: <F::Length as StaticMaybe<usize>>::Opposite) -> FF;
}

impl<T, Z, P, K, F> FindFreqS<F, F> for Zpk<T, Z, P, K>
where
    T: ComplexFloat,
    Z: MaybeList<T>,
    P: MaybeList<T>,
    K: ComplexFloat<Real = T::Real>,
    F: OwnedList<T::Real>,
    <F::Length as StaticMaybe<usize>>::Opposite: Sized
{
    fn find_freqs(self, numtaps: <F::Length as StaticMaybe<usize>>::Opposite) -> F
    {
        let (tz, mut ep) = (
            self.z.into_inner()
                .into_vec_option()
                .unwrap_or_else(|| vec![]),
            self.p.into_inner()
                .into_vec_option()
                .unwrap_or_else(|| vec![])
        );

        const BIG_NUM_P: f64 = 1000.0;
        const BIG_NUM_Z: f64 = 1e5;
        const SMALL_NUM: f64 = 1e-10;

        if ep.len() == 0
        {
            ep.push(T::from(-BIG_NUM_P).unwrap());
        }

        let zero = T::Real::zero();

        let big_num_z = <T::Real as NumCast>::from(BIG_NUM_Z).unwrap();
        let ez: Vec<T> = ep.into_iter()
            .filter(|&p| p.im() >= zero)
            .chain(tz.into_iter()
                .filter(|z| z.abs() < big_num_z && z.im() >= zero)
            ).collect();

        let small_num = <T::Real as NumCast>::from(SMALL_NUM).unwrap();
        let integ: Vec<_> = ez.iter()
            .map(|&z| z.abs() < small_num)
            .collect();

        let one = T::Real::one();
        let two = one + one;
        let three = two + one;
        let one_and_a_half = three/two;
        let half = Float::recip(two);
        let ten = <T::Real as NumCast>::from(10u8).unwrap();
        let hfreq = (Float::log10(
                ez.iter()
                    .zip(integ.iter())
                    .map(|(&z, &i)| {
                        Float::abs(z.re() + <T::Real as NumCast>::from(i as u8).unwrap())*three + z.im()*one_and_a_half
                    }).reduce(Float::max)
                    .unwrap()
            ) + half
        ).round();
        let lfreq = (Float::log10(
                ez.into_iter()
                    .zip(integ)
                    .map(|(z, i)| {
                        Float::abs(z.re() + <T::Real as NumCast>::from(i as u8).unwrap()) + z.im()*two
                    }).reduce(Float::min)
                    .unwrap()
                    *<T::Real as NumCast>::from(0.1).unwrap()
            ) - half
        ).round();
        
        let n = numtaps.as_option()
            .copied()
            .unwrap_or(F::LENGTH);
        let nm1 = <T::Real as NumCast>::from(n.saturating_sub(1)).unwrap();
        F::from_len_fn(numtaps, |i| {
            let i = <T::Real as NumCast>::from(i).unwrap();
            let p = if nm1.is_zero() {half} else {i/nm1};
            Float::powf(ten, lfreq + (hfreq - lfreq)*p)
        })
    }
}

impl<T, B, A, F> FindFreqS<F, B::RowsMapped<F>> for Tf<T, B, A>
where
    T: ComplexFloat,
    B: MaybeLists<T>,
    A: MaybeList<T>,
    F: OwnedList<T::Real>,
    <F::Length as StaticMaybe<usize>>::Opposite: Sized + Clone,
    B::RowsMapped<F>: OwnedLists<T::Real>,
    for<'a> A::View<'a>: MaybeList<T>,
    for<'a> Tf<T, B::RowOwned, A::View<'a>>: ToZpk<Complex<T::Real>, Vec<Complex<T::Real>>, Vec<Complex<T::Real>>, T, (), ()>,
    Zpk<Complex<T::Real>, Vec<Complex<T::Real>>, Vec<Complex<T::Real>>, T>: FindFreqS<F, F> + System<Set = T>
{
    fn find_freqs(self, numtaps: <F::Length as StaticMaybe<usize>>::Opposite) -> B::RowsMapped<F>
    {
        let (b, a) = (self.b.into_inner(), self.a.into_inner());

        b.map_rows_into_owned(|b| {
            Tf::new(b, a.as_view())
                .to_zpk((), ())
                .find_freqs(numtaps.clone())
        })
    }
}

impl<T, B, A, S, F> FindFreqS<F, F> for Sos<T, B, A, S>
where
    T: ComplexFloat,
    B: Maybe<[T; 3]> + MaybeOwnedList<T>,
    A: Maybe<[T; 3]> + MaybeOwnedList<T>,
    S: MaybeList<Tf<T, B, A>>,
    F: OwnedList<T::Real>,
    <F::Length as StaticMaybe<usize>>::Opposite: Sized,
    Self: ToZpk<Complex<T::Real>, Vec<Complex<T::Real>>, Vec<Complex<T::Real>>, T, (), ()> + System<Set = T>,
    Zpk<Complex<T::Real>, Vec<Complex<T::Real>>, Vec<Complex<T::Real>>, T>: FindFreqS<F, F> + System<Set = T>
{
    fn find_freqs(self, numtaps: <F::Length as StaticMaybe<usize>>::Opposite) -> F
    {
        self.to_zpk((), ())
            .find_freqs(numtaps)
    }
}

#[cfg(test)]
mod test
{
    use crate::systems::Tf;

    use super::FindFreqS;

    #[test]
    fn test()
    {
        let h = Tf::new([1.0, 0.0], [1.0, 8.0, 25.0]);

        let f: Vec<_> = h.find_freqs(9);
        println!("{:?}", f);
    }
}