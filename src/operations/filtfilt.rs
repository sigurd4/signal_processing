use core::{iter::Sum, ops::AddAssign};

use num::complex::ComplexFloat;

use crate::{ComplexOp, FilterMut, ListOrSingle, Lists, MaybeList, MaybeLists, Polynomial, Rtf, RtfOrSystem, System, Tf};

pub trait FiltFilt<'a, X, XX>: System
where
    Self::Domain: ComplexOp<X>,
    X: Into<<Self::Domain as ComplexOp<X>>::Output> + ComplexFloat<Real = <Self::Domain as ComplexFloat>::Real>,
    XX: Lists<X>
{
    type Output: ListOrSingle<XX::Mapped<<Self::Domain as ComplexOp<X>>::Output>>;

    fn filtfilt(&'a self, x: XX) -> Self::Output;
}

impl<'a, T, B, A, X, XX> FiltFilt<'a, X, XX> for Tf<T, B, A>
where
    T: ComplexOp<X> + ComplexOp<T, Output = T> + Sum + AddAssign,
    X: Into<<T as ComplexOp<X>>::Output> + ComplexFloat<Real = T::Real>,
    <T as ComplexOp<X>>::Output: ComplexOp<T, Real = T::Real, Output = <T as ComplexOp<X>>::Output> + ComplexOp<X, Output = <T as ComplexOp<X>>::Output> + ComplexOp<<T as ComplexOp<X>>::Output, Output = <T as ComplexOp<X>>::Output>,
    B: MaybeLists<T>,
    A: MaybeList<T>,
    XX: Lists<X>,
    Self: 'a,
    &'a Self: Into<Tf<T, Vec<Vec<T>>, Vec<T>>>,
    for<'b> Tf<T, &'b [T], &'b [T]>: System<Domain = T>,
    for<'b> Rtf<'b, <T as ComplexOp<X>>::Output, Tf<T, &'b [T], &'b [T]>>: FilterMut<X, Vec<X>, Output = Vec<<T as ComplexOp<X>>::Output>> + FilterMut<<T as ComplexOp<X>>::Output, Vec<<T as ComplexOp<X>>::Output>, Output = Vec<<T as ComplexOp<X>>::Output>> + RtfOrSystem<Domain = <T as ComplexOp<X>>::Output>
{
    type Output = B::RowsMapped<XX::Mapped<<Self::Domain as ComplexOp<X>>::Output>>;

    fn filtfilt(&'a self, x: XX) -> Self::Output
    {
        let zero = T::zero();

        let Tf {b, mut a}: Tf<_, Vec<Vec<_>>, Vec<_>> = self.into();

        let mut b = b.into_inner().into_iter();
        self.b.map_rows_to_owned(|_| {
            let mut b = b.next()
                .unwrap();

            let lb = b.len();
            let la = a.len();
            let n = lb.max(la);
            let lrefl = 3*n.saturating_sub(1);
            if la < n
            {
                a.resize(n, zero)
            }
            if lb < n
            {
                b.resize(n, zero)
            }

            let kdc = b.iter().copied()
                    .sum::<T>()
                /a.iter().copied()
                    .sum::<T>();
            let si = if kdc.is_finite()
            {
                let mut s = zero;
                b.iter()
                    .zip(a.iter())
                    .rev()
                    .map(|(&b, &a)| {
                        s += b - kdc*a;
                        s
                    }).rev()
                    .collect()
            }
            else
            {
                vec![zero; la]
            };

            let onex = X::one();
            let twox = onex + onex;

            let mut y = vec![];
            x.map_rows_to_owned(|x| {
                let x: Vec<X> = MaybeList::<X>::as_view_slice_option(&x)
                    .unwrap_or(&[onex])
                    .to_vec();
                let xl = x.len();

                if xl == 0
                {
                    return vec![]
                }

                let lrefl = lrefl.min(xl.saturating_sub(1));
                let v: Vec<_> = x[1..=lrefl].iter()
                    .map(|&xk| twox*x[0] - xk)
                    .rev()
                    .chain(x.iter().copied()
                    ).chain(x[(xl - lrefl).saturating_sub(1)..xl.saturating_sub(1)].iter()
                        .map(|&xk| twox*x[xl - 1] - xk)
                        .rev()
                    ).collect();

                let sys = Tf {
                    b: Polynomial::new(b.as_slice()),
                    a: Polynomial::new(a.as_slice())
                };
                let mut rtf = Rtf::new(&sys, Some(si.iter()
                    .map(|&si| Into::<<T as ComplexOp<X>>::Output>::into(si)*v[0].into())
                    .collect()
                ));
                let mut v: Vec<_> = rtf.filter_mut(v);
                v.reverse();
                
                let mut rtf = Rtf::new(&sys, Some(si.iter()
                    .map(|&si| Into::<<T as ComplexOp<X>>::Output>::into(si)*v[v.len() - 1])
                    .collect()
                ));
                let mut v: Vec<_> = rtf.filter_mut(v);
                v.reverse();

                let mut v = v[lrefl..lrefl + xl].to_vec();
                y.append(&mut v);
                v
            });
            let mut y = y.into_iter();
            x.map_to_owned(|_| y.next().unwrap())
        })
    }
}

#[cfg(test)]
mod test
{
    use array_math::ArrayOps;
    use linspace::LinspaceArray;
    use rand::distributions::uniform::SampleRange;

    use crate::{plot, Butter, FiltFilt, FilterGenPlane, FilterGenType, Tf};

    #[test]
    fn test()
    {
        let h = Tf::butter(2, [50.0], FilterGenType::LowPass, FilterGenPlane::Z { sampling_frequency: Some(1000.0) })
            .unwrap();

        const N: usize = 64;
        let mut rng = rand::thread_rng();
        let x: [_; N] = ArrayOps::fill(|_| (-1.0..1.0).sample_single(&mut rng));

        let y = h.filtfilt(x);

        let t: [_; N] = (0.0..N as f64).linspace_array();

        plot::plot_curves("x(t), y(t)", "plots/xy_t_filtfilt.png", [&t.zip(x), &t.zip(y)])
            .unwrap()
    }
}