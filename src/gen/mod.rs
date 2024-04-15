use num::Float;


moddef::moddef!(
    flat(pub) mod {
        cheb,
        chirp,
        diric,
        meyeraux,
        sigmoid_train,

        filter,
        pulse,
        wavelet
    }
);

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum Plane
{
    S,
    Z
}

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
    Z{
        sampling_frequency: Option<T>
    }
}