use crate::functions::Function;
use crate::parse::Derivative;
use std::fmt::{Display, Formatter};
use std::num::NonZeroU8;
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum Operator {
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
    Solve,
    Bracket(Bracket),
    Custom(u16, Derivative),
    Function(Function, Derivative),
}
impl Display for Operator {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}",
            match self {
                Operator::Add => "+",
                Operator::Sub => "-",
                Operator::Mul => "*",
                Operator::Div => "/",
                Operator::Pow => "^",
                Operator::Tetration => "^^",
                Operator::Root => "//",
                Operator::Rem => "%",
                Operator::Negate => "-",
                Operator::Factorial => "!",
                Operator::SubFactorial => "!",
                Operator::Equal => "==",
                Operator::NotEqual => "!=",
                Operator::Greater => ">",
                Operator::Less => "<",
                Operator::GreaterEqual => ">=",
                Operator::LessEqual => "<=",
                Operator::And => "&",
                Operator::Or => "?",
                Operator::Not => ";",
                Operator::Solve => "=",
                Operator::Bracket(_) => unreachable!(),
                Operator::Function(_, _) => unreachable!(),
                Operator::Custom(_, _) => unreachable!(),
            }
        )
    }
}
impl TryFrom<Function> for Operator {
    type Error = ();
    fn try_from(value: Function) -> Result<Self, Self::Error> {
        Ok(match value {
            Function::Add => Self::Add,
            Function::Sub => Self::Sub,
            Function::Mul => Self::Mul,
            Function::Div => Self::Div,
            Function::Pow => Self::Pow,
            Function::Tetration => Self::Tetration,
            Function::Root => Self::Root,
            Function::Rem => Self::Rem,
            Function::Negate => Self::Negate,
            Function::Factorial => Self::Factorial,
            Function::SubFactorial => Self::SubFactorial,
            Function::Equal => Self::Equal,
            Function::NotEqual => Self::NotEqual,
            Function::Greater => Self::Greater,
            Function::Less => Self::Less,
            Function::GreaterEqual => Self::GreaterEqual,
            Function::LessEqual => Self::LessEqual,
            Function::And => Self::And,
            Function::Or => Self::Or,
            Function::Not => Self::Not,
            _ => return Err(()),
        })
    }
}
impl From<Bracket> for Operator {
    fn from(value: Bracket) -> Self {
        Self::Bracket(value)
    }
}
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum Bracket {
    Absolute,
    Parenthesis,
}
impl TryFrom<&str> for Operator {
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
            ";" => Self::Not,
            "=" => Self::Solve,
            "(" => Self::Bracket(Bracket::Parenthesis),
            "|" => Self::Bracket(Bracket::Absolute),
            _ => return Err(()),
        })
    }
}
impl Operator {
    pub fn inputs(self) -> NonZeroU8 {
        NonZeroU8::new(match self {
            Self::Negate | Self::Factorial | Self::Not | Self::SubFactorial => 1,
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
            | Self::Solve
            | Self::Tetration => 2,
            Self::Function(fun, _) => return fun.inputs(),
            Self::Custom(_, _) => unreachable!(),
            Self::Bracket(_) => unreachable!(),
        })
        .unwrap()
    }
    pub fn inner_vars(self) -> u8 {
        if let Self::Function(fun, _) = self {
            fun.inner_vars()
        } else {
            0
        }
    }
    pub fn unary_left(self) -> bool {
        match self {
            Self::Negate | Self::Not | Self::SubFactorial => true,
            Self::Factorial => false,
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
            | Self::Solve
            | Self::Tetration
            | Self::Function(_, _)
            | Self::Custom(_, _)
            | Self::Bracket(_) => unreachable!(),
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
            Self::Solve => 3,
            Self::Add | Self::Sub => 4,
            Self::Mul | Self::Div => 5,
            Self::Negate | Self::Not => 6,
            Self::Pow | Self::Root | Self::Tetration => 7,
            Self::Rem => 8,
            Self::Factorial | Self::SubFactorial => 9,
            Self::Bracket(_) | Self::Function(_, _) | Self::Custom(_, _) => unreachable!(),
        }
    }
    pub fn left_associative(self) -> bool {
        match self {
            Self::Add
            | Self::Sub
            | Self::Mul
            | Self::Div
            | Self::Rem
            | Self::And
            | Self::Or
            | Self::Solve => true,
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
            Self::Bracket(_)
            | Self::Factorial
            | Self::Function(_, _)
            | Self::Custom(_, _)
            | Self::SubFactorial => {
                unreachable!()
            }
        }
    }
}
impl From<Operator> for Function {
    fn from(value: Operator) -> Self {
        match value {
            Operator::Add => Self::Add,
            Operator::Sub => Self::Sub,
            Operator::Mul => Self::Mul,
            Operator::Div => Self::Div,
            Operator::Pow => Self::Pow,
            Operator::Tetration => Self::Tetration,
            Operator::Root => Self::Root,
            Operator::Rem => Self::Rem,
            Operator::Negate => Self::Negate,
            Operator::Factorial => Self::Factorial,
            Operator::SubFactorial => Self::SubFactorial,
            Operator::Equal => Self::Equal,
            Operator::NotEqual => Self::NotEqual,
            Operator::Greater => Self::Greater,
            Operator::Less => Self::Less,
            Operator::GreaterEqual => Self::GreaterEqual,
            Operator::LessEqual => Self::LessEqual,
            Operator::And => Self::And,
            Operator::Or => Self::Or,
            Operator::Not => Self::Not,
            Operator::Function(function, _) => function,
            Operator::Custom(_, _) | Operator::Bracket(_) | Operator::Solve => unreachable!(),
        }
    }
}
