#![feature(if_let_guard)]
#![feature(iter_advance_by)]
mod compute;
mod functions;
mod operators;
mod parse;
#[cfg(test)]
mod tests;
mod variable;
pub use parse::Tokens;
pub use variable::{FunctionVar, Functions, InnerVariable, InnerVariables, Variable, Variables};
