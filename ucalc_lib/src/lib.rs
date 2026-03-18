#![feature(iter_advance_by)]
#![feature(vec_try_remove)]
mod cas;
mod compute;
mod functions;
mod functions_list;
mod inverse;
mod math;
mod operators;
mod parse;
mod polynomial;
mod polynomial_impls;
#[cfg(test)]
mod tests;
mod variable;
pub use functions::Function;
pub use functions_list::FUNCTION_LIST;
pub use operators::Operators;
pub use parse::{Token, Tokens};
#[cfg(feature = "units")]
use ucalc_numbers::Quantity;
pub use variable::{FunctionVar, Functions, Variable, Variables};
#[cfg(not(feature = "complex"))]
pub type NBase = ucalc_numbers::Float;
#[cfg(feature = "complex")]
pub type NBase = ucalc_numbers::Complex;
#[cfg(not(feature = "units"))]
pub type NumberBase = NBase;
#[cfg(feature = "units")]
pub type NumberBase = Quantity<NBase, f32, 8>;
#[cfg(any(feature = "list", feature = "vector", feature = "matrix",))]
pub type Number = ucalc_numbers::Number<NumberBase>;
#[cfg(not(any(feature = "list", feature = "vector", feature = "matrix",)))]
pub type Number = NumberBase;
