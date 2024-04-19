use core::ops::{Range, RangeBounds, RangeInclusive};

pub trait TwoSidedRange<T: ?Sized>: RangeBounds<T> {}

impl<T> TwoSidedRange<T> for Range<T> where Self: RangeBounds<T> {}
impl<T> TwoSidedRange<T> for RangeInclusive<T> where Self: RangeBounds<T> {}