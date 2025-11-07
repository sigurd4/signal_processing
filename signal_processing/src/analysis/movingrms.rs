use core::ops::{AddAssign, MulAssign};

use array_math::SliceMath;
use num::{complex::ComplexFloat, Complex, Float, NumCast, One, Zero};
use option_trait::Maybe;

use crate::{quantities::{Container, ContainerOrSingle, ListOrSingle, Lists, OwnedList, OwnedListOrSingle}, gen::pulse::SigmoidTrain};

pub trait MovingRms<T>: Lists<T, RowOwned: Container<T>>
where
    T: ComplexFloat
{
    fn moving_rms<FS>(self, width: T::Real, time_constant: T::Real, sampling_frequency: FS) -> Self::RowsMapped<(<Self::RowOwned as ContainerOrSingle<T>>::Mapped<T::Real>, <Self::RowOwned as ContainerOrSingle<T>>::Mapped<T::Real>)>
    where
        FS: Maybe<T::Real>;
}

impl<T, L> MovingRms<T> for L
where
    T: ComplexFloat + Into<Complex<T::Real>>,
    L: Lists<T>,
    L::RowOwned: OwnedList<T>,
    <L::RowOwned as ContainerOrSingle<T>>::Mapped<T::Real>: OwnedList<T::Real, Mapped<T::Real> = <L::RowOwned as ContainerOrSingle<T>>::Mapped<T::Real>> + SigmoidTrain<T::Real, <L::RowOwned as ContainerOrSingle<T>>::Mapped<T::Real>, ()>,
    <L::RowOwned as ContainerOrSingle<T>>::Mapped<Complex<T::Real>>: OwnedList<Complex<T::Real>, Mapped<T::Real> = <L::RowOwned as ContainerOrSingle<T>>::Mapped<T::Real>>,
    <<L::RowOwned as ContainerOrSingle<T>>::Mapped<T::Real> as ContainerOrSingle<T::Real>>::Mapped<Complex<T::Real>>: OwnedList<Complex<T::Real>>,
    Complex<T::Real>: AddAssign + MulAssign + MulAssign<T::Real>
{
    fn moving_rms<FS>(self, width: T::Real, time_constant: T::Real, sampling_frequency: FS) -> L::RowsMapped<(<L::RowOwned as ContainerOrSingle<T>>::Mapped<T::Real>, <L::RowOwned as ContainerOrSingle<T>>::Mapped<T::Real>)>
    where
        FS: Maybe<T::Real>
    {
        let zero = T::Real::zero();
        let one = T::Real::one();
        let two = one + one;

        let fs = sampling_frequency.into_option()
            .unwrap_or(one);

        self.map_rows_into_owned(|mut x| {
            let n = x.as_mut_slice().len();
            let nf = <T::Real as NumCast>::from(n).unwrap();
            let nfm1 = nf - one;
            let (idx, mut w) = if width*fs > nf/two
            {
                (
                    0..n,
                    x.map_to_owned(|_| one)
                )
            }
            else
            {
                let idx = ((nf - width*fs)/two).max(zero)..((nf + width*fs)/two).min(nf);
                let mut i = 0;
                (
                    NumCast::from(idx.start.round()).unwrap()..NumCast::from(idx.end.round()).unwrap(),
                    x.map_to_owned(|_| {
                        let i_f = <T::Real as NumCast>::from(i).unwrap();
                        i += 1;
                        i_f
                    }).sigmoid_train((), [(idx, time_constant*fs, time_constant*fs)]).1
                )
            };
            let mut xf = x.map_into_owned(|x| Into::<Complex<T::Real>>::into(x.conj()*x));
            xf.as_mut_slice()
                .fft();
            let mut wf = w.map_to_owned(|&w| Into::<Complex<T::Real>>::into(w.conj()*w));
            wf.as_mut_slice()
                .fft();

            for (x, w) in xf.as_mut_slice()
                .iter_mut()
                .zip(wf.into_vec()
                    .into_iter()
                )
            {
                *x *= w
            }
            xf.as_mut_slice()
                .ifft();
            let mut rmsx = xf.map_into_owned(|x| x.re()/nfm1);
            if let Some(rmsx_max) = rmsx.to_vec()
                .into_iter()
                .reduce(Float::max)
            {
                let tol = T::Real::epsilon()*rmsx_max;
                for x in rmsx.as_mut_slice()
                    .iter_mut()
                {
                    if Float::abs(*x) < tol
                    {
                        *x = zero
                    }
                }
            }
            for x in rmsx.as_mut_slice()
                .iter_mut()
            {
                *x = Float::sqrt(*x)
            }
            let s = ((idx.start + idx.end)*2 + 1)/4;
            rmsx.as_mut_slice()
                .rotate_right(s);
            w.as_mut_slice()
                .rotate_left(idx.start);
            
            (rmsx, w)
        })
    }
}

#[cfg(test)]
mod test
{
    use array_math::ArrayOps;
    use linspace::LinspaceArray;
    use rand::distributions::uniform::SampleRange;

    use crate::{plot, analysis::MovingRms};

    #[test]
    fn test()
    {
        const N: usize = 1024;

        let t: [_; N] = (0.0..1.0).linspace_array();

        let mut rng = rand::thread_rng();

        let x = t.map(|t| (-((t - 0.5)/0.1f64).powi(2)).exp() + (-0.1..0.1).sample_single(&mut rng));
        
        let fs = 1.0/(t[1] - t[0]);
        let width = 0.1;
        let rc = 2e-3;

        let (rmsx, w) = x.moving_rms(width, rc, fs);

        plot::plot_curves("x(t)", "plots/x_t_movingrms.png", [&t.zip(x), &t.zip(rmsx), &t.zip(w)]).unwrap();
    }
}