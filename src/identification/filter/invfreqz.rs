use core::ops::{AddAssign, Mul, MulAssign};

use ndarray::{Array1, Array2};
use num::{complex::ComplexFloat, traits::FloatConst, Complex, Float, One};
use option_trait::{Maybe, StaticMaybe};
use ndarray_linalg::{qr::QRInto, Lapack, svd::{SVDInto, SVD}, solve::Solve};

use crate::{quantities::{List, MaybeList}, System, systems::Tf, util::TruncateIm};

// FIXME: implement Steiglitz-McBride iterations
// FIXME: improve numerical stability for high order filters (matlab is a bit better)
// FIXME: modify to accept more argument configurations

pub enum InvFreqMethod
{
    LS,
    TLS,
    MLS,
    QR
}

pub trait InvFreqZ<H, W, HFW, NB, NA>: System + Sized
where
    H: ComplexFloat<Real = <Self::Set as ComplexFloat>::Real>,
    HFW: List<(H, <Self::Set as ComplexFloat>::Real, W)>,
    W: Maybe<<Self::Set as ComplexFloat>::Real>,
    NB: Maybe<usize>,
    NA: Maybe<usize>
{
    fn invfreqz<ZB>(
        hfw: HFW,
        nb: NB,
        na: NA,
        zb: ZB,
        method: InvFreqMethod
    ) -> (Self, <Self::Set as ComplexFloat>::Real)
    where
        ZB: Maybe<NB>;
}

impl<T, B, A, H, W, HFW> InvFreqZ<H, W, HFW, <B::Width as StaticMaybe<usize>>::Opposite, <A::Width as StaticMaybe<usize>>::Opposite> for Tf<T, B, A>
where
    T: Float + FloatConst + Lapack<Real = T> + AddAssign + 'static,
    B: List<T>,
    A: MaybeList<T>,
    H: ComplexFloat<Real = T>,
    HFW: List<(H, T, W)>,
    W: Maybe<T> + Clone,
    Complex<T>: Mul<T, Output = Complex<T>> + Mul<H, Output = Complex<T>> + Mul<Output = Complex<T>> + MulAssign,
    <B::Width as StaticMaybe<usize>>::Opposite: Sized,
    <A::Width as StaticMaybe<usize>>::Opposite: Sized,
    Vec<T>: TryInto<B> + TryInto<A>,
    [(); B::WIDTH - 1]:,
    [(); A::WIDTH - 1]:
{
    fn invfreqz<ZB>(
        hfw: HFW,
        nb: <B::Width as StaticMaybe<usize>>::Opposite,
        na: <A::Width as StaticMaybe<usize>>::Opposite,
        zb: ZB,
        method: InvFreqMethod
    ) -> (Self, T)
    where
        ZB: Maybe<<B::Width as StaticMaybe<usize>>::Opposite>
    {
        let nb = nb.into_option()
            .unwrap_or(B::WIDTH - 1);
        let na = na.into_option()
            .unwrap_or(A::WIDTH - 1);
        let zb = zb.into_option()
            .and_then(|zb| zb.into_option())
            .unwrap_or(0);
        let n = nb.max(na);
        let m = n + 1;
        let ma = na + 1;
        let mb = nb + 1;

        let one = T::one();

        let mut hfw: Vec<_> = hfw.into_vec()
            .into_iter()
            .map(|(h, f, w)| (h, f, w.into_option().unwrap_or(one)))
            .collect();
        let nf = hfw.len();

        let mut ruu = Array2::from_shape_fn((mb, mb), |(_, _)| T::zero());
        let mut ryy = Array2::from_shape_fn((na, na), |(_, _)| T::zero());
        let mut ryu = Array2::from_shape_fn((na, mb), |(_, _)| T::zero());
        let mut pu = vec![T::zero(); mb];
        let mut py = vec![T::zero(); na];

        for (_, f, _) in hfw.iter_mut()
        {
            *f = T::TAU() - (T::TAU() - (*f % T::TAU())) % T::TAU()
        }
        let s: Vec<_> = hfw.iter()
            .map(|(_, f, _)| Complex::cis(-*f))
            .collect();

        for k in 0..nf
        {
            let zk: Vec<_> = (0..m).map(|i| s[k].powi(i as i32))
                .collect();
            let hk = hfw[k].0;
            let ahks = (hk*hk.conj()).re();
            let rk = Array2::from_shape_fn((m, m), |(i, j)| (zk[i]*zk[j])*hfw[k].2);
            let rrk = rk.map(|rk| rk.truncate_im());
            ruu += &rrk.slice(ndarray::s![..mb, ..mb]);
            ryy += &rrk.slice(ndarray::s![1..ma, 1..ma])
                .map(|&rrk| rrk*ahks);
            ryu += &rk.slice(ndarray::s![1..ma, ..mb])
                .map(|&rk| (rk*hk).truncate_im());
            for (pu, &zk) in pu.iter_mut()
                .zip(&zk[..mb])
            {
                *pu += (zk*hk.conj()).truncate_im::<T>()*hfw[k].2
            }
            for (py, &zk) in py.iter_mut()
                .zip(&zk[1..ma])
            {
                *py += zk.truncate_im::<T>()*(hfw[k].2*ahks)
            }
        }

        let mut rr = Array2::from_shape_fn((nf, mb + na), |_| Complex::one());
        let mut zk = s.clone();
        for k in 0..na.min(nb)
        {
            for (i, ((zk, (h, _, _)), s)) in zk.iter_mut()
                .zip(hfw.iter())
                .zip(s.iter())
                .enumerate()
            {
                rr[(i, 1 + k)] = *zk;
                rr[(i, mb + k)] = -*zk**h;
                *zk *= *s;
            }
        }
        for k in na.min(nb)..na.max(nb) - 1
        {
            for (i, ((zk, (h, _, _)), s)) in zk.iter_mut()
                .zip(hfw.iter())
                .zip(s.iter())
                .enumerate()
            {
                if k < nb
                {
                    rr[(i, 1 + k)] = *zk;
                }
                if k < na
                {
                    rr[(i, mb + k)] = -*zk**h;
                }
                *zk *= *s;
            }
        }
        let k = na.max(nb) - 1;
        for (i, (zk, (h, _, _))) in zk.iter_mut()
            .zip(hfw.iter())
            .enumerate()
        {
            if k < nb
            {
                rr[(i, 1 + k)] = *zk;
            }
            if k < na
            {
                rr[(i, mb + k)] = -*zk**h;
            }
        }
        let rr = Array2::from_shape_fn((nf*2, mb + na - zb), |(i, j)| {
            let k = j + zb;
            if i < nf
            {
                rr[(i, k)].re
            }
            else
            {
                rr[(i - nf, k)].im
            }
        });
        let pr = Array2::from_shape_fn((nf*2, 1), |(i, _)| {
            if i < nf
            {
                hfw[i].0.re()
            }
            else
            {
                hfw[i - nf].0.im()
            }
        });
        let rrpr = Array2::from_shape_fn((nf*2, mb + na - zb + 1), |(i, j)| {
            if j == mb + na - zb
            {
                pr[(i, 0)]
            }
            else
            {
                rr[(i, j)]
            }
        });

        let (sign, theta) = match method
        {
            InvFreqMethod::LS => {
                let (_, r) = rrpr.qr_into()
                    .unwrap();
                let r_dim = r.dim();
                (
                    r[(r_dim.0 - 1, r_dim.1 - 1)],
                    r.slice(ndarray::s![..r_dim.0 - 1, ..r_dim.1 - 1])
                        .solve(&r.slice(ndarray::s![..r_dim.0 - 1, r_dim.1 - 1]))
                        .unwrap()
                        .to_vec()
                )
            },
            InvFreqMethod::TLS => {
                let (_, s, v) = rrpr.svd_into(false, true)
                    .unwrap();
                let v = v.unwrap();
                let v = v.t();
                let s_dim = s.dim();
                let v_dim = v.dim();
                (
                    s[s_dim - 1],
                    v.slice(ndarray::s![..v_dim.0 - 1, v_dim.1 - 1])
                        .to_vec()
                        .into_iter()
                        .map(|vk| -vk/v[(v_dim.0 - 1, v_dim.1 - 1)])
                        .collect()
                )
            },
            InvFreqMethod::MLS | InvFreqMethod::QR => {
                let (_, r) = rrpr.qr_into()
                    .unwrap();
                let eb = mb - zb;
                let r_dim = r.dim();
                let (_, s, v) = r.slice(ndarray::s![eb.., eb..])
                    .svd(false, true)
                    .unwrap();
                let v = v.unwrap();
                let v = v.t();
                let s_dim = s.dim();
                let v_dim = v.dim();

                let theta2: Vec<_> = v.slice(ndarray::s![..v_dim.0 - 1, v_dim.1 - 1])
                    .to_vec()
                    .into_iter()
                    .map(|vk| -vk/v[(v_dim.0 - 1, v_dim.1 - 1)])
                    .collect();
                let theta2_a = Array1::from_vec(theta2.clone());
                let theta_s = r.slice(ndarray::s![..eb, eb..r_dim.1 - 1])
                    .dot(&theta2_a);
                let theta1 = r.slice(ndarray::s![..eb, ..eb])
                    .solve(&(r.slice(ndarray::s![..eb, r_dim.1 - 1]).to_owned() - theta_s))
                    .unwrap()
                    .to_vec();
                (
                    s[s_dim - 1],
                    theta1.into_iter()
                        .chain(theta2)
                        .collect()
                )
            }
        };

        let b: Vec<_> = theta[..mb - zb].iter()
            .map(|&b| b.into())
            .collect();
        let a: Vec<_> = core::iter::once(T::one())
            .chain(theta[mb - zb..mb + na - zb].iter()
                .map(|&a| a.into())
            ).collect();

        (
            Tf::new(
                b.try_into()
                    .ok()
                    .unwrap(),
                a.try_into()
                    .ok()
                    .unwrap()
            ),
            sign
        )
    }
}

#[cfg(test)]
mod test
{
    use array_math::ArrayOps;

    use crate::{plot, gen::filter::{Butter, FilterGenPlane, FilterGenType}, identification::filter::{InvFreqMethod, InvFreqZ}, analysis::RealFreqZ, systems::Tf};

    #[test]
    fn test()
    {
        let h = Tf::butter(2, [0.5], FilterGenType::LowPass, FilterGenPlane::Z { sampling_frequency: None })
            .unwrap();

        const N: usize = 1024;
        let (h_f, w): ([_; N], _) = h.real_freqz(());

        let hfw = h_f.zip(w).map(|(h, w)| (h, w, ()));
        let (h2, _s) = Tf::<f64, Vec<_>, Vec<_>>::invfreqz(hfw, h.b.len() - 1, h.a.len(), (), InvFreqMethod::LS);

        let (h2_f, _): ([_; N], _) = h2.real_freqz(());

        plot::plot_curves("H(e^jw)", "plots/h_z_invfreqz.png", [
            &w.zip(h_f.map(|h| h.norm())),
            &w.zip(h2_f.map(|h| h.norm()))
        ]).unwrap();

        println!("h = {:?}", h);
        println!("h2 = {:?}", h2);
    }
}