use core::ops::{AddAssign, Mul, MulAssign};

use ndarray_linalg::{Lapack, QRInto, SVDInto, Solve, SVD};
use num::{complex::ComplexFloat, traits::FloatConst, Complex, Float, One};
use option_trait::{Maybe, StaticMaybe};
use ndarray::{Array1, Array2};

use crate::{InvFreqMethod, List, MaybeList, System, Tf, TruncateIm};

pub trait InvFreqS<H, S, W, HFW, NB, NA>: System + Sized
where
    H: ComplexFloat<Real = <Self::Domain as ComplexFloat>::Real>,
    S: ComplexFloat<Real = <Self::Domain as ComplexFloat>::Real>,
    HFW: List<(H, S, W)>,
    W: Maybe<<Self::Domain as ComplexFloat>::Real>,
    NB: Maybe<usize>,
    NA: Maybe<usize>
{
    fn invfreqs<ZB>(
        hfw: HFW,
        nb: NB,
        na: NA,
        zb: ZB,
        norm: bool,
        method: InvFreqMethod
    ) -> (Self, <Self::Domain as ComplexFloat>::Real)
    where
        ZB: Maybe<NB>;
}

impl<T, B, A, H, S, W, HFW> InvFreqS<H, S, W, HFW, <B::Width as StaticMaybe<usize>>::Opposite, <A::Width as StaticMaybe<usize>>::Opposite> for Tf<T, B, A>
where
    T: Float + FloatConst + Lapack<Real = T> + AddAssign + 'static,
    B: List<T>,
    A: MaybeList<T>,
    H: ComplexFloat<Real = T>,
    S: ComplexFloat<Real = T> + Into<Complex<T>>,
    HFW: List<(H, S, W)>,
    W: Maybe<T> + Clone,
    Complex<T>: Mul<T, Output = Complex<T>> + Mul<H, Output = Complex<T>> + Mul<Output = Complex<T>> + MulAssign,
    <B::Width as StaticMaybe<usize>>::Opposite: Sized,
    <A::Width as StaticMaybe<usize>>::Opposite: Sized,
    Vec<T>: TryInto<B> + TryInto<A>,
    [(); B::WIDTH - 1]:,
    [(); A::WIDTH - 1]:
{
    fn invfreqs<ZB>(
        hfw: HFW,
        nb: <B::Width as StaticMaybe<usize>>::Opposite,
        na: <A::Width as StaticMaybe<usize>>::Opposite,
        zb: ZB,
        norm: bool,
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

        let hfw: Vec<_> = hfw.into_vec()
            .into_iter()
            .map(|(h, f, w)| (h, f, w.into_option().unwrap_or(one)))
            .collect();
        let nf = hfw.len();

        let mut ruu = Array2::from_shape_fn((mb, mb), |(_, _)| T::zero());
        let mut ryy = Array2::from_shape_fn((na, na), |(_, _)| T::zero());
        let mut ryu = Array2::from_shape_fn((na, mb), |(_, _)| T::zero());
        let mut pu = vec![T::zero(); mb];
        let mut py = vec![T::zero(); na];

        let mut s: Vec<Complex<T>> = hfw.iter()
            .map(|(_, s, _)| (*s).into())
            .collect();
        let fmax = if norm && n > 5 && let Some(fmax) = hfw.iter()
            .map(|(_, s, _)| (*s).abs())
            .reduce(Float::max)
            && fmax > T::from(1e6).unwrap()
        {
            for s in s.iter_mut()
            {
                *s /= fmax
            }
            Some(fmax)
        }
        else
        {
            None
        };

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

        let mut b: Vec<_> = vec![T::zero(); zb].into_iter()
            .chain(theta[..mb - zb].iter()
                .map(|&b| b.into())
            ).collect();
        let mut a: Vec<_> = core::iter::once(T::one())
            .chain(theta[mb - zb..mb + na - zb].iter()
                .map(|&a| a.into())
            ).collect();

        if let Some(fmax) = fmax
        {
            let zk: Vec<_> = (0..m).rev()
                .map(|i| Float::powi(fmax, i as i32))
                .collect();
            for k in (zb..mb).rev()
            {
                b[k] /= zk[k]
            }
            for k in (0..ma).rev()
            {
                a[k] /= zk[k]
            }
        }
        b.reverse();
        a.reverse();

        let norm = a[0];
        for b in b.iter_mut()
        {
            *b /= norm
        }
        for a in a.iter_mut()
        {
            *a /= norm
        }

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
    use num::Complex;

    use crate::{plot, Butter, FilterGenPlane, FilterGenType, FreqS, InvFreqMethod, InvFreqS, Tf};

    #[test]
    fn test()
    {
        let h = Tf::butter(2, [220.0], FilterGenType::LowPass, FilterGenPlane::S)
            .unwrap();

        const N: usize = 1024;
        let f: [_; N] = core::array::from_fn(|i| (1 + i) as f64/N as f64*800.0);
        let s = f.map(|f| Complex::new(0.0, f));

        let h_f = h.freqs(s);

        let hfw = h_f.zip(s).map(|(h, s)| (h, s, ()));
        let (h2, _s) = Tf::<f64, Vec<_>, Vec<_>>::invfreqs(hfw, h.b.len() - 1, h.a.len() - 1, (), true, InvFreqMethod::MLS);

        let h2_f = h2.freqs(s);

        plot::plot_curves("H(jw)", "plots/h_s_invfreqs.png", [
            &f.zip(h_f.map(|h| h.norm())),
            &f.zip(h2_f.map(|h| h.norm()))
        ]).unwrap();

        println!("h = {:?}", h);
        println!("h2 = {:?}", h2);
    }
}