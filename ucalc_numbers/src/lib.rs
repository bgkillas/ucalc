#![feature(f16)]
#![feature(f128)]
#![feature(float_gamma)]
#![feature(float_erf)]
#![feature(min_specialization)]
#![allow(internal_features)]
#![feature(rustc_attrs)]
#![feature(dec2flt)]
#[cfg(feature = "float")]
mod float;
#[cfg(feature = "float")]
#[cfg(test)]
mod float_test;
#[cfg(feature = "rug")]
pub mod rug;
pub use traits::*;
pub use types::*;
mod impls;
mod traits;
mod types;
