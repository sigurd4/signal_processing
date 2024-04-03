use core::ops::{AddAssign, DivAssign, MulAssign, SubAssign};
use std::{f64::{consts::{FRAC_PI_2, PI}, EPSILON}, ops::Range};

use num::{traits::FloatConst, Complex, Float, NumCast, Zero};
use option_trait::MaybeCell;
use rand::{distributions::uniform::SampleUniform, rngs::ThreadRng, Rng};
use array_math::{SliceMath, ArrayOps};

use crate::{FirPmError, FirPmReport, FirPmType, Polynomial, Tf};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum FirType
{
    I,
    II,
    III,
    IV
}

type Real = f64;

#[derive(Debug, Clone, Copy, PartialEq, Default)]
struct LagrangeCoef
{
    x: Real,
    beta: Real,
    gamma: Real
}

#[derive(Debug, Clone, Copy, PartialEq, Default)]
struct Point<T>
where
    T: Float
{
    f: T,
    a: T,
    w: T,
    e: T
}

#[derive(Debug, Clone, Copy, PartialEq, Default)]
struct BandSpec<T>
where
    T: Float
{
    freql: T,
    freqr: T,
    ampl: T,
    ampr: T,
    weightl: T,
    weightr: T,

    ends: usize,
    weightf: bool,
    endp: usize,
    had_removal: usize,
    eav: T,
    epk: T,
    portion0: T,
    portion: T
}

pub enum Response<'a, T, const R: usize, const W: usize>
where
    T: Float
{
    Bands {
        response: [T; R],
        weight: [T; W]
    },
    #[allow(unused)]
    Fn(&'a dyn Fn(T) -> (T, T))
}

const USER_EPSILON: f64 = 1.0e-6;

/// Determine coefs of a 2nd-order polynomial from 3 of its points:
fn poly2<T>(x: [T; 3], y: [T; 3]) -> [T; 3]
where
    T: Float
{
    let [x0, x1, x2] = x;
    let [y0, y1, y2] = y;
    let [a0, a1, a2] = [
        y0/((x0 - x1)*(x0 - x2)),
        y1/((x1 - x2)*(x1 - x0)),
        y2/((x2 - x0)*(x2 - x1))
    ];
    [
        a0 + a1 + a2,
        -(a0*(x1 + x2) + a1*(x0 + x2) + a2*(x0 + x1)),
        a0*x1*x2 + x0*a1*x2 + x0*x1*a2
    ]
}

/// Evaluate a 2nd-order polynomial at given point:
fn poly2val<T>(b: [T; 3], x: T) -> T
where
    T: Float
{
    let [a, b, c] = b;
    (a*x + b)*x + c
}

const C1: f64 = 0.1235;
const C2: f64 = -0.8;
const C3: f64 = 1.41431;
const CA: f64 = 0.214437;
const CB: f64 = 0.006742;
const CC: f64 = 0.0;

/// Calculate warp in arbitrary units:
fn warp<T>(x: T) -> T
where
    T: Float
{
    let c2 = T::from(C2).unwrap();
    let t = T::from(C1).unwrap() + c2*x;
    let zero = T::zero();
    if x < zero
    {
        zero
    }
    else if x < T::one()
    {
        poly2val([CA, CB, CC].map(|c| T::from(c).unwrap()), x)
    }
    else
    {
        x - if t > T::from(-38.0).unwrap() {t.exp()/c2} else {zero} - T::from(C3).unwrap()
    }
}

fn iwarp<T>(y: T) -> T
where
    T: Float
{
    let zero = T::zero();
    if y < zero
    {
        return zero
    }
    let one = T::one();
    if y < warp(one)
    {
        return (T::from(-CB).unwrap() + (T::from(CB*CB).unwrap() - T::from(4.0*CA).unwrap()*(T::from(CC).unwrap() - y)).sqrt())/T::from(2.0*CA).unwrap();
    }

    let mut x1 = y + T::from(C3).unwrap();
    loop
    {
        let x = x1;
        let t = T::from(C1).unwrap() + T::from(C2).unwrap()*x;
        x1 = x - (warp(x) - y)/(one - if t > T::from(-38.0).unwrap() {t.exp()} else {zero});
        if !((x1 - x).abs() > T::from(1e-5).unwrap())
        {
            break;
        }
    }
    x1
}

fn rand<T>(x: T, rng: &mut ThreadRng) -> T
where
    T: Float + SampleUniform
{
    rng.gen_range(-x..=x)
}

fn warp_space<T>(p: &mut [Point<T>], m: T, f: Range<T>, density: usize, i0: usize, step: isize, rng: &mut ThreadRng) -> usize
where
    T: Float + SampleUniform
{
    let n = NumCast::from((m*T::from(density).unwrap()).ceil()).unwrap();
    let df = f.end - f.start;

    let mut s = i0;
    p[s].f = f.start;
    if step < 0 && s < (-step) as usize
    {
        return 1 + n
    }
    if step >= 0 {s += step as usize} else {s -= (-step) as usize}
    if n > 0
    {
        let scale = df/warp(T::from(n).unwrap()/T::from(density).unwrap());
        for i in 1..n
        {
            p[s].f = f.start + scale*warp((T::from(i).unwrap() + rand(T::from(USER_EPSILON).unwrap(), rng))/T::from(density).unwrap());
            if step < 0 && s < (-step) as usize
            {
                return 1 + n
            }
            if step >= 0 {s += step as usize} else {s -= (-step) as usize}
        }
        p[s].f = f.end
    }
    1 + n
}

fn warp_space_r<T>(p: &mut [Point<T>], m: T, f: Range<T>, density: usize, rng: &mut ThreadRng) -> usize
where
    T: Float + SampleUniform
{
    warp_space(p, m, f, density, 0, 1, rng)
}

fn warp_space_l<T>(p: &mut [Point<T>], m: T, f: Range<T>, density: usize, rng: &mut ThreadRng) -> usize
where
    T: Float + SampleUniform
{
    warp_space(p, m, f.end..f.start, density, NumCast::from((m*T::from(density).unwrap()).ceil()).unwrap(), -1, rng)
}

fn warp_space_rl<T>(p: &mut [Point<T>], m: T, f: Range<T>, density: usize, rng: &mut ThreadRng) -> usize
where
    T: Float + SampleUniform
{
    let half = T::from(0.5).unwrap();
    let df1 = (f.end - f.start)*half;
    let m1 = m*half;
    let n = warp_space_r(p, m1, f.start..(f.start + df1), density, rng);
    warp_space_l(&mut p[n - 1..], m1, (f.start + df1)..f.end, density, rng);
    n*2 - 1
}

fn apportion_space<T>(b: &mut [BandSpec<T>], num_bands: usize, n: usize)
where
    T: Float + DivAssign + AddAssign
{
    let zero = T::zero();
    for _ in 0..1024
    {
        let mut av = zero;
        for &b in b.iter().take(num_bands)
        {
            av += warp(b.portion*T::from(n).unwrap()/T::from(b.ends).unwrap())
                /(b.portion0/T::from(b.ends).unwrap());
        }
        av /= T::from(num_bands).unwrap();

        let mut changed = 0;
        let mut sum = zero;
        for b in b.iter_mut().take(num_bands)
        {
            let t = iwarp(av*b.portion0/T::from(b.ends).unwrap())/T::from(n).unwrap()*T::from(b.ends).unwrap();
            changed += ((t - b.portion).abs() > T::from(USER_EPSILON).unwrap()) as usize;
            b.portion = t;
            sum += t;
        }

        for b in b.iter_mut().take(num_bands)
        {
            b.portion /= sum;
        }

        if changed == 0
        {
            break
        }
    }
}

fn type_mod(f: Real, t: FirType) -> Real
{
    match t
    {
        FirType::I => 1.0,
        FirType::II => (f*FRAC_PI_2).cos(),
        FirType::III => (f*PI).sin(),
        FirType::IV => (f*FRAC_PI_2).sin(),
    }
}

fn resp_fn_mod<T, F>(
    resp_fn: F,
    t: FirType,
    b: &[BandSpec<T>],
    len: usize,
    p: &mut [Point<T>],
    check: bool
) -> Result<(), FirPmError>
where
    T: Float + MulAssign + DivAssign,
    F: Fn(T) -> (T, T)
{
    for p in p.iter_mut().take(len)
    {
        let g = p.f;
        let (ag, wg) = resp_fn(g);
        p.a = ag;
        p.w = wg;
    }
    
    if check
    {
        let bignum = T::from(1e10).unwrap();
        let smallnum = T::from(1e-10).unwrap();
        let mut max_a = T::zero();
        for p in p.iter().take(len)
        {
            let d = p.a.abs();
            let w = p.w;
            max_a = max_a.max(d);
            if d > bignum
            {
                return Err(FirPmError::AmplitudeOutOfRange)
            }
            if w < smallnum || w > bignum
            {
                return Err(FirPmError::WeightsOutOfRange)
            }
        }
        if max_a < smallnum
        {
            return Err(FirPmError::AmplitudeOutOfRange)
        }
    }

    for p in p.iter_mut().take(len)
    {
        let f = p.f;
        let b = set_f(b, f);
        let t = T::from(type_mod(NumCast::from(f).unwrap(), t)).unwrap();
        p.a /= t;
        p.w *= t;
        if b.weightf
        {
            p.w /= f
        }
    }
    Ok(())
}

/// Calculate the estimated amplitude response at a single, given
/// frequency using precalculated Lagrange interpolation coefficients.
fn a<T>(coefs: &[LagrangeCoef], r: usize, f: T) -> Real
where
    T: Float
{
    let mut n = 0.0;
    let mut d = 0.0;
    let x = (PI*<f64 as NumCast>::from(f).unwrap()).cos();
    for coef in coefs.iter().take(r)
    {
        let mut t = x - coef.x;
        if t.abs() < EPSILON
        {
            return coef.gamma
        }
        t = coef.beta/t;
        d += t;
        n += coef.gamma*t;
    }
    n/d
}

fn set_f<T>(b: &[BandSpec<T>], f: T) -> &BandSpec<T>
where
    T: Float
{
    for b in b
    {
        if b.freqr < f
        {
            continue;
        }
        return b
    }
    if f > T::from(0.5).unwrap()
    {
        &b[b.len() - 1]
    }
    else
    {
        &b[0]
    }
}

fn built_in_resp_fn<T>(b: &[BandSpec<T>]) -> Box<dyn Fn(T) -> (T, T)>
where
    T: Float + 'static
{
    let b = b.to_vec();
    Box::new(move |g| {
        let b = set_f(&b, g);
        let ag = b.ampl + b.ampr*(g - b.freql);
        let wg = b.weightl + b.weightr*(g - b.freql);
        (ag, wg)
    })
}

fn minimality_<T>(p: &[Point<T>], len: usize, threshold: T) -> T
where
    T: Float
{
    let one = T::one();
    let mut max = T::from(1e-9).unwrap() + one;
    for p in p.iter().take(len)
    {
        max = p.w.abs().max(max);
    }
    -threshold - (max - one).log10()
}

fn has_converged<T>(len: usize, peaks: &[Point<T>], minimality: &mut T, minimality_threshold: T, density: usize, density2: usize, stability: T) -> bool
where
    T: Float
{
    *minimality = minimality_(peaks, len, minimality_threshold);
    (*minimality > T::zero() && (density < density2 || stability > T::from(2.0).unwrap())) || stability > T::from(2.7).unwrap()
}

const DEBUG: bool = false;

pub fn mmfir<T, const B2: usize, const R: usize, const W: usize, const RES: bool>(
    n0: usize,
    bands: [T; B2],
    response: Response<'_, T, R, W>,
    filter_class: FirPmType,
    sampling_frequency: Option<T>,
    accuracy: T,
    persistence: T,
    robustness: T,
    target: T
) -> Result<(Tf<T, Vec<T>, ()>, T, MaybeCell<FirPmReport<T>, RES>), FirPmError>
where
    T: Float + FloatConst + Default + AddAssign + SubAssign + MulAssign + DivAssign + SampleUniform + 'static,
    Complex<T>: MulAssign + AddAssign,
    [(); 0 - B2%2]:,
    [(); B2/2 - 1]:,
    [(); B2 - R]:,
    [(); 0 - R % (B2/2)]:,
    [(); B2 - W]:,
    [(); 0 - W % (B2/2)]:,
    [(); RES as usize]:
{
    let num_bands = B2/2;
    let n = n0.max(2);
    if n != n0
    {
        return Err(FirPmError::ZeroOrder)
    }
    let t = match (filter_class != FirPmType::Symmetric, n % 2 != 1)
    {
        (false, false) => FirType::I,
        (false, true) => FirType::II,
        (true, false) => FirType::III,
        (true, true) => FirType::IV
    };
    let r = n/2 + (t == FirType::I) as usize;

    let zero = T::zero();
    let one = T::one();

    let accuracy_c = accuracy.max(zero).min(T::from(7.0).unwrap());
    let density2 = ((5 + (num_bands > 2) as usize) as f64*(1.0 + <f64 as NumCast>::from(accuracy_c).unwrap())).round() as usize;

    let persistence_c = persistence.max(T::from(-3.0).unwrap()).min(T::from(3.0).unwrap());
    let max_iterations = (128.0*1.587f64.powf(NumCast::from(persistence_c).unwrap())).round() as usize;

    let robustness_c = robustness.max(zero).min(T::from(3.0).unwrap());
    let mut density = 3 + (num_bands > 2) as usize + <f64 as NumCast>::from(robustness_c).unwrap().round() as usize;

    let mut done_init = false;
    let mut dolg = 1;
    let mut space_length = 0;
    let mut prev_num_peaks = 0;
    let mut delta_1 = 1e-30;
    let mut total_width = zero;

    let max_extras = num_bands + 10;//2;
    let max_peaks = r + 1 + max_extras;

    let mut coefs = vec![LagrangeCoef::default(); r + 1];
    let mut peaks = vec![Point::default(); max_peaks];
    let mut prev_peaks = vec![Point::default(); max_peaks];

    let mut band_specs = <[BandSpec<T>; B2/2]>::fill(|i| {
        BandSpec {
            freql: bands[i*2],
            freqr: bands[i*2 + 1],
            ..Default::default()
        }
    });
    let two = one + one;
    let half = two.recip();
    if let Some(fs) = sampling_frequency
    {
        if !(fs > zero) || !fs.is_finite()
        {
            return Err(FirPmError::InvalidSamplingFrequency)
        }
        let nyq = fs*half;
        for b in band_specs.iter_mut()
        {
            b.freql /= nyq;
            b.freqr /= nyq;
        }
    }
    if band_specs[0].freql < zero || band_specs[num_bands - 1].freqr > one
    {
        return Err(FirPmError::EdgesOutOfRange)
    }
    let eps = T::epsilon();
    let onemeps = one - eps;
    let user_eps = T::from(USER_EPSILON).unwrap();
    let mut f_prev = zero;
    for b in band_specs.iter_mut()
    {
        let df = b.freqr - b.freql;

        if df < zero || b.freql < f_prev
        {
            return Err(FirPmError::EdgesNotNondecreasing)
        }
        if df <= user_eps
        {
            return Err(FirPmError::BandTooNarrow)
        }

        f_prev = b.freqr;
        total_width += df;
    }

    let (resp_fn, is_resp_fn_built_in) = match response
    {
        Response::Bands { response, weight } => {
            for i in 0..num_bands
            {
                band_specs[i].ampl = response[i*2*(R/num_bands)/2];
                band_specs[i].ampr = response[(i*2 + 1)*(R/num_bands)/2];
                band_specs[i].weightl = weight[i*2*(W/num_bands)/2];
                band_specs[i].weightr = weight[(i*2 + 1)*(W/num_bands)/2];
            }
            for b in band_specs.iter_mut()
            {
                let df = b.freqr - b.freql;
        
                if filter_class == FirPmType::Differentiator
                {
                    b.ampl *= -one;
                    b.ampr *= -one;
                }
                b.weightf = filter_class == FirPmType::Differentiator && (b.ampl.abs() > user_eps || b.ampr.abs() > user_eps);

                if b.weightf
                {
                    b.weightl *= two;
                    b.weightr *= two;
                }

                b.ampr = (b.ampr - b.ampl)/df;
                b.weightr = (b.weightr - b.weightl)/df;
            }
            (
                built_in_resp_fn(&band_specs),
                true
            )
        },
        Response::Fn(f) => (Box::new(f) as Box<dyn Fn(T) -> (T, T)>, false),
    };

    // Avoid dividing by type_mod(0, t) = 0:
    if t == FirType::III || t == FirType::IV
    {
        let b = &mut band_specs[0];
        let f = eps;
        if b.freql < f
        {
            let g = b.freql;
            let (ag, _wg) = resp_fn(g);
            if ag.abs() > user_eps
            {
                return Err(FirPmError::NonZeroDC)
            }
            if is_resp_fn_built_in
            {
                b.ampl += b.ampr*(f - b.freql);
                b.weightl += b.weightr*(f - b.freql);
            }
            total_width -= f - b.freql;
            b.freql = f;
        }
    }

    // Avoid dividing by type_mod(1, t) = 0:
    if t == FirType::II || t == FirType::III
    {
        let b = &mut band_specs[num_bands - 1];
        let f = onemeps;
        if b.freqr > f
        {
            let g = b.freqr;
            let (ag, _wg) = resp_fn(g);
            if ag.abs() > user_eps
            {
                return Err(FirPmError::NonZeroNyq)
            }

            total_width -= b.freqr - f;
            b.freqr = f;
        }
    }

    // Separate any contiguous bands
    let mut prev_f = T::neg_infinity();
    for b in band_specs.iter_mut()
    {
        let f = b.freql - eps;
        if prev_f > f
        {
            total_width -= b.freqr - f;
            b.freqr = f;
        }
        prev_f = b.freqr;
    }

    #[derive(PartialEq, Eq)]
    enum MmFirResult
    {
        Success,
        Ongoing,
        GaveUp1,
        GaveUp
    }

    // Usually, start with a lower density, then switch to the final value:
    let density2 = density.max(density2);
    let mut result = MmFirResult::Success;
    let mut iterations = 0;
    //let mut fes = 0;
    let mut extras;
    let mut stability;
    let mut minimality = zero;
    let mut space0: Vec<Point<T>> = vec![];
    let mut rng = rand::thread_rng();
    while result == MmFirResult::Success && density <= density2
    {
        // Map current density to a convergence minimality threshold:
        let minimality_threshold = if density < density2
        {
            density as f64*0.35 - 0.45
        }
        else
        {
            (density + 35) as f64*0.05
        };
        if DEBUG
        {
            println!("Minimality threshold: {minimality_threshold}");
        }
        let minimality_threshold = T::from(minimality_threshold).unwrap();

        // Allocate & populate the analysis frequency space
        {
            for b in band_specs.iter_mut()
            {
                let df = b.freqr - b.freql;
                b.portion = NumCast::from(df/total_width).unwrap();
                b.portion0 = b.portion;
                b.ends = 2 - (b.freql <= eps || b.freqr >= onemeps) as usize;
            }
            apportion_space(&mut band_specs, num_bands, r + 1);

            // Determine # of points in analysis space:
            for b in band_specs.iter_mut()
            {
                let m = T::from(r + 1).unwrap()*b.portion;
                if b.freql <= eps || b.freqr >= onemeps
                {
                    space_length += 1 + <usize as NumCast>::from((m*T::from(density).unwrap()).ceil()).unwrap();
                }
                else
                {
                    space_length += 2*(1 + <usize as NumCast>::from((m*half*T::from(density).unwrap()).ceil()).unwrap()) - 1;
                }
                b.endp = space_length;
            }
            
            // Allocate space and calculate frequencies:
            space0 = vec![Point::default(); space_length + 3];
            let space1 = &mut space0[1..];
            let mut j = 0;
            for b in band_specs.iter_mut()
            {
                let m = T::from(r + 1).unwrap()*b.portion;
                let el = b.freql <= eps;
                let er = b.freqr >= onemeps;
                if el
                {
                    j += warp_space_l(&mut space1[j..], m, b.freql..b.freqr, density, &mut rng);
                }
                else if er
                {
                    j += warp_space_r(&mut space1[j..], m, b.freql..b.freqr, density, &mut rng);
                }
                else
                {
                    j += warp_space_rl(&mut space1[j..], m, b.freql..b.freqr, density, &mut rng);
                }
            }
            resp_fn_mod(&resp_fn, t, &band_specs, space_length, &mut space0, !done_init)?;
        }

        // Initial 'guess' distributes peaks evenly through warped space.  The
        // offset of half a step at each end facilitates longer filters:
        if !done_init
        {
            for i in 0..=r
            {
                peaks[i].f = space0[1 + ((i as f64 + 0.5)/(r + 1) as f64*(space_length - 1) as f64).round() as usize].f;
            }
        }
        done_init = true;

        let mut delta;
        result = MmFirResult::Ongoing;
        // Perform the Remez exchange (until break, below):
        loop
        {
            if dolg != 0
            {
                resp_fn_mod(&resp_fn, t, &band_specs, r + 1, &mut peaks, false)?;

                for i in 0..=r
                {
                    coefs[i].x = (PI*<Real as NumCast>::from(peaks[i].f).unwrap()).cos();
                }

                let mut numer = 0.0; //?
                let mut denom = 0.0; //?
                let mut sgn = -1i8;
                for i in 0..=r
                {
                    let mut t = 1.0;
                    let mut t1 = t;
                    for j in 0..=r
                    {
                        t1 = t;
                        t *= 2.0*(coefs[i].x - coefs[j].x) + ((i == j) as u8) as f64
                    }
                    if t1 == 0.0
                    {
                        t1 = USER_EPSILON
                    }
                    if t == 0.0
                    {
                        t = USER_EPSILON
                    }
                    /*if t1 == 0.0 || t == 0.0
                    {
                        return Err(FirPmError::NumericalError)
                    }*/
                    coefs[i].beta = 1.0/t1;
                    numer += <Real as NumCast>::from(peaks[i].a).unwrap()/t;
                    sgn = -sgn;
                    denom += sgn as f64/(t*<Real as NumCast>::from(peaks[i].w).unwrap());
                }
                if numer == 0.0
                {
                    numer = USER_EPSILON;
                }
                if denom == 0.0
                {
                    denom = USER_EPSILON;
                }
                /*if numer == 0.0 || denom == 0.0
                {
                    return Err(FirPmError::NumericalError)
                }*/
                delta = numer/denom;
                delta_1 = 1.0/delta;

                sgn = -1;
                for i in 0..r
                {
                    sgn = -sgn;
                    coefs[i].gamma = <Real as NumCast>::from(peaks[i].a).unwrap() - sgn as Real*delta/<Real as NumCast>::from(peaks[i].w).unwrap();
                }
            }

            // Stop Remez here if converged or no iterations remain:
            if result == MmFirResult::Success
            {
                break
            }
            if iterations == max_iterations
            {
                result = if density < density2
                {
                    MmFirResult::GaveUp1
                }
                else
                {
                    MmFirResult::GaveUp
                };
                break
            }
            iterations += 1;

            // Evaluate the normalised (i.e. converges to [-1,1]) error function:
            let mut i = 0;
            for b in band_specs.iter()
            {
                for j in i..b.endp
                {
                    let p = &mut space0[j + 1];
                    p.e = T::from(delta_1*(<Real as NumCast>::from(p.w).unwrap()*(<Real as NumCast>::from(p.a).unwrap() - a(&coefs, r, p.f)))).unwrap()
                }
                i = b.endp;
            }

            //fes += space_length;

            // Find and store all local peaks in error magnitude:
            let mut num_peaks = 0;
            for i in 0..space_length
            {
                if (space0[i + 2].e - space0[i + 1].e).signum() != (space0[i + 1].e - space0[i].e).signum()
                {
                    let mut f = space0[i + 1].f;
                    // Approx. continuous-space peak (by interpolation in discrete-space):
                    let b = set_f(&band_specs, f);
                    // Band-edge back-off
                    if (i + (f == b.freql) as usize) < ((f == b.freqr) as usize)
                    {
                        continue;
                    }
                    let j = i + (f == b.freql) as usize - (f == b.freqr) as usize;
                    let [a, b, c] = poly2([space0[j].f, space0[j + 1].f, space0[j + 2].f], [space0[j].e, space0[j + 1].e, space0[j + 2].e]);
                    // Freq. at which 1st derivative of poly is 0.
                    let fp = b/a*(-half);
                    let ok = j == i || ((space0[j + 2].e - space0[j + 1].e).signum()*(space0[j + 1].e - space0[j].e).signum() > zero && fp >= space0[1 + i - (j <= i) as usize].f && fp <= space0[1 + i + (j >= i) as usize].f);
                    if ok
                    {
                        f = fp
                    }
                    let e = if ok
                    {
                        poly2val([a, b, c], f)
                    }
                    else
                    {
                        space0[i + 1].e
                    };

                    // Store the peak: But avoid twin-peaks in this case:
                    if num_peaks == 0 || f > peaks[num_peaks - 1].f
                    {
                        // Likely due to numerical error.
                        if num_peaks == max_peaks
                        {
                            /*peaks.push(Point::default());
                            max_peaks += 1;*/
                            return Err(FirPmError::TooManyPeaks)
                        }
                        peaks[num_peaks].w = e;
                        peaks[num_peaks].e = e;
                        peaks[num_peaks].f = f;
                        num_peaks += 1;
                    }
                }
            }

            const FILE_DUMP: bool = false;
            // Dump arrays to files for debugging:
            if FILE_DUMP
            {
                todo!()
            }

            // Check that there are at least R+1 peaks:
            if num_peaks < r + 1
            {
                let f = space0[1].f;
                let e = space0[1].e;
                peaks[num_peaks].w = e;
                peaks[num_peaks].e = e;
                peaks[num_peaks].f = f;
                num_peaks += 1;
            }
            if num_peaks < r + 1
            {
                let f = space0[space_length].f;
                let e = space0[space_length].e;
                peaks[num_peaks].w = e;
                peaks[num_peaks].e = e;
                peaks[num_peaks].f = f;
                num_peaks += 1;
            }
            if num_peaks < r + 1
            {
                // Perhaps due to numerical error.
                return mmfir(n0 - 1, bands, response, filter_class, sampling_frequency, accuracy, persistence, robustness, target)
                    .map(|(mut h, m, r)| {
                        h.b.resize(n, zero);
                        (h, m, r)
                    })
                //return Err(FirPmError::TooFewPeaks)
            }
            extras = num_peaks - (r + 1);

            // Before discarding any peaks, check freq. stability of the entire set:
            stability = if num_peaks == prev_num_peaks
            {
                let mut max = T::from(1e-9).unwrap();
                for i in 1..num_peaks
                {
                    let m = one - (prev_peaks[i].f - prev_peaks[i - 1].f)/(peaks[i].f - peaks[i - 1].f);
                    max = max.max(m.abs())
                }
                -max.log10()
            }
            else
            {
                T::from(-9.99).unwrap()
            };

            // If converged, don't change peak-set:
            if extras != 0 && {
                density = density2;
                density != 0
            } && has_converged(num_peaks, &peaks, &mut minimality, minimality_threshold, density, density2, stability)
            {
                for i in 0..num_peaks
                {
                    peaks[i].e = prev_peaks[i].w;
                }
            }

            // If >R+1 peaks, reduce to R+1 by discarding lesser peaks/pairs:
            while num_peaks > r + 1
            {
                let mut n = 1;

                // Find a lesser peak, either overall or at one end:
                let try2 = num_peaks - (r + 1) > 1;
                let mut i = if try2 {1} else {num_peaks - 1};

                // d = index of peak to discard.
                let mut d = 0;
                let mut d_na = 0;
                while d_na == 0 && i < num_peaks
                {
                    if peaks[i].e*peaks[i - 1].e > zero
                    {
                        d_na = i
                    }
                    if peaks[i].e.abs() < peaks[d].e.abs()
                    {
                        d = i;
                    }
                    i += 1;
                }
                // Prefer to discard non-alternating.
                d = if d_na != 0 {d_na} else {d};
                if try2 && d != 0 && d != num_peaks - 1
                {
                    n = 2;
                    if peaks[d - 1].e.abs() < peaks[d + 1].e.abs()
                    {
                        d -= 1
                    }
                }
                num_peaks -= n;
                for _ in 0..n
                {
                    peaks.remove(d);
                    peaks.push(Point::default());
                }
                if DEBUG
                {
                    println!("x{d_na} {d}:{n}");
                }
            }
            prev_peaks = peaks.clone();
            prev_num_peaks = num_peaks;

            if has_converged(r + 1, &peaks, &mut minimality, minimality_threshold, density, density2, stability)
            {
                result = MmFirResult::Success
            }
            if DEBUG
            {
                println!("{iterations} {density} {extras} {:.2} {:.2}", <f64 as NumCast>::from(stability).unwrap(), <f64 as NumCast>::from(minimality).unwrap().max(-9.99));
            }

            dolg += 1;
        }

        dolg = 0;
        density += (density2 - density).max(1);
    }

    if target != zero && T::from(delta_1).unwrap().abs().recip() > target
    {
        return Err(FirPmError::MissedTarget)
    }

    // Generate filter coefficients
    let mut h = vec![zero; n];
    {
        let mut a_ = vec![0.0; n/2 + 1];
        let sgn = match t
        {
            FirType::I => 1i8,
            FirType::II => 1,
            FirType::III => -1,
            FirType::IV => -1,
        };
        let phi = match t
        {
            FirType::I => 0.5,
            FirType::II => 0.5,
            FirType::III => 0.0,
            FirType::IV => 0.0,
        };

        // Sample the final estimated response; modify for filter type:
        for (i, a_) in a_.iter_mut().enumerate().take(n/2 + 1)
        {
            let f = 2.0*i as f64/n as f64;
            *a_ = a(&coefs, r, f)*type_mod(f, t)
        }
        //fes += n/2 + 1;

        // --> time-domain using symmetry-aware IDFT (could also use IFFT):
        a_[n/2] /= (1 + t as u8/3) as f64;
        for i in 0..(n + 1)/2
        {
            let mut s = a_[0]*0.5;
            for (j, a_) in a_.iter().enumerate().take(n/2 + 1).skip(1)
            {
                s += (PI*((n as f64 - 1.0 - 2.0*i as f64)/n as f64*j as f64 + phi)).sin()**a_
            }
            h[n - 1 - i] = T::from(2.0*s/n as f64).unwrap();
            h[i] = T::from(sgn).unwrap()*h[n - 1 - i];
        }
    }

    let report = MaybeCell::from_fn(|| {
        let mut report = FirPmReport {
            fgrid: vec![zero; space_length],
            des: vec![zero; space_length],
            wt: vec![zero; space_length],
            h: vec![Complex::zero(); n.next_power_of_two()/2 + 1],
            error: vec![zero; space_length],
            iextr: vec![0; prev_num_peaks],
            fextr: vec![zero; prev_num_peaks],
        };

        let mut h_pad = h.clone();
        h_pad.resize(n.next_power_of_two(), zero);
        h_pad.real_fft(&mut report.h);

        for i in 0..space_length
        {
            report.fgrid[i] = space0[i + 1].f;
            report.des[i] = space0[i + 1].a;
            report.wt[i] = space0[i + 1].w;
            report.error[i] = space0[i + 1].e;
        }

        for (i, prev_peak) in prev_peaks.iter().enumerate().take(prev_num_peaks)
        {
            let f = prev_peak.f;
            report.fextr[i] = f;
            for j in 0..space_length
            {
                if space0[j + 1].f <= f
                {
                    report.iextr[i] = j;
                    break
                }
            }
        }

        report
    });
    
    Ok((Tf {b: Polynomial::new(h), a: Polynomial::new(())}, T::from(delta_1).unwrap().abs().recip(), report))
}