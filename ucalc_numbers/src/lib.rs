#![feature(f16)]
#![feature(f128)]
#![feature(float_gamma)]
#![feature(float_erf)]
#[cfg(feature = "float")]
mod float;
#[cfg(feature = "float")]
#[cfg(test)]
mod float_test;
#[cfg(feature = "rug")]
mod rug;
#[cfg(feature = "float")]
pub type Integer = float::Integer;
#[cfg(feature = "float")]
pub type Float = float::Float;
#[cfg(feature = "float")]
pub type Complex = float::Complex;
pub trait Pow<Rhs> {
    fn pow(self, rhs: Rhs) -> Self;
}
pub enum Constant {
    Pi,
    Tau,
    E,
    Infinity,
    NegInfinity,
    Nan,
}
