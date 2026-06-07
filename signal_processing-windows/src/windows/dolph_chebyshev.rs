use num_traits::{Float, FloatConst};

use crate::WindowFn;

#[derive(Clone, Copy)]
pub struct DolphChebyshev<T>
where
    T: Float
{
    pub alpha: T
}

impl<T> WindowFn<T> for DolphChebyshev<T>
where
    T: Float + FloatConst
{
    type Functor<S> = impl Fn(usize) -> T
    where
        S: FnOnce<(), Output: DerefMut<Target = [T]>>;

    fn window_fn<S>(self, len: usize, _scrach_space: S) -> Self::Functor<S>
    where
        S: FnOnce<(), Output: DerefMut<Target = [T]>>
    {
        let one = T::one();
        let two = one + one;
        let ten = T::from(10u8).unwrap();
        let l = T::from(m).unwrap();
        let t = |x: T| {
            if x <= -one
            {
                let s = one - two*T::from(m % 2).unwrap();
                s*(l*(-x).acosh()).cosh()
            }
            else if x >= one
            {
                (l*x.acosh()).cosh()
            }
            else
            {
                (l*x.acos()).cos()
            }
        };

        let gamma = ten.powf(-self.alpha);
        let beta = (gamma.recip().acosh()/l).cosh();

        let mut w: [Complex<T>; N] = ArrayOps::fill(|i| {
            let i = T::from(i).unwrap();
            let x = beta*(T::PI()*i/(l + one)).cos();
            (t(x)).into()
        });
        let wr = if m % 2 == 0
        {
            w.fft();
            let mut wr = [T::zero(); _];
            let mm = (m + 2)/2;
            for k in 0..mm
            {
                let ww = (w[k]/w[0]).re;
                wr[mm - k - 1] = ww;
                if k + m + 1 - mm < N
                {
                    wr[k + m + 1 - mm] = ww;
                }
            }
            wr
        }
        else
        {
            for (k, w) in w.iter_mut()
                .enumerate()
            {
                *w *= Complex::cis(T::PI()*T::from(k).unwrap()/(l + one))
            }
            w.fft();
            let mut wr = [T::zero(); _];
            let mm = (m + 1)/2 + 1;
            for k in 1..mm
            {
                let ww = (w[k]/w[1]).re;
                wr[mm - k - 1] = ww;
                if k + m + 1 - mm < N
                {
                    wr[k + m + 1 - mm] = ww;
                }
            }
            wr
        };
        |i| wr[i]
    }
}

#[cfg(test)]
mod test
{
    use crate::tests;

    use super::DolphChebyshev;

    #[test]
    fn test()
    {
        tests::plot_window(DolphChebyshev {
            alpha: 5.0
        })
    }
}

#[cfg(test)]
mod test
{
    use core::f64::consts::{PI, TAU};

    use array_math::ArrayOps;
    use linspace::LinspaceArray;

    use crate::{plot, gen::window::{WindowGen, WindowRange}, analysis::FreqZ, systems::Tf};

    use super::DolphChebyshev;

    #[test]
    fn test()
    {
        const N: usize = 1024;
        let w: [_; N/2] = DolphChebyshev {alpha: 5.0}.window_gen((), WindowRange::Periodic);
        let n = (0.0..1.0).linspace_array();

        plot::plot_curves("g(n/N)", "plots/windows/g_n_dolph_chebyshev.png", [&n.zip(w)]).unwrap();

        let (mut w_f, mut omega): ([_; N], _) = Tf::new(w, ()).freqz((), false);
        omega.map_assign(|omega| (omega + PI) % TAU - PI);
        w_f.rotate_right(N/2);
        omega.rotate_right(N/2);
        
        plot::plot_curves("G(e^jw)", "plots/windows/g_f_dolph_chebyshev.png", [&omega.zip(w_f.map(|w| 20.0*w.norm().log10()))]).unwrap();
    }
}