use core::ops::Range;

use num::{traits::FloatConst, Float};
use option_trait::Maybe;

use crate::{IntoList, List};

pub enum ChirpCurve
{
    Linear,
    Quadratic,
    Logarithmic
}

pub trait Chirp<T, L, N>: IntoList<T, L, N>
where
    T: Float,
    L: List<T>,
    N: Maybe<usize>
{
    fn chirp(self, numtaps: N, frequencies: Range<T>, times: Range<T>, curve: ChirpCurve, phase: T) -> (L::Mapped<T>, L);
}

impl<T, L, R, N> Chirp<T, L, N> for R
where
    T: Float + FloatConst,
    L: List<T>,
    R: IntoList<T, L, N>,
    N: Maybe<usize>
{
    fn chirp(self, n: N, frequencies: Range<T>, times: Range<T>, curve: ChirpCurve, phase: T) -> (L::Mapped<T>, L)
    {
        let t = self.into_list(n);
        
        let f0 = frequencies.start;
        let f1 = frequencies.end;
        let t0 = times.start;
        let t1 = times.end;

        (match curve
        {
            ChirpCurve::Linear => {
                let a = (f1 - f0)/(t1 - t0);
                let b = f0 - a*t0;
                
                t.map_to_owned(|&t| {
                    let f = a*t + b;
                    let w = T::TAU()*f;
                    (w*t + phase).cos()
                })
            },
            ChirpCurve::Quadratic => {
                let a = (f1 - f0)/(t1*t1 - t0*t0);
                let b = f0 - a*t0*t0;

                t.map_to_owned(|&t| {
                    let f = a*t*t + b;
                    let w = T::TAU()*f;
                    (w*t + phase).cos()
                })
            },
            ChirpCurve::Logarithmic => {
                let a = (f1/f0).ln()/(t1 - t0);
                let b = f0/(a*t0).exp();
                    
                t.map_to_owned(|&t| {
                    let f = b*(a*t).exp();
                    let w = T::TAU()*f;
                    (w*t + phase).cos()
                })
            },
        }, t)
    }
}

#[cfg(test)]
mod test
{
    use array_math::ArrayOps;

    use crate::{plot, Chirp, ChirpCurve};

    #[test]
    fn test()
    {
        const N: usize = 1024;
        let t = 0.0..1.0;
        let f = 1.0..10.0;
        let dt = 0.0..1.0;
        let (c_lin, t): (_, _) = t.chirp((), f.clone(), dt.clone(), ChirpCurve::Linear, 0.0);
        let (c_quad, t): (_, [_; N]) = t.chirp((), f.clone(), dt.clone(), ChirpCurve::Quadratic, 0.0);
        let (c_log, t): (_, [_; N]) = t.chirp((), f, dt, ChirpCurve::Logarithmic, 0.0);

        plot::plot_curves("x(t)", "plots/x_t_chirp.png", [&t.zip(c_lin), &t.zip(c_quad), &t.zip(c_log)]).unwrap()
    }
}