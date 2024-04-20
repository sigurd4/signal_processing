use core::ops::{AddAssign, Deref, DivAssign, MulAssign};

use num::{complex::ComplexFloat, Complex, Zero};
use option_trait::Maybe;
use array_math::SliceMath;

use crate::{ComplexOp, List, ListOrSingle, Lists, MaybeContainer, MaybeList, MaybeLists, System, Tf, TruncateIm};


pub trait FftFilt<'a, X, XX>: System
where
    Self::Domain: ComplexOp<X>,
    X: Into<<Self::Domain as ComplexOp<X>>::Output> + ComplexFloat<Real = <Self::Domain as ComplexFloat>::Real>,
    XX: Lists<X>
{
    type Output: ListOrSingle<XX::Mapped<<Self::Domain as ComplexOp<X>>::Output>>;

    fn fftfilt<N>(&'a self, x: XX, n: N) -> Self::Output
    where
        N: Maybe<usize>;
}

impl<'a, T, B, A, X, Y, XX, XXX> FftFilt<'a, X, XX> for Tf<T, B, A>
where
    T: ComplexFloat<Real: Into<Y> + 'static> + ComplexOp<X, Output = Y> + Into<Complex<T::Real>> + 'a,
    A: MaybeList<T, MaybeMapped<Complex<T::Real>>: MaybeList<Complex<T::Real>>>,
    B: MaybeLists<T, RowView<'a>: MaybeList<T, MaybeMapped<Complex<T::Real>>: MaybeList<Complex<T::Real>>>> + 'a,
    X: ComplexFloat<Real = T::Real> + Into<Y> + Into<Complex<T::Real>>,
    XX: Lists<X, Mapped<Complex<T::Real>>: Clone + Lists<Complex<T::Real>, RowOwned = XXX, RowsMapped<XXX::Mapped<Y>>: Into<XX::Mapped<Y>>>>,
    XXX: List<Complex<T::Real>>,
    Y: ComplexFloat<Real = T::Real> + 'static,
    Complex<T::Real>: Clone + AddAssign + MulAssign + DivAssign + MulAssign<T::Real>
{
    type Output = B::RowsMapped<XX::Mapped<Y>>;

    fn fftfilt<N>(&'a self, x: XX, n: N) -> Self::Output
    where
        N: Maybe<usize>
    {
        let x = x.map_into_owned(|x| x.into());
        
        let a = self.a.deref()
            .maybe_map_to_owned(|&a| a.into());

        self.b.map_rows_to_owned(|b| {
                x.clone().map_rows_into_owned(|x| {
                    let b: Option<Vec<Complex<_>>> = b.maybe_map_to_owned(|&b| b.into())
                        .into_vec_option();
                    let a: Option<Vec<Complex<_>>> = a.to_vec_option();
                    let mut y: Vec<Complex<_>> = x.to_vec();
                    
                    let m = y.len();

                    let n_min = (m + b.as_ref().map(|b| b.len()).unwrap_or(1).max(a.as_ref().map(|a| a.len()).unwrap_or(1)) - 1).next_power_of_two();
                    let (_overlap_add, n) = n.as_option()
                        .map(|&n| (true, n.max(n_min)))
                        .unwrap_or((false, n_min));

                    y.resize(n, Zero::zero());
                    y.fft();

                    if let Some(mut b) = b
                    {
                        b.resize(n, Zero::zero());
                        b.fft();
                        for (y, b) in y.iter_mut()
                            .zip(b)
                        {
                            *y *= b
                        }
                    }
                    if let Some(mut a) = a
                    {
                        a.resize(n, Zero::zero());
                        a.fft();
                        for (y, a) in y.iter_mut()
                            .zip(a)
                        {
                            *y /= a
                        }
                    }

                    y.ifft();

                    y.truncate(m);

                    let mut y = y.into_iter();
                    x.map_to_owned(|_| y.next().unwrap().truncate_im::<Y>())
                }).into()
            })
    }
}

#[cfg(test)]
mod test
{
    use array_math::ArrayOps;
    use linspace::LinspaceArray;
    use rand::distributions::uniform::SampleRange;

    use crate::{plot, Butter, FftFilt, FilterGenPlane, FilterGenType, Tf};

    #[test]
    fn test()
    {
        let h = Tf::butter(4, [0.5], FilterGenType::LowPass, FilterGenPlane::Z { sampling_frequency: None })
            .unwrap();

        const N: usize = 64;
        let mut rng = rand::thread_rng();
        let x: [f64; N] = ArrayOps::fill(|_| (-1.0..1.0).sample_single(&mut rng));

        let y = h.fftfilt(x, ());

        let t: [_; N] = (0.0..N as f64).linspace_array();

        plot::plot_curves("x(t), y(t)", "plots/xy_t_fftfilt.png", [
            &t.zip(x),
            &t.zip(y)
        ]).unwrap()
    }
}