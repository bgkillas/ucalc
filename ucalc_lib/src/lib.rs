#![feature(iter_advance_by)]
#![feature(vec_try_remove)]
#![feature(adt_const_params)]
mod compute;
mod functions;
mod functions_list;
mod inverse;
mod math;
mod operators;
mod parse;
mod polynomial;
mod polynomial_impls;
#[cfg(feature = "float_rand")]
mod rand;
mod simplify;
mod solver;
#[cfg(test)]
mod tests;
mod variable;
pub use compute::Compute;
pub use functions::Function;
pub use functions_list::{FUNCTION_LIST, get_help};
pub use operators::Operator;
pub use parse::{Derivative, Token, Tokens, TokensSlice, Volatility};
#[cfg(feature = "float_rand")]
pub use rand::{Rand, rng};
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
const UNIT_COUNT: usize = 9;
#[cfg(feature = "units")]
pub const UNITS: [&str; UNIT_COUNT] = ["s", "m", "g", "A", "K", "mol", "cd", "rad", "USD"];
#[cfg(feature = "units")]
type UnitType = f32;
#[cfg(feature = "units")]
pub type NumberBase = Quantity<NBase, UnitType, UNIT_COUNT>;
#[cfg(any(
    feature = "list",
    feature = "vector",
    feature = "matrix",
    feature = "units"
))]
#[cfg(not(feature = "units"))]
pub type Number = ucalc_numbers::Number<NumberBase>;
#[cfg(any(
    feature = "list",
    feature = "vector",
    feature = "matrix",
    feature = "units"
))]
#[cfg(feature = "units")]
pub type Number = ucalc_numbers::Number<NumberBase, UnitType, UNIT_COUNT>;
#[cfg(not(any(
    feature = "list",
    feature = "vector",
    feature = "matrix",
    feature = "units"
)))]
pub type Number = NumberBase;
