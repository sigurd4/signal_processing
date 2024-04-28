use core::{iter::Sum, ops::{AddAssign, DivAssign, MulAssign, SubAssign}};

use num::{traits::FloatConst, Float};

use crate::{ContainerOrSingle, FiltIC, Filter, ListOrSingle, Lists, OwnedList, OwnedListOrSingle, Tf};


pub trait QSpline1d<T>: Lists<T>
where
    T: Float
{
    fn qspline_1d(self) -> Self::Mapped<T>;
}

impl<T, X> QSpline1d<T> for X
where
    T: Float + FloatConst + MulAssign + Sum + AddAssign + SubAssign + DivAssign,
    X: Lists<T>,
    X::RowOwned: OwnedList<T>,
    X::RowsMapped<<X::RowOwned as ContainerOrSingle<T>>::Mapped<T>>: Into<X::Mapped<T>>
{
    fn qspline_1d(self) -> X::Mapped<T>
    {
        let one = T::one();
        let zero = T::zero();

        let zi = -T::from(3).unwrap() + T::from(2).unwrap()*T::SQRT_2();
        self.map_rows_into_owned(|mut x| {
            let k = x.length();
            
            let mut zik = one;
            let powers: Vec<_> = (0..k).map(|_| {
                let zikk = zik;
                zik *= zi;
                zikk
            }).collect();

            let xx = x.as_mut_slice();
            
            let xp = xx.iter()
                .zip(powers)
                .map(|(&x, p)| x*p)
                .sum::<T>();

            if k == 1
            {
                let yplus = xx[0] + zi*xp;
                let output = zi/(zi - one)*yplus;
                return x.map_to_owned(|_| output)
            }

            let ba = Tf::new((), [one, -zi]);
            let state = ba.as_view()
                .filtic([xp], [zero]);
            let mut yplus = ba.as_view()
                .filter(xx as &[T], state);

            let out_last = zi/(zi - one)*yplus.pop().unwrap();
            yplus.reverse();
            
            let ba = Tf::new([-zi], ba.a.into_inner());
            let state = ba.as_view()
                .filtic([out_last], [zero]);
            let mut y = ba.filter(yplus, state);
            y.reverse();
            y.push(out_last);

            let mut y = y.into_iter();
            x.map_to_owned(|_| y.next().unwrap()*T::from(8u8).unwrap())
        }).into()
    }
}