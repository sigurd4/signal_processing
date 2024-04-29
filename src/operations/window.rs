use num::complex::ComplexFloat;

use crate::{util::ComplexOp, analysis::ImpZ, quantities::{List, Lists, MaybeList, MaybeLists, Polynomial}, System, systems::Tf};

pub trait Window<W, WW, O>: System
where
    Self::Domain: ComplexOp<W>,
    W: ComplexFloat + Into<<Self::Domain as ComplexOp<W>>::Output>,
    WW: List<W>,
    O: System<Domain = <Self::Domain as ComplexOp<W>>::Output>
{
    fn window(&self, window: WW) -> O;
}

impl<T, B, A, W, WW, Y, A2> Window<W, WW, Tf<Y, B::RowsMapped<WW::Mapped<Y>>, A2>> for Tf<T, B, A>
where
    T: ComplexOp<W, Output = Y>,
    B: MaybeLists<T>,
    A: MaybeList<T>,
    W: ComplexFloat + Into<Y>,
    Y: ComplexFloat,
    WW: List<W>,
    A2: MaybeList<Y>,
    B::RowsMapped<Vec<T>>: Lists<T, RowsMapped<WW::Mapped<Y>> = B::RowsMapped<WW::Mapped<Y>>>,
    B::RowsMapped<WW::Mapped<Y>>: Lists<Y>,
    Polynomial<Y, ()>: Into<Polynomial<Y, A2>>,
    Self: for<'a> ImpZ<'a, B::RowsMapped<Vec<T>>, Vec<T::Real>, usize> + System<Domain = T>
{
    fn window(&self, window: WW) -> Tf<Y, B::RowsMapped<WW::Mapped<Y>>, A2>
    {
        let (h, _) = self.impz(window.as_view_slice().len(), None);
        let b = h.map_rows_to_owned(|h| {
            let mut h = h.as_view_slice_option()
                .unwrap()
                .iter();
            window.map_to_owned(|&w| Into::<Y>::into(*h.next().unwrap())*w.into())
        });
        Tf {
            b: Polynomial::new(b),
            a: Polynomial::one()
        }
    }
}

#[cfg(test)]
mod test
{
    use array_math::ArrayOps;

    use crate::{plot, windows, gen::{window::{WindowGen, WindowRange}, filter::{Butter, FilterGenPlane, FilterGenType}}, analysis::RealFreqZ, systems::Tf, operations::Window};

    #[test]
    fn test()
    {
        let h1 = Tf::butter(8, [100.0], FilterGenType::LowPass, FilterGenPlane::Z { sampling_frequency: Some(1000.0) })
            .unwrap();

        const N: usize = 10;
        let w1: [f64; 2*N] = windows::Boxcar.window_gen((), WindowRange::Symmetric);
        let w2: [f64; 2*N] = windows::BlackmanHarris.window_gen((), WindowRange::Symmetric);
        let w3: [f64; 2*N] = windows::Barthann.window_gen((), WindowRange::Symmetric);

        let h2: Tf<_, _, ()> = h1.window(w1.rsplit_array_ref2::<N>().1);
        let h3: Tf<_, _, ()> = h1.window(w2.rsplit_array_ref2::<N>().1);
        let h4: Tf<_, _, ()> = h1.window(w3.rsplit_array_ref2::<N>().1);

        const M: usize = 1024;
        let (h1_z, w): ([_; M], _) = h1.real_freqz(());
        let (h2_z, _) = h2.real_freqz(());
        let (h3_z, _) = h3.real_freqz(());
        let (h4_z, _) = h4.real_freqz(());

        plot::plot_curves("H(e^jw)", "plots/h_z_window.png", [
            &w.zip(h1_z.map(|h| h.norm())),
            &w.zip(h2_z.map(|h| h.norm())),
            &w.zip(h3_z.map(|h| h.norm())),
            &w.zip(h4_z.map(|h| h.norm()))
        ]).unwrap()
    }
}