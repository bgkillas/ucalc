use crate::{Function, Number};
use ucalc_numbers::{FloatTrait, PowAssign};
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
    None,
}
impl Inverse {
    pub fn is_none(self) -> bool {
        matches!(self, Self::None)
    }
    pub fn get_inverse(self) -> Option<Function> {
        Some(match self {
            Self::Sin => Function::Asin,
            Self::Cos => Function::Acos,
            Self::Ln => Function::Exp,
            Self::Asin => Function::Sin,
            Self::Acos => Function::Cos,
            Self::Exp => Function::Ln,
            Self::Recip => Function::Recip,
            #[cfg(feature = "complex")]
            Self::Conj => Function::Conj,
            Self::Sinh => Function::Asinh,
            Self::Cosh => Function::Acosh,
            Self::Asinh => Function::Sinh,
            Self::Acosh => Function::Cosh,
            Self::Tanh => Function::Atanh,
            Self::Atanh => Function::Tanh,
            Self::Tan => Function::Atan,
            Self::Atan => Function::Tan,
            Self::Sqrt => Function::Sq,
            Self::Sq => Function::Sqrt,
            Self::Cbrt => Function::Cb,
            Self::Cb => Function::Cbrt,
            Self::Negate => Function::Negate,
            _ => return None,
        })
    }
    pub fn left_inverse(self, a: &mut Number, b: Number) {
        match self {
            Self::Add => *a -= b,
            Self::Sub => *a += b,
            Self::Mul => *a /= b,
            Self::Div => *a *= b,
            Self::Pow => Inverse::pow_assign(a, b.recip()),
            Self::Root => Inverse::pow_assign(a, b),
            _ => unreachable!(),
        }
    }
    pub fn right_inverse(self, a: &mut Number, mut b: Number) {
        match self {
            Self::Add => *a -= b,
            Self::Sub => {
                std::mem::swap(a, &mut b);
                *a -= b
            }
            Self::Mul => *a /= b,
            Self::Div => {
                std::mem::swap(a, &mut b);
                *a /= b
            }
            Self::Pow => {
                a.ln_mut();
                *a /= b.ln();
            }
            Self::Root => {
                std::mem::swap(a, &mut b);
                a.ln_mut();
                *a /= b.ln();
            }
            _ => unreachable!(),
        }
    }
    pub fn pow_assign(a: &mut Number, b: Number) {
        //TODO
        a.pow_assign(b)
    }
}
impl From<Function> for Inverse {
    fn from(value: Function) -> Self {
        match value {
            Function::Add => Self::Add,
            Function::Sub => Self::Sub,
            Function::Mul => Self::Mul,
            Function::Div => Self::Div,
            Function::Pow => Self::Pow,
            Function::Root => Self::Root,
            Function::Negate => Self::Negate,
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
            Function::Tetration
            | Function::Rem
            | Function::Factorial
            | Function::SubFactorial
            | Function::Equal
            | Function::NotEqual
            | Function::Greater
            | Function::Less
            | Function::GreaterEqual
            | Function::LessEqual
            | Function::And
            | Function::Or
            | Function::Not
            | Function::Max
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
            | Function::Solve
            | Function::Custom(_) => Self::None,
            #[cfg(feature = "complex")]
            Function::Arg | Function::Real | Function::Imag => Self::None,
        }
    }
}
