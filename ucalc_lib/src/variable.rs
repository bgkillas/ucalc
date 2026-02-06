use crate::Number;
use crate::parse::Tokens;
use std::ops::{Deref, DerefMut};
use ucalc_numbers::Constant;
#[derive(Debug, PartialEq, Clone)]
pub struct Variable {
    pub name: String,
    pub value: Number,
}
#[derive(Debug, PartialEq, Clone)]
pub struct FunctionVar {
    pub name: String,
    pub inputs: usize,
    pub tokens: Tokens,
}
impl FunctionVar {
    pub fn new(name: impl Into<String>, inputs: usize, tokens: Tokens) -> Self {
        Self {
            name: name.into(),
            inputs,
            tokens,
        }
    }
}
impl Variable {
    pub fn new(name: impl Into<String>, value: Number) -> Self {
        Self {
            name: name.into(),
            value,
        }
    }
}
#[derive(Debug, PartialEq, Clone)]
pub struct Variables(pub Vec<Variable>);
#[derive(Default, Debug, PartialEq, Clone)]
pub struct Functions(pub Vec<FunctionVar>);
impl Default for Variables {
    fn default() -> Self {
        Self(vec![
            Variable::new("pi", Number::from(Constant::Pi)),
            Variable::new("tau", Number::from(Constant::Tau)),
            Variable::new("e", Number::from(Constant::E)),
            #[cfg(feature = "complex")]
            Variable::new("i", Number::from((0, 1))),
            Variable::new("inf", Number::from(Constant::Infinity)),
            Variable::new("nan", Number::from(Constant::Nan)),
        ])
    }
}
impl Deref for Variables {
    type Target = [Variable];
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl DerefMut for Variables {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}
impl Deref for Functions {
    type Target = [FunctionVar];
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl DerefMut for Functions {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}
