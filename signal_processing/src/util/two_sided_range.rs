use core::ops::{Range, RangeBounds, RangeInclusive};

pub trait TwoSidedRange<T: ?Sized>: RangeBounds<T>
{
    fn start(&self) -> &T;
    fn end(&self) -> &T;
    fn is_end_inclusive(&self) -> bool;
}

impl<T> TwoSidedRange<T> for Range<T> where Self: RangeBounds<T>
{
    fn end(&self) -> &T
    {
        &self.end
    }
    fn start(&self) -> &T
    {
        &self.start
    }
    fn is_end_inclusive(&self) -> bool
    {
        false
    }
}
impl<T> TwoSidedRange<T> for RangeInclusive<T> where Self: RangeBounds<T>
{
    fn end(&self) -> &T
    {
        self.end()
    }
    fn start(&self) -> &T
    {
        self.start()
    }
    fn is_end_inclusive(&self) -> bool
    {
        true
    }
}