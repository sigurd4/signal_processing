moddef::moddef!(
    pub mod {
        convolution,
        filtering,
        resampling
    },
    flat(pub) mod {
        decode,
        encode,
        simplify,
        window
    }
);