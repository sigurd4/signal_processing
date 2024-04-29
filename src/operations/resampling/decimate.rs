use array_math::max_len;
use num::{complex::ComplexFloat, NumCast};
use option_trait::Maybe;

use crate::{gen::filter::{Cheby1, FilterGenPlane, FilterGenType, Fir1, Fir1Type}, util::ComplexOp, operations::filtering::{FftFilt, FiltFilt}, quantities::{List, MaybeLists, ListOrSingle, Lists}, System, systems::{Tf, IrType}};

pub trait Decimate<T, Q, Y>: Lists<T>
where
    T: ComplexFloat,
    Y: List<T>,
    Q: Maybe<usize>
{
    fn decimate<N, F>(self, ratio: Q, order: N, filter_type: F) -> Self::RowsMapped<Y>
    where
        N: Maybe<usize>,
        F: Maybe<IrType>;
}

impl<T, L> Decimate<T, usize, Vec<T>> for L
where
    L: Lists<T, RowOwned: List<T>>,
    T: ComplexFloat<Real: ComplexOp<T, Output = T> + ComplexFloat<Real = T::Real>>,
    Tf<T::Real, Vec<T::Real>, ()>: Fir1<usize, [T::Real; 1], T::Real, (), false> + System<Domain = T::Real> + for<'a> FftFilt<'a, T, Vec<T>, Output = Vec<T>>,
    Tf<T::Real, Vec<T::Real>, Vec<T::Real>>: Cheby1<usize> + System<Domain = T::Real> + for<'a> FiltFilt<'a, T, Vec<T>, Output = Vec<T>>
{
    fn decimate<N, F>(self, ratio: usize, order: N, filter_type: F) -> Self::RowsMapped<Vec<T>>
    where
        N: Maybe<usize>,
        F: Maybe<IrType>
    {
        let ftype = filter_type.into_option()
            .unwrap_or(IrType::IIR);
        let n = order.into_option()
            .unwrap_or_else(|| match ftype
            {
                IrType::FIR => 30,
                IrType::IIR => 8
            });
        let qf = <T::Real as NumCast>::from(ratio).unwrap();

        self.map_rows_into_owned(|x| {
            let x = x.into_vec();

            let y = match ftype
            {
                IrType::FIR => {
                    let tf = Tf::fir1(
                            n,
                            [qf.recip()],
                            Fir1Type::LowPass,
                            (),
                            true,
                            None
                        ).unwrap();
                    tf.fftfilt(x, ())
                },
                IrType::IIR => {
                    let tf = Tf::cheby1(
                            n,
                            <T::Real as NumCast>::from(0.05).unwrap(),
                            [<T::Real as NumCast>::from(0.8).unwrap()/qf],
                            FilterGenType::LowPass,
                            FilterGenPlane::Z {sampling_frequency: None}
                        ).unwrap();
                    tf.filtfilt(x)
                }
            };

            y.into_iter()
                .step_by(ratio)
                .collect()
        })
    }
}

impl<T, L, const M: usize> Decimate<T, (), [T; M]> for L
where
    L: Lists<T, Width = usize> + Decimate<T, usize, Vec<T>>,
    L::RowsMapped<Vec<T>>: Lists<T, RowsMapped<[T; M]> = L::RowsMapped<[T; M]>, RowOwned = Vec<T>>,
    T: ComplexFloat,
    [(); 0 - M % max_len(L::WIDTH, 1)]:
{
    fn decimate<N, F>(self, (): (), order: N, filter_type: F) -> Self::RowsMapped<[T; M]>
    where
        N: Maybe<usize>,
        F: Maybe<IrType>
    {
        let n = M / L::WIDTH.max(1);

        self.decimate(n, order, filter_type)
            .map_rows_into_owned(|y| {
                TryInto::<[T; M]>::try_into(y)
                    .ok()
                    .unwrap()
            })
    }
}