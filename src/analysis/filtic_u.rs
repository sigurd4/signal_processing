use core::ops::DivAssign;

use ndarray::{Array1, Array2};
use ndarray_linalg::{Lapack, Solve};
use num::complex::ComplexFloat;
use array_math::{SliceMath, SliceOps};

use crate::{quantities::MaybeList, systems::Tf, System};

pub trait FiltIcU: System
{
    fn filtic_u(self) -> Option<Vec<Self::Set>>;
}

impl<'a, T, B, A> FiltIcU for Tf<T, B, A>
where
    T: ComplexFloat + DivAssign + Lapack,
    B: MaybeList<T>,
    A: MaybeList<T>
{
    fn filtic_u(self) -> Option<Vec<T>>
    {
        let Tf {mut b, mut a} = Tf::new(
            self.b.into_inner()
                .into_vec_option()
                .unwrap_or_else(|| vec![T::one()]),
            self.a.into_inner()
                .into_vec_option()
                .unwrap_or_else(|| vec![T::one()])
        );

        let nb = b.len();
        let na = a.len();
        let n = nb.max(na);

        if n == 0
        {
            return None
        }

        while let Some(a0) = a.first() && a0.is_zero()
        {
            a.remove(0);
        }
        while let Some(b0) = b.first() && b0.is_zero()
        {
            b.remove(0);
        }
        if let Some(a0) = a.first()
            .copied()
        {
            b.div_assign_all(a0);
            a.div_assign_all(a0);
        }
        else
        {
            return None
        }

        let zero = T::zero();
        a.resize(n, zero);
        b.resize(n, zero);

        let aa = a.rcompanion_matrix();
        let aa = Array2::from_shape_fn((n - 1, n - 1), |(i, j)| aa[(n - 2 - j, n - 2 - i)]);
        let ima = Array2::eye(n - 1) - aa.t();
        let bb = Array1::from_shape_fn(n - 1, |i| b[i + 1] - a[i + 1]*b[0]);
        let zi = match ima.solve(&bb)
        {
            Ok(zi) => zi,
            Err(_) => return None
        }.to_vec();
        
        Some(zi)
    }
}

#[cfg(test)]
mod test
{
    use array_math::ArrayOps;

    use crate::{analysis::FiltIcU, gen::filter::{Butter, FilterGenPlane, FilterGenType}, operations::filtering::Filter, plot, systems::Tf};

    #[test]
    fn test()
    {
        const FS: f64 = 1000.0;

        let h = Tf::butter(2, [100.0], FilterGenType::LowPass, FilterGenPlane::Z { sampling_frequency: Some(FS) })
            .unwrap();

        let w = h.as_view().filtic_u().unwrap();

        println!("{:?}", w);

        const N: usize = 32;
        let x = [0.0; N];
        let y = h.filter(x, w);

        let t = core::array::from_fn(|i| i as f64/FS);

        plot::plot_curves("y(t)", "plots/y_t_filtic_u.png", [&t.zip(y)])
            .unwrap();
    }
}