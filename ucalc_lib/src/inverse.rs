use crate::{Function, Number, Operators};
use ucalc_numbers::Pow;
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum Inverse {
    Add,
    Sub,
    Mul,
    Div,
    Pow,
    Root,
    Negate,
    Sin,
    Cos,
    Ln,
    Asin,
    Acos,
    Exp,
    Recip,
    #[cfg(feature = "complex")]
    Conj,
    Sinh,
    Cosh,
    Asinh,
    Acosh,
    Atanh,
    Tanh,
    Tan,
    Atan,
    Sqrt,
    Sq,
    Cbrt,
    Cb,
    Custom(usize), //TODO
    None,
}
impl Inverse {
    pub fn is_none(self) -> bool {
        matches!(self, Self::None)
    }
    pub fn get_inverse(self) -> Option<Operators> {
        //TODO
        Some(match self {
            Self::Sin => Function::Asin.into(),
            Self::Cos => Function::Acos.into(),
            Self::Ln => Function::Exp.into(),
            Self::Asin => Function::Sin.into(),
            Self::Acos => Function::Cos.into(),
            Self::Exp => Function::Ln.into(),
            Self::Recip => Function::Recip.into(),
            #[cfg(feature = "complex")]
            Self::Conj => Function::Conj.into(),
            Self::Sinh => Function::Asinh.into(),
            Self::Cosh => Function::Acosh.into(),
            Self::Asinh => Function::Sinh.into(),
            Self::Acosh => Function::Cosh.into(),
            Self::Tanh => Function::Atanh.into(),
            Self::Atanh => Function::Tanh.into(),
            Self::Tan => Function::Atan.into(),
            Self::Atan => Function::Tan.into(),
            Self::Sqrt => Function::Sq.into(),
            Self::Sq => Function::Sqrt.into(),
            Self::Cbrt => Function::Cb.into(),
            Self::Cb => Function::Cbrt.into(),
            Self::Negate => Operators::Negate,
            _ => return None,
        })
    }
    pub fn left_inverse(self, a: Number, b: Number) -> Number {
        //TODO
        match self {
            Self::Add => a - b,
            Self::Sub => a + b,
            Self::Mul => a / b,
            Self::Div => a * b,
            Self::Pow => a.pow(b.recip()),
            Self::Root => a.pow(b),
            _ => unreachable!(),
        }
    }
    pub fn right_inverse(self, a: Number, b: Number) -> Number {
        //TODO
        match self {
            Self::Add => a - b,
            Self::Sub => b - a,
            Self::Mul => a / b,
            Self::Div => b / a,
            Self::Pow => a.ln() / b.ln(),
            Self::Root => b.ln() / a.ln(),
            _ => unreachable!(),
        }
    }
}
impl From<Operators> for Inverse {
    fn from(value: Operators) -> Self {
        match value {
            Operators::Add => Self::Add,
            Operators::Sub => Self::Sub,
            Operators::Mul => Self::Mul,
            Operators::Div => Self::Div,
            Operators::Pow => Self::Pow,
            Operators::Root => Self::Root,
            Operators::Negate => Self::Negate,
            Operators::Function(fun) => fun.into(),
            Operators::Bracket(_)
            | Operators::Rem
            | Operators::Factorial
            | Operators::SubFactorial
            | Operators::Equal
            | Operators::NotEqual
            | Operators::Greater
            | Operators::Less
            | Operators::LessEqual
            | Operators::GreaterEqual
            | Operators::And
            | Operators::Or
            | Operators::Not
            | Operators::Tetration => Self::None,
        }
    }
}
impl From<Function> for Inverse {
    fn from(value: Function) -> Self {
        match value {
            Function::Sin => Self::Sin,
            Function::Cos => Self::Cos,
            Function::Ln => Self::Ln,
            Function::Asin => Self::Asin,
            Function::Acos => Self::Acos,
            Function::Exp => Self::Exp,
            Function::Recip => Self::Recip,
            #[cfg(feature = "complex")]
            Function::Conj => Self::Conj,
            Function::Sinh => Self::Sinh,
            Function::Cosh => Self::Cosh,
            Function::Asinh => Self::Asinh,
            Function::Acosh => Self::Acosh,
            Function::Tanh => Self::Tanh,
            Function::Atanh => Self::Atanh,
            Function::Tan => Self::Tan,
            Function::Atan => Self::Atan,
            Function::Sqrt => Self::Sqrt,
            Function::Sq => Self::Sq,
            Function::Cbrt => Self::Cbrt,
            Function::Cb => Self::Cb,
            Function::Max
            | Function::Min
            | Function::Quadratic
            | Function::Sum
            | Function::Prod
            | Function::Gamma
            | Function::Erf
            | Function::Erfc
            | Function::Abs
            | Function::Iter
            | Function::Atan2
            | Function::Ceil
            | Function::Floor
            | Function::Round
            | Function::Trunc
            | Function::Fract
            | Function::If
            | Function::Fold
            | Function::Set
            | Function::Solve => Self::None,
            #[cfg(feature = "complex")]
            Function::Arg | Function::Real | Function::Imag => Self::None,
            Function::Custom(i) => Self::Custom(i),
        }
    }
}
