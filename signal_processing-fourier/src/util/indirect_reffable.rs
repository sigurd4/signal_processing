use bulks::{AsBulk, Bulk, IntoBulk};

pub(crate) struct IndirectReffable<'a, T>(pub &'a mut [&'a mut T]);

fn flatten_mut<'a, T>(value: &'a mut &'a mut T) -> &'a mut T
{
    *value
}

fn flatten_ref<'a, T>(value: &'a &'a mut T) -> &'a T
{
    *value
}

impl<'a, T> IntoIterator for &'a mut IndirectReffable<'a, T>
where
    T: 'a
{
    type Item = &'a mut T;
    type IntoIter = core::iter::Map<core::slice::IterMut<'a, &'a mut T>, fn(&'a mut &'a mut T) -> &'a mut T>;
    
    fn into_iter(self) -> Self::IntoIter
    {
        let IndirectReffable(c) = self;
        (**c).iter_mut()
            .map(flatten_mut as fn(_) -> _)
    }
}
impl<'a, T> IntoBulk for &'a mut IndirectReffable<'a, T>
where
    T: 'a
{
    type IntoBulk = bulks::Map<bulks::slice::BulkMut<'a, &'a mut T>, fn(&'a mut &'a mut T) -> &'a mut T>;

    fn into_bulk(self) -> Self::IntoBulk
    {
        let IndirectReffable(c) = self;
        (**c).bulk_mut()
            .map(flatten_mut as fn(_) -> _)
    }
}
impl<'a, T> IntoIterator for &'a IndirectReffable<'a, T>
where
    T: 'a
{
    type Item = &'a T;
    type IntoIter = core::iter::Map<core::slice::Iter<'a, &'a mut T>, fn(&'a &'a mut T) -> &'a T>;
    
    fn into_iter(self) -> Self::IntoIter
    {
        let IndirectReffable(c) = self;
        (**c).iter()
            .map(flatten_ref as fn(_) -> _)
    }
}
impl<'a, T> IntoBulk for &'a IndirectReffable<'a, T>
where
    T: 'a
{
    type IntoBulk = bulks::Map<bulks::slice::Bulk<'a, &'a mut T>, fn(&'a &'a mut T) -> &'a T>;

    fn into_bulk(self) -> Self::IntoBulk
    {
        let IndirectReffable(c) = self;
        (**c).bulk()
            .map(flatten_ref as fn(_) -> _)
    }
}