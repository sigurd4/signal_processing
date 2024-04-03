use array_math::{ArrayOps, SliceMath};
use num::{One, Zero};

use crate::{MaybeLists, Polynomial};

impl<T1, T2, const N: usize> From<Polynomial<T1, ()>> for Polynomial<T2, [T2; N]>
where
    T1: One + Into<T2>,
    T2: Zero,
    [(); N - 1]:
{
    fn from(_: Polynomial<T1, ()>) -> Self
    {
        let mut p = <[T2; N]>::fill(|_| T2::zero());
        *p.last_mut().unwrap() = T1::one().into();
        Self::new(p)
    }
}
impl<T1, T2, const N: usize> From<Polynomial<T1, ()>> for Polynomial<T2, [[T2; N]; 1]>
where
    T1: One + Into<T2>,
    T2: Zero,
    [(); N - 1]:
{
    fn from(_: Polynomial<T1, ()>) -> Self
    {
        let mut p = <[T2; N]>::fill(|_| T2::zero());
        *p.last_mut().unwrap() = T1::one().into();
        Self::new([p])
    }
}
impl<T1, T2, const N: usize> From<Polynomial<T1, ()>> for Polynomial<T2, Vec<[T2; N]>>
where
    T1: One + Into<T2>,
    T2: Zero,
    [(); N - 1]:
{
    fn from(_: Polynomial<T1, ()>) -> Self
    {
        let mut p = <[T2; N]>::fill(|_| T2::zero());
        *p.last_mut().unwrap() = T1::one().into();
        Self::new(vec![p])
    }
}
impl<T1, T2> From<Polynomial<T1, ()>> for Polynomial<T2, Vec<T2>>
where
    T1: One + Into<T2>
{
    fn from(_: Polynomial<T1, ()>) -> Self
    {
        Self::new(vec![T1::one().into()])
    }
}
impl<T1, T2> From<Polynomial<T1, ()>> for Polynomial<T2, [Vec<T2>; 1]>
where
    T1: One + Into<T2>
{
    fn from(_: Polynomial<T1, ()>) -> Self
    {
        Self::new([vec![T1::one().into()]])
    }
}
impl<T1, T2> From<Polynomial<T1, ()>> for Polynomial<T2, Vec<Vec<T2>>>
where
    T1: One + Into<T2>
{
    fn from(_: Polynomial<T1, ()>) -> Self
    {
        Self::new(vec![vec![T1::one().into()]])
    }
}

/*impl<T, const N: usize, const M: usize> From<Polynomial<T, [T; M]>> for Polynomial<T, [T; N]>
where
    T: Zero,
    [(); N - M]:
{
    fn from(p: Polynomial<T, [T; M]>) -> Self
    {
        Self::new(
            p.c.rresize(|_| T::zero())
        )
    }
}*/
impl<T1, T2, const N: usize, const M: usize> From<Polynomial<T1, [T1; M]>> for Polynomial<T2, [[T2; N]; 1]>
where
    T1: Into<T2>,
    T2: Zero,
    [(); N - M]:
{
    fn from(p: Polynomial<T1, [T1; M]>) -> Self
    {
        Self::new(
            [p.c.map(Into::into).rresize(|_| T2::zero())]
        )
    }
}
impl<T1, T2, const N: usize, const M: usize> From<Polynomial<T1, [T1; M]>> for Polynomial<T2, Vec<[T2; N]>>
where
    T1: Into<T2>,
    T2: Zero,
    [(); N - M]:
{
    fn from(p: Polynomial<T1, [T1; M]>) -> Self
    {
        Self::new(
            vec![p.c.map(Into::into).rresize(|_| T2::zero())]
        )
    }
}
impl<T1, T2, const M: usize> From<Polynomial<T1, [T1; M]>> for Polynomial<T2, Vec<T2>>
where
    T1: Into<T2>
{
    fn from(p: Polynomial<T1, [T1; M]>) -> Self
    {
        Self::new(
            p.c.into_iter()
                .map(Into::into)
                .collect()
        )
    }
}
impl<T1, T2, const M: usize> From<Polynomial<T1, [T1; M]>> for Polynomial<T2, [Vec<T2>; 1]>
where
    T1: Into<T2>
{
    fn from(p: Polynomial<T1, [T1; M]>) -> Self
    {
        Self::new([
            p.c.into_iter()
                .map(Into::into)
                .collect()
        ])
    }
}
impl<T1, T2, const M: usize> From<Polynomial<T1, [T1; M]>> for Polynomial<T2, Vec<Vec<T2>>>
where
    T1: Into<T2>
{
    fn from(p: Polynomial<T1, [T1; M]>) -> Self
    {
        Self::new(vec![
            p.c.into_iter()
                .map(Into::into)
                .collect()
        ])
    }
}

impl<T1, T2, const N: usize, const M: usize> From<Polynomial<T1, &[T1; M]>> for Polynomial<T2, [T2; N]>
where
    T1: Clone + Into<T2>,
    T2: Zero,
    [(); N - M]:
{
    fn from(p: Polynomial<T1, &[T1; M]>) -> Self
    {
        Self::new(
            p.c.clone()
                .map(Into::into)
                .resize(|_| T2::zero())
        )
    }
}
impl<T1, T2, const N: usize, const M: usize> From<Polynomial<T1, &[T1; M]>> for Polynomial<T2, [[T2; N]; 1]>
where
    T1: Clone + Into<T2>,
    T2: Zero,
    [(); N - M]:
{
    fn from(p: Polynomial<T1, &[T1; M]>) -> Self
    {
        Self::new([
            p.c.clone()
                .map(Into::into)
                .resize(|_| T2::zero())
        ])
    }
}
impl<'a, T, const M: usize> From<Polynomial<T, &'a [T; M]>> for Polynomial<T, &'a [[T; M]; 1]>
{
    fn from(p: Polynomial<T, &'a [T; M]>) -> Self
    {
        Self::new(
            core::array::from_ref(p.c)
        )
    }
}
impl<T1, T2, const N: usize, const M: usize> From<Polynomial<T1, &[T1; M]>> for Polynomial<T2, Vec<[T2; N]>>
where
    T1: Clone + Into<T2>,
    T2: Zero,
    [(); N - M]:
{
    fn from(p: Polynomial<T1, &[T1; M]>) -> Self
    {
        Self::new(vec![
            p.c.clone()
                .map(Into::into)
                .resize(|_| T2::zero())
        ])
    }
}
impl<'a, T, const M: usize> From<Polynomial<T, &'a [T; M]>> for Polynomial<T, &'a [[T; M]]>
{
    fn from(p: Polynomial<T, &'a [T; M]>) -> Self
    {
        Self::new(
            core::slice::from_ref(p.c)
        )
    }
}
impl<'a, T, const M: usize> From<Polynomial<T, &'a [T; M]>> for Polynomial<T, [&'a [T; M]; 1]>
{
    fn from(p: Polynomial<T, &'a [T; M]>) -> Self
    {
        Self::new(
            [p.c]
        )
    }
}
impl<'a, T, const M: usize> From<Polynomial<T, &'a [T; M]>> for Polynomial<T, Vec<&'a [T; M]>>
{
    fn from(p: Polynomial<T, &'a [T; M]>) -> Self
    {
        Self::new(
            vec![p.c]
        )
    }
}
impl<T1, T2, const M: usize> From<Polynomial<T1, &[T1; M]>> for Polynomial<T2, Vec<T2>>
where
    T1: Zero + Clone + Into<T2>
{
    fn from(p: Polynomial<T1, &[T1; M]>) -> Self
    {
        Self::new(
            p.c.as_slice()
                .trim_zeros_front()
                .iter()
                .map(|c| c.clone().into())
                .collect()
        )
    }
}
impl<T1, T2, const M: usize> From<Polynomial<T1, &[T1; M]>> for Polynomial<T2, [Vec<T2>; 1]>
where
    T1: Clone + Zero + Into<T2>
{
    fn from(p: Polynomial<T1, &[T1; M]>) -> Self
    {
        Self::new([
            p.c.as_slice()
                .trim_zeros_front()
                .iter()
                .map(|c| c.clone().into())
                .collect()
        ])
    }
}
impl<T1, T2, const M: usize> From<Polynomial<T1, &[T1; M]>> for Polynomial<T2, Vec<Vec<T2>>>
where
    T1: Clone + Zero + Into<T2>
{
    fn from(p: Polynomial<T1, &[T1; M]>) -> Self
    {
        Self::new(vec![
            p.c.as_slice()
                .trim_zeros_front()
                .iter()
                .map(|c| c.clone().into())
                .collect()
        ])
    }
}
impl<'a, T, const M: usize> From<Polynomial<T, &'a [T; M]>> for Polynomial<T, &'a [T]>
where
    T: Zero
{
    fn from(p: Polynomial<T, &'a [T; M]>) -> Self
    {
        Self::new(
            p.c.as_slice().trim_zeros_front()
        )
    }
}
impl<'a, T, const M: usize> From<Polynomial<T, &'a [T; M]>> for Polynomial<T, [&'a [T]; 1]>
where
    T: Zero
{
    fn from(p: Polynomial<T, &'a [T; M]>) -> Self
    {
        Self::new(
            [p.c.as_slice().trim_zeros_front()]
        )
    }
}
impl<'a, T, const M: usize> From<Polynomial<T, &'a [T; M]>> for Polynomial<T, Vec<&'a [T]>>
where
    T: Zero
{
    fn from(p: Polynomial<T, &'a [T; M]>) -> Self
    {
        Self::new(
            vec![p.c.as_slice().trim_zeros_front()]
        )
    }
}

impl<T1, T2> From<Polynomial<T1, Vec<T1>>> for Polynomial<T2, [Vec<T2>; 1]>
where
    T1: Into<T2>
{
    fn from(p: Polynomial<T1, Vec<T1>>) -> Self
    {
        Self::new([
            p.c.into_iter()
                .map(Into::into)
                .collect()
        ])
    }
}
impl<T1, T2> From<Polynomial<T1, Vec<T1>>> for Polynomial<T2, Vec<Vec<T2>>>
where
    T1: Into<T2>
{
    fn from(p: Polynomial<T1, Vec<T1>>) -> Self
    {
        Self::new(vec![
            p.c.into_iter()
                .map(Into::into)
                .collect()
        ])
    }
}

impl<T1, T2> From<Polynomial<T1, &[T1]>> for Polynomial<T2, Vec<T2>>
where
    T1: Clone + Zero + Into<T2>
{
    fn from(p: Polynomial<T1, &[T1]>) -> Self
    {
        Self::new(
            p.c.trim_zeros_front()
                .iter()
                .map(|c| c.clone().into())
                .collect()
        )
    }
}
impl<T1, T2> From<Polynomial<T1, &[T1]>> for Polynomial<T2, [Vec<T2>; 1]>
where
    T1: Clone + Zero + Into<T2>
{
    fn from(p: Polynomial<T1, &[T1]>) -> Self
    {
        Self::new([
            p.c.trim_zeros_front()
                .iter()
                .map(|c| c.clone().into())
                .collect()
        ])
    }
}
impl<T1, T2> From<Polynomial<T1, &[T1]>> for Polynomial<T2, Vec<Vec<T2>>>
where
    T1: Clone + Zero + Into<T2>
{
    fn from(p: Polynomial<T1, &[T1]>) -> Self
    {
        Self::new(vec![
            p.c.trim_zeros_front()
                .iter()
                .map(|c| c.clone().into())
                .collect()
        ])
    }
}
impl<'a, T> From<Polynomial<T, &'a [T]>> for Polynomial<T, [&'a [T]; 1]>
where
    T: Zero
{
    fn from(p: Polynomial<T, &'a [T]>) -> Self
    {
        Self::new(
            [p.c.trim_zeros_front()]
        )
    }
}
impl<'a, T> From<Polynomial<T, &'a [T]>> for Polynomial<T, Vec<&'a [T]>>
where
    T: Zero
{
    fn from(p: Polynomial<T, &'a [T]>) -> Self
    {
        Self::new(
            vec![p.c.trim_zeros_front()]
        )
    }
}

/*impl<T, const N: usize, const M: usize, const K: usize> From<Polynomial<T, [[T; M]; K]>> for Polynomial<T, [[T; N]; K]>
where
    [(); N - M]:
{
    fn from(p: [[f64; M]; K]) -> Self
    {
        p.map(|p| p.resize(|_| 0.0))
    }
}*/
impl<T1, T2, const N: usize, const M: usize, const K: usize> From<Polynomial<T1, [[T1; M]; K]>> for Polynomial<T2, Vec<[T2; N]>>
where
    T1: Into<T2>,
    T2: Zero,
    [(); N - M]:
{
    fn from(p: Polynomial<T1, [[T1; M]; K]>) -> Self
    {
        Self::new(
            p.c.into_iter()
                .map(|p| p.map(Into::into).resize(|_| T2::zero()))
                .collect()
        )
    }
}
impl<T1, T2, const M: usize, const K: usize> From<Polynomial<T1, [[T1; M]; K]>> for Polynomial<T2, [Vec<T2>; K]>
where
    T1: Into<T2>
{
    fn from(p: Polynomial<T1, [[T1; M]; K]>) -> Self
    {
        Self::new(
            p.c.map(|p| p.into_iter()
                .map(Into::into)
                .collect()
            )
        )
    }
}
impl<T1, T2, const M: usize, const K: usize> From<Polynomial<T1, [[T1; M]; K]>> for Polynomial<T2, Vec<Vec<T2>>>
where
    T1: Into<T2>
{
    fn from(p: Polynomial<T1, [[T1; M]; K]>) -> Self
    {
        Self::new(
            p.c.into_iter()
                .map(|p| p.into_iter()
                    .map(Into::into)
                    .collect()
                ).collect()
        )
    }
}

impl<T1, T2, const N: usize, const M: usize, const K: usize> From<Polynomial<T1, [&[T1; M]; K]>> for Polynomial<T2, [[T2; N]; K]>
where
    T1: Clone + Into<T2>,
    T2: Zero,
    [(); N - M]:
{
    fn from(p: Polynomial<T1, [&[T1; M]; K]>) -> Self
    {
        Self::new(
            p.c.map(|p| p.clone().map(Into::into).resize(|_| T2::zero()))
        )
    }
}
impl<T1, T2, const N: usize, const M: usize, const K: usize> From<Polynomial<T1, [&[T1; M]; K]>> for Polynomial<T2, Vec<[T2; N]>>
where
    T1: Clone + Into<T2>,
    T2: Zero,
    [(); N - M]:
{
    fn from(p: Polynomial<T1, [&[T1; M]; K]>) -> Self
    {
        Self::new(
            p.c.into_iter()
                .map(|p| p.clone().map(Into::into).resize(|_| T2::zero()))
                .collect()
        )
    }
}
impl<'a, T, const M: usize, const K: usize> From<Polynomial<T, [&'a [T; M]; K]>> for Polynomial<T, Vec<&'a [T; M]>>
{
    fn from(p: Polynomial<T, [&'a [T; M]; K]>) -> Self
    {
        Self::new(
            p.c.into_iter()
                .collect()
        )
    }
}
impl<T1, T2, const M: usize, const K: usize> From<Polynomial<T1, [&[T1; M]; K]>> for Polynomial<T2, [Vec<T2>; K]>
where
    T1: Clone + Zero + Into<T2>
{
    fn from(p: Polynomial<T1, [&[T1; M]; K]>) -> Self
    {
        Self::new(
            p.c.map(|p| p.as_slice()
                .trim_zeros_front()
                .iter()
                .map(|c| c.clone().into())
                .collect()
            )
        )
    }
}
impl<T1, T2, const M: usize, const K: usize> From<Polynomial<T1, [&[T1; M]; K]>> for Polynomial<T2, Vec<Vec<T2>>>
where
    T1: Clone + Zero + Into<T2>
{
    fn from(p: Polynomial<T1, [&[T1; M]; K]>) -> Self
    {
        Self::new(
            p.c.into_iter()
                .map(|p| p.as_slice()
                    .trim_zeros_front()
                    .iter()
                    .map(|c| c.clone().into())
                    .collect()
                ).collect()
        )
    }
}
impl<'a, T, const M: usize, const K: usize> From<Polynomial<T, [&'a [T; M]; K]>> for Polynomial<T, [&'a [T]; K]>
where
    T: Zero
{
    fn from(p: Polynomial<T, [&'a [T; M]; K]>) -> Self
    {
        Self::new(
            p.c.map(|p| p.as_slice().trim_zeros_front())
        )
    }
}
impl<'a, T, const M: usize, const K: usize> From<Polynomial<T, [&'a [T; M]; K]>> for Polynomial<T, Vec<&'a [T]>>
where
    T: Zero
{
    fn from(p: Polynomial<T, [&'a [T; M]; K]>) -> Self
    {
        Self::new(
            p.c.into_iter()
                .map(|p| p.as_slice().trim_zeros_front())
                .collect()
        )
    }
}

impl<T1, T2, const K: usize> From<Polynomial<T1, [Vec<T1>; K]>> for Polynomial<T2, Vec<Vec<T2>>>
where
    T1: Into<T2>
{
    fn from(p: Polynomial<T1, [Vec<T1>; K]>) -> Self
    {
        Self::new(
            p.c.into_iter()
                .map(|p| p.into_iter()
                    .map(Into::into)
                    .collect()
                ).collect()
        )
    }
}

impl<T1, T2, const K: usize> From<Polynomial<T1, [&[T1]; K]>> for Polynomial<T2, [Vec<T2>; K]>
where
    T1: Clone + Zero + Into<T2>
{
    fn from(p: Polynomial<T1, [&[T1]; K]>) -> Self
    {
        Self::new(
            p.c.map(|p| p.trim_zeros_front()
                .iter()
                .map(|s| s.clone().into())
                .collect()
            )
        )
    }
}
impl<T1, T2, const K: usize> From<Polynomial<T1, [&[T1]; K]>> for Polynomial<T2, Vec<Vec<T2>>>
where
    T1: Clone + Zero + Into<T2>
{
    fn from(p: Polynomial<T1, [&[T1]; K]>) -> Self
    {
        Self::new(
            p.c.into_iter()
                .map(|p| p.trim_zeros_front()
                    .iter()
                    .map(|s| s.clone().into())
                    .collect()
                ).collect()
        )
    }
}
impl<'a, T, const K: usize> From<Polynomial<T, [&'a [T]; K]>> for Polynomial<T, Vec<&'a [T]>>
{
    fn from(p: Polynomial<T, [&'a [T]; K]>) -> Self
    {
        Self::new(
            p.c.into_iter()
                .collect()
        )
    }
}

impl<T1, T2, const N: usize, const M: usize, const K: usize> From<Polynomial<T1, &[[T1; M]; K]>> for Polynomial<T2, [[T2; N]; K]>
where
    T1: Clone + Into<T2>,
    T2: Zero,
    [(); N - M]:
{
    fn from(p: Polynomial<T1, &[[T1; M]; K]>) -> Self
    {
        Self::new(
            p.c.each_ref()
                .map(|p| p.clone().map(Into::into).resize(|_| T2::zero()))
        )
    }
}
impl<T1, T2, const N: usize, const M: usize, const K: usize> From<Polynomial<T1, &[[T1; M]; K]>> for Polynomial<T2, Vec<[T2; N]>>
where
    T1: Clone + Into<T2>,
    T2: Zero,
    [(); N - M]:
{
    fn from(p: Polynomial<T1, &[[T1; M]; K]>) -> Self
    {
        Self::new(
            p.c.iter()
                .map(|p| p.clone().map(Into::into).resize(|_| T2::zero()))
                .collect()
        )
    }
}
impl<'a, T, const M: usize, const K: usize> From<Polynomial<T, &'a [[T; M]; K]>> for Polynomial<T, &'a [[T; M]]>
{
    fn from(p: Polynomial<T, &'a [[T; M]; K]>) -> Self
    {
        Self::new(
            p.c.as_slice()
        )
    }
}
impl<'a, T, const M: usize, const K: usize> From<Polynomial<T, &'a [[T; M]; K]>> for Polynomial<T, [&'a [T; M]; K]>
{
    fn from(p: Polynomial<T, &'a [[T; M]; K]>) -> Self
    {
        Self::new(
            p.c.each_ref()
        )
    }
}
impl<'a, T, const M: usize, const K: usize> From<Polynomial<T, &'a [[T; M]; K]>> for Polynomial<T, Vec<&'a [T; M]>>
{
    fn from(p: Polynomial<T, &'a [[T; M]; K]>) -> Self
    {
        Self::new(
            p.c.iter()
                .collect()
        )
    }
}
impl<T1, T2, const M: usize, const K: usize> From<Polynomial<T1, &[[T1; M]; K]>> for Polynomial<T2, [Vec<T2>; K]>
where
    T1: Zero + Clone + Into<T2>
{
    fn from(p: Polynomial<T1, &[[T1; M]; K]>) -> Self
    {
        Self::new(
            p.c.each_ref()
                .map(|p| p.as_slice()
                    .trim_zeros_front()
                    .iter()
                    .map(|c| c.clone().into())
                    .collect()
                )
        )
    }
}
impl<T1, T2, const M: usize, const K: usize> From<Polynomial<T1, &[[T1; M]; K]>> for Polynomial<T2, Vec<Vec<T2>>>
where
    T1: Zero + Clone + Into<T2>
{
    fn from(p: Polynomial<T1, &[[T1; M]; K]>) -> Self
    {
        Self::new(
            p.c.iter()
                .map(|p| p.as_slice()
                    .trim_zeros_front()
                    .iter()
                    .map(|c| c.clone().into())
                    .collect()
                ).collect()
        )
    }
}
impl<'a, T, const M: usize, const K: usize> From<Polynomial<T, &'a [[T; M]; K]>> for Polynomial<T, [&'a [T]; K]>
where
    T: Zero + Clone
{
    fn from(p: Polynomial<T, &'a [[T; M]; K]>) -> Self
    {
        Self::new(
            p.c.each_ref()
                .map(|p| p.as_slice().trim_zeros_front())
        )
    }
}
impl<'a, T, const M: usize, const K: usize> From<Polynomial<T, &'a [[T; M]; K]>> for Polynomial<T, Vec<&'a [T]>>
where
    T: Zero + Clone
{
    fn from(p: Polynomial<T, &'a [[T; M]; K]>) -> Self
    {
        Self::new(
            p.c.iter()
                .map(|p| p.as_slice().trim_zeros_front())
                .collect()
        )
    }
}

impl<T1, T2, const N: usize, const M: usize, const K: usize> From<Polynomial<T1, &[&[T1; M]; K]>> for Polynomial<T2, [[T2; N]; K]>
where
    T1: Clone + Into<T2>,
    T2: Zero,
    [(); N - M]:
{
    fn from(p: Polynomial<T1, &[&[T1; M]; K]>) -> Self
    {
        Self::new(
            p.c.map(|p| p.clone().map(Into::into).resize(|_| T2::zero()))
        )
    }
}
impl<T1, T2, const N: usize, const M: usize, const K: usize> From<Polynomial<T1, &[&[T1; M]; K]>> for Polynomial<T2, Vec<[T2; N]>>
where
    T1: Clone + Into<T2>,
    T2: Zero,
    [(); N - M]:
{
    fn from(p: Polynomial<T1, &[&[T1; M]; K]>) -> Self
    {
        Self::new(
            p.c.iter()
                .map(|&p| p.clone().map(Into::into).resize(|_| T2::zero()))
                .collect()
        )
    }
}
impl<'a, T, const M: usize, const K: usize> From<Polynomial<T, &[&'a [T; M]; K]>> for Polynomial<T, [&'a [T; M]; K]>
{
    fn from(p: Polynomial<T, &[&'a [T; M]; K]>) -> Self
    {
        Self::new(
            *p.c
        )
    }
}
impl<'a, T, const M: usize, const K: usize> From<Polynomial<T, &[&'a [T; M]; K]>> for Polynomial<T, Vec<&'a [T; M]>>
{
    fn from(p: Polynomial<T, &[&'a [T; M]; K]>) -> Self
    {
        Self::new(
            p.c.iter().copied()
                .collect()
        )
    }
}
impl<'a, 'b, T, const M: usize, const K: usize> From<Polynomial<T, &'a [&'b [T; M]; K]>> for Polynomial<T, &'a [&'b [T; M]]>
{
    fn from(p: Polynomial<T, &'a [&'b [T; M]; K]>) -> Self
    {
        Self::new(
            p.c.as_slice()
        )
    }
}
impl<T1, T2, const M: usize, const K: usize> From<Polynomial<T1, &[&[T1; M]; K]>> for Polynomial<T2, [Vec<T2>; K]>
where
    T1: Zero + Clone + Into<T2>
{
    fn from(p: Polynomial<T1, &[&[T1; M]; K]>) -> Self
    {
        Self::new(
            p.c.map(|p| p.as_slice()
                .trim_zeros_front()
                .iter()
                .map(|c| c.clone().into())
                .collect()
            )
        )
    }
}
impl<T1, T2, const M: usize, const K: usize> From<Polynomial<T1, &[&[T1; M]; K]>> for Polynomial<T2, Vec<Vec<T2>>>
where
    T1: Zero + Clone + Into<T2>
{
    fn from(p: Polynomial<T1, &[&[T1; M]; K]>) -> Self
    {
        Self::new(
            p.c.iter()
                .map(|&p| p.as_slice()
                    .trim_zeros_front()
                    .iter()
                    .map(|c| c.clone().into())
                    .collect()
                ).collect()
        )
    }
}
impl<'a, T, const M: usize, const K: usize> From<Polynomial<T, &[&'a [T; M]; K]>> for Polynomial<T, [&'a [T]; K]>
where
    T: Zero
{
    fn from(p: Polynomial<T, &[&'a [T; M]; K]>) -> Self
    {
        Self::new(
            p.c.map(|p| p.as_slice().trim_zeros_front())
        )
    }
}
impl<'a, T, const M: usize, const K: usize> From<Polynomial<T, &[&'a [T; M]; K]>> for Polynomial<T, Vec<&'a [T]>>
where
    T: Zero
{
    fn from(p: Polynomial<T, &[&'a [T; M]; K]>) -> Self
    {
        Self::new(
            p.c.iter()
                .map(|&p| p.as_slice().trim_zeros_front())
                .collect()
        )
    }
}

impl<T1, T2, const K: usize> From<Polynomial<T1, &[Vec<T1>; K]>> for Polynomial<T2, [Vec<T2>; K]>
where
    T1: Clone + Into<T2>
{
    fn from(p: Polynomial<T1, &[Vec<T1>; K]>) -> Self
    {
        Self::new(
            p.c.each_ref()
                .map(|c| c.iter()
                    .map(|c| c.clone().into())
                    .collect()
                )
        )
    }
}
impl<T1, T2, const K: usize> From<Polynomial<T1, &[Vec<T1>; K]>> for Polynomial<T2, Vec<Vec<T2>>>
where
    T1: Clone + Into<T2>
{
    fn from(p: Polynomial<T1, &[Vec<T1>; K]>) -> Self
    {
        Self::new(
            p.c.iter()
                .map(|p| p.iter()
                    .map(|c| c.clone().into())
                    .collect()
                ).collect()
        )
    }
}
impl<'a, T, const K: usize> From<Polynomial<T, &'a [Vec<T>; K]>> for Polynomial<T, &'a [Vec<T>]>
{
    fn from(p: Polynomial<T, &'a [Vec<T>; K]>) -> Self
    {
        Self::new(
            p.c.as_slice()
        )
    }
}
impl<'a, T, const K: usize> From<Polynomial<T, &'a [Vec<T>; K]>> for Polynomial<T, [&'a [T]; K]>
{
    fn from(p: Polynomial<T, &'a [Vec<T>; K]>) -> Self
    {
        Self::new(
            p.c.each_ref()
                .map(|p| p.as_slice())
        )
    }
}
impl<'a, T, const K: usize> From<Polynomial<T, &'a [Vec<T>; K]>> for Polynomial<T, Vec<&'a [T]>>
where
    T: Zero
{
    fn from(p: Polynomial<T, &'a [Vec<T>; K]>) -> Self
    {
        Self::new(
            p.c.iter()
                .map(|p| p.as_slice().trim_zeros_front())
                .collect()
        )
    }
}

impl<T1, T2, const K: usize> From<Polynomial<T1, &[&[T1]; K]>> for Polynomial<T2, [Vec<T2>; K]>
where
    T1: Clone + Zero + Into<T2>
{
    fn from(p: Polynomial<T1, &[&[T1]; K]>) -> Self
    {
        Self::new(
            p.c.each_ref()
                .map(|&p| p.trim_zeros_front()
                    .iter()
                    .map(|c| c.clone().into())
                    .collect()
                )
        )
    }
}
impl<'a, T, const K: usize> From<Polynomial<T, &[&'a [T]; K]>> for Polynomial<T, [&'a [T]; K]>
where
    T: Zero
{
    fn from(p: Polynomial<T, &[&'a [T]; K]>) -> Self
    {
        Self::new(
            p.c.map(|p| p.trim_zeros_front())
        )
    }
}
impl<'a, 'b, T, const K: usize> From<Polynomial<T, &'a [&'b [T]; K]>> for Polynomial<T, &'a [&'b [T]]>
where
    T: Zero
{
    fn from(p: Polynomial<T, &'a [&'b [T]; K]>) -> Self
    {
        Self::new(
            p.c.as_slice()
        )
    }
}
impl<T1, T2, const K: usize> From<Polynomial<T1, &[&[T1]; K]>> for Polynomial<T2, Vec<Vec<T2>>>
where
    T1: Clone + Zero + Into<T2>
{
    fn from(p: Polynomial<T1, &[&[T1]; K]>) -> Self
    {
        Self::new(
            p.c.iter()
                .map(|&p| p.trim_zeros_front()
                    .iter()
                    .map(|c| c.clone().into())
                    .collect()
                ).collect()
        )
    }
}
impl<'a, T, const K: usize> From<Polynomial<T, &[&'a [T]; K]>> for Polynomial<T, Vec<&'a [T]>>
where
    T: Zero
{
    fn from(p: Polynomial<T, &[&'a [T]; K]>) -> Self
    {
        Self::new(
            (*p.c).into_iter()
                .map(|p| p.trim_zeros_front())
                .collect()
        )
    }
}

/*impl<T, const N: usize, const M: usize> From<Polynomial<T, Vec<[T; M]>>> for Polynomial<T, Vec<[T; N]>>
where
    [(); N - M]:
{
    fn from(p: Polynomial<T, Vec<[T; M]>>) -> Self
    {
        p.into_iter()
            .map(|p| p.resize(|_| 0.0))
            .collect()
    }
}*/
impl<T1, T2, const M: usize> From<Polynomial<T1, Vec<[T1; M]>>> for Polynomial<T2, Vec<Vec<T2>>>
where
    T1: Into<T2>
{
    fn from(p: Polynomial<T1, Vec<[T1; M]>>) -> Self
    {
        Self::new(
            p.c.into_iter()
                .map(|p| p.into_iter()
                    .map(Into::into)
                    .collect()
                ).collect()
        )
    }
}

impl<T1, T2, const N: usize, const M: usize> From<Polynomial<T1, Vec<&[T1; M]>>> for Polynomial<T2, Vec<[T2; N]>>
where
    T1: Clone + Into<T2>,
    T2: Zero,
    [(); N - M]:
{
    fn from(p: Polynomial<T1, Vec<&[T1; M]>>) -> Self
    {
        Self::new(
            p.c.into_iter()
                .map(|p| p.clone().map(Into::into).resize(|_| T2::zero()))
                .collect()
        )
    }
}
impl<T1, T2, const M: usize> From<Polynomial<T1, Vec<&[T1; M]>>> for Polynomial<T2, Vec<Vec<T2>>>
where
    T1: Clone + Zero + Into<T2>
{
    fn from(p: Polynomial<T1, Vec<&[T1; M]>>) -> Self
    {
        Self::new(
            p.c.into_iter()
                .map(|p| p.as_slice()
                    .trim_zeros_front()
                    .iter()
                    .map(|c| c.clone().into())
                    .collect()
                ).collect()
        )
    }
}
impl<'a, T, const M: usize> From<Polynomial<T, Vec<&'a [T; M]>>> for Polynomial<T, Vec<&'a [T]>>
where
    T: Zero
{
    fn from(p: Polynomial<T, Vec<&'a [T; M]>>) -> Self
    {
        Self::new(
            p.c.into_iter()
                .map(|p| p.as_slice().trim_zeros_front())
                .collect()
        )
    }
}

impl<T1, T2> From<Polynomial<T1, Vec<&[T1]>>> for Polynomial<T2, Vec<Vec<T2>>>
where
    T1: Clone + Zero + Into<T2>
{
    fn from(p: Polynomial<T1, Vec<&[T1]>>) -> Self
    {
        Self::new(
            p.c.into_iter()
                .map(|p| p.trim_zeros_front()
                    .iter()
                    .map(|c| c.clone().into())
                    .collect()
                ).collect()
        )
    }
}

impl<T1, T2, const N: usize, const M: usize> From<Polynomial<T1, &[[T1; M]]>> for Polynomial<T2, Vec<[T2; N]>>
where
    T1: Clone + Into<T2>,
    T2: Zero,
    [(); N - M]:
{
    fn from(p: Polynomial<T1, &[[T1; M]]>) -> Self
    {
        Self::new(
            p.c.iter()
                .map(|p| p.clone().map(Into::into).resize(|_| T2::zero()))
                .collect()
        )
    }
}
impl<'a, T, const M: usize> From<Polynomial<T, &'a [[T; M]]>> for Polynomial<T, Vec<&'a [T; M]>>
{
    fn from(p: Polynomial<T, &'a [[T; M]]>) -> Self
    {
        Self::new(
            p.c.iter()
                .collect()
        )
    }
}
impl<T1, T2, const M: usize> From<Polynomial<T1, &[[T1; M]]>> for Polynomial<T2, Vec<Vec<T2>>>
where
    T1: Zero + Clone + Into<T2>
{
    fn from(p: Polynomial<T1, &[[T1; M]]>) -> Self
    {
        Self::new(
            p.c.iter()
                .map(|p| p.as_slice()
                    .trim_zeros_front()
                    .iter()
                    .map(|c| c.clone().into())
                    .collect()
                ).collect()
        )
    }
}
impl<'a, T, const M: usize> From<Polynomial<T, &'a [[T; M]]>> for Polynomial<T, Vec<&'a [T]>>
where
    T: Zero + Clone
{
    fn from(p: Polynomial<T, &'a [[T; M]]>) -> Self
    {
        Self::new(
            p.c.iter()
                .map(|p| p.as_slice().trim_zeros_front())
                .collect()
        )
    }
}

impl<T1, T2, const N: usize, const M: usize> From<Polynomial<T1, &[&[T1; M]]>> for Polynomial<T2, Vec<[T2; N]>>
where
    T1: Clone + Into<T2>,
    T2: Zero,
    [(); N - M]:
{
    fn from(p: Polynomial<T1, &[&[T1; M]]>) -> Self
    {
        Self::new(
            p.c.iter()
                .map(|&p| p.clone().map(Into::into).resize(|_| T2::zero()))
                .collect()
        )
    }
}
impl<'a, T, const M: usize> From<Polynomial<T, &[&'a [T; M]]>> for Polynomial<T, Vec<&'a [T; M]>>
{
    fn from(p: Polynomial<T, &[&'a [T; M]]>) -> Self
    {
        Self::new(
            p.c.to_vec()
        )
    }
}
impl<T1, T2, const M: usize> From<Polynomial<T1, &[&[T1; M]]>> for Polynomial<T2, Vec<Vec<T2>>>
where
    T1: Zero + Clone + Into<T2>
{
    fn from(p: Polynomial<T1, &[&[T1; M]]>) -> Self
    {
        Self::new(
            p.c.iter()
                .map(|&p| p.as_slice()
                    .trim_zeros_front()
                    .iter()
                    .map(|c| c.clone().into())
                    .collect()
                ).collect()
        )
    }
}
impl<'a, T, const M: usize> From<Polynomial<T, &[&'a [T; M]]>> for Polynomial<T, Vec<&'a [T]>>
where
    T: Zero
{
    fn from(p: Polynomial<T, &[&'a [T; M]]>) -> Self
    {
        Self::new(
            p.c.iter()
                .map(|&p| p.as_slice().trim_zeros_front())
                .collect()
        )
    }
}

impl<T1, T2> From<Polynomial<T1, &[Vec<T1>]>> for Polynomial<T2, Vec<Vec<T2>>>
where
    T1: Clone + Into<T2>
{
    fn from(p: Polynomial<T1, &[Vec<T1>]>) -> Self
    {
        Self::new(
            p.c.iter()
                .map(|p| p.iter()
                    .map(|c| c.clone().into())
                    .collect()
                ).collect()
        )
    }
}
impl<'a, T> From<Polynomial<T, &'a [Vec<T>]>> for Polynomial<T, Vec<&'a [T]>>
where
    T: Zero
{
    fn from(p: Polynomial<T, &'a [Vec<T>]>) -> Self
    {
        Self::new(
            p.c.iter()
                .map(|p| p.as_slice().trim_zeros_front())
                .collect()
        )
    }
}

impl<T1, T2> From<Polynomial<T1, &[&[T1]]>> for Polynomial<T2, Vec<Vec<T2>>>
where
    T1: Clone + Zero + Into<T2>
{
    fn from(p: Polynomial<T1, &[&[T1]]>) -> Self
    {
        Self::new(
            p.c.iter()
                .map(|&p| p.trim_zeros_front()
                    .iter()
                    .map(|c| c.clone().into())
                    .collect()
                ).collect()
        )
    }
}
impl<'a, T> From<Polynomial<T, &[&'a [T]]>> for Polynomial<T, Vec<&'a [T]>>
where
    T: Zero
{
    fn from(p: Polynomial<T, &[&'a [T]]>) -> Self
    {
        Self::new(
            p.c.iter()
                .map(|&p| p.trim_zeros_front())
                .collect()
        )
    }
}

impl<'a, T, C1, C2> From<&'a Polynomial<T, C1>> for Polynomial<T, C2>
where
    C1: MaybeLists<T>,
    C2: MaybeLists<T>,
    C1::View<'a>: MaybeLists<T>,
    Polynomial<T, C1::View<'a>>: Into<Polynomial<T, C2>>
{
    fn from(p: &'a Polynomial<T, C1>) -> Self
    {
        Polynomial::new(p.c.as_view()).into()
    }
}