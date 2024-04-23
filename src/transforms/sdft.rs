use core::ops::{AddAssign, MulAssign, SubAssign};

use num::{complex::ComplexFloat, traits::FloatConst, Complex, NumCast, One};

use crate::OwnedList;

pub trait Sdft<T, X>: OwnedList<Complex<T::Real>>
where
    T: ComplexFloat,
    X: OwnedList<T>
{
    #[doc(alias = "sliding_dft")]
    fn sdft(&mut self, x: &mut X, buffer: &mut Vec<T>);
}

impl<T, X, Z> Sdft<T, X> for Z
where
    T: ComplexFloat,
    X: OwnedList<T>,
    Z: OwnedList<Complex<T::Real>>,
    Complex<T::Real>: AddAssign<T> + SubAssign<T> + MulAssign
{
    fn sdft(&mut self, xx: &mut X, buffer: &mut Vec<T>)
    {
        let n = self.length();
        buffer.truncate(n);
        let nf = <T::Real as NumCast>::from(n).unwrap();
        let w = Complex::cis(T::Real::TAU()/nf);
        let cone = Complex::one();

        let xn = xx.length();
        if xn == 0
        {
            return;
        }
        let bn = buffer.len();

        let xbn = (bn.min(n) + xn).saturating_sub(n);

        for x in xx.as_mut_slice()[..xn - xbn]
            .iter_mut()
        {
            let mut wn = cone;
            for z in self.as_mut_slice()
                .iter_mut()
            {
                *z += *x;
                *z *= wn;
                wn *= w;
            }
            let mut y = T::zero();
            core::mem::swap(x, &mut y);
            buffer.push(y);
        }
        let bnn = bn + xn - xbn;
        if bnn > 0
        {
            buffer.rotate_right((xn - xbn) % bnn);
            buffer[..xn - xbn].reverse();
        }
        let mut i = xn - xbn;
        while i < xn
        {
            let j = (i + n).min(xn);
            for (x, y) in xx.as_mut_slice()[i..j]
                .iter_mut()
                .zip(buffer.as_mut_slice()  
                    .iter_mut()
                    .rev()
                    .take(j - i)
                )
            {
                let mut wn = cone;
                for z in self.as_mut_slice()
                    .iter_mut()
                {
                    *z += *x;
                    *z -= *y;
                    *z *= wn;
                    wn *= w;
                }
                std::mem::swap(x, y);
            }
            buffer.rotate_right(j - i);
            i = j;
        }
    }
}

#[cfg(test)]
mod test
{
    use num::Complex;

    use crate::Sdft;

    #[test]
    fn test()
    {
        let mut b = vec![-1.0];

        let mut x = [1.0, 2.0, 3.0, 4.0, 5.0, 6.0, 7.0, 8.0, 9.0, 10.0];
        let mut z = [0.0, 0.0, 0.0, 0.0].map(|z| Complex::from(z));

        z.sdft(&mut x, &mut b);

        println!("x = {:?}", x);
        println!("b = {:?}", b);
        println!("z = {:?}", z);
    }
}