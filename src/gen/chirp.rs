use core::ops::{Range};

use num::{traits::FloatConst, Float};

use crate::List;

pub enum ChirpCurve
{
    Linear,
    Quadratic,
    Logarithmic
}

pub trait Chirp<T>: List<T>
where
    T: Float
{
    fn chirp(&self, frequencies: Range<T>, times: Range<T>, curve: ChirpCurve, phase: T) -> Self::Mapped<T>;
}

impl<T, L> Chirp<T> for L
where
    T: Float + FloatConst,
    L: List<T>
{
    fn chirp(&self, frequencies: Range<T>, times: Range<T>, curve: ChirpCurve, phase: T) -> Self::Mapped<T>
    {
        let f0 = frequencies.start;
        let f1 = frequencies.end;
        let t0 = times.start;
        let t1 = times.end;

        match curve
        {
            ChirpCurve::Linear => {
                let a = (f1 - f0)/(t1 - t0);
                let b = f0 - a*t0;
                
                self.map_to_owned(|&t| {
                    let f = a*t + b;
                    let w = T::TAU()*f;
                    (w*t + phase).cos()
                })
            },
            ChirpCurve::Quadratic => {
                let a = (f1 - f0)/(t1*t1 - t0*t0);
                let b = f0 - a*t0*t0;

                self.map_to_owned(|&t| {
                    let f = a*t*t + b;
                    let w = T::TAU()*f;
                    (w*t + phase).cos()
                })
            },
            ChirpCurve::Logarithmic => {
                let a = (f1/f0).ln()/(t1 - t0);
                let b = f0/(a*t0).exp();
                    
                self.map_to_owned(|&t| {
                    let f = b*(a*t).exp();
                    let w = T::TAU()*f;
                    (w*t + phase).cos()
                })
            },
        }
    }
}

#[cfg(test)]
mod test
{
    

    use array_math::ArrayOps;
    use linspace::LinspaceArray;

    use crate::{plot, Chirp, ChirpCurve};

    #[test]
    fn test()
    {
        const N: usize = 1024;
        let t: [_; N] = (0.0..1.0).linspace_array();
        let f = 1.0..10.0;
        let dt = 0.0..1.0;
        let c_lin = t.chirp(f.clone(), dt.clone(), ChirpCurve::Linear, 0.0);
        let c_quad = t.chirp(f.clone(), dt.clone(), ChirpCurve::Quadratic, 0.0);
        let c_log = t.chirp(f, dt, ChirpCurve::Logarithmic, 0.0);

        plot::plot_curves("x(t)", "plots/x_chirp.png", [&t.zip(c_lin), &t.zip(c_quad), &t.zip(c_log)]).unwrap()
    }
}