use core::{iter::Sum, ops::Mul};

use ndarray::Array2;
use num::{complex::ComplexFloat, traits::FloatConst, Complex, NumCast, Zero};
use option_trait::Maybe;

use crate::{List, Matrix};


pub trait ToKw<T, K, N>: List<T>
where
    T: ComplexFloat,
    K: Matrix<Complex<T::Real>>,
    N: Maybe<usize>
{
    fn to_kw(self, n: N) -> K;
}

impl<T, W, const N: usize> ToKw<T, [[Complex<T::Real>; N]; N], ()> for W
where
    T: ComplexFloat,
    W: List<T>,
    Complex<T::Real>: Mul<T, Output = Complex<T::Real>> + Sum
{
    fn to_kw(self, (): ()) -> [[Complex<T::Real>; N]; N]
    {
        let mf = <T::Real as NumCast>::from(self.length()).unwrap();
        let nf = <T::Real as NumCast>::from(N).unwrap();
        let mut kw = [[Complex::zero(); N]; N];
        for i in 1..2*N
        {
            let dk = <T::Real as NumCast>::from(i).unwrap() - nf;
            let w = self.as_view_slice()
                .iter()
                .enumerate()
                .map(|(i, w)| {
                    let i = <T::Real as NumCast>::from(i).unwrap();
                    Complex::cis(T::Real::TAU()*i/mf*dk)**w
                }).sum::<Complex<T::Real>>()/mf;
            for k_ in i.saturating_sub(N)..i.min(N)
            {
                let k = k_ + N - i;
                kw[k][k_] = w;
            }
        }
        kw
    }
}

impl<T, W> ToKw<T, Array2<Complex<T::Real>>, usize> for W
where
    T: ComplexFloat,
    W: List<T>,
    Complex<T::Real>: Mul<T, Output = Complex<T::Real>> + Sum
{
    fn to_kw(self, n: usize) -> Array2<Complex<T::Real>>
    {
        let mf = <T::Real as NumCast>::from(self.length()).unwrap();
        let nf = <T::Real as NumCast>::from(n).unwrap();
        let mut kw = Array2::from_elem((n, n), Complex::zero());
        for i in 1..2*n
        {
            let dk = <T::Real as NumCast>::from(i).unwrap() - nf;
            let w = self.as_view_slice()
                .iter()
                .enumerate()
                .map(|(i, w)| {
                    let i = <T::Real as NumCast>::from(i).unwrap();
                    Complex::cis(T::Real::TAU()*i/mf*dk)**w
                }).sum::<Complex<T::Real>>()/mf;
            for k_ in i.saturating_sub(n)..i.min(n)
            {
                let k = k_ + n - i;
                kw[(k, k_)] = w;
            }
        }
        kw
    }
}

#[cfg(test)]
mod test
{
    use array_math::Array2dOps;

    use crate::{plot, window::{Hann, WindowGen, WindowRange}, ToKw};

    #[test]
    fn test()
    {
        const W: usize = 1024;
        const N: usize = 10;
        let w: [f64; W] = Hann.window_gen((), WindowRange::Symmetric);
        let kw: [[_; N]; _] = w.to_kw(());

        println!("{:?}", kw.diagonal_ref());

        plot::plot_parametric_curve_2d("K[i, j]", "plots/k_ij_to_kw.svg",
            core::array::from_fn::<_, N, _>(|i| i as f64),
            core::array::from_fn::<_, N, _>(|i| i as f64),
            |i, j| [i, j, kw[i as usize][j as usize].norm().log10()*20.0]
        ).unwrap()
    }
}