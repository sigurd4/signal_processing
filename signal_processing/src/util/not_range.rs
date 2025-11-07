use core::ops::{Range, RangeInclusive};

pub auto trait NotRange
{

}

impl<T> !NotRange for Range<T>
{

}
impl<T> !NotRange for RangeInclusive<T>
{

}