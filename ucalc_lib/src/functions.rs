use crate::polynomial::PolyRef;
use crate::tokens::Token;
use crate::{Functions, Number, Tokens, Variables};
use std::fmt::{Display, Formatter};
use std::ops::Deref;
#[cfg(feature = "complex")]
use ucalc_numbers::ComplexTrait;
use ucalc_numbers::{Constant, Float, FloatTrait, NegAssign, PowAssign, RealTrait};
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum Function {
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
    #[cfg(feature = "complex")]
    Arg,
    Recip,
    #[cfg(feature = "complex")]
    Conj,
    Iter,
    Ceil,
    Floor,
    Round,
    Trunc,
    Fract,
    #[cfg(feature = "complex")]
    Real,
    #[cfg(feature = "complex")]
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
            #[cfg(feature = "complex")]
            "arg" => Self::Arg,
            "recip" => Self::Recip,
            #[cfg(feature = "complex")]
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
            #[cfg(feature = "complex")]
            "real" => Self::Real,
            #[cfg(feature = "complex")]
            "imag" => Self::Imag,
            "if" => Self::If,
            "set" => Self::Set,
            "fold" => Self::Fold,
            "solve" => Self::Solve,
            "add" => Self::Add,
            "sub" => Self::Sub,
            "mul" => Self::Mul,
            "div" => Self::Div,
            "pow" => Self::Pow,
            "tetration" => Self::Tetration,
            "root" => Self::Root,
            "rem" => Self::Rem,
            "negate" => Self::Negate,
            "factorial" => Self::Factorial,
            "subfactorial" => Self::SubFactorial,
            "equal" => Self::Equal,
            "notequal" => Self::NotEqual,
            "greater" => Self::Greater,
            "less" => Self::Less,
            "greaterequal" => Self::GreaterEqual,
            "lessequal" => Self::LessEqual,
            "and" => Self::And,
            "or" => Self::Or,
            "not" => Self::Not,
            _ => return Err(()),
        })
    }
}
impl Display for Function {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}",
            match self {
                Self::Exp => "exp",
                Self::Asin => "asin",
                Self::Acos => "acos",
                Self::Asinh => "asinh",
                Self::Acosh => "acosh",
                Self::Ln => "ln",
                Self::Min => "min",
                Self::Max => "max",
                Self::Sin => "sin",
                Self::Cos => "cos",
                Self::Sinh => "sinh",
                Self::Cosh => "cosh",
                Self::Atan => "arctan",
                Self::Atan2 => "atan",
                Self::Sqrt => "sqrt",
                Self::Sum => "sum",
                Self::Prod => "prod",
                Self::Quadratic => "quadratic",
                Self::Gamma => "gamma",
                Self::Erf => "erf",
                Self::Erfc => "erfc",
                Self::Abs => "abs",
                #[cfg(feature = "complex")]
                Self::Arg => "arg",
                Self::Recip => "recip",
                #[cfg(feature = "complex")]
                Self::Conj => "conj",
                Self::Atanh => "atanh",
                Self::Tanh => "tanh",
                Self::Tan => "tan",
                Self::Iter => "iter",
                Self::Sq => "sq",
                Self::Cbrt => "cbrt",
                Self::Cb => "cb",
                Self::Ceil => "ceil",
                Self::Floor => "floor",
                Self::Round => "round",
                Self::Trunc => "trunc",
                Self::Fract => "fract",
                #[cfg(feature = "complex")]
                Self::Real => "real",
                #[cfg(feature = "complex")]
                Self::Imag => "imag",
                Self::If => "if",
                Self::Set => "set",
                Self::Fold => "fold",
                Self::Solve => "solve",
                Self::Add => "add",
                Self::Sub => "sub",
                Self::Mul => "mul",
                Self::Div => "div",
                Self::Pow => "pow",
                Self::Tetration => "tetration",
                Self::Root => "root",
                Self::Rem => "rem",
                Self::Negate => "negate",
                Self::Factorial => "factorial",
                Self::SubFactorial => "subfactorial",
                Self::Equal => "equal",
                Self::NotEqual => "notequal",
                Self::Greater => "greater",
                Self::Less => "less",
                Self::GreaterEqual => "greaterequal",
                Self::LessEqual => "lessequal",
                Self::And => "and",
                Self::Or => "or",
                Self::Not => "not",
                Self::Custom(_) => unreachable!(),
            }
        )
    }
}
impl Function {
    pub const MAX_INPUT: usize = 3;
    pub fn inputs(self) -> usize {
        match self {
            Self::Not
            | Self::Factorial
            | Self::SubFactorial
            | Self::Negate
            | Self::Cos
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
            | Self::Recip
            | Self::Cbrt
            | Self::Cb
            | Self::Sq
            | Self::Atan
            | Self::Ceil
            | Self::Floor
            | Self::Round
            | Self::Trunc
            | Self::Fract
            | Self::Solve => 1,
            #[cfg(feature = "complex")]
            Self::Arg | Self::Conj | Self::Real | Self::Imag => 1,
            Self::Tetration
            | Self::Add
            | Self::Sub
            | Self::Mul
            | Self::Div
            | Self::Pow
            | Self::Root
            | Self::Rem
            | Self::Equal
            | Self::NotEqual
            | Self::Greater
            | Self::Less
            | Self::GreaterEqual
            | Self::LessEqual
            | Self::And
            | Self::Or
            | Self::Atan2
            | Self::Max
            | Self::Min
            | Self::Set => 2,
            Self::Quadratic | Self::Sum | Self::Prod | Self::Iter | Self::If => 3,
            Self::Fold => 4,
            Self::Custom(_) => unreachable!(),
        }
    }
    pub fn is_chainable(self) -> bool {
        matches!(
            self,
            Self::Equal
                | Self::NotEqual
                | Self::Greater
                | Self::Less
                | Self::LessEqual
                | Self::GreaterEqual
        )
    }
    pub fn compute(self, a: &mut [Token]) {
        let ([a], b) = a.split_first_chunk_mut().unwrap();
        let a = a.num_mut();
        self.compute_on(a, b)
    }
    pub fn compute_on(self, a: &mut Number, b: &[Token]) {
        match self {
            Self::Add => *a += b[0].num_ref(),
            Self::Sub => *a -= b[0].num_ref(),
            Self::Mul => *a *= b[0].num_ref(),
            Self::Div => *a /= b[0].num_ref(),
            Self::Rem => *a %= b[0].num_ref(),
            Self::Factorial => {
                *a += Float::from(1);
                a.gamma_mut()
            }
            Self::Pow => a.pow_assign(b[0].num_ref()),
            Self::Root => a.pow_assign(b[0].num_ref().clone().recip()),
            Self::Negate => a.neg_assign(),
            Self::Tetration => a.tetration_mut(b[0].num_ref()),
            Self::SubFactorial => a.subfactorial_mut(),
            Self::Equal => *a = Number::from(a == b[0].num_ref()),
            Self::NotEqual => *a = Number::from(a != b[0].num_ref()),
            Self::Greater => *a = Number::from(a.total_cmp(b[0].num_ref()).is_gt()),
            Self::Less => *a = Number::from(a.total_cmp(b[0].num_ref()).is_lt()),
            Self::GreaterEqual => *a = Number::from(a.total_cmp(b[0].num_ref()).is_ge()),
            Self::LessEqual => *a = Number::from(a.total_cmp(b[0].num_ref()).is_le()),
            Self::And => *a = Number::from(!a.is_zero() && !b[0].num_ref().is_zero()),
            Self::Or => *a = Number::from(!a.is_zero() || !b[0].num_ref().is_zero()),
            Self::Not => *a = Number::from(a.is_zero()),
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
            #[cfg(feature = "complex")]
            Self::Arg => a.arg_mut(),
            Self::Recip => a.recip_mut(),
            #[cfg(feature = "complex")]
            Self::Conj => a.conj_mut(),
            Self::Tan => a.tan_mut(),
            Self::Sinh => a.sinh_mut(),
            Self::Asinh => a.asinh_mut(),
            Self::Cosh => a.cosh_mut(),
            Self::Acosh => a.acosh_mut(),
            Self::Tanh => a.tanh_mut(),
            Self::Atanh => a.atanh_mut(),
            Self::Cbrt => a.cbrt_mut(),
            Self::Sq => *a *= a.clone(),
            Self::Cb => *a = a.clone() * a.deref() * a.deref(),
            Self::Atan => a.atan_mut(),
            Self::Atan2 => a.atan2_mut(b[0].num_ref()),
            Self::Max => a.max_mut(b[0].num_ref()),
            Self::Min => a.min_mut(b[0].num_ref()),
            Self::Ceil => a.ceil_mut(),
            Self::Floor => a.floor_mut(),
            Self::Round => a.round_mut(),
            Self::Trunc => a.trunc_mut(),
            Self::Fract => a.fract_mut(),
            #[cfg(feature = "complex")]
            Self::Real => a.zero_imag(),
            #[cfg(feature = "complex")]
            Self::Imag => a.zero_real(),
            Self::Quadratic => {
                let mut poly =
                    PolyRef(&[a.clone(), b[0].num_ref().clone(), b[1].num_ref().clone()])
                        .quadratic()
                        .into_iter();
                *a = poly.next().unwrap()
            }
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
        fun_vars: &mut Vec<Number>,
        vars: &[Number],
        funs: &Functions,
        custom_vars: &Variables,
        offset: usize,
    ) {
        let len = stack.len();
        match self {
            Self::Sum => {
                stack.range(fun_vars, vars, funs, custom_vars, offset, |iter| {
                    iter.sum::<Number>().into()
                });
            }
            Self::Prod => {
                stack.range(fun_vars, vars, funs, custom_vars, offset, |iter| {
                    iter.product::<Number>().into()
                });
            }
            Self::Fold => {
                let ([tokens], l) = stack.get_skip_tokens();
                let [end, start, value] = stack.get_skip_var(l);
                let start = start.num_ref().real().clone().into_isize();
                let end = end.num_ref().real().clone().into_isize();
                fun_vars.push(value.num_ref().clone());
                fun_vars.push(Number::from(start));
                let nl = fun_vars.len();
                let mut stck = Tokens(Vec::with_capacity(tokens.len()));
                (start..=end).for_each(|_| {
                    fun_vars[nl - 2] = tokens.compute_buffer_with(
                        fun_vars,
                        vars,
                        funs,
                        custom_vars,
                        &mut stck,
                        offset,
                    );
                    *fun_vars.last_mut().unwrap() += Float::from(1);
                });
                stack.drain(len - (l + 2)..);
                fun_vars.pop();
                *stack[len - (l + 3)].num_mut() = fun_vars.pop().unwrap();
            }
            Self::Set => {
                let ([tokens], l) = stack.get_skip_tokens();
                let [value] = stack.get_skip_var(l);
                fun_vars.push(value.num_ref().clone());
                let mut stck = Tokens(Vec::with_capacity(tokens.len()));
                *stack[len - (l + 1)].num_mut() = tokens.compute_buffer_with(
                    fun_vars,
                    vars,
                    funs,
                    custom_vars,
                    &mut stck,
                    offset,
                );
                stack.drain(len - l..);
                fun_vars.pop();
            }
            Self::Solve => {
                let ([tokens], l) = stack.get_skip_tokens();
                stack[len - l] = tokens
                    .get_inverse(fun_vars, vars, funs, custom_vars, offset)
                    .unwrap_or(Number::from(Constant::Nan))
                    .into();
                stack.drain(len - (l - 1)..);
            }
            Self::Iter => {
                let ([tokens], l) = stack.get_skip_tokens();
                let [steps, first] = stack.get_skip_var(l);
                fun_vars.push(first.num_ref().clone());
                let steps = steps.num_ref().real().clone().into_isize();
                let mut stck = Tokens(Vec::with_capacity(tokens.len()));
                (0..steps).for_each(|_| {
                    let next = tokens.compute_buffer_with(
                        fun_vars,
                        vars,
                        funs,
                        custom_vars,
                        &mut stck,
                        offset,
                    );
                    *fun_vars.last_mut().unwrap() = next;
                });
                stack.drain(len - (l + 1)..);
                *stack[len - (l + 2)].num_mut() = fun_vars.pop().unwrap();
            }
            Self::If => {
                //TODO remove recursion
                let ([ifelse, ifthen], l) = stack.get_skip_tokens();
                let [condition] = stack.get_skip_var(l);
                let condition = condition.num_ref();
                let tokens = if condition.is_zero() { ifelse } else { ifthen };
                let mut stck = Tokens(Vec::with_capacity(tokens.len()));
                *stack[len - (l + 1)].num_mut() = tokens.compute_buffer_with(
                    fun_vars,
                    vars,
                    funs,
                    custom_vars,
                    &mut stck,
                    offset,
                );
                stack.drain(len - l..);
            }
            _ => {}
        }
    }
}
