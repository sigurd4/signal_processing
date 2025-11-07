

use array_math::ArrayOps;
use num::{traits::FloatConst, Float};

use crate::generators::window::{WindowGen, WindowRange};

pub struct BlackmanHarris;

impl<T, N> WindowGen<T, N> for BlackmanHarris
where
    T: ComplexFloat,
    N: LengthValue
{
    type Output = impl Bulk<Item = T>;

    fn window_gen(&self, numtaps: N, range: WindowRange) -> Self::Output
    {
        let mut i = 0;
        let n = length::value::len(numtaps);
        let m = match range
        {
            WindowRange::Symmetric => n - 1,
            WindowRange::Periodic => n,
        };
        let a0 = T::from(0.35875).unwrap();
        let a1 = T::from(0.48829).unwrap();
        let a2 = T::from(0.14128).unwrap();
        let a3 = T::from(0.01168).unwrap();
        bulks::repeat_n_with(|| {
            let z1 = (T::TAU()*T::from(i).unwrap()/T::from(m).unwrap()).cos();
            let z2 = (T::TAU()*T::from(i*2).unwrap()/T::from(m).unwrap()).cos();
            let z3 = (T::TAU()*T::from(i*3).unwrap()/T::from(m).unwrap()).cos();
            i += 1;
            a0 - a1*z1 + a2*z2 - a3*z3
        }, numtaps)
    }
}

impl<T, const N: usize> WindowGen<T, [T; N], ()> for BlackmanHarris
where
    T: Float + FloatConst
{
    type Output = [T; N];

    fn window_gen(&self, (): (), r: WindowRange) -> Self::Output
    {
        if N <= 1
        {
            return [T::one(); N]
        }
        
        let m = match r
        {
            WindowRange::Symmetric => N - 1,
            WindowRange::Periodic => N,
        };

        let a0 = T::from(0.35875).unwrap();
        let a1 = T::from(0.48829).unwrap();
        let a2 = T::from(0.14128).unwrap();
        let a3 = T::from(0.01168).unwrap();
        ArrayOps::fill(|i| {
            let z1 = (T::TAU()*T::from(i).unwrap()/T::from(m).unwrap()).cos();
            let z2 = (T::TAU()*T::from(i*2).unwrap()/T::from(m).unwrap()).cos();
            let z3 = (T::TAU()*T::from(i*3).unwrap()/T::from(m).unwrap()).cos();
            a0 - a1*z1 + a2*z2 - a3*z3
        })
    }
}
impl<T> WindowGen<T, Vec<T>, usize> for BlackmanHarris
where
    T: Float + FloatConst
{
    type Output = Vec<T>;

    fn window_gen(&self, n: usize, r: WindowRange) -> Self::Output
    {
        if n <= 1
        {
            return vec![T::one(); n]
        }
        
        let m = match r
        {
            WindowRange::Symmetric => n - 1,
            WindowRange::Periodic => n,
        };

        let a0 = T::from(0.35875).unwrap();
        let a1 = T::from(0.48829).unwrap();
        let a2 = T::from(0.14128).unwrap();
        let a3 = T::from(0.01168).unwrap();
        (0..n).map(|i| {
            let z1 = (T::TAU()*T::from(i).unwrap()/T::from(m).unwrap()).cos();
            let z2 = (T::TAU()*T::from(i*2).unwrap()/T::from(m).unwrap()).cos();
            let z3 = (T::TAU()*T::from(i*3).unwrap()/T::from(m).unwrap()).cos();
            a0 - a1*z1 + a2*z2 - a3*z3
        }).collect()
    }
}

#[cfg(test)]
mod test
{
    use core::f64::consts::{PI, TAU};

    use array_math::ArrayOps;
    use linspace::LinspaceArray;

    use crate::{plot, gen::window::{WindowGen, WindowRange}, analysis::FreqZ, systems::Tf};

    use super::BlackmanHarris;

    #[test]
    fn test()
    {
        const N: usize = 1024;
        let w: [_; N/2] = BlackmanHarris.window_gen((), WindowRange::Symmetric);
        let n = (0.0..1.0).linspace_array();

        plot::plot_curves("g(n/N)", "plots/windows/g_n_blackman_harris.png", [&n.zip(w)]).unwrap();

        let (mut w_f, mut omega): ([_; N], _) = Tf::new(w, ()).freqz((), false);
        omega.map_assign(|omega| (omega + PI) % TAU - PI);
        w_f.rotate_right(N/2);
        omega.rotate_right(N/2);
        
        plot::plot_curves("G(e^jw)", "plots/windows/g_f_blackman_harris.png", [&omega.zip(w_f.map(|w| 20.0*w.norm().log10()))]).unwrap();
    }
}