use core::ops::{Neg};
use num_traits::{Num};


pub trait Scal: Clone + PartialEq + Num + Neg<Output=Self> {}
impl<T: Clone + PartialEq + Num + Neg<Output=T>> Scal for T {}
