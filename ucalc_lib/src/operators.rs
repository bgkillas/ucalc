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
    Mod,
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
    #[cfg(feature = "units")]
    Convert,
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
                Self::Add => "+",
                Self::Sub => "-",
                Self::Mul => "*",
                Self::Div => "/",
                Self::Pow => "^",
                Self::Tetration => "^^",
                Self::Root => "//",
                Self::Mod => "%",
                Self::Negate => "-",
                Self::Factorial => "!",
                Self::SubFactorial => "!",
                Self::Equal => "==",
                Self::NotEqual => "!=",
                Self::Greater => ">",
                Self::Less => "<",
                Self::GreaterEqual => ">=",
                Self::LessEqual => "<=",
                Self::And => "&",
                Self::Or => "?",
                Self::Not => ";",
                Self::Solve => "=",
                #[cfg(feature = "units")]
                Self::Convert => "->",
                Self::Bracket(_) | Self::Function(_, _) | Self::Custom(_, _) => unreachable!(),
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
            Function::Mod => Self::Mod,
            Function::Negate => Self::Negate,
            Function::Factorial => Self::Factorial,
            Function::SubFactorial => Self::SubFactorial,
            Function::Equal => Self::Equal,
            Function::NotEqual => Self::NotEqual,
            Function::Greater => Self::Greater,
            Function::Less => Self::Less,
            Function::GreaterEqual => Self::GreaterEqual,
            Function::LessEqual => Self::LessEqual,
            #[cfg(feature = "units")]
            Function::Convert => Self::Convert,
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
            "%" => Self::Mod,
            "^" | "**" => Self::Pow,
            "^^" => Self::Tetration,
            "*" => Self::Mul,
            "/" => Self::Div,
            "+" => Self::Add,
            "-" => Self::Sub,
            "~" => Self::Negate,
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
            #[cfg(feature = "units")]
            "->" => Self::Convert,
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
            | Self::Mod
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
            #[cfg(feature = "units")]
            Self::Convert => 2,
            Self::Function(fun, _) => return fun.inputs(),
            Self::Custom(_, _) | Self::Bracket(_) => unreachable!(),
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
    pub fn unary_right(self) -> bool {
        matches!(self, Self::Factorial)
    }
    pub fn get_unary_left(self) -> Option<Self> {
        Some(match self {
            Self::Add => Self::Add,
            Self::Sub => Self::Negate,
            Self::Factorial => Self::SubFactorial,
            _ => return None,
        })
    }
    pub fn is_unary(self) -> bool {
        matches!(
            self,
            Self::Negate | Self::Not | Self::SubFactorial | Self::Factorial
        )
    }
    pub fn unary_left(self) -> bool {
        match self {
            Self::Negate | Self::Not | Self::SubFactorial => true,
            Self::Factorial => false,
            _ => unreachable!(),
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
            #[cfg(feature = "units")]
            Self::Convert => 4,
            Self::Add | Self::Sub => 5,
            Self::Mul | Self::Div => 6,
            Self::Negate | Self::Not => 7,
            Self::Pow | Self::Root | Self::Tetration => 8,
            Self::Mod => 9,
            Self::Factorial | Self::SubFactorial => 10,
            Self::Bracket(_) | Self::Function(_, _) | Self::Custom(_, _) => unreachable!(),
        }
    }
    pub fn left_associative(self) -> bool {
        match self {
            Self::Add | Self::Sub | Self::Mul | Self::Div | Self::Mod | Self::And | Self::Or => {
                true
            }
            #[cfg(feature = "units")]
            Self::Convert => true,
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
            | Self::GreaterEqual
            | Self::Solve => false,
            _ => {
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
            Operator::Mod => Self::Mod,
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
            #[cfg(feature = "units")]
            Operator::Convert => Self::Convert,
            Operator::Function(function, _) => function,
            Operator::Custom(_, _) | Operator::Bracket(_) | Operator::Solve => unreachable!(),
        }
    }
}
