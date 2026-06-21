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

impl<'b, T> IntoIterator for IndirectReffable<'b, T>
where
    T: 'b
{
    type Item = &'b mut T;
    type IntoIter = core::iter::Map<core::slice::IterMut<'b, &'b mut T>, fn(&'b mut &'b mut T) -> &'b mut T>;
    
    fn into_iter(self) -> Self::IntoIter
    {
        let IndirectReffable(c) = self;
        (*c).iter_mut()
            .map(flatten_mut as fn(_) -> _)
    }
}
impl<'b, T> IntoBulk for IndirectReffable<'b, T>
where
    T: 'b
{
    type IntoBulk = bulks::Map<bulks::slice::BulkMut<'b, &'b mut T>, fn(&'b mut &'b mut T) -> &'b mut T>;

    fn into_bulk(self) -> Self::IntoBulk
    {
        let IndirectReffable(c) = self;
        (*c).bulk_mut()
            .map(flatten_mut as fn(_) -> _)
    }
}
impl<'a, 'b, T> IntoIterator for &'a mut IndirectReffable<'b, T>
where
    T: 'a,
    'a: 'b
{
    type Item = &'a mut T;
    type IntoIter = core::iter::Map<core::slice::IterMut<'b, &'b mut T>, fn(&'a mut &'b mut T) -> &'a mut T>;
    
    fn into_iter(self) -> Self::IntoIter
    {
        let IndirectReffable(c) = self;
        (**c).iter_mut()
            .map(flatten_mut as fn(_) -> _)
    }
}
impl<'a, 'b, T> IntoBulk for &'a mut IndirectReffable<'b, T>
where
    T: 'a,
    'a: 'b
{
    type IntoBulk = bulks::Map<bulks::slice::BulkMut<'b, &'b mut T>, fn(&'a mut &'b mut T) -> &'a mut T>;

    fn into_bulk(self) -> Self::IntoBulk
    {
        let IndirectReffable(c) = self;
        (**c).bulk_mut()
            .map(flatten_mut as fn(_) -> _)
    }
}
impl<'a, 'b, T> IntoIterator for &'a IndirectReffable<'b, T>
where
    T: 'a,
    'a: 'b
{
    type Item = &'a T;
    type IntoIter = core::iter::Map<core::slice::Iter<'b, &'b mut T>, fn(&'a &'b mut T) -> &'a T>;
    
    fn into_iter(self) -> Self::IntoIter
    {
        let IndirectReffable(c) = self;
        (**c).iter()
            .map(flatten_ref as fn(_) -> _)
    }
}
impl<'a, 'b, T> IntoBulk for &'a IndirectReffable<'b, T>
where
    T: 'a,
    'a: 'b
{
    type IntoBulk = bulks::Map<bulks::slice::Bulk<'b, &'b mut T>, fn(&'a &'b mut T) -> &'a T>;

    fn into_bulk(self) -> Self::IntoBulk
    {
        let IndirectReffable(c) = self;
        (**c).bulk()
            .map(flatten_ref as fn(_) -> _)
    }
}