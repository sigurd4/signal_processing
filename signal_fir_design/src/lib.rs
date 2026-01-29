use core::{f64::consts::TAU, ops::Neg};

use array_trait::length;
use bulks::{ArrayChunks, Bulk, CollectNearest, Enumerate, FlatMap, IntoBulk, Map, RepeatN};
use num_complex::ComplexFloat;
use num_traits::{Float, FloatConst, Zero};
use signal_windows::Window;

struct FreqIndex(usize);

struct RadPerS<F>(F)
where
    F: Float;

struct Rad<F>(F)
where
    F: Float;

struct Hz<F>(F)
where
    F: Float;

type TruncatedFir<F, H, N> = <Map<Map<Enumerate<RepeatN<(), N>>, impl Fn((usize, ())) -> F>, H> as CollectNearest>::Nearest;

pub fn discrete_n_to_normf<F>(i: usize, n: usize) -> F
where
    F: Float + FloatConst
{
    F::from(i).unwrap()/F::from(n).unwrap()*F::TAU()
}

pub fn truncated_fir<F, H, N>(h: H, n: N) -> TruncatedFir<F, H, N>
where
    F: Float + FloatConst,
    H: Fn(F) -> Complex<F>,
    N: LengthValue
{
    let nf = F::from(length::value::len(n)).unwrap();
    let mut x = bulks::repeat_n((), n)
        .enumerate()
        .map(|i| F::from(i).unwrap()/nf*F::TAU())
        .map(h)
        .collect_nearest();
    x.ifft();
    x
}

pub fn windowed_fir<F, H, N, W>(h: H, n: N, w: W) -> Windowed<W, <TruncatedFir<F, H, N> as IntoBulk>::IntoBulk>
where
    F: Float + FloatConst,
    H: Fn(F) -> Complex<F>,
    N: LengthValue,
    W: Window
{
    truncated_fir(h, n)
        .into_bulk()
        .window(w)
}

pub fn fir_lpf_to_hpf<X>(x: X) -> FlatMap<ArrayChunks<X::IntoBulk, 2>, fn([X::Item; 2]) -> [X::Item; 2]>
where
    X: IntoBulk<Item: ComplexFloat>
{
    fn neg_odd<T>([x1, x2]: [T; 2]) -> [T; 2]
    where
        T: Neg<Output = T>
    {
        [x1, -x2]
    }

    x.into_bulk()
        .array_chunks::<2>()
        .flat_map(neg_odd)
}

pub fn fir_lpf_to_bpf(x: &mut [f64], f_c: f64)
{
    let n = x.len();
    for (i, x) in x.iter_mut()
        .enumerate()
    {
        *x = discrete_n_to_normf(i, n).cos();
    }
}

pub fn remez<const N: usize>(
    f_edges: [(f64, f64); N - 1],
    attenuation: [f64; N], // dB : 20 log10(attenuation),
    ripple: [f64; N], // dB : 20 log10(1 + ripple)
    n_min: usize // n = stopband_ripple/22/(f_stop - f_pass)
)
{

}

pub fn dft_phase_delay<X>(x: X) -> _
where
    X: IntoBulk<Item: ComplexFloat>
{
    let x = x.into_bulk();
    let n = x.len();
    x.enumerate()
        .map(|(i, x)| -x.arg()/discrete_n_to_normf(i, n))
}

pub fn dft_group_delay(x: &[Complex<f64>]) -> _
{
    let n = x.len();
    let domega = discrete_n_to_normf(1, n);
    
    let phi0 = x[n - 1].arg() - x[0].arg();
    let mut phi_up = phi0;
    let mut phi_down = phi0;
    
    let (x1, x2) = x.split_at(n/2 + 1);

    x1.iter()
        .map(|x| {
            let phi = x.arg();
            let dphi = phi - phi_up;
            phi_up = phi;
            -dphi/domega
        })
        .chain(x2.iter()
            .map(|x| {
                let phi = x.arg();
                let dphi = phi_down - phi;
                phi_down = phi;
                -dphi/domega
            })
        )
}

// linear phase (loosely): phi = alpha + G*omega
// linear phase (strict): phi = G*omega

#[cfg(test)]
mod tests
{
    use super::*;

    #[test]
    fn it_works()
    {
        
    }
}
