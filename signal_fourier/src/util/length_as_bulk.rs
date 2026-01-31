use core::marker::Destruct;

use array_trait::{AsSlice, length::Length, same::Same};
use bulks::{AsBulk, InplaceBulk, IntoBulk, RandomAccessBulk};

pub const trait LengthAsBulk: Length
{
    type Bulk<'a>: RandomAccessBulk<Item = &'a Self::Elem, ItemPointee = Self::Elem> + const Destruct
    where
        Self: 'a;
    type BulkMut<'a>: InplaceBulk<Item = &'a mut Self::Elem, ItemPointee = Self::Elem> + const Destruct
    where
        Self: 'a;
    
    fn as_bulk<'a>(&'a self) -> Self::Bulk<'a>
    where
        Self: 'a;
    fn as_bulk_mut<'a>(&'a mut self) -> Self::BulkMut<'a>
    where
        Self: 'a;
}
impl<L> const LengthAsBulk for L
where
    L: Length + ~const AsSlice + ?Sized
{
    default type Bulk<'a> = <&'a [L::Elem] as IntoBulk>::IntoBulk
    where
        Self: 'a;
    default type BulkMut<'a> = <&'a mut [L::Elem] as IntoBulk>::IntoBulk
    where
        Self: 'a;
    
    default fn as_bulk<'a>(&'a self) -> Self::Bulk<'a>
    where
        Self: 'a
    {
        self.as_slice().bulk().same().ok().unwrap()
    }
    default fn as_bulk_mut<'a>(&'a mut self) -> Self::BulkMut<'a>
    where
        Self: 'a
    {
        self.as_mut_slice().bulk_mut().same().ok().unwrap()
    }
}
impl<T, const N: usize> const LengthAsBulk for [T; N]
{
    type Bulk<'a> = <&'a [T; N] as IntoBulk>::IntoBulk
    where
        Self: 'a;
    type BulkMut<'a> = <&'a mut [T; N] as IntoBulk>::IntoBulk
    where
        Self: 'a;
    
    fn as_bulk<'a>(&'a self) -> Self::Bulk<'a>
    where
        Self: 'a
    {
        self.bulk()
    }
    fn as_bulk_mut<'a>(&'a mut self) -> Self::BulkMut<'a>
    where
        Self: 'a
    {
        self.bulk_mut()
    }
}
impl<T> const LengthAsBulk for [T]
{
    type Bulk<'a> = <&'a [T] as IntoBulk>::IntoBulk
    where
        Self: 'a;
    type BulkMut<'a> = <&'a mut [T] as IntoBulk>::IntoBulk
    where
        Self: 'a;
    
    fn as_bulk<'a>(&'a self) -> Self::Bulk<'a>
    where
        Self: 'a
    {
        self.bulk()
    }
    fn as_bulk_mut<'a>(&'a mut self) -> Self::BulkMut<'a>
    where
        Self: 'a
    {
        self.bulk_mut()
    }
}