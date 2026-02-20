#![feature(iter_advance_by)]
#![feature(vec_try_remove)]
mod cas;
mod compute;
mod functions;
mod inverse;
mod operators;
mod polynomial;
mod polynomial_impls;
#[cfg(test)]
mod tests;
mod tokens;
mod variable;
pub use functions::Function;
pub use operators::Operators;
pub use tokens::{Token, Tokens};
pub use variable::{FunctionVar, Functions, Variable, Variables};
#[cfg(not(feature = "complex"))]
pub type NumberBase = ucalc_numbers::Float;
#[cfg(feature = "complex")]
pub type NumberBase = ucalc_numbers::Complex;
#[cfg(any(
    feature = "list",
    feature = "vector",
    feature = "matrix",
    feature = "units"
))]
pub type Number = ucalc_numbers::Number<NumberBase>;
#[cfg(not(any(
    feature = "list",
    feature = "vector",
    feature = "matrix",
    feature = "units"
)))]
pub type Number = NumberBase;
