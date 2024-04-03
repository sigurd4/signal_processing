use core::ops::DivAssign;

use ndarray::{Array1, Array2};
use num::{complex::ComplexFloat, Zero};

use crate::{Ss, Tf};

pub trait Normalize
{
    fn normalize(&mut self);
}

impl<T> Normalize for Tf<T, Vec<T>, Vec<T>>
where
    T: ComplexFloat + DivAssign
{
    fn normalize(&mut self)
    {
        // Trim zeros
        while self.b.first() == Some(&T::zero())
        {
            self.b.remove(0);
        }
        while self.a.first() == Some(&T::zero())
        {
            self.a.remove(0);
        }
    
        if let Some(&norm) = self.a.first()
        {
            for b in self.b.iter_mut()
            {
                *b /= norm
            }
            for a in self.a.iter_mut()
            {
                *a /= norm
            }
        }
    }
}
impl<T> Normalize for Tf<T, Vec<Vec<T>>, Vec<T>>
where
    T: ComplexFloat + DivAssign
{
    fn normalize(&mut self)
    {
        // Trim zeros
        for b in self.b.iter_mut()
        {
            while b.first() == Some(&T::zero())
            {
                b.remove(0);
            }
        }
        while self.a.first() == Some(&T::zero())
        {
            self.a.remove(0);
        }
    
        if let Some(&norm) = self.a.first()
        {
            for b in self.b.iter_mut()
            {
                for b in b.iter_mut()
                {
                    *b /= norm
                }
            }
            for a in self.a.iter_mut()
            {
                *a /= norm
            }
        }
    }
}
impl<T, const K: usize> Normalize for Tf<T, [Vec<T>; K], Vec<T>>
where
    T: ComplexFloat + DivAssign
{
    fn normalize(&mut self)
    {
        // Trim zeros
        for b in self.b.iter_mut()
        {
            while b.first() == Some(&T::zero())
            {
                b.remove(0);
            }
        }
        while self.a.first() == Some(&T::zero())
        {
            self.a.remove(0);
        }
    
        if let Some(&norm) = self.a.first()
        {
            for b in self.b.iter_mut()
            {
                for b in b.iter_mut()
                {
                    *b /= norm
                }
            }
            for a in self.a.iter_mut()
            {
                *a /= norm
            }
        }
    }
}

impl<T> Normalize for Ss<T, Array2<T>, Array2<T>, Array2<T>, Array2<T>>
where
    T: ComplexFloat
{
    fn normalize(&mut self)
    {
        let (ma, na) = self.a.dim();
        let (mb, nb) = self.b.dim();
        let (mc, nc) = self.c.dim();
        let (md, nd) = self.d.dim();

        let p = ma.max(na).max(mb).max(nc);
        let q = nb.max(nd);
        let r = mc.max(md);

        fn resize<T>(m: &mut Array2<T>, dim: (usize, usize))
        where
            T: Zero + Clone
        {
            let row = Array1::from_elem(m.dim().1, T::zero());
            while m.dim().0 < dim.0
            {
                m.push_row(row.view()).unwrap();
            }
            let col = Array1::from_elem(m.dim().0, T::zero());
            while m.dim().1 < dim.1
            {
                m.push_column(col.view()).unwrap();
            }
        }

        resize(&mut self.a, (p, p));
        resize(&mut self.b, (p, q));
        resize(&mut self.c, (r, p));
        resize(&mut self.d, (r, q));
    }
}