#![feature(if_let_guard)]
#![feature(iter_advance_by)]
mod compute;
mod parse;
#[cfg(test)]
mod tests;
mod variable;
pub use parse::Parsed;
pub use variable::{FunctionVar, Functions, InnerVariable, InnerVariables, Variable, Variables};
