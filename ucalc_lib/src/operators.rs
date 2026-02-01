use crate::functions::Function;
use std::ops::Neg;
use ucalc_numbers::{Complex, Pow};
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum Operators {
    Add,
    Sub,
    Mul,
    Div,
    Pow,
    Root,
    Rem,
    LeftParenthesis,
    Negate,
    Factorial,
    Fun(Function),
}
impl TryFrom<&str> for Operators {
    type Error = ();
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        Ok(match value {
            "//" => Operators::Root,
            "%" => Operators::Rem,
            "^" | "**" => Operators::Pow,
            "*" => Operators::Mul,
            "/" => Operators::Div,
            "+" => Operators::Add,
            "-" => Operators::Sub,
            "_" => Operators::Negate,
            "!" => Operators::Factorial,
            "(" => Operators::LeftParenthesis,
            _ => return Err(()),
        })
    }
}
impl Operators {
    pub const MAX_INPUT: usize = 3;
    pub fn inverse(self) -> Option<Self> {
        Some(match self {
            Operators::Add => Operators::Sub,
            Operators::Sub => Operators::Add,
            Operators::Mul => Operators::Div,
            Operators::Div => Operators::Mul,
            Operators::Pow => Operators::Root,
            Operators::Root => Operators::Pow,
            Operators::Negate => Operators::Negate,
            Operators::Fun(fun) => return fun.inverse().map(|a| a.into()),
            Operators::LeftParenthesis | Operators::Rem | Operators::Factorial => return None,
        })
    }
    pub fn inputs(self) -> usize {
        match self {
            Operators::Mul
            | Operators::Div
            | Operators::Add
            | Operators::Sub
            | Operators::Pow
            | Operators::Root
            | Operators::Rem => 2,
            Operators::Negate | Operators::Factorial => 1,
            Operators::Fun(fun) => fun.inputs(),
            Operators::LeftParenthesis => unreachable!(),
        }
    }
    pub fn precedence(self) -> u8 {
        match self {
            Operators::Add | Operators::Sub => 0,
            Operators::Mul | Operators::Div => 1,
            Operators::Negate => 2,
            Operators::Pow | Operators::Root => 3,
            Operators::Rem => 4,
            Operators::Factorial => 5,
            Operators::LeftParenthesis | Operators::Fun(_) => unreachable!(),
        }
    }
    pub fn left_associative(self) -> bool {
        match self {
            Operators::Add | Operators::Sub | Operators::Mul | Operators::Div | Operators::Rem => {
                true
            }
            Operators::Pow | Operators::Root | Operators::Negate => false,
            Operators::LeftParenthesis | Operators::Factorial | Operators::Fun(_) => unreachable!(),
        }
    }
    pub fn is_operator(self) -> bool {
        !matches!(self, Operators::Fun(_) | Operators::LeftParenthesis)
    }
    pub fn compute(self, a: &mut Complex, b: &[Complex]) {
        match self {
            Operators::Add => *a += b[0],
            Operators::Sub => *a -= b[0],
            Operators::Mul => *a *= b[0],
            Operators::Div => *a /= b[0],
            Operators::Rem => *a %= b[0],
            Operators::Factorial => *a = (*a + 1).gamma(),
            Operators::Pow => *a = a.pow(b[0]),
            Operators::Root => *a = a.pow(b[0].recip()),
            Operators::Negate => *a = a.neg(),
            Operators::Fun(fun) => fun.compute(a, b),
            Operators::LeftParenthesis => {
                unreachable!()
            }
        }
    }
}
