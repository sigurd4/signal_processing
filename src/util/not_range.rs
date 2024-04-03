use core::ops::{Range};

pub auto trait NotRange
{

}

impl<T> !NotRange for Range<T>
{

}