use crate::functions::Function;
use crate::parse::Token;
use std::ops::Neg;
use ucalc_numbers::{Complex, Pow};
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum Operators {
    Add,
    Sub,
    Mul,
    Div,
    Pow,
    Tetration,
    Root,
    Rem,
    Negate,
    Factorial,
    SubFactorial,
    Equal,
    NotEqual,
    Greater,
    Less,
    GreaterEqual,
    LessEqual,
    And,
    Or,
    Not,
    Bracket(Bracket),
    Function(Function),
}
impl From<Function> for Operators {
    fn from(value: Function) -> Self {
        Self::Function(value)
    }
}
impl From<Bracket> for Operators {
    fn from(value: Bracket) -> Self {
        Self::Bracket(value)
    }
}
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum Bracket {
    Absolute,
    Parenthesis,
}
impl TryFrom<&str> for Operators {
    type Error = ();
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        Ok(match value {
            "//" => Self::Root,
            "%" => Self::Rem,
            "^" | "**" => Self::Pow,
            "^^" => Self::Tetration,
            "*" => Self::Mul,
            "/" => Self::Div,
            "+" => Self::Add,
            "-" => Self::Sub,
            "_" => Self::Negate,
            "!" => Self::Factorial,
            "." => Self::SubFactorial,
            "==" => Self::Equal,
            "!=" => Self::NotEqual,
            ">" => Self::Greater,
            "<" => Self::Less,
            ">=" => Self::GreaterEqual,
            "<=" => Self::LessEqual,
            "&" => Self::And,
            "?" => Self::Or,
            "'" => Self::Not,
            "(" => Self::Bracket(Bracket::Parenthesis),
            "|" => Self::Bracket(Bracket::Absolute),
            _ => return Err(()),
        })
    }
}
impl Operators {
    pub const MAX_INPUT: usize = Function::MAX_INPUT;
    pub fn inverse(self) -> Option<Self> {
        Some(match self {
            Self::Add => Self::Sub,
            Self::Sub => Self::Add,
            Self::Mul => Self::Div,
            Self::Div => Self::Mul,
            Self::Pow => Self::Root,
            Self::Root => Self::Pow,
            Self::Negate => Self::Negate,
            Self::Function(fun) => return fun.inverse().map(|a| a.into()),
            Self::Bracket(_)
            | Self::Rem
            | Self::Factorial
            | Self::SubFactorial
            | Self::Equal
            | Self::NotEqual
            | Self::Greater
            | Self::Less
            | Self::LessEqual
            | Self::GreaterEqual
            | Self::And
            | Self::Or
            | Self::Not
            | Self::Tetration => return None,
        })
    }
    pub fn inputs(self) -> usize {
        match self {
            Self::Mul
            | Self::Div
            | Self::Add
            | Self::Sub
            | Self::Pow
            | Self::Root
            | Self::Rem
            | Self::Equal
            | Self::NotEqual
            | Self::Greater
            | Self::Less
            | Self::LessEqual
            | Self::GreaterEqual
            | Self::And
            | Self::Or
            | Self::Tetration => 2,
            Self::Negate | Self::Factorial | Self::Not | Self::SubFactorial => 1,
            Self::Function(fun) => fun.inputs(),
            Self::Bracket(_) => unreachable!(),
        }
    }
    pub fn precedence(self) -> u8 {
        match self {
            Self::Or => 0,
            Self::And => 1,
            Self::Equal
            | Self::NotEqual
            | Self::Greater
            | Self::Less
            | Self::LessEqual
            | Self::GreaterEqual => 2,
            Self::Add | Self::Sub => 3,
            Self::Mul | Self::Div => 4,
            Self::Negate | Self::Not => 5,
            Self::Pow | Self::Root | Self::Tetration => 6,
            Self::Rem => 7,
            Self::Factorial | Self::SubFactorial => 8,
            Self::Bracket(_) | Self::Function(_) => unreachable!(),
        }
    }
    pub fn left_associative(self) -> bool {
        match self {
            Self::Add | Self::Sub | Self::Mul | Self::Div | Self::Rem | Self::And | Self::Or => {
                true
            }
            Self::Pow
            | Self::Root
            | Self::Negate
            | Self::Not
            | Self::Tetration
            | Self::Equal
            | Self::NotEqual
            | Self::Greater
            | Self::Less
            | Self::LessEqual
            | Self::GreaterEqual => false,
            Self::Bracket(_) | Self::Factorial | Self::Function(_) | Self::SubFactorial => {
                unreachable!()
            }
        }
    }
    pub fn is_chainable(self) -> bool {
        matches!(
            self,
            Self::Equal
                | Self::NotEqual
                | Self::Greater
                | Self::Less
                | Self::LessEqual
                | Self::GreaterEqual
        )
    }
    pub fn is_operator(self) -> bool {
        !matches!(self, Self::Function(_) | Self::Bracket(_))
    }
    pub fn compute(self, a: &mut [Token]) {
        let ([a], b) = a.split_first_chunk_mut().unwrap();
        let a = a.num_mut();
        self.compute_on(a, b)
    }
    pub fn compute_on(self, a: &mut Complex, b: &[Token]) {
        match self {
            Self::Add => *a += b[0].num_ref(),
            Self::Sub => *a -= b[0].num_ref(),
            Self::Mul => *a *= b[0].num_ref(),
            Self::Div => *a /= b[0].num_ref(),
            Self::Rem => *a %= b[0].num_ref(),
            Self::Factorial => *a = (*a + 1).gamma(),
            Self::Pow => *a = a.pow(b[0].num_ref()),
            Self::Root => *a = a.pow(b[0].num_ref().recip()),
            Self::Negate => *a = a.neg(),
            Self::Tetration => a.tetration_mut(&b[0].num_ref()),
            Self::SubFactorial => a.subfactorial_mut(),
            Self::Equal => *a = Complex::from(*a == b[0].num_ref()),
            Self::NotEqual => *a = Complex::from(*a != b[0].num_ref()),
            Self::Greater => *a = Complex::from(a.total_cmp(&b[0].num_ref()).is_gt()),
            Self::Less => *a = Complex::from(a.total_cmp(&b[0].num_ref()).is_lt()),
            Self::GreaterEqual => *a = Complex::from(a.total_cmp(&b[0].num_ref()).is_ge()),
            Self::LessEqual => *a = Complex::from(a.total_cmp(&b[0].num_ref()).is_le()),
            Self::And => *a = Complex::from(!a.is_zero() && !b[0].num_ref().is_zero()),
            Self::Or => *a = Complex::from(!a.is_zero() || !b[0].num_ref().is_zero()),
            Self::Not => *a = Complex::from(a.is_zero()),
            Self::Function(fun) => fun.compute(a, b),
            Self::Bracket(_) => {
                unreachable!()
            }
        }
    }
}
