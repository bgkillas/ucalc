use ucalc_numbers::{Complex, Float, Pow};
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum Function {
    Sin,
    Asin,
    Cos,
    Acos,
    Tan,
    Sinh,
    Asinh,
    Cosh,
    Acosh,
    Tanh,
    Atanh,
    Ln,
    Exp,
    Atan,
    Atan2,
    Max,
    Min,
    Quadratic,
    Sqrt,
    Cbrt,
    Sq,
    Cb,
    Sum,
    Prod,
    Gamma,
    Erf,
    Erfc,
    Abs,
    Arg,
    Recip,
    Conj,
    Iter,
    Ceil,
    Floor,
    Round,
    Trunc,
    Fract,
    Real,
    Imag,
    If,
    Custom(usize),
}
impl TryFrom<&str> for Function {
    type Error = ();
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        Ok(match value {
            "exp" => Self::Exp,
            "asin" => Self::Asin,
            "acos" => Self::Acos,
            "asinh" => Self::Asinh,
            "acosh" => Self::Acosh,
            "ln" => Self::Ln,
            "min" => Self::Min,
            "max" => Self::Max,
            "sin" => Self::Sin,
            "cos" => Self::Cos,
            "sinh" => Self::Sinh,
            "cosh" => Self::Cosh,
            "arctan" => Self::Atan,
            "atan" => Self::Atan2,
            "sqrt" => Self::Sqrt,
            "sum" => Self::Sum,
            "prod" => Self::Prod,
            "quadratic" => Self::Quadratic,
            "gamma" => Self::Gamma,
            "erf" => Self::Erf,
            "erfc" => Self::Erfc,
            "abs" => Self::Abs,
            "arg" => Self::Arg,
            "recip" => Self::Recip,
            "conj" => Self::Conj,
            "atanh" => Self::Atanh,
            "tanh" => Self::Tanh,
            "tan" => Self::Tan,
            "iter" => Self::Iter,
            "sq" => Self::Sq,
            "cbrt" => Self::Cbrt,
            "cb" => Self::Cb,
            "ceil" => Self::Ceil,
            "floor" => Self::Floor,
            "round" => Self::Round,
            "trunc" => Self::Trunc,
            "fract" => Self::Fract,
            "real" => Self::Real,
            "imag" => Self::Imag,
            "if" => Self::If,
            _ => return Err(()),
        })
    }
}
impl Function {
    pub const MAX_INPUT: usize = 3;
    pub fn inputs(self) -> usize {
        match self {
            Self::Cos
            | Self::Sin
            | Self::Tan
            | Self::Tanh
            | Self::Atanh
            | Self::Cosh
            | Self::Sinh
            | Self::Ln
            | Self::Acos
            | Self::Asin
            | Self::Acosh
            | Self::Asinh
            | Self::Exp
            | Self::Sqrt
            | Self::Gamma
            | Self::Erf
            | Self::Erfc
            | Self::Abs
            | Self::Arg
            | Self::Recip
            | Self::Cbrt
            | Self::Cb
            | Self::Sq
            | Self::Atan
            | Self::Conj
            | Self::Ceil
            | Self::Floor
            | Self::Round
            | Self::Trunc
            | Self::Fract
            | Self::Real
            | Self::Imag => 1,
            Self::Atan2 | Self::Max | Self::Min => 2,
            Self::Quadratic | Self::Sum | Self::Prod | Self::Iter | Self::If => 3,
            Self::Custom(_) => unreachable!(),
        }
    }
    pub fn compact(self) -> usize {
        match self {
            Self::Sum | Self::Prod | Self::Iter => 1,
            Self::If => 2,
            _ => 0,
        }
    }
    pub fn compute(self, a: &mut Complex, b: &[Complex]) {
        match self {
            Self::Sin => a.sin_mut(),
            Self::Ln => a.ln_mut(),
            Self::Cos => a.cos_mut(),
            Self::Acos => a.acos_mut(),
            Self::Asin => a.asin_mut(),
            Self::Exp => a.exp_mut(),
            Self::Sqrt => a.sqrt_mut(),
            Self::Gamma => a.gamma_mut(),
            Self::Erf => a.erf_mut(),
            Self::Erfc => a.erfc_mut(),
            Self::Abs => a.abs_mut(),
            Self::Arg => a.arg_mut(),
            Self::Recip => a.recip_mut(),
            Self::Conj => a.conj_mut(),
            Self::Tan => a.tan_mut(),
            Self::Sinh => a.sinh_mut(),
            Self::Asinh => a.asinh_mut(),
            Self::Cosh => a.cosh_mut(),
            Self::Acosh => a.acosh_mut(),
            Self::Tanh => a.tanh_mut(),
            Self::Atanh => a.atanh_mut(),
            Self::Cbrt => *a = a.pow(Float::from(3).recip()),
            Self::Sq => *a *= *a,
            Self::Cb => *a = *a * *a * *a,
            Self::Atan => a.atan_mut(),
            Self::Atan2 => a.atan2_mut(&b[0]),
            Self::Max => a.max_mut(&b[0]),
            Self::Min => a.min_mut(&b[0]),
            Self::Ceil => a.ceil_mut(),
            Self::Floor => a.floor_mut(),
            Self::Round => a.round_mut(),
            Self::Trunc => a.trunc_mut(),
            Self::Fract => a.fract_mut(),
            Self::Real => *a = a.real.into(),
            Self::Imag => *a = a.imag.into(),
            Self::Quadratic => {
                *a = ((b[0] * b[0] - *a * b[1] * 4).sqrt() - b[0]) / (*a * 2);
            }
            Self::Custom(_) | Self::Sum | Self::Prod | Self::Iter | Self::If => unreachable!(),
        }
    }
    pub fn inverse(self) -> Option<Self> {
        Some(match self {
            Self::Sin => Self::Asin,
            Self::Cos => Self::Acos,
            Self::Ln => Self::Exp,
            Self::Asin => Self::Sin,
            Self::Acos => Self::Cos,
            Self::Exp => Self::Ln,
            Self::Recip => Self::Recip,
            Self::Conj => Self::Conj,
            Self::Sinh => Self::Asinh,
            Self::Cosh => Self::Acosh,
            Self::Asinh => Self::Sinh,
            Self::Acosh => Self::Cosh,
            Self::Tanh => Self::Atanh,
            Self::Atanh => Self::Tanh,
            Self::Tan => Self::Atan,
            Self::Atan => Self::Tan,
            Self::Sqrt => Self::Sq,
            Self::Sq => Self::Sqrt,
            Self::Cbrt => Self::Cb,
            Self::Cb => Self::Cbrt,
            Self::Max
            | Self::Min
            | Self::Quadratic
            | Self::Sum
            | Self::Prod
            | Self::Gamma
            | Self::Erf
            | Self::Erfc
            | Self::Abs
            | Self::Arg
            | Self::Iter
            | Self::Atan2
            | Self::Ceil
            | Self::Floor
            | Self::Round
            | Self::Trunc
            | Self::Fract
            | Self::Real
            | Self::Imag
            | Self::If => return None,
            Self::Custom(_) => unreachable!(),
        })
    }
}
