use crate::parse::Token;
use crate::polynomial::PolyRef;
use crate::{Functions, Tokens};
use ucalc_numbers::{Complex, Constant, Float, PowAssign};
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
    Fold,
    Set,
    Solve,
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
            "set" => Self::Set,
            "fold" => Self::Fold,
            "solve" => Self::Solve,
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
            | Self::Imag
            | Self::Solve => 1,
            Self::Atan2 | Self::Max | Self::Min | Self::Set => 2,
            Self::Quadratic | Self::Sum | Self::Prod | Self::Iter | Self::If => 3,
            Self::Fold => 4,
            Self::Custom(_) => unreachable!(),
        }
    }
    pub fn compute(self, a: &mut Complex, b: &[Token]) {
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
            Self::Cbrt => a.pow_assign(Float::from(3).recip()),
            Self::Sq => *a *= *a,
            Self::Cb => *a = *a * *a * *a,
            Self::Atan => a.atan_mut(),
            Self::Atan2 => a.atan2_mut(&b[0].num_ref()),
            Self::Max => a.max_mut(&b[0].num_ref()),
            Self::Min => a.min_mut(&b[0].num_ref()),
            Self::Ceil => a.ceil_mut(),
            Self::Floor => a.floor_mut(),
            Self::Round => a.round_mut(),
            Self::Trunc => a.trunc_mut(),
            Self::Fract => a.fract_mut(),
            Self::Real => *a = a.real.into(),
            Self::Imag => *a = a.imag.into(),
            Self::Quadratic => *a = PolyRef(&[*a, b[0].num_ref(), b[1].num_ref()]).quadratic()[0],
            Self::Custom(_)
            | Self::Sum
            | Self::Prod
            | Self::Iter
            | Self::If
            | Self::Fold
            | Self::Set
            | Self::Solve => unreachable!(),
        }
    }
    pub fn compact(self) -> usize {
        match self {
            Self::Sum | Self::Prod | Self::Iter | Self::Fold | Self::Set | Self::Solve => 1,
            Self::If => 2,
            _ => 0,
        }
    }
    pub fn inner_vars(self) -> usize {
        match self {
            Self::Sum | Self::Prod | Self::Iter | Self::Set | Self::Solve => 1,
            Self::Fold => 2,
            _ => 0,
        }
    }
    pub fn has_var(self) -> bool {
        matches!(
            self,
            Self::Sum | Self::Prod | Self::Iter | Self::Fold | Self::Set | Self::Solve | Self::If
        )
    }
    pub fn compute_var(
        self,
        stack: &mut Tokens,
        fun_vars: &mut Vec<Complex>,
        vars: &[Complex],
        funs: &Functions,
        offset: usize,
    ) {
        let len = stack.len();
        match self {
            Self::Sum => {
                stack.range(fun_vars, vars, funs, offset, |iter| {
                    iter.sum::<Complex>().into()
                });
            }
            Self::Prod => {
                stack.range(fun_vars, vars, funs, offset, |iter| {
                    iter.product::<Complex>().into()
                });
            }
            Self::Fold => {
                let ([tokens], l) = stack.get_skip_tokens();
                let [end, start, value] = stack.get_skip_var(l);
                let start = start.num_ref().real.to_isize();
                let end = end.num_ref().real.to_isize();
                fun_vars.push(value.num_ref());
                fun_vars.push(Complex::from(start));
                let nl = fun_vars.len();
                let mut stck = Tokens(Vec::with_capacity(tokens.len()));
                (start..=end).for_each(|_| {
                    fun_vars[nl - 2] =
                        tokens.compute_buffer_with(fun_vars, vars, funs, &mut stck, offset);
                    fun_vars.last_mut().unwrap().real += Float::from(1);
                });
                *stack[len - (l + 3)].num_mut() = fun_vars[nl - 2];
                stack.drain(len - (l + 2)..);
                fun_vars.pop();
                fun_vars.pop();
            }
            Self::Set => {
                let ([tokens], l) = stack.get_skip_tokens();
                let [value] = stack.get_skip_var(l);
                fun_vars.push(value.num_ref());
                let mut stck = Tokens(Vec::with_capacity(tokens.len()));
                *stack[len - (l + 1)].num_mut() =
                    tokens.compute_buffer_with(fun_vars, vars, funs, &mut stck, offset);
                stack.drain(len - l..);
                fun_vars.pop();
            }
            Self::Solve => {
                let ([tokens], l) = stack.get_skip_tokens();
                stack[len - l] = tokens
                    .get_inverse(fun_vars, vars, funs, offset)
                    .map(|a| a[0])
                    .unwrap_or(Complex::from(Constant::Nan))
                    .into();
                stack.drain(len - (l - 1)..);
            }
            Self::Iter => {
                let ([tokens], l) = stack.get_skip_tokens();
                let [steps, first] = stack.get_skip_var(l);
                fun_vars.push(first.num_ref());
                let steps = steps.num_ref().real.to_isize();
                let mut stck = Tokens(Vec::with_capacity(tokens.len()));
                (0..steps).for_each(|_| {
                    let next = tokens.compute_buffer_with(fun_vars, vars, funs, &mut stck, offset);
                    *fun_vars.last_mut().unwrap() = next;
                });
                *stack[len - (l + 2)].num_mut() = *fun_vars.last().unwrap();
                stack.drain(len - (l + 1)..);
                fun_vars.pop();
            }
            Self::If => {
                let ([ifelse, ifthen], l) = stack.get_skip_tokens();
                let [condition] = stack.get_skip_var(l);
                let condition = condition.num_ref();
                let tokens = if condition.is_zero() { ifelse } else { ifthen };
                let mut stck = Tokens(Vec::with_capacity(tokens.len()));
                *stack[len - (l + 1)].num_mut() =
                    tokens.compute_buffer_with(fun_vars, vars, funs, &mut stck, offset);
                stack.drain(len - l..);
            }
            _ => {}
        }
    }
}
