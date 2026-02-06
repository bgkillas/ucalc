#![feature(if_let_guard)]
#![feature(iter_advance_by)]
mod cas;
mod compute;
mod functions;
mod inverse;
mod operators;
mod parse;
mod polynomial;
mod polynomial_impls;
#[cfg(test)]
mod tests;
mod variable;
pub use functions::Function;
pub use operators::Operators;
pub use parse::{Token, Tokens};
pub use variable::{FunctionVar, Functions, Variable, Variables};
#[cfg(not(feature = "complex"))]
pub type NumberBase = ucalc_numbers::Float;
#[cfg(feature = "complex")]
pub type NumberBase = ucalc_numbers::Complex;
pub type Number = NumberBase;
