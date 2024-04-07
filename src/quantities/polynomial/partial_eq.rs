use array_math::SliceMath;
use num::Zero;

use crate::{MaybeLists, Polynomial};

impl<T, C1, C2> PartialEq<Polynomial<T, C2>> for Polynomial<T, C1>
where
    T: Zero + PartialEq,
    C1: MaybeLists<T>,
    C2: MaybeLists<T>
{
    fn eq(&self, other: &Polynomial<T, C2>) -> bool
    {
        let a = self.as_view_slices_option();
        let b = other.as_view_slices_option();
        
        if a.is_some() != b.is_some()
        {
            return false
        }
        
        if let Some(a) = a && let Some(b) = b
        {
            if a.len() != b.len()
            {
                return false
            }

            return a.into_iter()
                .zip(b.into_iter())
                .all(|(a, b)| a.trim_zeros_front() == b.trim_zeros_front())
        }
        true
    }
}