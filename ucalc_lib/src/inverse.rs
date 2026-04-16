use crate::functions::AtanInputs;
use crate::{Function, Number};
#[cfg(feature = "complex")]
use ucalc_numbers::{ComplexFunctions, ComplexFunctionsMut};
use ucalc_numbers::{FloatFunctions, FloatFunctionsMut, Pow, PowAssign};
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum Inverse {
    Add,
    #[cfg(feature = "complex")]
    Addi,
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
}
impl Inverse {
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
            Self::Tan => Function::Atan(AtanInputs::One),
            Self::Atan => Function::Tan,
            Self::Sqrt => Function::Sq,
            Self::Sq => Function::Sqrt,
            Self::Cbrt => Function::Cb,
            Self::Cb => Function::Cbrt,
            Self::Negate => Function::Negate,
            _ => return None,
        })
    }
    pub fn inverse_on_2<const N: usize>(self, a: &mut Number, mut b: Number) {
        match self {
            Self::Add => *a -= b,
            #[cfg(feature = "complex")]
            Self::Addi => {
                if N == 0 {
                    *a -= b.mul_i(false)
                } else {
                    *a -= b;
                    a.mul_i_mut(true);
                }
            }
            Self::Sub => {
                if N == 0 {
                    *a += b
                } else {
                    std::mem::swap(a, &mut b);
                    *a -= b
                }
            }
            Self::Mul => *a /= b,
            Self::Div => {
                if N == 0 {
                    *a *= b
                } else {
                    std::mem::swap(a, &mut b);
                    *a /= b
                }
            }
            Self::Pow => {
                if N == 0 {
                    Inverse::pow_assign(a, b.recip())
                } else {
                    a.ln_mut();
                    *a /= b.ln();
                }
            }
            Self::Root => {
                if N == 0 {
                    Inverse::pow_assign(a, b)
                } else {
                    std::mem::swap(a, &mut b);
                    a.ln_mut();
                    *a /= b.ln();
                }
            }
            _ => unreachable!(),
        }
    }
    pub fn rooti(a: Number, b: usize) -> Vec<Number> {
        //TODO
        vec![a.pow(Number::from(b).recip())]
    }
    pub fn pow_assign(a: &mut Number, b: Number) {
        //TODO
        a.pow_assign(b)
    }
}
impl TryFrom<Function> for Inverse {
    type Error = ();
    fn try_from(value: Function) -> Result<Self, Self::Error> {
        Ok(match value {
            Function::Add => Self::Add,
            #[cfg(feature = "complex")]
            Function::Addi => Self::Addi,
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
            Function::Atan(AtanInputs::One) => Self::Atan,
            Function::Sqrt => Self::Sqrt,
            Function::Sq => Self::Sq,
            Function::Cbrt => Self::Cbrt,
            Function::Cb => Self::Cb,
            _ => return Err(()),
        })
    }
}
