#![feature(if_let_guard)]
#![feature(iter_advance_by)]
mod cas;
mod compute;
mod functions;
mod operators;
mod parse;
mod polynomial;
#[cfg(test)]
mod tests;
mod variable;
pub use functions::Function;
pub use operators::Operators;
pub use parse::{Token, Tokens};
pub use ucalc_numbers::Complex;
pub use variable::{FunctionVar, Functions, Variable, Variables};
