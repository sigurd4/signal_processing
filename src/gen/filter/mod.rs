use num::Float;

moddef::moddef!(
    flat(pub) mod {
        besselap,
        besself,
        buttap,
        butter,
        buttord,
        cheb1ap,
        cheb1ord,
        cheb2ap,
        cheb2ord,
        cheby1,
        cheby2,
        ellip,
        ellipap,
        ellipord,
        fir1,
        fir2,
        firgr,
        firls,
        firpm,
        firpmord,
        kaiserord,
        pei_tseng_notch,
        qp_kaiser,
        sgolay
    }
);

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum FilterGenType
{
    LowPass,
    HighPass,
    BandPass,
    BandStop
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum FilterClassType
{
    Symmetric,
    Antisymmetric,
    Differentiator
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum FilterGenPlane<T>
where
    T: Float
{
    S,
    Z {
        sampling_frequency: Option<T>
    }
}