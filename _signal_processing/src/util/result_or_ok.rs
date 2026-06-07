pub trait ResultOrOk<T, E>
where
    T: ?Sized,
    E: ?Sized
{
    fn into_result(self) -> Result<T, E>
    where
        Self: Sized,
        T: Sized,
        E: Sized;
    fn as_result(&self) -> Result<&T, &E>;
    fn as_result_mut(&mut self) -> Result<&mut T, &mut E>;
}

impl<T, E> ResultOrOk<T, E> for T
{
    fn into_result(self) -> Result<T, E>
    where
        Self: Sized,
        T: Sized,
        E: Sized
    {
        Ok(self)
    }
    fn as_result(&self) -> Result<&T, &E>
    {
        Ok(self)
    }
    fn as_result_mut(&mut self) -> Result<&mut T, &mut E>
    {
        Ok(self)
    }
}
impl<T, E> ResultOrOk<T, E> for Result<T, E>
{
    fn into_result(self) -> Result<T, E>
    where
        Self: Sized,
        T: Sized,
        E: Sized
    {
        self
    }
    fn as_result(&self) -> Result<&T, &E>
    {
        self.as_ref()
    }
    fn as_result_mut(&mut self) -> Result<&mut T, &mut E>
    {
        self.as_mut()
    }
}