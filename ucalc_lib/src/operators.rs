use crate::functions::Function;
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
}
impl From<Operators> for Function {
    fn from(value: Operators) -> Self {
        match value {
            Operators::Add => Self::Add,
            Operators::Sub => Self::Sub,
            Operators::Mul => Self::Mul,
            Operators::Div => Self::Div,
            Operators::Pow => Self::Pow,
            Operators::Tetration => Self::Tetration,
            Operators::Root => Self::Root,
            Operators::Rem => Self::Rem,
            Operators::Negate => Self::Negate,
            Operators::Factorial => Self::Factorial,
            Operators::SubFactorial => Self::SubFactorial,
            Operators::Equal => Self::Equal,
            Operators::NotEqual => Self::NotEqual,
            Operators::Greater => Self::Greater,
            Operators::Less => Self::Less,
            Operators::GreaterEqual => Self::GreaterEqual,
            Operators::LessEqual => Self::LessEqual,
            Operators::And => Self::And,
            Operators::Or => Self::Or,
            Operators::Not => Self::Not,
            Operators::Function(function) => function,
            Operators::Bracket(_) => unreachable!(),
        }
    }
}
