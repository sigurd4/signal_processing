use ndarray::Array2;
use num::{traits::FloatConst, Float, Zero};
use option_trait::Maybe;

use crate::{List, Matrix, OwnedList};

pub trait ConvMatrix<T, M, N>: List<T>
where
    T: Float + FloatConst,
    M: Matrix<T> + Sized,
    N: Maybe<usize>
{
    fn conv_matrix(self, n: N) -> M;
}

impl<T, A, const N: usize> ConvMatrix<T, [A::ResizedList<{A::LENGTH + N - 1}>; N], ()> for A
where
    T: Float + FloatConst,
    A: List<T, Length = usize>,
    A::ResizedList<{A::LENGTH + N - 1}>: OwnedList<T> + Clone,
    [A::ResizedList<{A::LENGTH + N - 1}>; N]: Matrix<T>
{
    fn conv_matrix(self: A, (): ()) -> [A::ResizedList<{A::LENGTH + N - 1}>; N]
    {
        let len = self.length();
        let mut a = self.static_resize_list((len + N).saturating_sub(1), Zero::zero);
        core::array::from_fn(|_| {
            let r = a.clone();
            a.as_mut_slice()
                .rotate_right(1);
            r
        })
    }
}

impl<T, A> ConvMatrix<T, Array2<T>, usize> for A
where
    T: Float + FloatConst,
    A: List<T>
{
    fn conv_matrix(self, n: usize) -> Array2<T>
    {
        let mut a = self.into_vec();
        let len = a.len();
        let m = (len + n).saturating_sub(1);
        a.resize(m, Zero::zero());
        Array2::from_shape_fn((n, m), |(i, j)| a[(j + m - i) % m])
    }
}

#[cfg(test)]
mod test
{
    use crate::ConvMatrix;

    #[test]
    fn test()
    {
        let a = [3.0, 4.0, 5.0];

        const N: usize = 3;
        let m: [_; N] = a.conv_matrix(());

        println!("{:?}", m);

        let m = a.conv_matrix(N);

        println!("{:?}", m);
    }
}