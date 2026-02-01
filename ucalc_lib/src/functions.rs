use ucalc_numbers::Complex;
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
    Max,
    Min,
    Quadratic,
    Sqrt,
    Sum,
    Prod,
    Gamma,
    Erf,
    Erfc,
    Abs,
    Arg,
    Recip,
    Conj,
    Custom(usize),
}
impl TryFrom<&str> for Function {
    type Error = ();
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        Ok(match value {
            "exp" => Function::Exp,
            "asin" => Function::Asin,
            "acos" => Function::Acos,
            "asinh" => Function::Asinh,
            "acosh" => Function::Acosh,
            "ln" => Function::Ln,
            "min" => Function::Min,
            "max" => Function::Max,
            "sin" => Function::Sin,
            "cos" => Function::Cos,
            "sinh" => Function::Sinh,
            "cosh" => Function::Cosh,
            "atan" => Function::Atan,
            "sqrt" => Function::Sqrt,
            "sum" => Function::Sum,
            "prod" => Function::Prod,
            "quadratic" => Function::Quadratic,
            "gamma" => Function::Gamma,
            "erf" => Function::Erf,
            "erfc" => Function::Erfc,
            "abs" => Function::Abs,
            "arg" => Function::Arg,
            "recip" => Function::Recip,
            "conj" => Function::Conj,
            "atanh" => Function::Atanh,
            "tanh" => Function::Tanh,
            "tan" => Function::Tan,
            _ => return Err(()),
        })
    }
}
impl Function {
    pub fn inputs(self) -> usize {
        match self {
            Function::Cos
            | Function::Sin
            | Function::Tan
            | Function::Tanh
            | Function::Atanh
            | Function::Cosh
            | Function::Sinh
            | Function::Ln
            | Function::Acos
            | Function::Asin
            | Function::Acosh
            | Function::Asinh
            | Function::Exp
            | Function::Sqrt
            | Function::Gamma
            | Function::Erf
            | Function::Erfc
            | Function::Abs
            | Function::Arg
            | Function::Recip
            | Function::Conj => 1,
            Function::Atan | Function::Max | Function::Min => 2,
            Function::Quadratic | Function::Sum | Function::Prod => 3,
            Function::Custom(_) => unreachable!(),
        }
    }
    pub fn has_var(self) -> bool {
        matches!(self, Function::Sum | Function::Prod)
    }
    pub fn compute(self, a: &mut Complex, b: &[Complex]) {
        match self {
            Function::Sin => a.sin_mut(),
            Function::Ln => a.ln_mut(),
            Function::Cos => a.cos_mut(),
            Function::Acos => a.acos_mut(),
            Function::Asin => a.asin_mut(),
            Function::Exp => a.exp_mut(),
            Function::Sqrt => a.sqrt_mut(),
            Function::Gamma => a.gamma_mut(),
            Function::Erf => a.erf_mut(),
            Function::Erfc => a.erfc_mut(),
            Function::Abs => a.abs_mut(),
            Function::Arg => a.arg_mut(),
            Function::Recip => a.recip_mut(),
            Function::Conj => a.conj_mut(),
            Function::Tan => a.tan_mut(),
            Function::Sinh => a.sinh_mut(),
            Function::Asinh => a.asinh_mut(),
            Function::Cosh => a.cosh_mut(),
            Function::Acosh => a.acosh_mut(),
            Function::Tanh => a.tanh_mut(),
            Function::Atanh => a.atanh_mut(),
            Function::Atan => a.atan2_mut(&b[0]),
            Function::Max => a.max_mut(&b[0]),
            Function::Min => a.min_mut(&b[0]),
            Function::Quadratic => {
                *a = ((b[0] * b[0] - *a * b[1] * 4).sqrt() - b[0]) / (*a * 2);
            }
            Function::Custom(_) | Function::Sum | Function::Prod => unreachable!(),
        }
    }
    pub fn inverse(self) -> Option<Self> {
        Some(match self {
            Function::Sin => Function::Asin,
            Function::Cos => Function::Acos,
            Function::Ln => Function::Exp,
            Function::Asin => Function::Sin,
            Function::Acos => Function::Cos,
            Function::Exp => Function::Ln,
            Function::Recip => Function::Recip,
            Function::Conj => Function::Conj,
            Function::Sinh => Function::Asinh,
            Function::Cosh => Function::Acosh,
            Function::Asinh => Function::Sinh,
            Function::Acosh => Function::Cosh,
            Function::Tanh => Function::Atanh,
            Function::Atanh => Function::Tanh,
            Function::Tan => return None,
            Function::Max
            | Function::Min
            | Function::Quadratic
            | Function::Atan
            | Function::Sqrt
            | Function::Sum
            | Function::Prod
            | Function::Gamma
            | Function::Erf
            | Function::Erfc
            | Function::Abs
            | Function::Arg => return None,
            Function::Custom(_) => unreachable!(),
        })
    }
}
