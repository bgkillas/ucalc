use crate::parse::Tokens;
use std::f64::consts::{E, PI};
use std::ops::{Deref, DerefMut};
#[derive(Debug, PartialEq, Clone)]
pub struct Variable {
    pub name: String,
    pub value: f64,
    pub place: bool,
}
#[derive(Debug, PartialEq, Clone)]
pub struct InnerVariable {
    pub value: f64,
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
    pub fn new(name: impl Into<String>, value: f64, place: bool) -> Self {
        Self {
            name: name.into(),
            value,
            place,
        }
    }
}
impl InnerVariable {
    pub fn new(value: f64) -> Self {
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
            Variable::new("pi", PI, true),
            Variable::new("e", E, true),
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
