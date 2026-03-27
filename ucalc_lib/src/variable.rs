use crate::Number;
use crate::parse::Tokens;
use std::num::NonZeroU8;
use std::ops::{Deref, DerefMut};
use ucalc_numbers::Constant;
#[derive(Debug, Clone, PartialEq)]
pub struct Variable {
    pub name: Option<Box<str>>,
    pub value: Number,
}
#[derive(Debug, Clone, PartialEq)]
pub struct FunctionVar {
    pub name: Option<Box<str>>,
    pub tokens: Tokens,
    pub inputs: NonZeroU8,
}
impl FunctionVar {
    pub fn new(name: impl Into<Box<str>>, inputs: NonZeroU8, tokens: Tokens) -> Self {
        Self {
            name: Some(name.into()),
            inputs,
            tokens,
        }
    }
    pub fn null(inputs: NonZeroU8, tokens: Tokens) -> Self {
        Self {
            name: None,
            inputs,
            tokens,
        }
    }
}
impl Variable {
    pub fn new(name: impl Into<Box<str>>, value: Number) -> Self {
        Self {
            name: Some(name.into()),
            value,
        }
    }
    pub fn null(value: Number) -> Self {
        Self { name: None, value }
    }
}
#[derive(Debug, Clone, PartialEq)]
#[repr(transparent)]
pub struct Variables(pub Vec<Variable>);
impl Variables {
    pub fn position(&self, name: &str) -> Option<u16> {
        self.iter()
            .position(|v| v.name.as_ref().is_some_and(|n| n.as_ref() == name))
            .map(|i| i as u16)
    }
    pub fn get_mut(&mut self, name: &str) -> &mut Variable {
        self.iter_mut()
            .find(|v| v.name.as_ref().is_some_and(|n| n.as_ref() == name))
            .unwrap()
    }
}
#[derive(Default, Debug, Clone, PartialEq)]
#[repr(transparent)]
pub struct Functions(pub Vec<FunctionVar>);
impl Functions {
    pub fn position(&self, name: &str) -> Option<u16> {
        self.iter()
            .position(|v| v.name.as_ref().is_some_and(|n| n.as_ref() == name))
            .map(|i| i as u16)
    }
    pub fn add(&mut self, vars: &mut Variables, name: &str, inputs: NonZeroU8) {
        vars.iter_mut().for_each(|v| {
            if v.name.as_ref().is_some_and(|n| n.as_ref() == name) {
                v.name = None;
            }
        });
        if let Some(v) = self.position(name) {
            self[v as usize].inputs = inputs;
        } else {
            self.push(FunctionVar::new(name, inputs, Tokens::default()));
        }
    }
}
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
            Variable::new("true", Number::from(true)),
            Variable::new("false", Number::from(false)),
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
