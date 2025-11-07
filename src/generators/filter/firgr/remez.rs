use core::ops::{AddAssign, DivAssign, MulAssign};


use num::{traits::FloatConst, Float, NumCast};

use crate::generators::filter::{FirGrError, FilterClassType};

pub fn pre_remez<T, const B: usize>(
    numtaps: usize,
    mut bands: [T; B*2],
    response: [T; B],
    weight: [T; B],
    filter_type: FilterClassType,
    sampling_frequency: Option<T>,
    max_iter: usize,
    grid_density: usize,
    max_deviation: T
) -> Result<(Vec<T>, T), FirGrError>
where
    T: Float + FloatConst + DivAssign + MulAssign + AddAssign
{
    let zero = T::zero();
    let one = T::one();
    let two = one + one;
    if let Some(fs) = sampling_frequency
    {
        if !(fs > zero) || !fs.is_finite()
        {
            return Err(FirGrError::InvalidSamplingFrequency)
        }
        for f in bands.iter_mut()
        {
            *f /= fs
        }
    }
    else
    {
        for f in bands.iter_mut()
        {
            *f /= two
        }
    }
    if !bands.is_sorted()
    {
        return Err(FirGrError::EdgesNotNondecreasing)
    }
    let half = one/two;
    if bands.iter()
        .any(|f| *f < zero) || bands[0] != zero || bands[B*2 - 1] < half
    {
        return Err(FirGrError::EdgesOutOfRange)
    }
    bands[B*2 - 1] = bands[B*2 - 1].min(half);

    let lgrid = grid_density;
    let dimsize = (numtaps as f32/2.0 + 2.0).ceil() as usize;
    let wrksize = grid_density*dimsize;
    let nfilt = numtaps;
    let jtype = filter_type;
    let nbands = B;

    let edge = &bands;
    let mut h = vec![zero; nfilt];
    let fx = &response;
    let wtx = &weight;

    let mut des = vec![zero; wrksize + 1];
    let mut grid = vec![zero; wrksize + 1];
    let mut wt = vec![zero; wrksize + 1];
    let mut iext = vec![0; dimsize + 1];
    let mut alpha = vec![zero; dimsize + 1];

    let neg = jtype != FilterClassType::Symmetric;
    let nodd = nfilt % 2 == 1;
    let mut nfcns = nfilt/2;
    if nodd && !neg
    {
        nfcns += 1;
    }

    // Setup dense grid
    grid[1] = edge[0];
    let delf = half/(T::from(lgrid).unwrap()*T::from(nfcns).unwrap());
    if neg && edge[0] < delf
    {
        grid[1] = delf;
    }
    let mut j = 1;
    let mut l = 1;
    let mut lband = 1;

    loop
    {
        let fup = edge[l];
        loop
        {
            let temp = grid[j];
            des[j] = eff(temp, fx, lband, jtype);
            wt[j] = wate(temp, fx, wtx, lband, jtype);
            j += 1;
            if j > wrksize
            {
                return Err(FirGrError::TooDense)
            }
            grid[j] = temp + delf;
            if !(grid[j] <= fup)
            {
                break;
            }
        }

        grid[j - 1] = fup;
        des[j - 1] = eff(fup, fx, lband, jtype);
        wt[j - 1] = wate(fup, fx, wtx, lband, jtype);
        lband += 1;
        l += 2;
        if lband > nbands
        {
            break;
        }
        grid[j] = edge[l - 1];
    }
    
    let mut ngrid = j - 1;
    if neg == nodd && grid[ngrid] > half - delf
    {
        ngrid -= 1;
    }

    if !neg
    {
        if !nodd
        {
            for j in 1..=ngrid
            {
                let change = (T::PI()*grid[j]).cos();
                des[j] /= change;
                wt[j] *= change;
            }
        }
    }
    else if !nodd
    {
        for j in 1..=ngrid
        {
            let change = (T::PI()*grid[j]).sin();
            des[j] /= change;
            wt[j] *= change;
        }
    }
    else
    {
        for j in 1..=ngrid
        {
            let change = (T::TAU()*grid[j]).sin();
            des[j] /= change;
            wt[j] *= change;
        }
    }

    let temp = (ngrid - 1) as f64/nfcns as f64;
    for j in 1..=nfcns
    {
        iext[j - 1] = ((j - 1) as f64*temp) as usize + 1;
    }
    iext[nfcns] = ngrid;
    let nm1 = nfcns - 1;
    let nz = nfcns + 1;

    grid[0] = zero;
    wt[0] = weight[0];
    des[0] = response[0];

    let mut dev = zero;
    remez(&mut dev, &des, &mut grid, edge, &wt, ngrid, nbands, &mut iext, &mut alpha, dimsize, nfcns, max_iter, max_deviation)?;

    let quarter = half*half;
    if !neg
    {
        if nodd
        {
            for j in 1..=nm1
            {
                h[j - 1] = half*alpha[nz - j - 1];
            }
            h[nfcns - 1] = alpha[0];
        }
        else
        {
            h[0] = quarter*alpha[nfcns - 1];
            for j in 2..=nm1
            {
                h[j - 1] = quarter*(alpha[nz - j - 1] + alpha[nfcns + 1 - j]);
            }
            h[nfcns - 1] = half*alpha[0] + quarter*alpha[1];
        }
    }
    else if nodd
    {
        h[0] = quarter*alpha[nfcns - 1];
        if nm1 > 0
        {
            h[1] = quarter*alpha[nm1 - 1];
        }
        else
        {
            h[1] = zero;
        }
        for j in 3..=nm1
        {
            h[j - 1] = quarter*(alpha[nz - j - 1] - alpha[nfcns + 2 - j]);
        }
        h[nfcns - 1] = half*alpha[0] - quarter*alpha[2];
        h[nz - 1] = zero;
    }
    else
    {
        h[0] = quarter*alpha[nfcns - 1];
        for j in 2..=nm1
        {
            h[j - 1] = quarter*(alpha[nz - j - 1] - alpha[nfcns + 1 - j]);
        }
        h[nfcns - 1] = half*alpha[0] - quarter*alpha[1];
    }

    for j in 1..=nfcns
    {
        let k = nfilt + 1 - j;
        if !neg
        {
            h[k - 1] = h[j - 1];
        }
        else
        {
            h[k - 1] = -h[j - 1];
        }
    }
    if neg && nodd
    {
        h[nz - 1] = zero;
    }

    Ok((h, dev))
}

fn eff<T>(freq: T, fx: &[T], lband: usize, jtype: FilterClassType) -> T
where T: Float
{
    if jtype != FilterClassType::Differentiator
    {
        return fx[lband - 1];
    }
    fx[lband - 1]*freq
}

fn wate<T>(freq: T, fx: &[T], wtx: &[T], lband: usize, jtype: FilterClassType) -> T
where
    T: Float
{
    if jtype != FilterClassType::Differentiator
    {
        return wtx[lband - 1];
    }
    if fx[lband - 1] >= NumCast::from(0.0001).unwrap()
    {
        return wtx[lband - 1]/freq;
    }
    wtx[lband - 1]
}

fn lagrange_interp<T>(
    k: usize,
    n: usize,
    m: usize,
    x: &[T]
) -> T
where
    T: Float + MulAssign
{
    let one = T::one();
    let two = one + one;
    let mut retval = one;
    let q = x[k - 1];
    for l in 1..=m
    {
        for j in (l..=n).step_by(m)
        {
            if j != k
            {
                retval *= two*(q - x[j - 1])
            }
        }
    }
    let eps = T::epsilon();
    if retval.abs() < eps
    {
        retval = eps.copysign(retval);
    }
    /*if retval == 0.0
    {
        retval = 1.0;
        for l in 1..=m
        {
            for j in (l..=n).step_by(m)
            {
                if j != k
                {
                    retval *= 2.0
                }
            }
        }
    }*/
    retval.recip()
}

fn freq_eval<T>(
    k: usize,
    n: usize,
    grid: &[T],
    x: &[T],
    y: &[T],
    ad: &[T]
) -> T
where
    T: Float + FloatConst + AddAssign
{
    let zero = T::zero();
    let mut d = zero;
    let mut p = zero;
    let xf = (T::TAU()*grid[k]).cos();

    for j in 1..=n
    {
        let c = ad[j - 1] / (xf - x[j - 1]);
        d += c;
        p += c * y[j - 1];
    }

    p/d
}

fn remez<T>(
    dev: &mut T, // Deviation
    des: &[T], // [ngrid + 1] Band responses
    grid: &mut [T], // [ngrid + 1]
    edge: &[T], //[nbands*2]
    wt: &[T], //[ngrid + 1]
    ngrid: usize, // grid size
    nbands: usize, // band count
    iext: &mut [usize], // [nfcns + 2]
    alpha: &mut [T], // [nfcns + 2]
    dimsize: usize,
    nfcns: usize, // 
    itrmax: usize, // max iterations
    max_deviation: T
) -> Result<(), FirGrError>
where
    T: Float + FloatConst + MulAssign + AddAssign + DivAssign
{
    let one = T::one();
    let zero = T::zero();

    let mut ynz = zero;
    let mut comp = zero;
    let mut y1 = zero;
    let mut aa = zero;
    let mut bb = zero;

    let mut a = vec![zero; dimsize + 1];
    let mut p = vec![zero; dimsize + 1];
    let mut q = vec![zero; dimsize + 1];
    let mut ad = vec![zero; dimsize + 1];
    let mut x = vec![zero; dimsize + 1];
    let mut y = vec![zero; dimsize + 1];
    let mut devl = -one;
    let nz = nfcns + 1;
    let nzz = nfcns + 2;
    let mut niter = 0;

    enum Goto1
    {
        L100,
        L200,
        L210,
        L215,
        L220,
        L225,
        L230,
        L235,
        L240,
        L250,
        L255,
        L260,
        L300,
        L310,
        L315,
        L320,
        L325,
        L330,
        L340,
        L350,
        L370
    }

    let mut l = 0;
    let mut kup = 0;
    let mut klow = 0;
    let mut jchnge = 0;
    let mut err = zero;
    let mut j = 0;
    let mut nut = 0;
    let mut k1 = 0;
    let mut knz = 0;
    let mut nu = 0;
    let mut nut1 = 0;
    let mut luck = 0;
    let mut goto = Goto1::L100;
    loop
    {
        match goto
        {
            Goto1::L100 => {
                iext[nzz - 1] = ngrid + 1;
                niter += 1;

                if niter > itrmax
                {
                    break
                }

                //println!("ITERATIONS {niter}");

                for j in 1..=nz
                {
                    x[j - 1] = (T::TAU()*grid[iext[j - 1]]).cos()
                }
                let jet = (nfcns - 1)/15 + 1;

                for j in 1..=nz
                {
                    ad[j - 1] = lagrange_interp(j, nz, jet, &x);
                }

                let mut dnum = zero;
                let mut dden = zero;
                let mut k = 1i8;

                for j in 1..=nz
                {
                    l = iext[j - 1];
                    dnum += ad[j - 1]*des[l];
                    dden += T::from(k).unwrap()/wt[l]*ad[j - 1];
                    k = -k
                }
                *dev = dnum/dden;

                //println!("DEVIATION = {dev} = {dnum}/{dden}");

                nu = if *dev > zero {-1i8} else {1};
                *dev *= -T::from(nu).unwrap();
                k = nu;
                for j in 1..=nz
                {
                    l = iext[j - 1];
                    y[j - 1] = des[l] + T::from(k).unwrap()**dev/wt[l];
                    k = -k;
                }
                if *dev <= devl || (*dev).abs() > max_deviation.abs()
                {
                    return Err(FirGrError::FailureToConverge{
                        niter,
                        dev: <f64 as NumCast>::from(*dev).unwrap()
                    })
                }
                devl = *dev;
                jchnge = 0;
                k1 = iext[0];
                knz = iext[nz - 1];
                klow = 0;
                nut = -nu;
                j = 1;

                goto = Goto1::L200
            },
            Goto1::L200 => {
                if j == nzz
                {
                    ynz = comp;
                }
                if j >= nzz
                {
                    goto = Goto1::L300;
                    continue
                }
                kup = iext[j];
                l = iext[j - 1] + 1;
                nut = -nut;
                if j == 2
                {
                    y1 = comp;
                }
                comp = *dev;
                if l >= kup
                {
                    goto = Goto1::L220;
                    continue
                }
                err = (freq_eval(l, nz, grid, &x, &y, &ad) - des[l])*wt[l];
                if T::from(nut).unwrap()*err - comp <= zero
                {
                    goto = Goto1::L220;
                    continue
                }
                comp = T::from(nut).unwrap()*err;

                goto = Goto1::L210
            },
            Goto1::L210 => {
                l += 1;
                if l >= kup
                {
                    goto = Goto1::L215;
                    continue
                }
                err = (freq_eval(l, nz, grid, &x, &y, &ad) - des[l])*wt[l];
                if T::from(nut).unwrap()*err - comp <= zero
                {
                    goto = Goto1::L215;
                    continue
                }
                comp = T::from(nut).unwrap()*err;

                goto = Goto1::L210
            },
            Goto1::L215 => {
                iext[j - 1] = l - 1;
                j += 1;
                klow = l - 1;
                jchnge += 1;

                goto = Goto1::L200
            },
            Goto1::L220 => {
                l -= 1;

                goto = Goto1::L225
            }
            Goto1::L225 => {
                l -= 1;
                if l <= klow
                {
                    goto = Goto1::L250;
                    continue
                }
                err = (freq_eval(l, nz, grid, &x, &y, &ad) - des[l])*wt[l];
                if T::from(nut).unwrap()*err - comp > zero
                {
                    goto = Goto1::L230;
                    continue
                }
                if jchnge <= 0
                {
                    goto = Goto1::L225;
                    continue
                }

                goto = Goto1::L260
            },
            Goto1::L230 => {
                comp = T::from(nut).unwrap()*err;

                goto = Goto1::L235
            },
            Goto1::L235 => {
                l -= 1;
                if l <= klow
                {
                    goto = Goto1::L240;
                    continue
                }
                err = (freq_eval(l, nz, grid, &x, &y, &ad) - des[l])*wt[l];
                if T::from(nut).unwrap()*err - comp <= zero
                {
                    goto = Goto1::L240;
                    continue
                }
                comp = T::from(nut).unwrap()*err;

                goto = Goto1::L235
            },
            Goto1::L240 => {
                klow = iext[j - 1];
                iext[j - 1] = l + 1;
                j += 1;
                jchnge += 1;

                goto = Goto1::L200
            },
            Goto1::L250 => {
                l = iext[j - 1] + 1;
                if jchnge > 0
                {
                    goto = Goto1::L215;
                    continue
                }

                goto = Goto1::L255
            },
            Goto1::L255 => {
                l += 1;
                if l >= kup
                {
                    goto = Goto1::L260;
                    continue
                }
                err = (freq_eval(l, nz, grid, &x, &y, &ad) - des[l])*wt[l];
                if T::from(nut).unwrap()*err - comp <= zero
                {
                    goto = Goto1::L255;
                    continue
                }
                comp = T::from(nut).unwrap()*err;

                goto = Goto1::L210
            },
            Goto1::L260 => {
                klow = iext[j - 1];
                j += 1;

                goto = Goto1::L200
            },
            Goto1::L300 => {
                if j > nzz
                {
                    goto = Goto1::L320;
                    continue
                }
                if k1 > iext[0]
                {
                    k1 = iext[0]
                }
                if knz < iext[nz - 1]
                {
                    knz = iext[nz - 1]
                }
                nut1 = nut;
                nut = -nu;
                l = 0;
                kup = k1;
                comp = ynz*T::from(1.00001).unwrap();
                luck = 1;

                goto = Goto1::L310
            },
            Goto1::L310 => {
                l += 1;
                if l >= kup
                {
                    goto = Goto1::L315;
                    continue
                }
                err = (freq_eval(l, nz, grid, &x, &y, &ad) - des[l])*wt[l];
                if T::from(nut).unwrap()*err - comp <= zero
                {
                    goto = Goto1::L310;
                    continue
                }
                comp = T::from(nut).unwrap()*err;
                j = nzz;

                goto = Goto1::L210
            },
            Goto1::L315 => {
                luck = 6;

                goto = Goto1::L325
            },
            Goto1::L320 => {
                if luck > 9
                {
                    goto = Goto1::L350;
                    continue
                }
                y1 = y1.max(comp);
                k1 = iext[nzz - 1];

                goto = Goto1::L325
            },
            Goto1::L325 => {
                l = ngrid + 1;
                klow = knz;
                nut = -nut1;
                comp = y1*T::from(1.00001).unwrap();

                goto = Goto1::L330
            },
            Goto1::L330 => {
                l -= 1;
                if l <= klow
                {
                    goto = Goto1::L340;
                    continue
                }
                err = (freq_eval(l, nz, grid, &x, &y, &ad) - des[l])*wt[l];
                if T::from(nut).unwrap()*err - comp <= zero
                {
                    goto = Goto1::L330;
                    continue
                }
                j = nzz;
                comp = T::from(nut).unwrap()*err;
                luck += 10;

                goto = Goto1::L235
            },
            Goto1::L340 => {
                if luck == 6
                {
                    goto = Goto1::L370;
                    continue
                }
                for j in 1..=nfcns
                {
                    iext[nzz - j - 1] = iext[nz - j - 1]
                }
                iext[0] = k1;

                goto = Goto1::L100
            },
            Goto1::L350 => {
                let kn = iext[nzz];
                for j in 1..=nfcns
                {
                    iext[j - 1] = iext[j]
                }
                iext[nz - 1] = kn;

                goto = Goto1::L100
            },
            Goto1::L370 => {
                if jchnge <= 0
                {
                    break
                }
                goto = Goto1::L100
            }
        }
    }

    /*
    *    CALCULATION OF THE COEFFICIENTS OF THE BEST APPROXIMATION
    *    USING THE INVERSE DISCRETE FOURIER TRANSFORM
    */
    let two = one + one;
    let half = two.recip();
    let nm1 = nfcns - 1;
    let fsh = T::from(1.0e-06).unwrap();
    let gtemp = grid[1];
    x[nzz - 1] = -two;
    let cn = 2*nfcns - 1;
    let delf = T::from(cn).unwrap().recip();
    l = 1;
    let kkk = (edge[0] <= zero && edge[2*nbands - 1] >= half) || nfcns <= 3;

    if !kkk
    {
        let dtemp = (T::TAU()*grid[1]).cos();
        let dnum = (T::TAU()*grid[ngrid]).cos();
        aa = two/(dtemp - dnum);
        bb = -(dtemp + dnum)/(dtemp - dnum);
    }

    enum Goto2
    {
        L400,
        L410,
        L415,
        L420,
        L425,
        L450
    }

    let mut j = 1;
    let mut ft = zero;
    let mut xt = zero;
    let mut xe = zero;
    let mut goto = Goto2::L450;
    loop
    {
        match goto
        {
            Goto2::L400 => {
                ft = T::from(j - 1).unwrap()*delf;
                xt = (T::TAU()*ft).cos();
                if !kkk
                {
                    xt = (xt - bb)/aa;
                    ft = xt.acos()/T::TAU()
                }
                
                goto = Goto2::L410
            }
            Goto2::L410 => {
                xe = x[l - 1];
                if xt > xe
                {
                    goto = Goto2::L420;
                    continue
                }
                if xe - xt < fsh
                {
                    goto = Goto2::L415;
                    continue
                }
                l += 1;

                goto = Goto2::L410
            },
            Goto2::L415 => {
                a[j - 1] = y[l - 1];

                goto = Goto2::L425
            },
            Goto2::L420 => {
                if xt - xe < fsh
                {
                    goto = Goto2::L415;
                    continue
                }
                grid[1] = ft;
                a[j - 1] = freq_eval(1, nz, grid, &x, &y, &ad);

                goto = Goto2::L425
            },
            Goto2::L425 => {
                if l > 1
                {
                    l -= 1
                }

                j += 1;

                goto = Goto2::L450
            },
            Goto2::L450 => {
                if j > nfcns
                {
                    break
                }

                goto = Goto2::L400
            },
        }
    }

    grid[1] = gtemp;
    let dden = T::TAU()/T::from(cn).unwrap();
    for j in 1..=nfcns
    {
        let mut dtemp = zero;
        let dnum = T::from(j - 1).unwrap()*dden;
        if nm1 >= 1
        {
            for k in 1..=nm1
            {
                dtemp += a[k]*(dnum*T::from(k).unwrap()).cos()
            }
        }
        alpha[j - 1] = two*dtemp + a[0];
    }

    for j in 2..=nfcns
    {
        alpha[j - 1] *= two/T::from(cn).unwrap();
    }
    alpha[0] /= T::from(cn).unwrap();

    if !kkk
    {
        p[0] = two*alpha[nfcns - 1]*bb + alpha[nm1 - 1];
        p[1] = two*aa*alpha[nfcns - 1];
        q[0] = alpha[nfcns - 3] - alpha[nfcns - 1];
        for j in 2..=nm1
        {
            if j >= nm1
            {
                aa *= half;
                bb *= half;
            }
            p[j] = zero;
            for k in 1..=j
            {
                a[k - 1] = p[k - 1];
                p[k - 1] = two*bb*a[k - 1];
            }
            p[1] += a[0]*two*aa;
            let jm1 = j - 1;
            for k in 1..=jm1
            {
                p[k - 1] += q[k - 1] + aa*a[k]
            }
            let jp1 = j + 1;
            for k in 3..=jp1
            {
                p[k - 1] += aa*a[k - 2]
            }

            if j != nm1
            {
                for k in 1..=j
                {
                    q[k - 1] = -a[k - 1]
                }
                q[0] += alpha[nfcns - 2 - j]
            }
        }
        alpha[..nfcns].copy_from_slice(&p[..nfcns]);
    }

    if nfcns <= 3
    {
        alpha[nfcns] = zero;
        alpha[nfcns + 1] = zero
    }

    Ok(())
}