use num_traits::{Num};


pub trait Scal: Num {}

impl<T: Num> Scal for T {}
