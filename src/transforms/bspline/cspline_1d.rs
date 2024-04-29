use core::{iter::Sum, ops::{AddAssign, DivAssign, MulAssign, SubAssign}};

use num::{traits::FloatConst, Float};
use option_trait::Maybe;

use crate::{quantities::{ContainerOrSingle, ListOrSingle, Lists, OwnedList, OwnedListOrSingle}, analysis::FiltIC, operations::filtering::Filter, systems::{Sos, Tf}};

pub trait CSpline1d<T>: Lists<T>
where
    T: Float
{
    fn cspline_1d<L>(self, lambda: L) -> Self::Mapped<T>
    where
        L: Maybe<T>;
}

impl<T, X> CSpline1d<T> for X
where
    T: Float + FloatConst + Sum + MulAssign + AddAssign + SubAssign + DivAssign,
    X: Lists<T>,
    X::RowOwned: OwnedList<T>,
    X::RowsMapped<<X::RowOwned as ContainerOrSingle<T>>::Mapped<T>>: Into<X::Mapped<T>>
{
    fn cspline_1d<L>(self, lambda: L) -> X::Mapped<T>
    where
        L: Maybe<T>
    {
        let zero = T::zero();
        let one = T::one();
        let two = one + one;

        let coeff_smooth = |lambda: T| {
            let b = T::from(144u8).unwrap()*lambda;
            let c = T::from(24u8).unwrap()*lambda;
            let a = c*(T::from(3u8).unwrap() + b).sqrt();
            let xi = one - T::from(96u8).unwrap()*lambda + a;
            let xi_sqrt = xi.sqrt();
            let omega = (b - one).sqrt().atan2(xi_sqrt);
            let mut rho = (c - one - xi_sqrt)/c;
            rho = rho*((T::from(48u8).unwrap()*lambda + a)/xi).sqrt();
            (rho, omega)
        };

        let lambda = lambda.into_option()
            .and_then(|lambda| if lambda.is_zero() {None} else {Some(lambda)});
        if let Some(lambda) = lambda
        {
            let (rho, omega) = coeff_smooth(lambda);
            let cs = one - two*rho*omega.cos() + rho*rho;
            
            let hc = |k: isize| {
                let kf = T::from(k).unwrap();
                cs/omega.sin()*rho.powi(k as i32)*(omega*(kf + one)).sin()*kf.max(-one)
            };
            
            let cs2 = cs*cs;
            let rho2 = rho*rho;
            let rho4 = rho2*rho2;
            let c0 = cs2*(one + rho2)/(one - rho2)
                /(one - two*rho2*(two*omega).cos() + rho4);
            let gamma = (one - rho2)/(one + rho2)/omega.tan();

            let hs = |k: isize| {
                let ak = k.abs();
                let akf = T::from(ak).unwrap();
                c0*rho.powi(ak as i32)*((omega*akf).cos() + gamma*(omega*akf).sin())
            };

            self.map_rows_into_owned(|mut x| {
                let xx = x.as_mut_slice();

                let hc0 = hc(0);
                let hc1 = hc(1);
                let zi_2 = hc0*xx[0] + xx.iter()
                    .enumerate()
                    .map(|(k, &x)| hc(k as isize + 1)*x)
                    .sum::<T>();
                let zi_1 = hc0*xx[0] + hc1*xx[1] + xx.iter()
                    .enumerate()
                    .map(|(k, &x)| hc(k as isize + 2)*x)
                    .sum::<T>();
                let a = [one, -two*rho*omega.cos(), rho*rho];
                let ba = Tf::new([cs], a);
                let zi = ba.as_view().filtic([zi_1, zi_2], [zero, zero]);

                let sos = Sos::new([Tf::new([cs, zero, zero], a)]);

                let mut yp = sos.as_view().filter(&xx[2..], zi);
                let _ = yp.pop();
                let _ = yp.pop();
                yp.reverse();
                yp.push(zi_1);
                yp.push(zi_2);

                let zi_2 = xx.iter()
                    .rev()
                    .enumerate()
                    .map(|(k, &x)| (hs(k as isize) + hc(k as isize + 1))*x)
                    .sum::<T>();
                let zi_1 = xx.iter()
                    .rev()
                    .enumerate()
                    .map(|(k, &x)| (hs(k as isize - 1) + hc(k as isize + 2))*x)
                    .sum::<T>();
                let zi = ba.filtic([zi_1, zi_2], [zero, zero]);

                let mut y = sos.filter(yp, zi);
                y.reverse();
                y.push(zi_1);
                y.push(zi_2);

                let mut y = y.into_iter();
                x.map_to_owned(|_| y.next().unwrap())
            }).into()
        }
        else
        {
            let zi = T::from(3u8).unwrap().sqrt() - two;

            self.map_rows_into_owned(|mut x| {
                let k = x.length();

                let mut zik = one;
                let powers: Vec<_> = (0..k).map(|_| {
                    let zikk = zik;
                    zik *= zi;
                    zikk
                }).collect();

                let xx = x.as_mut_slice();

                let xp = xx.iter()
                    .zip(powers)
                    .map(|(&x, p)| x*p)
                    .sum::<T>();

                if k == 1
                {
                    let yplus = xx[0] + zi*xp;
                    let output = zi/(zi - one)*yplus;
                    return x.map_to_owned(|_| output)
                }

                let ba = Tf::new((), [one, -zi]);
                let state = ba.as_view()
                    .filtic([xp], [zero]);
                let mut yplus = ba.as_view()
                    .filter(xx as &[T], state);

                let out_last = zi/(zi - one)*yplus.pop().unwrap();
                yplus.reverse();
                
                let ba = Tf::new([-zi], ba.a.into_inner());
                let state = ba.as_view()
                    .filtic([out_last], [zero]);
                let mut y = ba.filter(yplus, state);
                y.reverse();
                y.push(out_last);

                let mut y = y.into_iter();
                x.map_to_owned(|_| y.next().unwrap()*T::from(6u8).unwrap())
            }).into()
        }
    }
}

#[cfg(test)]
mod test
{
    use array_math::ArrayOps;
    use linspace::LinspaceArray;
    use rand::distributions::uniform::SampleRange;

    use crate::{plot, transforms::bspline::{CSpline1d, CSpline1dEval}};

    #[test]
    fn test()
    {
        const N: usize = 300;
        let mut rng = rand::thread_rng();
        let x: [_; N] = [0.0; N/3].chain([1.0; N/3])
            .chain([0.0; N/3])
            .add_each(core::array::from_fn(|_| (-0.05..0.05).sample_single(&mut rng)));
        let t: [_; N] = (0.0..x.len() as f64).linspace_array();

        let (y, t): (_, [_; N]) = t.cspline_1d_eval((), x.cspline_1d(2.5));

        plot::plot_curves("x[n]", "plots/x_n_cspline_1d.png", [
            &t.zip(x),
            &t.zip(y)
        ]).unwrap();
    }
}