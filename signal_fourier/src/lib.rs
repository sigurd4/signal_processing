#![feature(unboxed_closures)]
#![feature(fn_traits)]
#![feature(const_trait_impl)]
#![feature(const_convert)]
#![feature(try_trait_v2)]
#![feature(const_option_ops)]
#![feature(const_try)]
#![feature(trusted_random_access)]
#![feature(maybe_uninit_uninit_array_transpose)]
#![feature(const_destruct)]
#![feature(const_result_trait_fn)]
#![feature(macro_metavar_expr_concat)]
#![feature(const_ops)]
#![feature(str_as_str)]
#![feature(generic_const_exprs)]
#![feature(core_intrinsics)]
#![feature(specialization)]

use core::{marker::Destruct, ops::{Add, AddAssign, Mul, MulAssign}};

use array_trait::{AsSlice, length::{self, Length, LengthValue}, same::Same};
use bulks::{AsBulk, Bulk, InplaceBulk, IntoBulk, RandomAccessBulk};

#[cfg(not(any(feature = "std", feature = "libm")))]
compile_error!("Either the \"std\" or the \"libm\" feature must be enabled to compile");

const trait AddAssignSpec: ~const Add<Output = Self> + Copy
{
    fn _add_assign(&mut self, rhs: Self);
}
impl<T> const AddAssignSpec for T
where
    T: ~const Add<Output = Self> + Copy
{
    default fn _add_assign(&mut self, rhs: Self)
    {
        *self = *self + rhs
    }
}
impl<T> const AddAssignSpec for T
where
    T: ~const Add<Output = Self> + Copy + ~const AddAssign
{
    fn _add_assign(&mut self, rhs: Self)
    {
        *self += rhs
    }
}

const trait MulAssignSpec: ~const Mul<Output = Self> + Copy
{
    fn _mul_assign(&mut self, rhs: Self);
}
impl<T> const MulAssignSpec for T
where
    T: ~const Mul<Output = Self> + Copy
{
    default fn _mul_assign(&mut self, rhs: Self)
    {
        *self = *self * rhs
    }
}
impl<T> const MulAssignSpec for T
where
    T: ~const Mul<Output = Self> + Copy + ~const MulAssign
{
    fn _mul_assign(&mut self, rhs: Self)
    {
        *self *= rhs
    }
}

moddef::moddef!(
    pub mod {
        fft,
        permute
    },
    flat(pub) mod {
        scratch_space
    },
    mod {
        util
    }
);

macro_rules! temp {
    ($temp:ident for $len:expr) => {
        temp!($temp for $len => $temp)
    };
    ($temp:ident for $len:expr => $x:ident) => {
        let mut ${concat($temp, _owned)} = None;
        let $x = match $temp.take()
        {
            Some($temp) => $temp,
            None => ${concat($temp, _owned)}.insert(crate::ScratchLength::scratch_space($len, num_traits::Zero::zero())).borrow_mut()
        };
    };
}
use temp as temp;

const trait LengthAsBulk: Length
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


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works()
    {
        
    }
}