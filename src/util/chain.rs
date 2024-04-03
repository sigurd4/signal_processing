use array_math::ArrayOps;

pub trait Chain<Rhs>
{
    type Output;

    fn chain(self, rhs: Rhs) -> Self::Output;
}

impl Chain<()> for ()
{
    type Output = ();

    fn chain(self, (): ()) -> Self::Output
    {
        
    }
}
impl<T, const M: usize> Chain<[T; M]> for ()
{
    type Output = [T; M];

    fn chain(self, rhs: [T; M]) -> Self::Output
    {
        rhs
    }
}
impl<'a, T, const M: usize> Chain<&'a [T; M]> for ()
{
    type Output = &'a [T; M];

    fn chain(self, rhs: &'a [T; M]) -> Self::Output
    {
        rhs
    }
}
impl<T> Chain<Vec<T>> for ()
{
    type Output = Vec<T>;

    fn chain(self, rhs: Vec<T>) -> Self::Output
    {
        rhs
    }
}
impl<'a, T> Chain<&'a [T]> for ()
{
    type Output = &'a [T];

    fn chain(self, rhs: &'a [T]) -> Self::Output
    {
        rhs
    }
}

impl<T, const N: usize> Chain<()> for [T; N]
{
    type Output = [T; N];

    fn chain(self, (): ()) -> Self::Output
    {
        self
    }
}
impl<T, const N: usize, const M: usize> Chain<[T; M]> for [T; N]
where
    [(); N + M]:
{
    type Output = [T; N + M];

    fn chain(self, rhs: [T; M]) -> Self::Output
    {
        ArrayOps::chain(self, rhs)
    }
}
impl<'a, T, const N: usize, const M: usize> Chain<&'a [T; M]> for [T; N]
where
    T: Clone,
    [(); N + M]:
{
    type Output = [T; N + M];

    fn chain(self, rhs: &'a [T; M]) -> Self::Output
    {
        ArrayOps::chain(self, rhs.clone())
    }
}
impl<T, const N: usize> Chain<Vec<T>> for [T; N]
{
    type Output = Vec<T>;

    fn chain(self, mut rhs: Vec<T>) -> Self::Output
    {
        let mut v = self.into_iter().collect::<Vec<_>>();
        v.append(&mut rhs);
        v
    }
}
impl<'a, T, const N: usize> Chain<&'a [T]> for [T; N]
where
    T: Clone
{
    type Output = Vec<T>;

    fn chain(self, rhs: &'a [T]) -> Self::Output
    {
        let mut v = self.into_iter().collect::<Vec<_>>();
        v.append(&mut rhs.to_vec());
        v
    }
}

impl<'a, T, const N: usize> Chain<()> for &'a [T; N]
{
    type Output = &'a [T; N];

    fn chain(self, (): ()) -> Self::Output
    {
        self
    }
}
impl<T, const N: usize, const M: usize> Chain<[T; M]> for &[T; N]
where
    T: Clone,
    [(); N + M]:
{
    type Output = [T; N + M];

    fn chain(self, rhs: [T; M]) -> Self::Output
    {
        ArrayOps::chain(self.clone(), rhs)
    }
}
impl<'a, T, const N: usize, const M: usize> Chain<&'a [T; M]> for &[T; N]
where
    T: Clone,
    [(); N + M]:
{
    type Output = [T; N + M];

    fn chain(self, rhs: &'a [T; M]) -> Self::Output
    {
        self.chain(rhs.clone())
    }
}
impl<T, const N: usize> Chain<Vec<T>> for &[T; N]
where
    T: Clone
{
    type Output = Vec<T>;

    fn chain(self, mut rhs: Vec<T>) -> Self::Output
    {
        let mut v = self.to_vec();
        v.append(&mut rhs);
        v
    }
}
impl<'a, T, const N: usize> Chain<&'a [T]> for &[T; N]
where
    T: Clone
{
    type Output = Vec<T>;

    fn chain(self, rhs: &'a [T]) -> Self::Output
    {
        let mut v = self.to_vec();
        v.append(&mut rhs.to_vec());
        v
    }
}

impl<T> Chain<()> for Vec<T>
{
    type Output = Vec<T>;

    fn chain(self, (): ()) -> Self::Output
    {
        self
    }
}
impl<T, const M: usize> Chain<[T; M]> for Vec<T>
{
    type Output = Vec<T>;

    fn chain(mut self, rhs: [T; M]) -> Self::Output
    {
        self.append(&mut rhs.into_iter().collect());
        self
    }
}
impl<'a, T, const M: usize> Chain<&'a [T; M]> for Vec<T>
where
    T: Clone
{
    type Output = Vec<T>;

    fn chain(mut self, rhs: &'a [T; M]) -> Self::Output
    {
        self.append(&mut rhs.to_vec());
        self
    }
}
impl<T> Chain<Vec<T>> for Vec<T>
{
    type Output = Vec<T>;

    fn chain(mut self, mut rhs: Vec<T>) -> Self::Output
    {
        self.append(&mut rhs);
        self
    }
}
impl<'a, T> Chain<&'a [T]> for Vec<T>
where
    T: Clone
{
    type Output = Vec<T>;

    fn chain(mut self, rhs: &'a [T]) -> Self::Output
    {
        self.append(&mut rhs.to_vec());
        self
    }
}

impl<'a, T> Chain<()> for &'a [T]
{
    type Output = &'a [T];

    fn chain(self, (): ()) -> Self::Output
    {
        self
    }
}
impl<T, const M: usize> Chain<[T; M]> for &[T]
where
    T: Clone
{
    type Output = Vec<T>;

    fn chain(self, rhs: [T; M]) -> Self::Output
    {
        let mut v = self.to_vec();
        v.append(&mut rhs.into_iter().collect());
        v
    }
}
impl<'a, T, const M: usize> Chain<&'a [T; M]> for &[T]
where
    T: Clone
{
    type Output = Vec<T>;

    fn chain(self, rhs: &'a [T; M]) -> Self::Output
    {
        let mut v = self.to_vec();
        v.append(&mut rhs.to_vec());
        v
    }
}
impl<T> Chain<Vec<T>> for &[T]
where
    T: Clone
{
    type Output = Vec<T>;

    fn chain(self, mut rhs: Vec<T>) -> Self::Output
    {
        let mut v = self.to_vec();
        v.append(&mut rhs);
        v
    }
}
impl<'a, T> Chain<&'a [T]> for &[T]
where
    T: Clone
{
    type Output = Vec<T>;

    fn chain(self, rhs: &'a [T]) -> Self::Output
    {
        let mut v = self.to_vec();
        v.append(&mut rhs.to_vec());
        v
    }
}