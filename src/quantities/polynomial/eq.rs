use crate::quantities::{MaybeLists, Polynomial};

impl<T, C> Eq for Polynomial<T, C>
where
    C: MaybeLists<T>,
    T: Eq,
    Self: PartialEq
{

}