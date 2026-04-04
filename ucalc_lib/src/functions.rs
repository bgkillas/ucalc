use crate::compute::Compute;
use crate::polynomial::PolyRef;
use crate::{Number, Tokens};
use std::fmt::{Display, Formatter};
use std::mem;
use std::num::NonZeroU8;
use std::ops::Deref;
#[cfg(feature = "complex")]
use ucalc_numbers::{ComplexFunctionsMut, ComplexTrait};
use ucalc_numbers::{
    Constant, Float, FloatFunctions, FloatFunctionsMut, FloatTrait, NegAssign, PowAssign, RealTrait,
};
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum AtanInputs {
    One,
    Two,
}
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum ModifyInputs {
    Two,
    Three,
}
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
    Atan(AtanInputs),
    Max,
    Min,
    Quadratic,
    #[cfg(feature = "complex")]
    Cubic,
    #[cfg(feature = "complex")]
    Quartic,
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
    Modify(ModifyInputs),
    While(ModifyInputs),
    Exprs(NonZeroU8),
    Solve,
    NumericalDerivative,
    NumericalDifferential,
    NumericalIntegral,
    NumericalSolve,
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
            "atan" => Self::Atan(AtanInputs::One),
            "sqrt" => Self::Sqrt,
            "sum" => Self::Sum,
            "prod" => Self::Prod,
            "quadratic" => Self::Quadratic,
            #[cfg(feature = "complex")]
            "cubic" => Self::Cubic,
            #[cfg(feature = "complex")]
            "quartic" => Self::Quartic,
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
            "modify" => Self::Modify(ModifyInputs::Two),
            "while" => Self::While(ModifyInputs::Two),
            "exprs" => Self::Exprs(NonZeroU8::new(1).unwrap()),
            "fold" => Self::Fold,
            "solve" => Self::Solve,
            "add" => Self::Add,
            "sub" => Self::Sub,
            "mul" => Self::Mul,
            "div" => Self::Div,
            "pow" => Self::Pow,
            "numerical_differential" => Self::NumericalDifferential,
            "numerical_derivative" => Self::NumericalDerivative,
            "numerical_integral" => Self::NumericalIntegral,
            "numerical_solve" => Self::NumericalSolve,
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
                Self::Atan(_) => "atan",
                Self::Sqrt => "sqrt",
                Self::Sum => "sum",
                Self::Prod => "prod",
                Self::Quadratic => "quadratic",
                #[cfg(feature = "complex")]
                Self::Cubic => "cubic",
                #[cfg(feature = "complex")]
                Self::Quartic => "quartic",
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
                Self::NumericalDifferential => "numerical_differential",
                Self::NumericalDerivative => "numerical_derivative",
                Self::NumericalIntegral => "numerical_integral",
                Self::NumericalSolve => "numerical_solve",
                #[cfg(feature = "complex")]
                Self::Real => "real",
                #[cfg(feature = "complex")]
                Self::Imag => "imag",
                Self::If => "if",
                Self::Set => "set",
                Self::Modify(_) => "modify",
                Self::While(_) => "while",
                Self::Exprs(_) => "exprs",
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
            }
        )
    }
}
impl Function {
    pub fn set_inputs(&mut self, inputs: NonZeroU8) {
        match self {
            Self::Atan(a) if inputs.get() == 2 => *a = AtanInputs::Two,
            Self::Modify(a) if inputs.get() == 3 => *a = ModifyInputs::Three,
            Self::While(a) if inputs.get() == 3 => *a = ModifyInputs::Three,
            Self::Exprs(a) => *a = inputs,
            _ => {}
        }
    }
    pub fn is_default_inputs(self) -> bool {
        !matches!(
            self,
            Self::Atan(AtanInputs::Two)
                | Self::Modify(ModifyInputs::Three)
                | Self::While(ModifyInputs::Three)
        )
    }
    pub fn inputs(self) -> NonZeroU8 {
        NonZeroU8::new(match self {
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
            | Self::Atan(AtanInputs::One)
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
            | Self::Atan(AtanInputs::Two)
            | Self::Max
            | Self::Min
            | Self::Set
            | Self::Modify(ModifyInputs::Two)
            | Self::While(ModifyInputs::Two)
            | Self::NumericalDerivative
            | Self::NumericalSolve => 2,
            Self::Quadratic
            | Self::Sum
            | Self::Prod
            | Self::Iter
            | Self::If
            | Self::Modify(ModifyInputs::Three)
            | Self::While(ModifyInputs::Three)
            | Self::NumericalIntegral => 3,
            Self::Fold | Self::NumericalDifferential => 4,
            #[cfg(feature = "complex")]
            Self::Cubic => 4,
            #[cfg(feature = "complex")]
            Self::Quartic => 5,
            Self::Exprs(n) => return n,
        })
        .unwrap()
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
    pub fn compute(self, v: &mut Tokens, inputs: NonZeroU8) {
        match inputs.get() {
            1 => self.compute_on_1(v.last_mut().unwrap().num_mut()),
            2 => {
                let b = v.pop().unwrap().num();
                let a = v.last_mut().unwrap().num_mut();
                self.compute_on_2(a, b)
            }
            3 => {
                let c = v.pop().unwrap().num();
                let b = v.pop().unwrap().num();
                let a = v.last_mut().unwrap().num_mut();
                self.compute_on_3(a, b, c)
            }
            #[cfg(feature = "complex")]
            4 => {
                let d = v.pop().unwrap().num();
                let c = v.pop().unwrap().num();
                let b = v.pop().unwrap().num();
                let a = v.last_mut().unwrap().num_mut();
                self.compute_on_4(a, b, c, d)
            }
            #[cfg(feature = "complex")]
            5 => {
                let e = v.pop().unwrap().num();
                let d = v.pop().unwrap().num();
                let c = v.pop().unwrap().num();
                let b = v.pop().unwrap().num();
                let a = v.last_mut().unwrap().num_mut();
                self.compute_on_5(a, b, c, d, e)
            }
            _ => unreachable!(),
        }
    }
    pub fn compute_on_1(self, a: &mut Number) {
        match self {
            Self::Factorial => {
                *a += Float::from(1);
                a.gamma_mut()
            }
            Self::Negate => a.neg_assign(),
            Self::SubFactorial => a.subfactorial_mut(),
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
            Self::Cb => *a *= a.clone() * a.deref(),
            Self::Atan(AtanInputs::One) => a.atan_mut(),
            Self::Ceil => a.ceil_mut(),
            Self::Floor => a.floor_mut(),
            Self::Round => a.round_mut(),
            Self::Trunc => a.trunc_mut(),
            Self::Fract => a.fract_mut(),
            #[cfg(feature = "complex")]
            Self::Real => a.zero_imag(),
            #[cfg(feature = "complex")]
            Self::Imag => a.zero_real(),
            _ => unreachable!(),
        }
    }
    pub fn compute_on_2(self, a: &mut Number, b: Number) {
        match self {
            Self::Add => *a += b,
            Self::Sub => *a -= b,
            Self::Mul => *a *= b,
            Self::Div => *a /= b,
            Self::Rem => *a %= b,
            Self::Pow => a.pow_assign(b),
            Self::Root => a.pow_assign(b.recip()),
            Self::Tetration => a.tetration_mut(&b),
            Self::Equal => *a = Number::from(a == &b),
            Self::NotEqual => *a = Number::from(a != &b),
            Self::Greater => *a = Number::from(a.total_cmp(&b).is_gt()),
            Self::Less => *a = Number::from(a.total_cmp(&b).is_lt()),
            Self::GreaterEqual => *a = Number::from(a.total_cmp(&b).is_ge()),
            Self::LessEqual => *a = Number::from(a.total_cmp(&b).is_le()),
            Self::And => *a = Number::from(!a.is_zero() && !b.is_zero()),
            Self::Or => *a = Number::from(!a.is_zero() || !b.is_zero()),
            Self::Atan(AtanInputs::Two) => a.atan2_mut(&b),
            Self::Max => a.max_mut(&b),
            Self::Min => a.min_mut(&b),
            _ => unreachable!(),
        }
    }
    pub fn compute_on_3(self, a: &mut Number, b: Number, c: Number) {
        match self {
            Self::Quadratic => {
                let ac = mem::take(a);
                let mut poly = PolyRef(&[c, b, ac]).quadratic().into_iter();
                *a = poly.next().unwrap()
            }
            _ => unreachable!(),
        }
    }
    #[cfg(feature = "complex")]
    pub fn compute_on_4(self, a: &mut Number, b: Number, c: Number, d: Number) {
        match self {
            #[cfg(feature = "complex")]
            Self::Cubic => {
                let ac = mem::take(a);
                let mut poly = PolyRef(&[d, c, b, ac]).cubic().into_iter();
                *a = poly.next().unwrap()
            }
            _ => unreachable!(),
        }
    }
    #[cfg(feature = "complex")]
    pub fn compute_on_5(self, a: &mut Number, b: Number, c: Number, d: Number, e: Number) {
        match self {
            #[cfg(feature = "complex")]
            Self::Quartic => {
                let ac = mem::take(a);
                let mut poly = PolyRef(&[e, d, c, b, ac]).quartic().into_iter();
                *a = poly.next().unwrap()
            }
            _ => unreachable!(),
        }
    }
    pub fn compact(self) -> u8 {
        match self {
            Self::Sum
            | Self::Prod
            | Self::Iter
            | Self::Fold
            | Self::Set
            | Self::Solve
            | Self::NumericalSolve
            | Self::NumericalIntegral
            | Self::NumericalDerivative
            | Self::Modify(ModifyInputs::Two)
            | Self::NumericalDifferential => 1,
            Self::If | Self::Modify(ModifyInputs::Three) | Self::While(ModifyInputs::Two) => 2,
            Self::While(ModifyInputs::Three) => 3,
            Self::Exprs(n) => n.get(),
            _ => 0,
        }
    }
    pub fn inner_vars(self) -> u8 {
        match self {
            Self::Sum
            | Self::Prod
            | Self::Iter
            | Self::Set
            | Self::Solve
            | Self::NumericalDerivative
            | Self::NumericalSolve
            | Self::NumericalIntegral => 1,
            Self::Fold | Self::NumericalDifferential => 2,
            _ => 0,
        }
    }
    pub fn expected_var(self, n: NonZeroU8) -> bool {
        match self {
            Self::Solve => n.get() == 1,
            Self::Set | Self::NumericalDerivative | Self::NumericalSolve => n.get() == 2,
            Self::Sum | Self::Prod | Self::Iter | Self::NumericalIntegral => n.get() == 3,
            Self::Fold | Self::NumericalDifferential => matches!(n.get(), 4 | 5),
            _ => false,
        }
    }
    pub fn first_expected_var(self, n: NonZeroU8) -> bool {
        match self {
            Self::Solve => n.get() == 1,
            Self::Set | Self::NumericalDerivative | Self::NumericalSolve => n.get() == 2,
            Self::Sum | Self::Prod | Self::Iter | Self::NumericalIntegral => n.get() == 3,
            Self::Fold | Self::NumericalDifferential => n.get() == 4,
            _ => false,
        }
    }
    pub fn has_var(self) -> bool {
        matches!(
            self,
            Self::Sum
                | Self::Prod
                | Self::Iter
                | Self::Fold
                | Self::Set
                | Self::Solve
                | Self::NumericalIntegral
                | Self::NumericalDerivative
                | Self::NumericalDifferential
                | Self::NumericalSolve
        )
    }
    pub fn has_inner_fn(self) -> bool {
        matches!(
            self,
            Self::Sum
                | Self::Prod
                | Self::Iter
                | Self::Fold
                | Self::Set
                | Self::Solve
                | Self::If
                | Self::Modify(_)
                | Self::While(_)
                | Self::Exprs(_)
                | Self::NumericalIntegral
                | Self::NumericalDerivative
                | Self::NumericalDifferential
                | Self::NumericalSolve
        )
    }
    pub(crate) fn compute_var(
        self,
        compute: Compute,
        stack: &mut Tokens,
        inner_vars: &mut Vec<Number>,
    ) {
        match self {
            Self::Sum => {
                let (start, [end], [tokens]) = stack.get_skip_mut(compute.tokens);
                let start = mem::take(start);
                let start = start.to_real().into_isize();
                let end = end.to_real().into_isize();
                inner_vars.push(Number::from(start));
                *stack.last_mut().unwrap().num_mut() = (start..=end)
                    .map(|_| {
                        let ret = compute
                            .tokens(tokens)
                            .compute_buffer_with(inner_vars, stack);
                        *inner_vars.last_mut().unwrap() += Float::from(1);
                        ret
                    })
                    .sum();
                inner_vars.pop().unwrap();
            }
            Self::Prod => {
                let (start, [end], [tokens]) = stack.get_skip_mut(compute.tokens);
                let start = mem::take(start);
                let start = start.to_real().into_isize();
                let end = end.to_real().into_isize();
                inner_vars.push(Number::from(start));
                *stack.last_mut().unwrap().num_mut() = (start..=end)
                    .map(|_| {
                        let ret = compute
                            .tokens(tokens)
                            .compute_buffer_with(inner_vars, stack);
                        *inner_vars.last_mut().unwrap() += Float::from(1);
                        ret
                    })
                    .product();
                inner_vars.pop().unwrap();
            }
            Self::Fold => {
                let (start, [end, value], [tokens]) = stack.get_skip_mut(compute.tokens);
                let start = mem::take(start);
                let start = start.to_real().into_isize();
                let end = end.to_real().into_isize();
                inner_vars.push(value);
                inner_vars.push(Number::from(start));
                let nl = inner_vars.len();
                (start..=end).for_each(|_| {
                    inner_vars[nl - 2] = compute
                        .tokens(tokens)
                        .compute_buffer_with(inner_vars, stack);
                    *inner_vars.last_mut().unwrap() += Float::from(1);
                });
                inner_vars.pop().unwrap();
                *stack.last_mut().unwrap().num_mut() = inner_vars.pop().unwrap();
            }
            Self::Set => {
                let (value, [], [tokens]) = stack.get_skip_mut(compute.tokens);
                let value = mem::take(value);
                inner_vars.push(value);
                *stack.last_mut().unwrap().num_mut() = compute
                    .tokens(tokens)
                    .compute_buffer_with(inner_vars, stack);
                inner_vars.pop().unwrap();
            }
            Self::Modify(ModifyInputs::Two) => {
                let (value, [], [var]) = stack.get_skip_mut(compute.tokens);
                let value = mem::take(value);
                inner_vars[var[0].inner_var_ref() as usize] = value;
            }
            Self::Modify(ModifyInputs::Three) => {
                let (value, [], [var, tokens]) = stack.get_skip_mut(compute.tokens);
                let value = mem::take(value);
                inner_vars[var[0].inner_var_ref() as usize] = value;
                *stack.last_mut().unwrap().num_mut() = compute
                    .tokens(tokens)
                    .compute_buffer_with(inner_vars, stack);
            }
            Self::While(ModifyInputs::Two) => {
                let [cond, expr] = stack.get_skip_tokens_keep_one(compute.tokens);
                let mut last = Number::default();
                while !compute
                    .tokens(cond)
                    .compute_buffer_with(inner_vars, stack)
                    .is_zero()
                {
                    last = compute.tokens(expr).compute_buffer_with(inner_vars, stack);
                }
                *stack.last_mut().unwrap() = last.into();
            }
            Self::While(ModifyInputs::Three) => {
                let [cond, expr, ret] = stack.get_skip_tokens_keep_one(compute.tokens);
                while !compute
                    .tokens(cond)
                    .compute_buffer_with(inner_vars, stack)
                    .is_zero()
                {
                    compute.tokens(expr).compute_buffer_with(inner_vars, stack);
                }
                *stack.last_mut().unwrap() = compute
                    .tokens(ret)
                    .compute_buffer_with(inner_vars, stack)
                    .into();
            }
            Self::Exprs(n) => {
                let n = n.get();
                let mut tokens = stack
                    .get_skip_tokens_keep_one_vec(compute.tokens, n as usize)
                    .into_iter();
                for _ in 1..n {
                    compute
                        .tokens(tokens.next().unwrap())
                        .compute_buffer_with(inner_vars, stack);
                }
                let last = compute
                    .tokens(tokens.next().unwrap())
                    .compute_buffer_with(inner_vars, stack);
                *stack.last_mut().unwrap() = last.into();
            }
            Self::Solve => {
                let [tokens] = stack.get_skip_tokens_keep_one(compute.tokens);
                *stack.last_mut().unwrap() = compute
                    .tokens(tokens)
                    .get_inverse(inner_vars, stack)
                    .unwrap_or(Number::from(Constant::Nan))
                    .into();
            }
            Self::Iter => {
                let (first, [steps], [tokens]) = stack.get_skip_mut(compute.tokens);
                let first = mem::take(first);
                inner_vars.push(first);
                let steps = steps.to_real().into_isize();
                (0..steps).for_each(|_| {
                    *inner_vars.last_mut().unwrap() = compute
                        .tokens(tokens)
                        .compute_buffer_with(inner_vars, stack);
                });
                *stack.last_mut().unwrap().num_mut() = inner_vars.pop().unwrap();
            }
            Self::If => {
                let (condition, [], [ifthen, ifelse]) = stack.get_skip_mut(compute.tokens);
                let tokens = if condition.is_zero() { ifelse } else { ifthen };
                *stack.last_mut().unwrap().num_mut() =
                    stacker::maybe_grow(2usize.pow(16), 2usize.pow(20), || {
                        compute
                            .tokens(tokens)
                            .compute_buffer_with(inner_vars, stack)
                    });
            }
            Self::NumericalDerivative => {
                let (point, [], [tokens]) = stack.get_skip_mut(compute.tokens);
                let point = mem::take(point);
                inner_vars.push(Number::default());
                *stack.last_mut().unwrap().num_mut() = compute.tokens(tokens).numerical_derivative(
                    inner_vars,
                    stack,
                    point,
                    inner_vars.len() - 1,
                );
                inner_vars.pop().unwrap();
            }
            Self::NumericalIntegral => {
                let (start, [end], [tokens]) = stack.get_skip_mut(compute.tokens);
                let start = mem::take(start);
                inner_vars.push(Number::default());
                *stack.last_mut().unwrap().num_mut() = compute.tokens(tokens).numerical_integral(
                    inner_vars,
                    stack,
                    start,
                    end,
                    inner_vars.len() - 1,
                );
                inner_vars.pop().unwrap();
            }
            Self::NumericalDifferential => {
                let (x_0, [t_0, t_1], [tokens]) = stack.get_skip_mut(compute.tokens);
                let x_0 = mem::take(x_0);
                inner_vars.push(Number::default());
                inner_vars.push(Number::default());
                *stack.last_mut().unwrap().num_mut() =
                    compute.tokens(tokens).numerical_differential(
                        inner_vars,
                        stack,
                        x_0,
                        t_0,
                        t_1,
                        inner_vars.len() - 2,
                        inner_vars.len() - 1,
                    );
                inner_vars.pop().unwrap();
                inner_vars.pop().unwrap();
            }
            Self::NumericalSolve => {
                let (point, [], [tokens]) = stack.get_skip_mut(compute.tokens);
                let point = mem::take(point);
                inner_vars.push(Number::default());
                *stack.last_mut().unwrap().num_mut() = compute.tokens(tokens).numerical_solve(
                    inner_vars,
                    stack,
                    point,
                    inner_vars.len() - 1,
                );
                inner_vars.pop().unwrap();
            }
            _ => {}
        }
    }
}
