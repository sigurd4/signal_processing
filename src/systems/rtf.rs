use num::{complex::ComplexFloat};
use option_trait::Maybe;

use crate::{System};
pub struct Rtf<'a, W, S>
where
    W: ComplexFloat<Real = <S::Domain as ComplexFloat>::Real>,
    S::Domain: Into<W>,
    S: System
{
    pub sys: &'a S,
    pub w: Vec<W>
}

impl<'a, W, S> Rtf<'a, W, S>
where
    W: ComplexFloat<Real = <S::Domain as ComplexFloat>::Real>,
    S::Domain: Into<W>,
    S: System
{
    pub fn new<WW: Maybe<Vec<W>>>(sys: &'a S, w: WW) -> Self
    {
        Rtf {
            sys,
            w: w.into_option()
                .unwrap_or_else(std::vec::Vec::new)
        }
    }
}