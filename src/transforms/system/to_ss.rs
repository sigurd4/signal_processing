use ndarray::Array2;
use num::complex::ComplexFloat;
use option_trait::Maybe;

use crate::{quantities::{MaybeList, MaybeLists, MaybeOwnedList}, operations::Simplify, systems::{Sos, Ss, SsAMatrix, SsBMatrix, SsCMatrix, SsDMatrix, Tf, Zpk}, System, transforms::system::ToTf};

/*#[derive(Debug, Clone, Copy, PartialEq, Error)]
pub enum ToSsError
{
    #[error("Non causal transfer function, i.e. it contains one or more poles at infinity.")]
    NonCausal
}*/

pub trait ToSs<T, A, B, C, D>: System
where
    T: ComplexFloat,
    A: SsAMatrix<T, B, C, D>,
    B: SsBMatrix<T, A, C, D>,
    C: SsCMatrix<T, A, B, D>,
    D: SsDMatrix<T, A, B, C>
{
    fn to_ss(self) -> Ss<T, A, B, C, D>;
}

impl<T1, A1, B1, C1, D1, T2, A2, B2, C2, D2> ToSs<T2, A2, B2, C2, D2> for Ss<T1, A1, B1, C1, D1>
where
    T1: ComplexFloat + Clone + Into<T2>,
    T2: ComplexFloat,
    A1: SsAMatrix<T1, B1, C1, D1>,
    A2: SsAMatrix<T2, B2, C2, D2>,
    B1: SsBMatrix<T1, A1, C1, D1>,
    B2: SsBMatrix<T2, A2, C2, D2>,
    C1: SsCMatrix<T1, A1, B1, D1>,
    C2: SsCMatrix<T2, A2, B2, D2>,
    D1: SsDMatrix<T1, A1, B1, C1>,
    D2: SsDMatrix<T2, A2, B2, C2>,
    A1::Mapped<T2>: Into<A2>,
    B1::Mapped<T2>: Into<B2>,
    C1::Mapped<T2>: Into<C2>,
    D1::Mapped<T2>: Into<D2>,
{
    fn to_ss(self) -> Ss<T2, A2, B2, C2, D2>
    {
        Ss::new(
            self.a.map_into_owned(|x| x.into()).into(),
            self.b.map_into_owned(|x| x.into()).into(),
            self.c.map_into_owned(|x| x.into()).into(),
            self.d.map_into_owned(|x| x.into()).into()
        )
    }
}

impl<T1, T2, B, A> ToSs<T2, Array2<T2>, Array2<T2>, Array2<T2>, Array2<T2>> for Tf<T1, B, A>
where
    T1: ComplexFloat,
    T2: ComplexFloat + Default,
    B: MaybeLists<T1>,
    A: MaybeList<T1>,
    Self: ToTf<T2, Vec<Vec<T2>>, Vec<T2>, (), ()>,
    Tf<T2, Vec<Vec<T2>>, Vec<T2>>: Simplify<Output = Tf<T2, Vec<Vec<T2>>, Vec<T2>>>,
    Array2<T2>: SsAMatrix<T2, Array2<T2>, Array2<T2>, Array2<T2>> + SsBMatrix<T2, Array2<T2>, Array2<T2>, Array2<T2>> + SsCMatrix<T2, Array2<T2>, Array2<T2>, Array2<T2>>+ SsDMatrix<T2, Array2<T2>, Array2<T2>, Array2<T2>>
{
    fn to_ss(self) -> Ss<T2, Array2<T2>, Array2<T2>, Array2<T2>, Array2<T2>>
    {
        let tf: Tf<T2, Vec<Vec<T2>>, Vec<T2>> = self.to_tf((), ());
        let mut tf = tf.simplify();
        let m_min = tf.b.iter()
            .map(|b| b.len())
            .min()
            .unwrap_or(0);
        let k = tf.a.len();
        if m_min > k
        {
            //return Err(ToSsError::NonCausal)
        }
    
        if m_min == 0 || k == 0
        {
            return Ss::new(
                Array2::default((0, 0)),
                Array2::default((0, 0)),
                Array2::default((0, 0)),
                Array2::default((0, 0))
            )
        }
    
        for b in tf.b.iter_mut()
        {
            let m = b.len();
            if k > m
            {
                let mut b_ = vec![T2::zero(); k - m];
                b_.append(b);
                b.append(&mut b_)
            }
        }
    
        let d = if k > 0
        {
            Array2::from_shape_fn((tf.b.len(), 1), |(i, _)| tf.b[i][0])
        }
        else
        {
            Array2::default((1, 1))
        };
    
        if k == 1
        {
            return Ss::new(
                Array2::default((1, 1)),
                Array2::default((1, k)),
                Array2::default((tf.b.len(), 1)),
                d
            )
        }
    
        let a = Array2::from_shape_fn((k - 1, k - 1), |(i, j)| if i == 0
        {
            -tf.a[j + 1]
        }
        else
        {
            T2::from((1 + j == i) as u8).unwrap()
        });
        let b = Array2::from_shape_fn((k - 1, 1), |(i, j)| T2::from((j == i) as u8).unwrap());
        let c = Array2::from_shape_fn((tf.b.len(), k - 1), |(i, j)| tf.b[i][j + 1] - tf.b[i][0]*tf.a[j + 1]);
    
        Ss::new(
            a,
            b,
            c,
            d
        )
    }
}

impl<T1, T2, Z, P, K> ToSs<T2, Array2<T2>, Array2<T2>, Array2<T2>, Array2<T2>> for Zpk<T1, Z, P, K>
where
    T1: ComplexFloat,
    T2: ComplexFloat,
    K: ComplexFloat<Real = T1::Real>,
    Z: MaybeList<T1>,
    P: MaybeList<T1>,
    Self: ToTf<T2, Vec<T2>, Vec<T2>, (), ()>,
    Tf<T2, Vec<T2>, Vec<T2>>: ToSs<T2, Array2<T2>, Array2<T2>, Array2<T2>, Array2<T2>>,
    Array2<T2>: SsAMatrix<T2, Array2<T2>, Array2<T2>, Array2<T2>> + SsBMatrix<T2, Array2<T2>, Array2<T2>, Array2<T2>> + SsCMatrix<T2, Array2<T2>, Array2<T2>, Array2<T2>>+ SsDMatrix<T2, Array2<T2>, Array2<T2>, Array2<T2>>
{
    fn to_ss(self) -> Ss<T2, Array2<T2>, Array2<T2>, Array2<T2>, Array2<T2>>
    {
        self.to_tf((), ())
            .to_ss()
    }
}

impl<'a, T1, T2, B, A, S> ToSs<T2, Array2<T2>, Array2<T2>, Array2<T2>, Array2<T2>> for Sos<T1, B, A, S>
where
    T1: ComplexFloat,
    T2: ComplexFloat,
    B: Maybe<[T1; 3]> + MaybeOwnedList<T1>,
    A: Maybe<[T1; 3]> + MaybeOwnedList<T1>,
    S: MaybeList<Tf<T1, B, A>>,
    Self: ToTf<T2, Vec<T2>, Vec<T2>, (), ()>,
    Tf<T2, Vec<T2>, Vec<T2>>: ToSs<T2, Array2<T2>, Array2<T2>, Array2<T2>, Array2<T2>>,
    Array2<T2>: SsAMatrix<T2, Array2<T2>, Array2<T2>, Array2<T2>> + SsBMatrix<T2, Array2<T2>, Array2<T2>, Array2<T2>> + SsCMatrix<T2, Array2<T2>, Array2<T2>, Array2<T2>>+ SsDMatrix<T2, Array2<T2>, Array2<T2>, Array2<T2>>
{
    fn to_ss(self) -> Ss<T2, Array2<T2>, Array2<T2>, Array2<T2>, Array2<T2>>
    {
        self.to_tf((), ())
            .to_ss()
    }
}