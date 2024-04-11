pub trait LenEq<const A: usize, const B: usize, const EQ: bool> {}

impl<const A: usize, const B: usize> LenEq<A, B, {A == B}> for usize {}