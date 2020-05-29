#![no_std]

#[cfg(feature = "std")]
extern crate std;

mod scal;
pub use scal::*;

mod vec;
pub use vec::*;
