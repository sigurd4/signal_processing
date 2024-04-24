use core::ops::{AddAssign, DivAssign, MulAssign, SubAssign};

use ndarray_linalg::Lapack;
use num::{complex::ComplexFloat, traits::FloatConst, Complex, Float, One};
use array_math::SliceMath;
use option_trait::Maybe;

use crate::{MaybeList, MaybeLists, MaybeOwnedList, MaybeOwnedLists, Polynomial, Sos, System, Tf, TruncateIm, Zpk};

pub trait Qmf: System
{
    type Output: System<Domain = Self::Domain>;

    fn qmf(self) -> Self::Output;
}

impl<T, Z, P, K> Qmf for Zpk<T, Z, P, K>
where
    T: ComplexFloat,
    Z: MaybeList<T, Owned: MaybeOwnedList<T>>,
    P: MaybeList<T, Owned: MaybeOwnedList<T>>,
    K: ComplexFloat<Real = T::Real>
{
    type Output = Zpk<T, Z::Owned, P::Owned, K>;

    fn qmf(self) -> Self::Output
    {
        let mut h = self.to_owned();
        for z in [h.z.as_mut_slice_option(), h.p.as_mut_slice_option()]
        {
            if let Some(z) = z
            {
                for z in z.iter_mut()
                {
                    *z = -*z
                }
            }
        }
        h
    }
}

impl<T, R, B, A> Qmf for Tf<T, B, A>
where
    R: Float + FloatConst + Into<T> + 'static,
    T: ComplexFloat<Real = R> + Lapack<Complex = Complex<R>> + Into<Complex<R>> + 'static,
    B: MaybeLists<T, Owned: MaybeOwnedLists<T>>,
    A: MaybeList<T, Owned: MaybeOwnedList<T>>,
    Complex<R>: AddAssign + SubAssign + MulAssign + DivAssign + DivAssign<R> + MulAssign<T>,
{
    type Output = Tf<T, B::Owned, A::Owned>;

    fn qmf(self) -> Self::Output
    {
        let cone = Complex::one();

        let qmf_poly = |b: &mut [T]| {
            let b = b.trim_zeros_front_mut();
            let nb = b.len();
            if nb > 1
            {
                let mut z: Vec<_> = b.rpolynomial_roots();
                for z in z.iter_mut()
                {
                    *z = -*z
                }
                let mut bb: Vec<_> = z.into_iter()
                    .map(|z| Polynomial::new(vec![cone, -z]))
                    .product::<Polynomial<_, Vec<_>>>()
                    .into_inner();
                for bb in bb.iter_mut()
                {
                    *bb *= b[0]
                }
                for (b, bb) in b.iter_mut()
                    .zip(bb)
                {
                    *b = bb.truncate_im()
                }
            }
        };
        
        let mut h = self.to_owned();

        if let Some(b) = h.b.as_mut_slices_option()
        {
            for b in b
            {
                qmf_poly(b)
            }
        }
        if let Some(a) = h.a.as_mut_slice_option()
        {
            qmf_poly(a)
        }

        h
    }
}

impl<T, B, A, B2, A2, S> Qmf for Sos<T, B, A, S>
where
    T: ComplexFloat,
    B: Maybe<[T; 3]> + MaybeOwnedList<T> + Clone,
    A: Maybe<[T; 3]> + MaybeOwnedList<T> + Clone,
    S: MaybeList<Tf<T, B, A>>,
    B2: Maybe<[T; 3]> + MaybeOwnedList<T>,
    A2: Maybe<[T; 3]> + MaybeOwnedList<T>,
    S::MaybeMapped<Tf<T, B2, A2>>: MaybeList<Tf<T, B2, A2>>,
    Tf<T, B, A>: Qmf<Output = Tf<T, B2, A2>> + System<Domain = T>
{
    type Output = Sos<T, B2, A2, S::MaybeMapped<Tf<T, B2, A2>>>;

    fn qmf(self) -> Self::Output
    {
        Sos::new(self.sos.into_inner().maybe_map_into_owned(|sos| sos.qmf()))
    }
}

#[cfg(test)]
mod test
{
    use array_math::ArrayOps;

    use crate::{plot, Cheby1, FilterGenPlane, FilterGenType, Qmf, RealFreqZ, Sos};

    #[test]
    fn test()
    {
        let h = Sos::cheby1(10, 7.0, [0.5], FilterGenType::LowPass, FilterGenPlane::Z { sampling_frequency: None })
            .unwrap();
        let hq = h.as_view()
            .qmf();

        const N: usize = 1024;
        let (hf, f): (_, [_; N]) = h.real_freqz(());
        let (hqf, _): (_, [_; N]) = hq.real_freqz(());

        plot::plot_curves("|H(e^jw)|", "plots/h_z_qmf.png", [
                &f.zip(hf.map(|h| h.norm())),
                &f.zip(hqf.map(|h| h.norm())),
                &f.zip(hf.comap(hqf, |h, hq| h.norm_sqr() + hq.norm_sqr()))
            ]).unwrap();
    }
}