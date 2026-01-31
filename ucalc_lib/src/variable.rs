use crate::parse::Tokens;
use std::ops::{Deref, DerefMut};
use ucalc_numbers::{Complex, Constant};
#[derive(Debug, PartialEq, Clone)]
pub struct Variable {
    pub name: String,
    pub value: Complex,
    pub place: bool,
}
#[derive(Debug, PartialEq, Clone)]
pub struct InnerVariable {
    pub value: Complex,
}
#[derive(Debug, PartialEq, Clone)]
pub struct FunctionVar {
    pub name: String,
    pub vars: InnerVariables,
    pub tokens: Tokens,
}
impl FunctionVar {
    pub fn new(name: impl Into<String>, vars: InnerVariables, tokens: Tokens) -> Self {
        Self {
            name: name.into(),
            vars,
            tokens,
        }
    }
}
impl Variable {
    pub fn new(name: impl Into<String>, value: Complex, place: bool) -> Self {
        Self {
            name: name.into(),
            value,
            place,
        }
    }
}
impl InnerVariable {
    pub fn new(value: Complex) -> Self {
        Self { value }
    }
}
#[derive(Debug, PartialEq, Clone)]
pub struct Variables(pub Vec<Variable>);
#[derive(Default, Debug, PartialEq, Clone)]
pub struct InnerVariables(pub Vec<InnerVariable>);
#[derive(Default, Debug, PartialEq, Clone)]
pub struct Functions(pub Vec<FunctionVar>);
impl Default for Variables {
    fn default() -> Self {
        Self(vec![
            Variable::new("pi", Complex::from(Constant::Pi), true),
            Variable::new("tau", Complex::from(Constant::Tau), true),
            Variable::new("e", Complex::from(Constant::E), true),
            Variable::new("i", Complex::from((0, 1)), true),
            Variable::new("inf", Complex::from(Constant::Infinity), true),
            Variable::new("nan", Complex::from(Constant::Nan), true),
        ])
    }
}
impl Deref for Variables {
    type Target = Vec<Variable>;
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl DerefMut for Variables {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}
impl Deref for InnerVariables {
    type Target = Vec<InnerVariable>;
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl DerefMut for InnerVariables {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}
impl Deref for Functions {
    type Target = Vec<FunctionVar>;
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl DerefMut for Functions {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}
