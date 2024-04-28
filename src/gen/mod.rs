moddef::moddef!(
    flat(pub) mod {
        cheb,
        chirp,
        diric,
        meyeraux,

        ar,
        bspline,
        filter,
        matrix,
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