#![feature(f16)]
#![feature(f128)]
#![feature(float_gamma)]
#![feature(float_erf)]
#![feature(min_specialization)]
#![allow(internal_features)]
#![feature(rustc_attrs)]
#![feature(ptr_cast_slice)]
#![feature(const_trait_impl)]
#![feature(const_convert)]
#[cfg(feature = "float")]
mod float;
#[cfg(feature = "float_rand")]
pub use rand;
#[cfg(feature = "float")]
#[cfg(test)]
mod float_test;
#[cfg(feature = "float")]
mod integer;
#[cfg(feature = "rug")]
pub mod rug;
pub use traits::*;
pub use types::*;
mod impls;
mod traits;
mod types;
mod units;
